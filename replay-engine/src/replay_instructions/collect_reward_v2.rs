use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCollectRewardV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_reward = whirlpool_data.reward_infos[ix.data_reward_index as usize].mint;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  let amount_reward = ix.transfer_0.amount;

  let reward_token_trait = util::determine_token_trait(&ix.key_reward_token_program, &ix.transfer_0);

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // position_authority
  // position
  replayer.set_whirlpool_account(&ix.key_position, accounts);
  // position_token_amount
  replayer.set_token_account(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // reward_owner_account
  replayer.set_token_account_with_trait(
    reward_token_trait,
    pubkey(&ix.key_reward_owner_account),
    mint_reward,
    pubkey(&ix.key_position_authority),
    0u64
  );
  // reward_mint
  replayer.set_token_mint_with_trait(
    reward_token_trait,
    pubkey(&ix.key_reward_mint),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // reward_vault
  replayer.set_token_account_with_trait(
    reward_token_trait,
    pubkey(&ix.key_reward_vault),
    mint_reward,
    pubkey(&ix.key_whirlpool),
    amount_reward
  );
  // reward_token_program
  // memo_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::CollectRewardV2 {
      reward_index: ix.data_reward_index,
      // don't replay transfer hook
      remaining_accounts_info: None,
    },
    whirlpool_ix_accounts::CollectRewardV2 {
      whirlpool: pubkey(&ix.key_whirlpool),
      position_authority: pubkey(&ix.key_position_authority),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      reward_owner_account: pubkey(&ix.key_reward_owner_account),
      reward_mint: pubkey(&ix.key_reward_mint),
      reward_vault: pubkey(&ix.key_reward_vault),
      reward_token_program: pubkey(&ix.key_reward_token_program),
      memo_program:   pubkey(&ix.key_memo_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);
  
  let transaction_status = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);

  ReplayInstructionResult::new(transaction_status, pre_snapshot, post_snapshot)
}

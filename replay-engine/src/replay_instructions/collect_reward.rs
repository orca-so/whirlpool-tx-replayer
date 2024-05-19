use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCollectReward>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_reward = whirlpool_data.reward_infos[ix.data_reward_index as usize].mint;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  let amount_reward = ix.transfer_amount_0;

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
  replayer.set_token_account(
    pubkey(&ix.key_reward_owner_account),
    mint_reward,
    pubkey(&ix.key_position_authority),
    0u64
  );
  // reward_vault
  replayer.set_token_account(
    pubkey(&ix.key_reward_vault),
    mint_reward,
    pubkey(&ix.key_whirlpool),
    amount_reward
  );
  // token_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::CollectReward {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::CollectReward {
      whirlpool: pubkey(&ix.key_whirlpool),
      position_authority: pubkey(&ix.key_position_authority),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      reward_owner_account: pubkey(&ix.key_reward_owner_account),
      reward_vault: pubkey(&ix.key_reward_vault),
      token_program: pubkey(&ix.key_token_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

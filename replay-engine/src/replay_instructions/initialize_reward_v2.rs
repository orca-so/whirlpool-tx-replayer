use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, TokenTrait};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeRewardV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let reward_token_trait = if util::is_token_2022_program(&ix.key_reward_token_program) { TokenTrait::TokenExtensions } else { TokenTrait::Token };

  // reward_authority
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // reward_mint
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_reward_mint),
    reward_token_trait,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // reward_token_badge (no need to set because reward_mint has no extensions and freeze authority in replay)
  // reward_vault
  // reward_token_program
  // system_program
  // rent

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeRewardV2 {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::InitializeRewardV2 {
      reward_authority: pubkey(&ix.key_reward_authority),
      funder: pubkey(&ix.key_funder),
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_mint: pubkey(&ix.key_reward_mint),
      reward_vault: pubkey(&ix.key_reward_vault),
      reward_token_badge: pubkey(&ix.key_reward_token_badge),
      reward_token_program: pubkey(&ix.key_reward_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

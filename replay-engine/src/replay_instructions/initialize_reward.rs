use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeReward>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // reward_authority
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, account_map);
  // reward_mint
  replayer.set_token_mint(
    pubkey(&ix.key_reward_mint),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // reward_vault
  // token_program
  // system_program
  // rent

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeReward {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::InitializeReward {
      reward_authority: pubkey(&ix.key_reward_authority),
      funder: pubkey(&ix.key_funder),
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_mint: pubkey(&ix.key_reward_mint),
      reward_vault: pubkey(&ix.key_reward_vault),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

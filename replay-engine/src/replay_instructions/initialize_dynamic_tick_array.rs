use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeDynamicTickArray>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let mut already_initialized = false;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // tick_array
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array, accounts) {
    already_initialized = true;
  }
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeDynamicTickArray {
      start_tick_index: ix.data_start_tick_index,
      idempotent: ix.data_idempotent,
    },
    whirlpool_ix_accounts::InitializeDynamicTickArray {
      whirlpool: pubkey(&ix.key_whirlpool),
      funder: pubkey(&ix.key_funder),
      tick_array: pubkey(&ix.key_tick_array),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = if already_initialized {
    replayer.take_snapshot(
      &[&ix.key_tick_array] // already initialized
    )
  } else {
    replayer.take_snapshot(&[
    ])
  };
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_tick_array, // created or already initialized
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

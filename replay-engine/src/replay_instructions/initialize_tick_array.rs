use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeTickArray>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // tick_array
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeTickArray {
      start_tick_index: ix.data_start_tick_index,
    },
    whirlpool_ix_accounts::InitializeTickArray {
      whirlpool: pubkey(&ix.key_whirlpool),
      funder: pubkey(&ix.key_funder),
      tick_array: pubkey(&ix.key_tick_array),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let transaction_status = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_tick_array, // created
  ]);

  ReplayInstructionResult::new(transaction_status, pre_snapshot, post_snapshot)
}

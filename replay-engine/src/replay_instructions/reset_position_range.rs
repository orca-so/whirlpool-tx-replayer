use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedResetPositionRange>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  // funder
  replayer.set_funder_account(&ix.key_funder);
  // position_authority
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // position
  replayer.set_whirlpool_account(&ix.key_position, accounts);
  // position_token_account
  replayer.set_token_account(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::ResetPositionRange {
      new_tick_lower_index: ix.data_new_tick_lower_index,
      new_tick_upper_index: ix.data_new_tick_upper_index,
    },
    whirlpool_ix_accounts::ResetPositionRange {
      funder: pubkey(&ix.key_funder),
      position_authority: pubkey(&ix.key_position_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_position,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_position,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

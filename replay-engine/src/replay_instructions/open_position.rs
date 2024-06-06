use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;
use whirlpool_base::state as whirlpool_ix_bumps;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::derive_position_bump;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedOpenPosition>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // funder
  replayer.set_funder_account(&ix.key_funder);
  // owner
  // position
  // position_mint
  // position_token_account
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // token_program
  // system_program
  // rent
  // associated_token_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::OpenPosition {
      bumps: whirlpool_ix_bumps::OpenPositionBumps {
        // position_bump: after slot 189278833 this can be a dummy value, but older slots need to derive the bump
        position_bump: derive_position_bump(&pubkey(&ix.key_position_mint)),
      },
      tick_lower_index: ix.data_tick_lower_index,
      tick_upper_index: ix.data_tick_upper_index,
    },
    whirlpool_ix_accounts::OpenPosition {
      funder: pubkey(&ix.key_funder),
      owner: pubkey(&ix.key_owner),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_token_account: pubkey(&ix.key_position_token_account),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
      associated_token_program: pubkey(&ix.key_associated_token_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedOpenPositionWithTokenExtensions>) -> ReplayInstructionResult {
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
  // token_2022_program
  // system_program
  // associated_token_program
  // metadata_update_auth

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::OpenPositionWithTokenExtensions {
      tick_lower_index: ix.data_tick_lower_index,
      tick_upper_index: ix.data_tick_upper_index,
      with_token_metadata_extension: ix.data_with_token_metadata_extension,
    },
    whirlpool_ix_accounts::OpenPositionWithTokenExtensions {
      funder: pubkey(&ix.key_funder),
      owner: pubkey(&ix.key_owner),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_token_account: pubkey(&ix.key_position_token_account),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_2022_program: pubkey(&ix.key_token_2022_program),
      system_program: pubkey(&ix.key_system_program),
      associated_token_program: pubkey(&ix.key_associated_token_program),
      metadata_update_auth: pubkey(&ix.key_metadata_update_auth),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_position, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

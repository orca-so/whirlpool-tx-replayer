use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedOpenBundledPosition>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let position_bundle_data = util::get_position_bundle_data(&ix.key_position_bundle, accounts);
  let position_bundle_mint = position_bundle_data.position_bundle_mint;

  // bundled_position
  // position_bundle
  replayer.set_whirlpool_account(&ix.key_position_bundle, accounts);
  // position_bundle_token_account
  replayer.set_token_account(
    pubkey(&ix.key_position_bundle_token_account),
    position_bundle_mint,
    pubkey(&ix.key_position_bundle_authority),
    1u64
  );
  // position_bundle_authority
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // system_program
  // rent

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::OpenBundledPosition {
      bundle_index: ix.data_bundle_index,
      tick_lower_index: ix.data_tick_lower_index,
      tick_upper_index: ix.data_tick_upper_index,
    },
    whirlpool_ix_accounts::OpenBundledPosition {
      bundled_position: pubkey(&ix.key_bundled_position),
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_authority: pubkey(&ix.key_position_bundle_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      funder: pubkey(&ix.key_funder),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_position_bundle,
    &ix.key_whirlpool,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_bundled_position, // created
    &ix.key_position_bundle,
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

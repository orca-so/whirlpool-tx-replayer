use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCloseBundledPosition>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let position_bundle_data = util::get_position_bundle_data(&ix.key_position_bundle, accounts);
  let position_bundle_mint = position_bundle_data.position_bundle_mint;

  // bundled_position
  replayer.set_whirlpool_account(&ix.key_bundled_position, accounts);
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
  // receiver

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::CloseBundledPosition {
      bundle_index: ix.data_bundle_index,
    },
    whirlpool_ix_accounts::CloseBundledPosition {
      bundled_position: pubkey(&ix.key_bundled_position),
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_authority: pubkey(&ix.key_position_bundle_authority),
      receiver: pubkey(&ix.key_receiver),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_bundled_position,
    &ix.key_position_bundle,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    // closed
    &ix.key_position_bundle,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

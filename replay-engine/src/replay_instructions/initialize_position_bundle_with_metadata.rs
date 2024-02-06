use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePositionBundleWithMetadata>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let _account_map = req.account_map;

  // position_bundle
  // position_bundle_mint
  // position_bundle_metadata
  // position_bundle_token_account
  // position_bundle_owner
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // metadata_update_auth
  // token_program
  // system_program
  // rent
  // associated_token_program
  // metadata_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializePositionBundleWithMetadata {
    },
    whirlpool_ix_accounts::InitializePositionBundleWithMetadata {
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_mint: pubkey(&ix.key_position_bundle_mint),
      position_bundle_metadata: pubkey(&ix.key_position_bundle_metadata),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_owner: pubkey(&ix.key_position_bundle_owner),
      funder: pubkey(&ix.key_funder),
      metadata_update_auth: pubkey(&ix.key_metadata_update_auth),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
      associated_token_program: pubkey(&ix.key_associated_token_program),
      metadata_program: pubkey(&ix.key_metadata_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_position_bundle, // created
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

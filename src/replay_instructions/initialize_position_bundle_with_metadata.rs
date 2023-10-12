use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePositionBundleWithMetadata>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let _account_map = req.account_map;

  // position_bundle
  // position_bundle_mint
  // position_bundle_metadata
  // position_bundle_token_account
  // position_bundle_owner
  // funder
  util_replay::add_funder_account(builder, &ix.key_funder);
  // metadata_update_auth
  // token_program
  // system_program
  // rent
  // associated_token_program
  // metadata_program

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::InitializePositionBundle {
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
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
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
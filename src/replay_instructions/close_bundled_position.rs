use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCloseBundledPosition>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let position_bundle_data = util_replay::get_position_bundle_data(&ix.key_position_bundle, account_map);
  let position_bundle_mint = position_bundle_data.position_bundle_mint;

  // bundled_position
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_bundled_position, &account_map);
  // position_bundle
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_position_bundle, &account_map);
  // position_bundle_token_account
  builder.add_account_with_tokens(
    pubkey(&ix.key_position_bundle_token_account),
    position_bundle_mint,
    pubkey(&ix.key_position_bundle_authority),
    1u64
  );
  // position_bundle_authority
  // receiver

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
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
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_bundled_position,
    &ix.key_position_bundle,
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
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

use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::replay_environment;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedDeletePositionBundle>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let position_bundle_data = util_replay::get_position_bundle_data(&ix.key_position_bundle, account_map);
  let position_bundle_mint = position_bundle_data.position_bundle_mint;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // position_bundle
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_position_bundle, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_position_bundle),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_position_bundle).unwrap(),
    false,
  );
  // position_bundle_mint
  //builder.add_token_mint(
  replayer.set_token_mint(
    pubkey(&ix.key_position_bundle_mint),
    None,
    1u64,
    0u8,
    None
  );
  // position_bundle_token_account
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_position_bundle_token_account),
    position_bundle_mint,
    pubkey(&ix.key_position_bundle_owner),
    1u64
  );
  // position_bundle_owner
  // receiver
  // token_program

  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::DeletePositionBundle {
    },
    whirlpool_ix_accounts::DeletePositionBundle {
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_mint: pubkey(&ix.key_position_bundle_mint),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_owner: pubkey(&ix.key_position_bundle_owner),
      receiver: pubkey(&ix.key_receiver),
      token_program: pubkey(&ix.key_token_program),
    },
    &payer,
    latest_blockhash,
    nonce
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_position_bundle,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    // closed
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

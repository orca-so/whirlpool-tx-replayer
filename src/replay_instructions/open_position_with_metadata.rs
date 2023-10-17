use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::replay_environment;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedOpenPositionWithMetadata>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // funder
  //util_replay::add_funder_account(builder, &ix.key_funder);
  util_replay::replayer_add_funder_account(replayer, &ix.key_funder);
  // owner
  // position
  // position_mint
  // position_metadata_account
  // position_token_account
  // whirlpool
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpool),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpool).unwrap(),
    false,
  );
  // token_program
  // system_program
  // rent
  // associated_token_program
  // metadata_program
  // metadata_update_auth

  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::OpenPositionWithMetadata {
      bumps: whirlpool_base::state::OpenPositionWithMetadataBumps {
        position_bump: 0, // dummy
        metadata_bump: 0, // dummy
      },
      tick_lower_index: ix.data_tick_lower_index,
      tick_upper_index: ix.data_tick_upper_index,
    },
    whirlpool_ix_accounts::OpenPositionWithMetadata {
      funder: pubkey(&ix.key_funder),
      owner: pubkey(&ix.key_owner),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_metadata_account: pubkey(&ix.key_position_metadata_account),
      position_token_account: pubkey(&ix.key_position_token_account),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
      associated_token_program: pubkey(&ix.key_associated_token_program),
      metadata_program: pubkey(&ix.key_metadata_program),
      metadata_update_auth: pubkey(&ix.key_metadata_update_auth),
    },
    &payer,
    latest_blockhash,
    nonce
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
    &ix.key_position, // created
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

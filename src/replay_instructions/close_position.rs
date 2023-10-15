use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::util_bank;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedClosePosition>, replayer: &mut util_bank::ReplayEnvironment) -> ReplayInstructionResult {
  //let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let position_data = util_replay::get_position_data(&ix.key_position, account_map);
  let position_mint = position_data.position_mint;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // position_authority
  // receiver
  // position
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_position, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_position),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_position).unwrap(),
    false,
  );
  // position_mint
  //builder.add_token_mint(
  replayer.set_token_mint(
    pubkey(&ix.key_position_mint),
    None,
    1u64,
    0u8,
    None
  );
  // position_token_amount
  //builder.add_account_with_tokens(
  replayer.set_account_with_tokens(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // token_program

  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::ClosePosition {
    },
    whirlpool_ix_accounts::ClosePosition {
      position_authority: pubkey(&ix.key_position_authority),
      receiver: pubkey(&ix.key_receiver),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_token_account: pubkey(&ix.key_position_token_account),
      token_program: pubkey(&ix.key_token_program),
    },
    &payer,
    latest_blockhash,
    nonce
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_position,
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

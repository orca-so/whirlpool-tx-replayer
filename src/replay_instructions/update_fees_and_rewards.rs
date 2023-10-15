use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::util_bank;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedUpdateFeesAndRewards>, replayer: &mut util_bank::ReplayEnvironment) -> ReplayInstructionResult {
  //let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // whirlpool
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpool),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpool).unwrap(),
    false,
  );
  // position
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_position, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_position),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_position).unwrap(),
    false,
  );
  // tick_array_lower
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_lower, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_tick_array_lower),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_tick_array_lower).unwrap(),
    false,
  );
  // tick_array_upper
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_upper, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_tick_array_upper),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_tick_array_upper).unwrap(),
    false,
  );

  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::UpdateFeesAndRewards {
    },
    whirlpool_ix_accounts::UpdateFeesAndRewards {
      whirlpool: pubkey(&ix.key_whirlpool),
      position: pubkey(&ix.key_position),
      tick_array_lower: pubkey(&ix.key_tick_array_lower),
      tick_array_upper: pubkey(&ix.key_tick_array_upper),
    },
    &payer,
    latest_blockhash,
    nonce,
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

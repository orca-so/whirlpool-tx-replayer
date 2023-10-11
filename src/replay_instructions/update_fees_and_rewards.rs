use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSwap>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let mint_a_is_input = ix.data_a_to_b;
  let mint_b_is_input = !mint_a_is_input;
  let input_amount = ix.transfer_amount_0;
  let output_amount = ix.transfer_amount_1;

  // token_program
  // token_authority
  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // token_owner_account_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_a),
    mint_a,
    pubkey(&ix.key_token_authority),
    if mint_a_is_input { input_amount } else { 0u64 }
  );
  // vault_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    if mint_a_is_input { 0u64 } else { output_amount }
  );
  // token_owner_account_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_b),
    mint_b,
    pubkey(&ix.key_token_authority),
    if mint_b_is_input { input_amount } else { 0u64 }
  );
  // vault_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    if mint_b_is_input { 0u64 } else { output_amount }
  );
  // tick_array_0
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_0, &account_map);
  // tick_array_1
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_1, &account_map);
  // tick_array_2
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_2, &account_map);
  // oracle

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::Swap {
      amount: ix.data_amount,
      other_amount_threshold: ix.data_other_amount_threshold,
      sqrt_price_limit: ix.data_sqrt_price_limit,
      amount_specified_is_input: ix.data_amount_specified_is_input,
      a_to_b: ix.data_a_to_b,
    },
    whirlpool_ix_accounts::Swap {
      token_program: pubkey(&ix.key_token_program),
      token_authority: pubkey(&ix.key_token_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_owner_account_a: pubkey(&ix.key_token_owner_account_a),
      token_vault_a: pubkey(&ix.key_vault_a),
      token_owner_account_b: pubkey(&ix.key_token_owner_account_b),
      token_vault_b: pubkey(&ix.key_vault_b),
      tick_array_0: pubkey(&ix.key_tick_array_0),
      tick_array_1: pubkey(&ix.key_tick_array_1),
      tick_array_2: pubkey(&ix.key_tick_array_2),
      oracle: pubkey(&ix.key_oracle),
    },
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_tick_array_0,
    &ix.key_tick_array_1,
    &ix.key_tick_array_2,
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_tick_array_0,
    &ix.key_tick_array_1,
    &ix.key_tick_array_2,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

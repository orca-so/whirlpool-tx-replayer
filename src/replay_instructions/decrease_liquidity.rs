use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedDecreaseLiquidity>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let position_data = util_replay::get_position_data(&ix.key_position, account_map);
  let position_mint = position_data.position_mint;

  let amount_a = ix.transfer_amount_0;
  let amount_b = ix.transfer_amount_1;

  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // token_program
  // position_authority
  // position
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_position, &account_map);
  // position_token_amount
  builder.add_account_with_tokens(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // token_owner_account_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_a),
    mint_a,
    pubkey(&ix.key_position_authority),
    amount_a
  );
  // token_owner_account_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_b),
    mint_b,
    pubkey(&ix.key_position_authority),
    amount_b
  );
  // token_vault_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    0u64
  );
  // token_vault_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    0u64
  );
  // tick_array_lower
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_lower, &account_map);
  // tick_array_upper
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array_upper, &account_map);

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::DecreaseLiquidity {
      liquidity_amount: ix.data_liquidity_amount,
      token_min_a: ix.data_token_amount_min_a,
      token_min_b: ix.data_token_amount_min_b,
    },
    whirlpool_ix_accounts::ModifyLiquidity {
      whirlpool: pubkey(&ix.key_whirlpool),
      token_program: pubkey(&ix.key_token_program),
      position_authority: pubkey(&ix.key_position_authority),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      token_owner_account_a: pubkey(&ix.key_token_owner_account_a),
      token_owner_account_b: pubkey(&ix.key_token_owner_account_b),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      tick_array_lower: pubkey(&ix.key_tick_array_lower),
      tick_array_upper: pubkey(&ix.key_tick_array_upper),
    },
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
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

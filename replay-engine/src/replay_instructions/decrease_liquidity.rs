use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedDecreaseLiquidity>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  let amount_a = ix.transfer_amount_0;
  let amount_b = ix.transfer_amount_1;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // token_program
  // position_authority
  // position
  replayer.set_whirlpool_account(&ix.key_position, accounts);
  // position_token_amount
  replayer.set_token_account(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // token_owner_account_a
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_a),
    mint_a,
    pubkey(&ix.key_position_authority),
    0u64
  );
  // token_owner_account_b
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_b),
    mint_b,
    pubkey(&ix.key_position_authority),
    0u64
  );
  // token_vault_a
  replayer.set_token_account(
    pubkey(&ix.key_token_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    amount_a
  );
  // token_vault_b
  replayer.set_token_account(
    pubkey(&ix.key_token_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    amount_b
  );
  // tick_array_lower
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_tick_array_lower, accounts);
  // tick_array_upper
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_tick_array_upper, accounts);

  let tx = replayer.build_whirlpool_replay_transaction(
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
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

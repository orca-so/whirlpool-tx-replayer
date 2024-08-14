use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedTwoHopSwap>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_one_data = util::get_whirlpool_data(&ix.key_whirlpool_one, accounts);
  let whirlpool_two_data = util::get_whirlpool_data(&ix.key_whirlpool_two, accounts);
  let mint_one_a = whirlpool_one_data.token_mint_a;
  let mint_one_b = whirlpool_one_data.token_mint_b;
  let mint_two_a = whirlpool_two_data.token_mint_a;
  let mint_two_b = whirlpool_two_data.token_mint_b;

  let input_mint = if ix.data_a_to_b_one { mint_one_a } else { mint_one_b };
  let output_mint = if ix.data_a_to_b_two { mint_two_b } else { mint_two_a };

  let input_token_owner_account = if ix.data_a_to_b_one { ix.key_token_owner_account_one_a.clone() } else { ix.key_token_owner_account_one_b.clone() };

  let input_amount = ix.transfer_amount_0;
  let intermediate_amount = ix.transfer_amount_1;
  let output_amount = ix.transfer_amount_3;

  // token_program
  // token_authority
  // whirlpool_one
  replayer.set_whirlpool_account(&ix.key_whirlpool_one, accounts);
  // whirlpool_two
  replayer.set_whirlpool_account(&ix.key_whirlpool_two, accounts);
  // token_owner_account_one_a
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_one_a),
    mint_one_a,
    pubkey(&ix.key_token_authority),
    if ix.key_token_owner_account_one_a == input_token_owner_account { input_amount } else { 0u64 }
  );
  // vault_one_a
  replayer.set_token_account(
    pubkey(&ix.key_vault_one_a),
    mint_one_a,
    pubkey(&ix.key_whirlpool_one),
    if mint_one_a == input_mint { 0u64 } else { intermediate_amount }
  );
  // token_owner_account_one_b
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_one_b),
    mint_one_b,
    pubkey(&ix.key_token_authority),
    if ix.key_token_owner_account_one_b == input_token_owner_account { input_amount } else { 0u64 }
  );
  // vault_one_b
  replayer.set_token_account(
    pubkey(&ix.key_vault_one_b),
    mint_one_b,
    pubkey(&ix.key_whirlpool_one),
    if mint_one_b == input_mint { 0u64 } else { intermediate_amount }
  );
  // token_owner_account_two_a
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_two_a),
    mint_two_a,
    pubkey(&ix.key_token_authority),
    if ix.key_token_owner_account_two_a == input_token_owner_account { input_amount } else { 0u64 }
  );
  // vault_two_a
  replayer.set_token_account(
    pubkey(&ix.key_vault_two_a),
    mint_two_a,
    pubkey(&ix.key_whirlpool_two),
    if mint_two_a == output_mint { output_amount } else { 0u64 }
  );
  // token_owner_account_two_b
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_two_b),
    mint_two_b,
    pubkey(&ix.key_token_authority),
    if ix.key_token_owner_account_two_b == input_token_owner_account { input_amount } else { 0u64 }
  );
  // vault_two_b
  replayer.set_token_account(
    pubkey(&ix.key_vault_two_b),
    mint_two_b,
    pubkey(&ix.key_whirlpool_two),
    if mint_two_b == output_mint { output_amount } else { 0u64 }
  );
  // tick_array_one_0
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_0, accounts);
  // tick_array_one_1
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_1, accounts);
  // tick_array_one_2
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_2, accounts);
  // tick_array_two_0
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_0, accounts);
  // tick_array_two_1
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_1, accounts);
  // tick_array_two_2
  replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_2, accounts);
  // oracle_one
  // oracle_two

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::TwoHopSwap {
      amount: ix.data_amount,
      other_amount_threshold: ix.data_other_amount_threshold,
      sqrt_price_limit_one: ix.data_sqrt_price_limit_one,
      sqrt_price_limit_two: ix.data_sqrt_price_limit_two,
      amount_specified_is_input: ix.data_amount_specified_is_input,
      a_to_b_one: ix.data_a_to_b_one,
      a_to_b_two: ix.data_a_to_b_two,
    },
    whirlpool_ix_accounts::TwoHopSwap {
      token_program: pubkey(&ix.key_token_program),
      token_authority: pubkey(&ix.key_token_authority),
      whirlpool_one: pubkey(&ix.key_whirlpool_one),
      whirlpool_two: pubkey(&ix.key_whirlpool_two),
      token_owner_account_one_a: pubkey(&ix.key_token_owner_account_one_a),
      token_vault_one_a: pubkey(&ix.key_vault_one_a),
      token_owner_account_one_b: pubkey(&ix.key_token_owner_account_one_b),
      token_vault_one_b: pubkey(&ix.key_vault_one_b),
      token_owner_account_two_a: pubkey(&ix.key_token_owner_account_two_a),
      token_vault_two_a: pubkey(&ix.key_vault_two_a),
      token_owner_account_two_b: pubkey(&ix.key_token_owner_account_two_b),
      token_vault_two_b: pubkey(&ix.key_vault_two_b),
      tick_array_one_0: pubkey(&ix.key_tick_array_one_0),
      tick_array_one_1: pubkey(&ix.key_tick_array_one_1),
      tick_array_one_2: pubkey(&ix.key_tick_array_one_2),
      tick_array_two_0: pubkey(&ix.key_tick_array_two_0),
      tick_array_two_1: pubkey(&ix.key_tick_array_two_1),
      tick_array_two_2: pubkey(&ix.key_tick_array_two_2),
      oracle_one: pubkey(&ix.key_oracle_one),
      oracle_two: pubkey(&ix.key_oracle_two),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool_one,
    &ix.key_whirlpool_two,
    &ix.key_tick_array_one_0,
    &ix.key_tick_array_one_1,
    &ix.key_tick_array_one_2,
    &ix.key_tick_array_two_0,
    &ix.key_tick_array_two_1,
    &ix.key_tick_array_two_2,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool_one,
    &ix.key_whirlpool_two,
    &ix.key_tick_array_one_0,
    &ix.key_tick_array_one_1,
    &ix.key_tick_array_one_2,
    &ix.key_tick_array_two_0,
    &ix.key_tick_array_two_1,
    &ix.key_tick_array_two_2,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

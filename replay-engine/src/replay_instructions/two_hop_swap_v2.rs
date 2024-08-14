use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedTwoHopSwapV2>) -> ReplayInstructionResult {
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
  let intermediate_mint = if ix.data_a_to_b_one { mint_one_b } else { mint_one_a };
  let output_mint = if ix.data_a_to_b_two { mint_two_b } else { mint_two_a };

  let input_amount = ix.transfer_0.amount;
  let intermediate_amount = ix.transfer_1.amount;
  let output_amount = ix.transfer_2.amount;

  let input_token_trait = util::determine_token_trait(&ix.key_token_program_input, &ix.transfer_0);
  let intermediate_token_trait = util::determine_token_trait(&ix.key_token_program_intermediate, &ix.transfer_1);
  let output_token_trait = util::determine_token_trait(&ix.key_token_program_output, &ix.transfer_2);

  let mut writable_accounts = vec![];

  // whirlpool_one
  replayer.set_whirlpool_account(&ix.key_whirlpool_one, accounts);
  // whirlpool_two
  replayer.set_whirlpool_account(&ix.key_whirlpool_two, accounts);
  // token_mint_input
  replayer.set_token_mint_with_trait(
    input_mint,
    input_token_trait,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_intermediate
  replayer.set_token_mint_with_trait(
    intermediate_mint,
    intermediate_token_trait,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_output
  replayer.set_token_mint_with_trait(
    output_mint,
    output_token_trait,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_program_input
  // token_program_intermediate
  // token_program_output
  // token_owner_account_input
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_input),
    input_token_trait,
    input_mint,
    pubkey(&ix.key_token_authority),
    input_amount,
  );
  // token_vault_one_input
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_one_input),
    input_token_trait,
    input_mint,
    pubkey(&ix.key_whirlpool_one),
    0u64,
  );
  // token_vault_one_intermediate
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_one_intermediate),
    intermediate_token_trait,
    intermediate_mint,
    pubkey(&ix.key_whirlpool_one),
    intermediate_amount,
  );
  // token_vault_two_intermediate
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_two_intermediate),
    intermediate_token_trait,
    intermediate_mint,
    pubkey(&ix.key_whirlpool_two),
    0u64,
  );
  // token_vault_two_output
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_two_output),
    output_token_trait,
    output_mint,
    pubkey(&ix.key_whirlpool_two),
    output_amount,
  );
  // token_owner_account_output
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_output),
    output_token_trait,
    output_mint,
    pubkey(&ix.key_token_authority),
    0u64,
  );
  // token_authority
  // tick_array_one_0
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_0, accounts) {
    writable_accounts.push(&ix.key_tick_array_one_0);
  }
  // tick_array_one_1
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_1, accounts) {
    writable_accounts.push(&ix.key_tick_array_one_1);
  }
  // tick_array_one_2
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_one_2, accounts) {
    writable_accounts.push(&ix.key_tick_array_one_2);
  }
  // tick_array_two_0
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_0, accounts) {
    writable_accounts.push(&ix.key_tick_array_two_0);
  }
  // tick_array_two_1
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_1, accounts) {
    writable_accounts.push(&ix.key_tick_array_two_1);
  }
  // tick_array_two_2
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_two_2, accounts) {
    writable_accounts.push(&ix.key_tick_array_two_2);
  }
  // oracle_one
  // oracle_two
  // memo_program

  // remaining_accounts (SupplementalTickArraysOne)
  let supplemental_tick_arrays_one = util::get_remaining_accounts(
    &ix.remaining_accounts_info,
    &ix.remaining_accounts_keys,
    whirlpool_base::util::remaining_accounts_utils::AccountsType::SupplementalTickArraysOne,
  );
  for supplemental_tick_array in &supplemental_tick_arrays_one {
    if replayer.set_whirlpool_account_if_exists(supplemental_tick_array, accounts) {
      writable_accounts.push(supplemental_tick_array);
    }
  }
  // remaining_accounts (SupplementalTickArraysTwo)
  let supplemental_tick_arrays_two = util::get_remaining_accounts(
    &ix.remaining_accounts_info,
    &ix.remaining_accounts_keys,
    whirlpool_base::util::remaining_accounts_utils::AccountsType::SupplementalTickArraysTwo,
  );
  for supplemental_tick_array in &supplemental_tick_arrays_two {
    if replayer.set_whirlpool_account_if_exists(supplemental_tick_array, accounts) {
      writable_accounts.push(supplemental_tick_array);
    }
  }

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::TwoHopSwapV2 {
      amount: ix.data_amount,
      other_amount_threshold: ix.data_other_amount_threshold,
      sqrt_price_limit_one: ix.data_sqrt_price_limit_one,
      sqrt_price_limit_two: ix.data_sqrt_price_limit_two,
      amount_specified_is_input: ix.data_amount_specified_is_input,
      a_to_b_one: ix.data_a_to_b_one,
      a_to_b_two: ix.data_a_to_b_two,
      // don't replay transfer hook
      // revisit if additional tickarrays is supported
      remaining_accounts_info: None,
    },
    whirlpool_ix_accounts::TwoHopSwapV2 {
      whirlpool_one: pubkey(&ix.key_whirlpool_one),
      whirlpool_two: pubkey(&ix.key_whirlpool_two),
      token_mint_input: pubkey(&ix.key_token_mint_input),
      token_mint_intermediate: pubkey(&ix.key_token_mint_intermediate),
      token_mint_output: pubkey(&ix.key_token_mint_output),
      token_program_input: pubkey(&ix.key_token_program_input),
      token_program_intermediate: pubkey(&ix.key_token_program_intermediate),
      token_program_output: pubkey(&ix.key_token_program_output),
      token_owner_account_input: pubkey(&ix.key_token_owner_account_input),
      token_vault_one_input: pubkey(&ix.key_vault_one_input),
      token_vault_one_intermediate: pubkey(&ix.key_vault_one_intermediate),
      token_vault_two_intermediate: pubkey(&ix.key_vault_two_intermediate),
      token_vault_two_output: pubkey(&ix.key_vault_two_output),
      token_owner_account_output: pubkey(&ix.key_token_owner_account_output),
      token_authority: pubkey(&ix.key_token_authority),
      tick_array_one_0: pubkey(&ix.key_tick_array_one_0),
      tick_array_one_1: pubkey(&ix.key_tick_array_one_1),
      tick_array_one_2: pubkey(&ix.key_tick_array_one_2),
      tick_array_two_0: pubkey(&ix.key_tick_array_two_0),
      tick_array_two_1: pubkey(&ix.key_tick_array_two_1),
      tick_array_two_2: pubkey(&ix.key_tick_array_two_2),
      oracle_one: pubkey(&ix.key_oracle_one),
      oracle_two: pubkey(&ix.key_oracle_two),
      memo_program: pubkey(&ix.key_memo_program),
    },
  );

  writable_accounts.dedup();
  writable_accounts.push(&ix.key_whirlpool_one);
  writable_accounts.push(&ix.key_whirlpool_two);

  let pre_snapshot = replayer.take_snapshot(
    &writable_accounts,
  );

  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(
    &writable_accounts,
  );

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

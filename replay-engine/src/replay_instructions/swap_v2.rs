use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSwapV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let mint_a_is_input = ix.data_a_to_b;
  let mint_b_is_input = !mint_a_is_input;
  let input_amount = ix.transfer_0.amount;
  let output_amount = ix.transfer_1.amount;

  let token_trait_a = util::determine_token_trait(&ix.key_token_program_a, if mint_a_is_input { &ix.transfer_0 } else { &ix.transfer_1 });
  let token_trait_b = util::determine_token_trait(&ix.key_token_program_b, if mint_b_is_input { &ix.transfer_0 } else { &ix.transfer_1 });

  let mut writable_accounts = vec![];

  // token_program_a
  // token_program_b
  // memo_program
  // token_authority
  // whirlpool
  // token_mint_a
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_token_mint_a),
    token_trait_a,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_b
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_token_mint_b),
    token_trait_b,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // token_owner_account_a
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_a),
    token_trait_a,
    mint_a,
    pubkey(&ix.key_token_authority),
    if mint_a_is_input { input_amount } else { 0u64 }
  );
  // vault_a
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_a),
    token_trait_a,
    mint_a,
    pubkey(&ix.key_whirlpool),
    if mint_a_is_input { 0u64 } else { output_amount }
  );
  // token_owner_account_b
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_b),
    token_trait_b,
    mint_b,
    pubkey(&ix.key_token_authority),
    if mint_b_is_input { input_amount } else { 0u64 }
  );
  // vault_b
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_vault_b),
    token_trait_b,
    mint_b,
    pubkey(&ix.key_whirlpool),
    if mint_b_is_input { 0u64 } else { output_amount }
  );
  // tick_array_0
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_0, accounts) {
    writable_accounts.push(&ix.key_tick_array_0);
  }
  // tick_array_1
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_1, accounts) {
    writable_accounts.push(&ix.key_tick_array_1);
  }
  // tick_array_2
  if replayer.set_whirlpool_account_if_exists(&ix.key_tick_array_2, accounts) {
    writable_accounts.push(&ix.key_tick_array_2);
  }
  // oracle

  // remaining_accounts (SupplementalTickArrays)
  let supplemental_tick_arrays = util::get_remaining_accounts(
    &ix.remaining_accounts_info,
    &ix.remaining_accounts_keys,
    whirlpool_base::util::remaining_accounts_utils::AccountsType::SupplementalTickArrays,
  );
  for supplemental_tick_array in &supplemental_tick_arrays {
    if replayer.set_whirlpool_account_if_exists(supplemental_tick_array, accounts) {
      writable_accounts.push(supplemental_tick_array);
    }
  }

  let (remaining_accounts_info, remaining_account_metas) = util::build_swap_v2_remaining_accounts(
    &supplemental_tick_arrays,
  );

  let tx = replayer.build_whirlpool_replay_transaction_with_remaining_accounts(
      whirlpool_ix_args::SwapV2 {
      amount: ix.data_amount,
      other_amount_threshold: ix.data_other_amount_threshold,
      sqrt_price_limit: ix.data_sqrt_price_limit,
      amount_specified_is_input: ix.data_amount_specified_is_input,
      a_to_b: ix.data_a_to_b,
      // don't replay transfer hook
      remaining_accounts_info: remaining_accounts_info,
    },
    whirlpool_ix_accounts::SwapV2 {
      token_program_a: pubkey(&ix.key_token_program_a),
      token_program_b: pubkey(&ix.key_token_program_b),
      memo_program: pubkey(&ix.key_memo_program),
      token_authority: pubkey(&ix.key_token_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      token_owner_account_a: pubkey(&ix.key_token_owner_account_a),
      token_vault_a: pubkey(&ix.key_vault_a),
      token_owner_account_b: pubkey(&ix.key_token_owner_account_b),
      token_vault_b: pubkey(&ix.key_vault_b),
      tick_array_0: pubkey(&ix.key_tick_array_0),
      tick_array_1: pubkey(&ix.key_tick_array_1),
      tick_array_2: pubkey(&ix.key_tick_array_2),
      oracle: pubkey(&ix.key_oracle),
    },
    remaining_account_metas
  );

  writable_accounts.dedup();
  writable_accounts.push(&ix.key_whirlpool);

  let pre_snapshot = replayer.take_snapshot(
    &writable_accounts,
  );

  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(
    &writable_accounts,
  );

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

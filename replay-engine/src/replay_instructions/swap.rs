use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSwap>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let mint_a_is_input = ix.data_a_to_b;
  let mint_b_is_input = !mint_a_is_input;
  let input_amount = ix.transfer_amount_0;
  let output_amount = ix.transfer_amount_1;

  let mut writable_accounts = vec![];

  // token_program
  // token_authority
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // token_owner_account_a
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_a),
    mint_a,
    pubkey(&ix.key_token_authority),
    if mint_a_is_input { input_amount } else { 0u64 }
  );
  // vault_a
  replayer.set_token_account(
    pubkey(&ix.key_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    if mint_a_is_input { 0u64 } else { output_amount }
  );
  // token_owner_account_b
  replayer.set_token_account(
    pubkey(&ix.key_token_owner_account_b),
    mint_b,
    pubkey(&ix.key_token_authority),
    if mint_b_is_input { input_amount } else { 0u64 }
  );
  // vault_b
  replayer.set_token_account(
    pubkey(&ix.key_vault_b),
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
  if replayer.set_whirlpool_account_if_exists(&ix.key_oracle, accounts) {
    writable_accounts.push(&ix.key_oracle);
  }

  // HACK: make oracle account writable
  let remaining_account_metas = vec![
    solana_program::instruction::AccountMeta::new(pubkey(&ix.key_oracle), false),
  ];

  let tx = replayer.build_whirlpool_replay_transaction_with_remaining_accounts(
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
    remaining_account_metas,
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

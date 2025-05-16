use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, TokenTrait};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePoolWithAdaptiveFee>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let token_trait_a = if util::is_token_2022_program(&ix.key_token_program_a) { TokenTrait::TokenExtensions } else { TokenTrait::Token };
  let token_trait_b = if util::is_token_2022_program(&ix.key_token_program_b) { TokenTrait::TokenExtensions } else { TokenTrait::Token };

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
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
  // token_badge_a (no need to set because token_mint_a has no extensions and freeze authority in replay)
  // token_badge_b (no need to set because token_mint_b has no extensions and freeze authority in replay)
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // initialize_pool_authority
  // whirlpool
  // oracle
  // token_vault_a
  // token_vault_b
  // adaptive_fee_tier
  replayer.set_whirlpool_account(&ix.key_adaptive_fee_tier, accounts);
  // token_program_a
  // token_program_b
  // system_program
  // rent

  let trade_enable_timestamp = if ix.data_trade_enable_timestamp == 0 {
    None
  } else {
    Some(ix.data_trade_enable_timestamp)
  };

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializePoolWithAdaptiveFee {
      initial_sqrt_price: ix.data_initial_sqrt_price,
      trade_enable_timestamp,
    },
    whirlpool_ix_accounts::InitializePoolWithAdaptiveFee {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      token_badge_a: pubkey(&ix.key_token_badge_a),
      token_badge_b: pubkey(&ix.key_token_badge_b),
      funder: pubkey(&ix.key_funder),
      initialize_pool_authority: pubkey(&ix.key_initialize_pool_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      oracle: pubkey(&ix.key_oracle),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      adaptive_fee_tier: pubkey(&ix.key_adaptive_fee_tier),
      token_program_a: pubkey(&ix.key_token_program_a),
      token_program_b: pubkey(&ix.key_token_program_b),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool, // created
    &ix.key_oracle, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

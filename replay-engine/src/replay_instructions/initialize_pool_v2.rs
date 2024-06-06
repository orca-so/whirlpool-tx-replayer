use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, TokenTrait};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePoolV2>) -> ReplayInstructionResult {
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
  // whirlpool
  // token_vault_a
  // token_vault_b
  // fee_tier
  replayer.set_whirlpool_account(&ix.key_fee_tier, accounts);
  // token_program_a
  // token_program_b
  // system_program
  // rent

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializePoolV2 {
      initial_sqrt_price: ix.data_initial_sqrt_price,
      tick_spacing: ix.data_tick_spacing,
    },
    whirlpool_ix_accounts::InitializePoolV2 {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      token_badge_a: pubkey(&ix.key_token_badge_a),
      token_badge_b: pubkey(&ix.key_token_badge_b),
      funder: pubkey(&ix.key_funder),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      fee_tier: pubkey(&ix.key_fee_tier),
      token_program_a: pubkey(&ix.key_token_program_a),
      token_program_b: pubkey(&ix.key_token_program_b),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_fee_tier,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_fee_tier,
    &ix.key_whirlpool, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

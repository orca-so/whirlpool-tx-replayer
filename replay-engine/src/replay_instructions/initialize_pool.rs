use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util::derive_whirlpool_bump;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePool>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, account_map);
  // token_mint_a
  replayer.set_token_mint(
    pubkey(&ix.key_token_mint_a),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_b
  replayer.set_token_mint(
    pubkey(&ix.key_token_mint_b),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // whirlpool
  // token_vault_a
  // token_vault_b
  // fee_tier
  replayer.set_whirlpool_account(&ix.key_fee_tier, account_map);
  // token_program
  // system_program
  // rent

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializePool {
      bumps: whirlpool_base::state::WhirlpoolBumps {
        // whirlpool_bump: after slot 189278833 this can be a dummy value, but older slots need to derive the bump
        whirlpool_bump: derive_whirlpool_bump(
          &pubkey(&ix.key_whirlpools_config),
          &pubkey(&ix.key_token_mint_a),
          &pubkey(&ix.key_token_mint_b),
          ix.data_tick_spacing,
        ),
      },
      initial_sqrt_price: ix.data_initial_sqrt_price,
      tick_spacing: ix.data_tick_spacing,
    },
    whirlpool_ix_accounts::InitializePool {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      funder: pubkey(&ix.key_funder),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      fee_tier: pubkey(&ix.key_fee_tier),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_fee_tier,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_fee_tier,
    &ix.key_whirlpool, // created
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::util_bank;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePool>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // whirlpools_config
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpools_config, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpools_config),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpools_config).unwrap(),
    false,
  );
  // token_mint_a
  //builder.add_token_mint(
  replayer.set_token_mint(
    pubkey(&ix.key_token_mint_a),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_b
  //builder.add_token_mint(
  replayer.set_token_mint(
    pubkey(&ix.key_token_mint_b),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // funder
  //util_replay::add_funder_account(builder, &ix.key_funder);
  util_replay::replayer_add_funder_account(replayer, &ix.key_funder);
  // whirlpool
  // token_vault_a
  // token_vault_b
  // fee_tier
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_fee_tier, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_fee_tier),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_fee_tier).unwrap(),
    false,
  );
  // token_program
  // system_program
  // rent

  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::InitializePool {
      bumps: whirlpool_base::state::WhirlpoolBumps {
        whirlpool_bump: 0, // dummy
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
    &payer,
    latest_blockhash,
    nonce
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpools_config,
    &ix.key_fee_tier,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
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

use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeReward>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // reward_authority
  // funder
  util_replay::add_funder_account(builder, &ix.key_funder);
  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // reward_mint
  builder.add_token_mint(
    pubkey(&ix.key_reward_mint),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // reward_vault
  // token_program
  // system_program
  // rent

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::InitializeReward {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::InitializeReward {
      reward_authority: pubkey(&ix.key_reward_authority),
      funder: pubkey(&ix.key_funder),
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_mint: pubkey(&ix.key_reward_mint),
      reward_vault: pubkey(&ix.key_reward_vault),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
    },
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

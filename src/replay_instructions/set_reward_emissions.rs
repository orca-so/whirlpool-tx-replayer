use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetRewardEmissions>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_reward = whirlpool_data.reward_infos[ix.data_reward_index as usize].mint;

  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // reward_authority
  // reward_vault
  builder.add_account_with_tokens(
    pubkey(&ix.key_reward_vault),
    mint_reward,
    pubkey(&ix.key_whirlpool),
    u64::MAX // dummy
  );
    
  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::SetRewardEmissions {
      reward_index: ix.data_reward_index,
      emissions_per_second_x64: ix.data_emissions_per_second_x64,
    },
    whirlpool_ix_accounts::SetRewardEmissions {
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_authority: pubkey(&ix.key_reward_authority),
      reward_vault: pubkey(&ix.key_reward_vault),
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

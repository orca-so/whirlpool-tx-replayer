use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetRewardEmissionsSuperAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, account_map);
  // reward_emissions_super_authority
  // new_reward_emissions_super_authority

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetRewardEmissionsSuperAuthority {    
    },
    whirlpool_ix_accounts::SetRewardEmissionsSuperAuthority {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      reward_emissions_super_authority: pubkey(&ix.key_reward_emissions_super_authority),
      new_reward_emissions_super_authority: pubkey(&ix.key_new_reward_emissions_super_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

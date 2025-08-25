use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::decoded_instructions::ConfigFeatureFlag;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetConfigFeatureFlag>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let feature_flag = match ix.data_feature_flag {
    ConfigFeatureFlag::TokenBadge { enabled } => whirlpool_base::state::ConfigFeatureFlag::TokenBadge(enabled),
  };

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // authority
    
  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetConfigFeatureFlag {
      feature_flag,
    },
    whirlpool_ix_accounts::SetConfigFeatureFlag {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      authority: pubkey(&ix.key_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

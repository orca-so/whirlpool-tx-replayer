use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetInitializePoolAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // adaptive_fee_tier
  replayer.set_whirlpool_account(&ix.key_adaptive_fee_tier, accounts);
  // fee_authority
  // new_initialize_pool_authority

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetInitializePoolAuthority {
    },
    whirlpool_ix_accounts::SetInitializePoolAuthority {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      adaptive_fee_tier: pubkey(&ix.key_adaptive_fee_tier),
      fee_authority: pubkey(&ix.key_fee_authority),
      new_initialize_pool_authority: pubkey(&ix.key_new_initialize_pool_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_adaptive_fee_tier,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_adaptive_fee_tier,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

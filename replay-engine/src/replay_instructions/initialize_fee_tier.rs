use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeFeeTier>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // fee_tier
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // fee_authority
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeFeeTier {
      tick_spacing: ix.data_tick_spacing,
      default_fee_rate: ix.data_default_fee_rate,
    },
    whirlpool_ix_accounts::InitializeFeeTier {
      config: pubkey(&ix.key_whirlpools_config),
      fee_tier: pubkey(&ix.key_fee_tier),
      funder: pubkey(&ix.key_funder),
      fee_authority: pubkey(&ix.key_fee_authority),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_fee_tier, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

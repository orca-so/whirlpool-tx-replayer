use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeAdaptiveFeeTier>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // adaptive_fee_tier
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // fee_authority
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeAdaptiveFeeTier {
      fee_tier_index: ix.data_fee_tier_index,
      tick_spacing: ix.data_tick_spacing,
      initialize_pool_authority: pubkey(&ix.data_initialize_pool_authority),
      delegated_fee_authority: pubkey(&ix.data_delegated_fee_authority),
      default_base_fee_rate: ix.data_default_base_fee_rate,
      filter_period: ix.data_filter_period,
      decay_period: ix.data_decay_period,
      reduction_factor: ix.data_reduction_factor,
      adaptive_fee_control_factor: ix.data_adaptive_fee_control_factor,
      max_volatility_accumulator: ix.data_max_volatility_accumulator,
      tick_group_size: ix.data_tick_group_size,
      major_swap_threshold_ticks: ix.data_major_swap_threshold_ticks,
    },
    whirlpool_ix_accounts::InitializeAdaptiveFeeTier {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      adaptive_fee_tier: pubkey(&ix.key_adaptive_fee_tier),
      funder: pubkey(&ix.key_funder),
      fee_authority: pubkey(&ix.key_fee_authority),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_adaptive_fee_tier, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

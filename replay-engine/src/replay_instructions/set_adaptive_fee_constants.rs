use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

use anchor_lang::{InstructionData, ToAccountMetas, Discriminator, AnchorSerialize};
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize)]
struct SetAdaptiveFeeConstantsInstructionArgs {
  pub filter_period: Option<u16>,
  pub decay_period: Option<u16>,
  pub reduction_factor: Option<u16>,
  pub adaptive_fee_control_factor: Option<u32>,
  pub max_volatility_accumulator: Option<u32>,
  pub tick_group_size: Option<u16>,
  pub major_swap_threshold_ticks: Option<u16>,
}
impl Discriminator for SetAdaptiveFeeConstantsInstructionArgs {
  const DISCRIMINATOR: [u8; 8] = [0x85, 0x9e, 0xd4, 0xbd, 0xed, 0x0c, 0x49, 0x27];
}
impl InstructionData for SetAdaptiveFeeConstantsInstructionArgs {}

struct SetAdaptiveFeeConstantsInstructionAccounts {
  pub whirlpool: Pubkey,
  pub whirlpools_config: Pubkey,
  pub oracle: Pubkey,
  pub fee_authority: Pubkey,
}
impl ToAccountMetas for SetAdaptiveFeeConstantsInstructionAccounts {
  fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<solana_program::instruction::AccountMeta> {
    vec![
      solana_program::instruction::AccountMeta::new_readonly(self.whirlpool, false),
      solana_program::instruction::AccountMeta::new_readonly(self.whirlpools_config, false),
      solana_program::instruction::AccountMeta::new(self.oracle, false),
      solana_program::instruction::AccountMeta::new_readonly(self.fee_authority, is_signer.unwrap_or(true)),
    ]
  }
}

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetAdaptiveFeeConstants>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // oracle
  replayer.set_whirlpool_account(&ix.key_oracle, accounts);
  // fee_authority
    
  let tx = replayer.build_whirlpool_replay_transaction(
    SetAdaptiveFeeConstantsInstructionArgs {
      filter_period: ix.data_adaptive_fee_constants.filter_period,
      decay_period: ix.data_adaptive_fee_constants.decay_period,
      reduction_factor: ix.data_adaptive_fee_constants.reduction_factor,
      adaptive_fee_control_factor: ix.data_adaptive_fee_constants.adaptive_fee_control_factor,
      max_volatility_accumulator: ix.data_adaptive_fee_constants.max_volatility_accumulator,
      tick_group_size: ix.data_adaptive_fee_constants.tick_group_size,
      major_swap_threshold_ticks: ix.data_adaptive_fee_constants.major_swap_threshold_ticks,
    },
    SetAdaptiveFeeConstantsInstructionAccounts {
      whirlpool: pubkey(&ix.key_whirlpool),
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      oracle: pubkey(&ix.key_oracle),
      fee_authority: pubkey(&ix.key_fee_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_oracle,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_oracle,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

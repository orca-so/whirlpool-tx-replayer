use serde_derive::{Deserialize, Serialize};
use serde::de;
use base64::prelude::{Engine as _, BASE64_STANDARD};
use crate::errors::ErrorCode;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
pub enum DecodedWhirlpoolInstruction {
  AdminIncreaseLiquidity(DecodedAdminIncreaseLiquidity),
  CloseBundledPosition(DecodedCloseBundledPosition),
  ClosePosition(DecodedClosePosition),
  CollectFees(DecodedCollectFees),
  CollectProtocolFees(DecodedCollectProtocolFees),
  CollectReward(DecodedCollectReward),
  DecreaseLiquidity(DecodedDecreaseLiquidity),
  DeletePositionBundle(DecodedDeletePositionBundle),
  IncreaseLiquidity(DecodedIncreaseLiquidity),
  InitializeConfig(DecodedInitializeConfig),
  InitializeFeeTier(DecodedInitializeFeeTier),
  InitializePool(DecodedInitializePool),
  InitializePositionBundle(DecodedInitializePositionBundle),
  InitializePositionBundleWithMetadata(DecodedInitializePositionBundleWithMetadata),
  InitializeReward(DecodedInitializeReward),
  InitializeTickArray(DecodedInitializeTickArray),
  OpenBundledPosition(DecodedOpenBundledPosition),
  OpenPosition(DecodedOpenPosition),
  OpenPositionWithMetadata(DecodedOpenPositionWithMetadata),
  SetCollectProtocolFeesAuthority(DecodedSetCollectProtocolFeesAuthority),
  SetDefaultFeeRate(DecodedSetDefaultFeeRate),
  SetDefaultProtocolFeeRate(DecodedSetDefaultProtocolFeeRate),
  SetFeeAuthority(DecodedSetFeeAuthority),
  SetFeeRate(DecodedSetFeeRate),
  SetProtocolFeeRate(DecodedSetProtocolFeeRate),
  SetRewardAuthority(DecodedSetRewardAuthority),
  SetRewardAuthorityBySuperAuthority(DecodedSetRewardAuthorityBySuperAuthority),
  SetRewardEmissions(DecodedSetRewardEmissions),
  SetRewardEmissionsSuperAuthority(DecodedSetRewardEmissionsSuperAuthority),
  Swap(DecodedSwap),
  TwoHopSwap(DecodedTwoHopSwap),
  UpdateFeesAndRewards(DecodedUpdateFeesAndRewards),
  CollectFeesV2(DecodedCollectFeesV2),
  CollectProtocolFeesV2(DecodedCollectProtocolFeesV2),
  CollectRewardV2(DecodedCollectRewardV2),
  DecreaseLiquidityV2(DecodedDecreaseLiquidityV2),
  IncreaseLiquidityV2(DecodedIncreaseLiquidityV2),
  SwapV2(DecodedSwapV2),
  TwoHopSwapV2(DecodedTwoHopSwapV2),
  InitializePoolV2(DecodedInitializePoolV2),
  InitializeRewardV2(DecodedInitializeRewardV2),
  SetRewardEmissionsV2(DecodedSetRewardEmissionsV2),
  InitializeConfigExtension(DecodedInitializeConfigExtension),
  InitializeTokenBadge(DecodedInitializeTokenBadge),
  DeleteTokenBadge(DecodedDeleteTokenBadge),
  SetConfigExtensionAuthority(DecodedSetConfigExtensionAuthority),
  SetTokenBadgeAuthority(DecodedSetTokenBadgeAuthority),
  OpenPositionWithTokenExtensions(DecodedOpenPositionWithTokenExtensions),
  ClosePositionWithTokenExtensions(DecodedClosePositionWithTokenExtensions),
  LockPosition(DecodedLockPosition),
  ResetPositionRange(DecodedResetPositionRange),
  TransferLockedPosition(DecodedTransferLockedPosition),
}

#[derive(Debug, PartialEq, Eq)]
pub enum DecodedInstruction {
  ProgramDeployInstruction(DecodedProgramDeployInstruction),
  WhirlpoolInstruction(DecodedWhirlpoolInstruction),
}

pub fn from_json(ix: &String, json: &String) -> Result<DecodedInstruction, ErrorCode> {
  fn from_str<'de, T>(json: &'de String) -> Result<T, ErrorCode>
  where T: de::Deserialize<'de>,
  {
    serde_json::from_str(json).map_err(|_| ErrorCode::InvalidWhirlpoolInstructionJsonString)
  }

  if ix.as_str() == "programDeploy" {
    let ix = from_str::<DecodedProgramDeployInstruction>(&json)?;
    return Ok(DecodedInstruction::ProgramDeployInstruction(ix));
  }

  let ix = match ix.as_str() {
    "adminIncreaseLiquidity" => Ok(DecodedWhirlpoolInstruction::AdminIncreaseLiquidity(from_str(&json)?)),
    "closeBundledPosition" => Ok(DecodedWhirlpoolInstruction::CloseBundledPosition(from_str(&json)?)),
    "closePosition" => Ok(DecodedWhirlpoolInstruction::ClosePosition(from_str(&json)?)),
    "collectFees" => Ok(DecodedWhirlpoolInstruction::CollectFees(from_str(&json)?)),
    "collectProtocolFees" => Ok(DecodedWhirlpoolInstruction::CollectProtocolFees(from_str(&json)?)),
    "collectReward" => Ok(DecodedWhirlpoolInstruction::CollectReward(from_str(&json)?)),
    "decreaseLiquidity" => Ok(DecodedWhirlpoolInstruction::DecreaseLiquidity(from_str(&json)?)),
    "deletePositionBundle" => Ok(DecodedWhirlpoolInstruction::DeletePositionBundle(from_str(&json)?)),
    "increaseLiquidity" => Ok(DecodedWhirlpoolInstruction::IncreaseLiquidity(from_str(&json)?)),
    "initializeConfig" => Ok(DecodedWhirlpoolInstruction::InitializeConfig(from_str(&json)?)),
    "initializeFeeTier" => Ok(DecodedWhirlpoolInstruction::InitializeFeeTier(from_str(&json)?)),
    "initializePool" => Ok(DecodedWhirlpoolInstruction::InitializePool(from_str(&json)?)),
    "initializePositionBundle" => Ok(DecodedWhirlpoolInstruction::InitializePositionBundle(from_str(&json)?)),
    "initializePositionBundleWithMetadata" => Ok(DecodedWhirlpoolInstruction::InitializePositionBundleWithMetadata(from_str(&json)?)),
    "initializeReward" => Ok(DecodedWhirlpoolInstruction::InitializeReward(from_str(&json)?)),
    "initializeTickArray" => Ok(DecodedWhirlpoolInstruction::InitializeTickArray(from_str(&json)?)),
    "openBundledPosition" => Ok(DecodedWhirlpoolInstruction::OpenBundledPosition(from_str(&json)?)),
    "openPosition" => Ok(DecodedWhirlpoolInstruction::OpenPosition(from_str(&json)?)),
    "openPositionWithMetadata" => Ok(DecodedWhirlpoolInstruction::OpenPositionWithMetadata(from_str(&json)?)),
    "setCollectProtocolFeesAuthority" => Ok(DecodedWhirlpoolInstruction::SetCollectProtocolFeesAuthority(from_str(&json)?)),
    "setDefaultFeeRate" => Ok(DecodedWhirlpoolInstruction::SetDefaultFeeRate(from_str(&json)?)),
    "setDefaultProtocolFeeRate" => Ok(DecodedWhirlpoolInstruction::SetDefaultProtocolFeeRate(from_str(&json)?)),
    "setFeeAuthority" => Ok(DecodedWhirlpoolInstruction::SetFeeAuthority(from_str(&json)?)),
    "setFeeRate" => Ok(DecodedWhirlpoolInstruction::SetFeeRate(from_str(&json)?)),
    "setProtocolFeeRate" => Ok(DecodedWhirlpoolInstruction::SetProtocolFeeRate(from_str(&json)?)),
    "setRewardAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardAuthority(from_str(&json)?)),
    "setRewardAuthorityBySuperAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority(from_str(&json)?)),
    "setRewardEmissions" => Ok(DecodedWhirlpoolInstruction::SetRewardEmissions(from_str(&json)?)),
    "setRewardEmissionsSuperAuthority" => Ok(DecodedWhirlpoolInstruction::SetRewardEmissionsSuperAuthority(from_str(&json)?)),
    "swap" => Ok(DecodedWhirlpoolInstruction::Swap(from_str(&json)?)),
    "twoHopSwap" => Ok(DecodedWhirlpoolInstruction::TwoHopSwap(from_str(&json)?)),
    "updateFeesAndRewards" => Ok(DecodedWhirlpoolInstruction::UpdateFeesAndRewards(from_str(&json)?)),
    "collectFeesV2" => Ok(DecodedWhirlpoolInstruction::CollectFeesV2(from_str(&json)?)),
    "collectProtocolFeesV2" => Ok(DecodedWhirlpoolInstruction::CollectProtocolFeesV2(from_str(&json)?)),
    "collectRewardV2" => Ok(DecodedWhirlpoolInstruction::CollectRewardV2(from_str(&json)?)),
    "decreaseLiquidityV2" => Ok(DecodedWhirlpoolInstruction::DecreaseLiquidityV2(from_str(&json)?)),
    "increaseLiquidityV2" => Ok(DecodedWhirlpoolInstruction::IncreaseLiquidityV2(from_str(&json)?)),
    "swapV2" => Ok(DecodedWhirlpoolInstruction::SwapV2(from_str(&json)?)),
    "twoHopSwapV2" => Ok(DecodedWhirlpoolInstruction::TwoHopSwapV2(from_str(&json)?)),
    "initializePoolV2" => Ok(DecodedWhirlpoolInstruction::InitializePoolV2(from_str(&json)?)),
    "initializeRewardV2" => Ok(DecodedWhirlpoolInstruction::InitializeRewardV2(from_str(&json)?)),
    "setRewardEmissionsV2" => Ok(DecodedWhirlpoolInstruction::SetRewardEmissionsV2(from_str(&json)?)),
    "initializeConfigExtension" => Ok(DecodedWhirlpoolInstruction::InitializeConfigExtension(from_str(&json)?)),
    "initializeTokenBadge" => Ok(DecodedWhirlpoolInstruction::InitializeTokenBadge(from_str(&json)?)),
    "deleteTokenBadge" => Ok(DecodedWhirlpoolInstruction::DeleteTokenBadge(from_str(&json)?)),
    "setConfigExtensionAuthority" => Ok(DecodedWhirlpoolInstruction::SetConfigExtensionAuthority(from_str(&json)?)),
    "setTokenBadgeAuthority" => Ok(DecodedWhirlpoolInstruction::SetTokenBadgeAuthority(from_str(&json)?)),
    "openPositionWithTokenExtensions" => Ok(DecodedWhirlpoolInstruction::OpenPositionWithTokenExtensions(from_str(&json)?)),
    "closePositionWithTokenExtensions" => Ok(DecodedWhirlpoolInstruction::ClosePositionWithTokenExtensions(from_str(&json)?)),
    "lockPosition" => Ok(DecodedWhirlpoolInstruction::LockPosition(from_str(&json)?)),
    "resetPositionRange" => Ok(DecodedWhirlpoolInstruction::ResetPositionRange(from_str(&json)?)),
    "transferLockedPosition" => Ok(DecodedWhirlpoolInstruction::TransferLockedPosition(from_str(&json)?)),
    _ => Err(ErrorCode::UnknownWhirlpoolInstruction(ix.to_string())),
  };

  match ix {
    Ok(ix) => Ok(DecodedInstruction::WhirlpoolInstruction(ix)),
    Err(err) => Err(err),
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedProgramDeployInstruction {
  #[serde(deserialize_with = "deserialize_base64")]
  pub program_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedAdminIncreaseLiquidity {
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_liquidity: u128,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCloseBundledPosition {
  pub data_bundle_index: u16,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePosition {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFees {
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_b: String,
  pub key_token_program: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFees {
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_token_destination_a: String,
  pub key_token_destination_b: String,
  pub key_token_program: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectReward {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_reward_owner_account: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidity {
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_liquidity_amount: u128,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_min_a: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_min_b: u64,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeletePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_receiver: String,
  pub key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidity {
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_liquidity_amount: u128,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_max_a: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_max_b: u64,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfig {
  pub data_default_protocol_fee_rate: u16,
  pub data_fee_authority: String,
  pub data_collect_protocol_fees_authority: String,
  pub data_reward_emissions_super_authority: String,
  pub key_whirlpools_config: String,
  pub key_funder: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeFeeTier {
  pub data_tick_spacing: u16,
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_funder: String,
  pub key_fee_authority: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePool {
  pub data_tick_spacing: u16,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_fee_tier: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundle {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundleWithMetadata {
  pub key_position_bundle: String,
  pub key_position_bundle_mint: String,
  pub key_position_bundle_metadata: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_owner: String,
  pub key_funder: String,
  pub key_metadata_update_auth: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeReward {
  pub data_reward_index: u8,
  pub key_reward_authority: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_reward_mint: String,
  pub key_reward_vault: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTickArray {
  pub data_start_tick_index: i32,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_tick_array: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenBundledPosition {
  pub data_bundle_index: u16,
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_bundled_position: String,
  pub key_position_bundle: String,
  pub key_position_bundle_token_account: String,
  pub key_position_bundle_authority: String,
  pub key_whirlpool: String,
  pub key_funder: String,
  pub key_system_program: String,
  pub key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPosition {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithMetadata {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_metadata_account: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  pub key_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  pub key_associated_token_program: String,
  pub key_metadata_program: String,
  pub key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetCollectProtocolFeesAuthority {
  pub key_whirlpools_config: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_new_collect_protocol_fees_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultFeeRate {
  pub data_default_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_tier: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultProtocolFeeRate {
  pub data_default_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeAuthority {
  pub key_whirlpools_config: String,
  pub key_fee_authority: String,
  pub key_new_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeRate {
  pub data_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetProtocolFeeRate {
  pub data_protocol_fee_rate: u16,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthority {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthorityBySuperAuthority {
  pub data_reward_index: u8,
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissions {
  pub data_reward_index: u8,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsSuperAuthority {
  pub key_whirlpools_config: String,
  pub key_reward_emissions_super_authority: String,
  pub key_new_reward_emissions_super_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwap {
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_amount: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_other_amount_threshold: u64,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit: u128,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b: bool,
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool: String,
  pub key_token_owner_account_a: String,
  pub key_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_vault_b: String,
  pub key_tick_array_0: String,
  pub key_tick_array_1: String,
  pub key_tick_array_2: String,
  pub key_oracle: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwap {
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_amount: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_other_amount_threshold: u64,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_one: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_two: bool,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit_one: u128,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit_two: u128,
  pub key_token_program: String,
  pub key_token_authority: String,
  pub key_whirlpool_one: String,
  pub key_whirlpool_two: String,
  pub key_token_owner_account_one_a: String,
  pub key_vault_one_a: String,
  pub key_token_owner_account_one_b: String,
  pub key_vault_one_b: String,
  pub key_token_owner_account_two_a: String,
  pub key_vault_two_a: String,
  pub key_token_owner_account_two_b: String,
  pub key_vault_two_b: String,
  pub key_tick_array_one_0: String,
  pub key_tick_array_one_1: String,
  pub key_tick_array_one_2: String,
  pub key_tick_array_two_0: String,
  pub key_tick_array_two_1: String,
  pub key_tick_array_two_2: String,
  pub key_oracle_one: String,
  pub key_oracle_two: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_0: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_1: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_2: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_amount_3: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedUpdateFeesAndRewards {
  pub key_whirlpool: String,
  pub key_position: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFeesV2 {
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_b: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFeesV2 {
  pub key_whirlpools_config: String,
  pub key_whirlpool: String,
  pub key_collect_protocol_fees_authority: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_token_destination_a: String,
  pub key_token_destination_b: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectRewardV2 {
  pub data_reward_index: u8,
  pub key_whirlpool: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_reward_owner_account: String,
  pub key_reward_mint: String,
  pub key_reward_vault: String,
  pub key_reward_token_program: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidityV2 {
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_liquidity_amount: u128,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_min_a: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_min_b: u64,
  pub key_whirlpool: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidityV2 {
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_liquidity_amount: u128,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_max_a: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_token_amount_max_b: u64,
  pub key_whirlpool: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_token_owner_account_b: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_tick_array_lower: String,
  pub key_tick_array_upper: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwapV2 {
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_amount: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_other_amount_threshold: u64,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit: u128,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b: bool,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_memo_program: String,
  pub key_token_authority: String,
  pub key_whirlpool: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_owner_account_a: String,
  pub key_vault_a: String,
  pub key_token_owner_account_b: String,
  pub key_vault_b: String,
  pub key_tick_array_0: String,
  pub key_tick_array_1: String,
  pub key_tick_array_2: String,
  pub key_oracle: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwapV2 {
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_amount: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub data_other_amount_threshold: u64,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_amount_specified_is_input: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_one: bool,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_a_to_b_two: bool,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit_one: u128,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_sqrt_price_limit_two: u128,
  pub key_whirlpool_one: String,
  pub key_whirlpool_two: String,
  pub key_token_mint_input: String,
  pub key_token_mint_intermediate: String,
  pub key_token_mint_output: String,
  pub key_token_program_input: String,
  pub key_token_program_intermediate: String,
  pub key_token_program_output: String,
  pub key_token_owner_account_input: String,
  pub key_vault_one_input: String,
  pub key_vault_one_intermediate: String,
  pub key_vault_two_intermediate: String,
  pub key_vault_two_output: String,
  pub key_token_owner_account_output: String,
  pub key_token_authority: String,
  pub key_tick_array_one_0: String,
  pub key_tick_array_one_1: String,
  pub key_tick_array_one_2: String,
  pub key_tick_array_two_0: String,
  pub key_tick_array_two_1: String,
  pub key_tick_array_two_2: String,
  pub key_oracle_one: String,
  pub key_oracle_two: String,
  pub key_memo_program: String,
  pub remaining_accounts_info: RemainingAccountsInfo,
  pub remaining_accounts_keys: RemainingAccountsKeys,
  pub transfer_0: TransferAmountWithTransferFeeConfig,
  pub transfer_1: TransferAmountWithTransferFeeConfig,
  pub transfer_2: TransferAmountWithTransferFeeConfig,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePoolV2 {
  pub data_tick_spacing: u16,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_initial_sqrt_price: u128,
  pub key_whirlpools_config: String,
  pub key_token_mint_a: String,
  pub key_token_mint_b: String,
  pub key_token_badge_a: String,
  pub key_token_badge_b: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_token_vault_a: String,
  pub key_token_vault_b: String,
  pub key_fee_tier: String,
  pub key_token_program_a: String,
  pub key_token_program_b: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_a: u8,
  #[cfg(feature = "decimals")]
  pub decimals_token_mint_b: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeRewardV2 {
  pub data_reward_index: u8,
  pub key_reward_authority: String,
  pub key_funder: String,
  pub key_whirlpool: String,
  pub key_reward_mint: String,
  pub key_reward_token_badge: String,
  pub key_reward_vault: String,
  pub key_reward_token_program: String,
  pub key_system_program: String,
  pub key_rent: String,
  #[cfg(feature = "decimals")]
  pub decimals_reward_mint: u8,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsV2 {
  pub data_reward_index: u8,
  #[serde(deserialize_with = "deserialize_u128")]
  pub data_emissions_per_second_x64: u128,
  pub key_whirlpool: String,
  pub key_reward_authority: String,
  pub key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfigExtension {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_funder: String,
  pub key_fee_authority: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTokenBadge {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_token_badge_authority: String,
  pub key_token_mint: String,
  pub key_token_badge: String,
  pub key_funder: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeleteTokenBadge {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_token_badge_authority: String,
  pub key_token_mint: String,
  pub key_token_badge: String,
  pub key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetConfigExtensionAuthority {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_config_extension_authority: String,
  pub key_new_config_extension_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetTokenBadgeAuthority {
  pub key_whirlpools_config: String,
  pub key_whirlpools_config_extension: String,
  pub key_config_extension_authority: String,
  pub key_new_token_badge_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithTokenExtensions {
  pub data_tick_lower_index: i32,
  pub data_tick_upper_index: i32,
  #[serde(deserialize_with = "deserialize_bool")]
  pub data_with_token_metadata_extension: bool,
  pub key_funder: String,
  pub key_owner: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_whirlpool: String,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: String,
  pub key_system_program: String,
  pub key_associated_token_program: String,
  pub key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePositionWithTokenExtensions {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedLockPosition {
  pub data_lock_type: LockType,
  pub key_funder: String,
  pub key_position_authority: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_lock_config: String,
  pub key_whirlpool: String,
  pub key_token_2022_program: String,
  pub key_system_program: String,
  pub aux_key_position_token_account_owner: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedResetPositionRange {
  pub data_new_tick_lower_index: i32,
  pub data_new_tick_upper_index: i32,
  pub key_funder: String,
  pub key_position_authority: String,
  pub key_whirlpool: String,
  pub key_position: String,
  pub key_position_token_account: String,
  pub key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTransferLockedPosition {
  pub key_position_authority: String,
  pub key_receiver: String,
  pub key_position: String,
  pub key_position_mint: String,
  pub key_position_token_account: String,
  pub key_destination_token_account: String,
  pub key_lock_config: String,
  // note: we can read and write "keyToken2022Program" field as expected
  pub key_token_2022_program: String,
  pub aux_key_destination_token_account_owner: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase", tag = "name")]
pub enum LockType {
  Permanent,
}

pub type RemainingAccountsInfo = Vec<[u8; 2]>;
pub type RemainingAccountsKeys = Vec<String>;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransferAmountWithTransferFeeConfig {
  #[serde(deserialize_with = "deserialize_u64")]
  pub amount: u64,
  #[serde(deserialize_with = "deserialize_bool")]
  pub transfer_fee_config_opt: bool,
  pub transfer_fee_config_bps: u16,
  #[serde(deserialize_with = "deserialize_u64")]
  pub transfer_fee_config_max: u64,
}


// 0 to false, 1 to true
pub fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: u8 = de::Deserialize::deserialize(deserializer)?;
    match n {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(de::Error::custom("expected 0 or 1")),
    }
}

// string to u64
pub fn deserialize_u64<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: String = de::Deserialize::deserialize(deserializer)?;
    match n.parse::<u64>() {
        Ok(n) => Ok(n),
        Err(_) => Err(de::Error::custom("expected u64")),
    }
}

// string to u128
pub fn deserialize_u128<'de, D>(deserializer: D) -> Result<u128, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: String = de::Deserialize::deserialize(deserializer)?;
    match n.parse::<u128>() {
        Ok(n) => Ok(n),
        Err(_) => Err(de::Error::custom("expected u128")),
    }
}

// base64 string to Vec<u8>
pub fn deserialize_base64<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let base64: String = de::Deserialize::deserialize(deserializer)?;
    match BASE64_STANDARD.decode(base64).ok() {
        Some(data) => Ok(data),
        None => Err(de::Error::custom("expected base64 string")),
    }
}

// Vec<u8> to base64 string
pub fn serialize_base64<S>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let base64: String = BASE64_STANDARD.encode(data);
    serializer.serialize_str(&base64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_increase_liquidity_v2() {
        let json_str = r#"{"dataLiquidityAmount": "3453450", "dataTokenAmountMaxA": "19337", "dataTokenAmountMaxB": "19341", "keyWhirlpool": "9tXiuRRw7kbejLhZXtxDxYs2REe43uH2e7k1kocgdM9B", "keyTokenProgramA": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keyTokenProgramB": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA", "keyMemoProgram": "MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr", "keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "CR39mQe5b87s1Qf4XMSyo12P99buoUaqLprrgQ4ccady", "keyPositionTokenAccount": "ChfxQHG4fV9FZaABv8N3v4vf1wWUhgfFB1VLES3tZVqu", "keyTokenMintA": "2b1kV6DkPAnxd5ixfnxCpjxmKwqjjaYmCZfHsFu24GXo", "keyTokenMintB": "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v", "keyTokenOwnerAccountA": "2A7Cc48jwWWoixM5CWquQKEqk9KNQvY2Xw3WJbBRc6Ei", "keyTokenOwnerAccountB": "FbQdXCQgGQYj3xcGeryVVFjKCTsAuu53vmCRtmjQEqM5", "keyTokenVaultA": "EeF6oBy6AQiBJoRx5xiRNxa6cmpQE3ayVagj28QFZuyg", "keyTokenVaultB": "MvB8poDgpDPbRgx8MXeb7EPEsawGuiBTqpkpM9exeLi", "keyTickArrayLower": "8hXTpuvJQRar4Pf6BZiEWquFgtAtSf2RFDM6EL2FCcf1", "keyTickArrayUpper": "B1jXbjDzenSy8kPNaGw3GSKAVQis5K5tRLeXuaskZTpS", "remainingAccountsInfo": [], "remainingAccountsKeys": [], "transfer0": {"amount": "10000", "transferFeeConfigOpt": 1, "transferFeeConfigBps": 0, "transferFeeConfigMax": "0"}, "transfer1": {"amount": "9312", "transferFeeConfigOpt": 0, "transferFeeConfigBps": 0, "transferFeeConfigMax": "0"}}"#;
        let _ = from_json(&"increaseLiquidityV2".to_string(), &json_str.to_string()).unwrap();
    }

    #[test]
    fn test_decode_open_position_with_token_extensions() {
        let json_str = r#"{"dataTickLowerIndex": 29440, "dataTickUpperIndex": 33536, "dataWithTokenMetadataExtension": 1, "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyOwner": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPosition": "3Gm8DyRFFwaixymojnP1uS1PiXx8KQujuBi3ixEj9Lvv", "keyPositionMint": "9yk8n6b7S2d2XE1GRqRqJs7JDYhKa1t3po27kxcCMZZD", "keyPositionTokenAccount": "DQyNecBmT1SjXRhqeGzGtywjv9BDFdAiuXafSP7Lk1DR", "keyWhirlpool": "6DKRF7rvSiwCNuVM5HC97Rz4n1R4w1dg65DrQjeypoLc", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keySystemProgram": "11111111111111111111111111111111", "keyAssociatedTokenProgram": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL", "keyMetadataUpdateAuth": "3axbTs2z5GBy6usVbNVoqEgZMng3vZvMnAoX29BFfwhr"}"#;
        let _ = from_json(&"openPositionWithTokenExtensions".to_string(), &json_str.to_string()).unwrap();
    }

    #[test]
    fn test_decode_close_position_with_token_extensions() {
        let json_str = r#"{"keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyReceiver": "vvcvRBSqzAGjTKaPV3hECaGNbw94gLcoWFFpbvFHyP9", "keyPosition": "BbEMeYPTstMDgmohucEBj7H6obkinZQRcxZ2Gpt3cz3X", "keyPositionMint": "Hw3afBx59tPLCwVmE5rt6KpqWVGd8dfqKzSndKtuxHxa", "keyPositionTokenAccount": "EyExmEKtA9E45TKoBKjNRLyxuS2Bn5NsBrxZQ2fKrLE1", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb"}"#;
        let _ = from_json(&"closePositionWithTokenExtensions".to_string(), &json_str.to_string()).unwrap();
    }

    #[test]
    fn test_decode_lock_position() {
      let json_str = r#"{"dataLockType": {"name":"permanent"}, "keyFunder": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyPositionAuthority": "2ViXvF8djjCFWLMfjPeyYFxH3eDo9D5eLnWvFscGy3Zn", "keyPosition": "4smUufScXDa5E6HDYEjDUAVGvrkGyU7378GNr89vY8h1", "keyPositionMint": "5qper4gVLigi1HTg3FELzt36Y3BHd59y37APZ6BEW2Ke", "keyPositionTokenAccount": "966wMa5TJGv3aCGiBEgFpwoeT84opPnLbXSq6AZPpeKX", "keyLockConfig": "BoK2279WTwe3yAia7GTbae8bZY6MyiCpWr5qiAvNm8Cb", "keyWhirlpool": "CTuhhDTq1shvQwd5aeezjNYrw1q4KvDszSQWc9EmDMVd", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "keySystemProgram": "11111111111111111111111111111111", "auxKeyPositionTokenAccountOwner": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6"}"#;
      let _ = from_json(&"lockPosition".to_string(), &json_str.to_string()).unwrap();
    }

    #[test]
    fn test_decode_reset_position_range() {
      let json_str = r#"{"dataNewTickLowerIndex": 128, "dataNewTickUpperIndex": 32640, "keyFunder": "FEL7n299Zav7WRv4tszuGBeS4e1Xfq5wkXNyeZsNt586", "keyPositionAuthority": "3otH3AHWqkqgSVfKFkrxyDqd2vK6LcaqigHrFEmWcGuo", "keyWhirlpool": "F1xbtqx8cDaumqHc2MzknADQVHMdY9Vc5WvhxHYipERG", "keyPosition": "Coje7Bvp4yCyxum7Y3BSbvhWHuJJfi4S57ohqkT4mMHo", "keyPositionTokenAccount": "DMHFpHSFGJ5e5Xf6FAmFHAqFyhTHHHQojJQdtdtP6sLC", "keySystemProgram": "11111111111111111111111111111111"}"#;
      let _ = from_json(&"resetPositionRange".to_string(), &json_str.to_string()).unwrap();
    }

    #[test]
    fn test_decode_transfer_locked_position() {
      let json_str = r#"{"keyPositionAuthority": "r21Gamwd9DtyjHeGywsneoQYR39C1VDwrw7tWxHAwh6", "keyReceiver": "HTmy49kE8Vug2wCh2hSqE1xH1EyXy9x26G6wJg1i9A1P", "keyPosition": "Bvxjz3othczrEmSpodoqwXjJLqcsqawaqHv5NFRDq98V", "keyPositionMint": "6u7DbXkw82Nm5xCfVH5nTC9C4M2PP8FSfGS7aQqHoqRh", "keyPositionTokenAccount": "HkB4St4k7NQ9YjtrikSmzenTVPykxCSwbcw9vFyv2t2c", "keyDestinationTokenAccount": "5wayh2tapWNqMZz4x81AMhJDUVAanMyBK5TLqQHew88a", "keyLockConfig": "CnbYq7WjrehZgLdKTTzibUPT75EtpaAzZY6k9p3Lpoj2", "keyToken2022Program": "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb", "auxKeyDestinationTokenAccountOwner": "ECyvxDUzA8LyYjHR67Doj21WuVBvJcSipSjw8HauegyY"}"#;
      let _ = from_json(&"transferLockedPosition".to_string(), &json_str.to_string()).unwrap();
    }
}

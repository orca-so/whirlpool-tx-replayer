use serde_derive::{Deserialize, Serialize};
use serde::de;

use crate::errors::ErrorCode;

#[derive(Debug, PartialEq, Eq)]
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
}

pub fn from_json(ix: &String, json: &String) -> Result<DecodedWhirlpoolInstruction, ErrorCode> {
  fn from_str<'de, T>(json: &'de String) -> Result<T, ErrorCode>
  where T: de::Deserialize<'de>,
  {
    serde_json::from_str(json).map_err(|_| ErrorCode::InvalidWhirlpoolInstructionJsonString)
  }

  match ix.as_str() {
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
    _ => Err(ErrorCode::UnknownWhirlpoolInstruction(ix.to_string())),
  }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedAdminIncreaseLiquidity {
    data_liquidity: u128,
    key_whirlpools_config: String,
    key_whirlpool: String,
    key_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCloseBundledPosition {
    data_bundle_index: u16,
    key_bundled_position: String,
    key_position_bundle: String,
    key_position_bundle_token_account: String,
    key_position_bundle_authority: String,
    key_receiver: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedClosePosition {
    key_position_authority: String,
    key_receiver: String,
    key_position: String,
    key_position_mint: String,
    key_position_token_account: String,
    key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectFees {
    key_whirlpool: String,
    key_position_authority: String,
    key_position: String,
    key_position_token_account: String,
    key_token_owner_account_a: String,
    key_token_vault_a: String,
    key_token_owner_account_b: String,
    key_token_vault_b: String,
    key_token_program: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectProtocolFees {
    key_whirlpools_config: String,
    key_whirlpool: String,
    key_collect_protocol_fees_authority: String,
    key_token_vault_a: String,
    key_token_vault_b: String,
    key_token_destination_a: String,
    key_token_destination_b: String,
    key_token_program: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedCollectReward {
    data_reward_index: u8,
    key_whirlpool: String,
    key_position_authority: String,
    key_position: String,
    key_position_token_account: String,
    key_reward_owner_account: String,
    key_reward_vault: String,
    key_token_program: String,
    transfer_amount0: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDecreaseLiquidity {
    data_liquidity_amount: u128,
    data_token_amount_min_a: u64,
    data_token_amount_min_b: u64,
    key_whirlpool: String,
    key_token_program: String,
    key_position_authority: String,
    key_position: String,
    key_position_token_account: String,
    key_token_owner_account_a: String,
    key_token_owner_account_b: String,
    key_token_vault_a: String,
    key_token_vault_b: String,
    key_tick_array_lower: String,
    key_tick_array_upper: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedDeletePositionBundle {
    key_position_bundle: String,
    key_position_bundle_mint: String,
    key_position_bundle_token_account: String,
    key_position_bundle_owner: String,
    key_receiver: String,
    key_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedIncreaseLiquidity {
    data_liquidity_amount: u128,
    data_token_amount_max_a: u64,
    data_token_amount_max_b: u64,
    key_whirlpool: String,
    key_token_program: String,
    key_position_authority: String,
    key_position: String,
    key_position_token_account: String,
    key_token_owner_account_a: String,
    key_token_owner_account_b: String,
    key_token_vault_a: String,
    key_token_vault_b: String,
    key_tick_array_lower: String,
    key_tick_array_upper: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeConfig {
    data_default_protocol_fee_rate: u16,
    data_fee_authority: String,
    data_collect_protocol_fees_authority: String,
    data_reward_emissions_super_authority: String,
    key_whirlpools_config: String,
    key_funder: String,
    key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeFeeTier {
    data_tick_spacing: u16,
    data_default_fee_rate: u16,
    key_whirlpools_config: String,
    key_fee_tier: String,
    key_funder: String,
    key_fee_authority: String,
    key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePool {
    data_tick_spacing: u16,
    data_initial_sqrt_price: u128,
    key_whirlpools_config: String,
    key_token_mint_a: String,
    key_token_mint_b: String,
    key_funder: String,
    key_whirlpool: String,
    key_token_vault_a: String,
    key_token_vault_b: String,
    key_fee_tier: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundle {
    key_position_bundle: String,
    key_position_bundle_mint: String,
    key_position_bundle_token_account: String,
    key_position_bundle_owner: String,
    key_funder: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
    key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializePositionBundleWithMetadata {
    key_position_bundle: String,
    key_position_bundle_mint: String,
    key_position_bundle_metadata: String,
    key_position_bundle_token_account: String,
    key_position_bundle_owner: String,
    key_funder: String,
    key_metadata_update_auth: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
    key_associated_token_program: String,
    key_metadata_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeReward {
    data_reward_index: u8,
    key_reward_authority: String,
    key_funder: String,
    key_whirlpool: String,
    key_reward_mint: String,
    key_reward_vault: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedInitializeTickArray {
    data_start_tick_index: u16,
    key_whirlpool: String,
    key_funder: String,
    key_tick_array: String,
    key_system_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenBundledPosition {
    data_bundle_index: u16,
    data_tick_lower_index: i32,
    data_tick_upper_index: i32,
    key_bundled_position: String,
    key_position_bundle: String,
    key_position_bundle_token_account: String,
    key_position_bundle_authority: String,
    key_whirlpool: String,
    key_funder: String,
    key_system_program: String,
    key_rent: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPosition {
    data_tick_lower_index: i32,
    data_tick_upper_index: i32,
    key_funder: String,
    key_owner: String,
    key_position: String,
    key_position_mint: String,
    key_position_token_account: String,
    key_whirlpool: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
    key_associated_token_program: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedOpenPositionWithMetadata {
    data_tick_lower_index: i32,
    data_tick_upper_index: i32,
    key_funder: String,
    key_owner: String,
    key_position: String,
    key_position_mint: String,
    key_position_metadata_account: String,
    key_position_token_account: String,
    key_whirlpool: String,
    key_token_program: String,
    key_system_program: String,
    key_rent: String,
    key_associated_token_program: String,
    key_metadata_program: String,
    key_metadata_update_auth: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetCollectProtocolFeesAuthority {
    key_whirlpools_config: String,
    key_collect_protocol_fees_authority: String,
    key_new_collect_protocol_fees_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultFeeRate {
    data_default_fee_rate: u16,
    key_whirlpools_config: String,
    key_fee_tier: String,
    key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetDefaultProtocolFeeRate {
    data_default_protocol_fee_rate: u16,
    key_whirlpools_config: String,
    key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeAuthority {
    key_whirlpools_config: String,
    key_fee_authority: String,
    key_new_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetFeeRate {
    data_fee_rate: u16,
    key_whirlpools_config: String,
    key_whirlpool: String,
    key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetProtocolFeeRate {
    data_protocol_fee_rate: u16,
    key_whirlpools_config: String,
    key_whirlpool: String,
    key_fee_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthority {
    data_reward_index: u8,
    key_whirlpool: String,
    key_reward_authority: String,
    key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardAuthorityBySuperAuthority {
    data_reward_index: u8,
    key_whirlpools_config: String,
    key_whirlpool: String,
    key_reward_emissions_super_authority: String,
    key_new_reward_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissions {
    data_reward_index: u8,
    data_emissions_per_second_x64: u128,
    key_whirlpool: String,
    key_reward_authority: String,
    key_reward_vault: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSetRewardEmissionsSuperAuthority {
    key_whirlpools_config: String,
    key_reward_emissions_super_authority: String,
    key_new_reward_emissions_super_authority: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwap {
    data_amount: u64,
    data_other_amount_threshold: u64,
    data_sqrt_price_limit: u128,
    #[serde(deserialize_with = "deserialize_bool")]
    data_amount_specified_is_input: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    data_a_to_b: bool,
    key_token_program: String,
    key_token_authority: String,
    key_whirlpool: String,
    key_token_owner_account_a: String,
    key_vault_a: String,
    key_token_owner_account_b: String,
    key_vault_b: String,
    key_tick_array0: String,
    key_tick_array1: String,
    key_tick_array2: String,
    key_oracle: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedTwoHopSwap {
    data_amount: u64,
    data_other_amount_threshold: u64,
    #[serde(deserialize_with = "deserialize_bool")]
    data_amount_specified_is_input: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    data_a_to_b_one: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    data_a_to_b_two: bool,
    data_sqrt_price_limit_one: u128,
    data_sqrt_price_limit_two: u128,
    key_token_program: String,
    key_token_authority: String,
    key_whirlpool_one: String,
    key_whirlpool_two: String,
    key_token_owner_account_one_a: String,
    key_vault_one_a: String,
    key_token_owner_account_one_b: String,
    key_vault_one_b: String,
    key_token_owner_account_two_a: String,
    key_vault_two_a: String,
    key_token_owner_account_two_b: String,
    key_vault_two_b: String,
    key_tick_array_one_0: String,
    key_tick_array_one_1: String,
    key_tick_array_one_2: String,
    key_tick_array_two_0: String,
    key_tick_array_two_1: String,
    key_tick_array_two_2: String,
    key_oracle_one: String,
    key_oracle_two: String,
    transfer_amount_0: u64,
    transfer_amount_1: u64,
    transfer_amount_2: u64,
    transfer_amount_3: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct DecodedUpdateFeesAndRewards {
    key_whirlpool: String,
    key_position: String,
    key_tick_array_lower: String,
    key_tick_array_upper: String,
}


// 0 to false, 1 to true
fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
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

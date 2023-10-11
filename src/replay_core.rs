use solana_sdk::pubkey::Pubkey;

use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use poc_framework::{Environment, LocalEnvironment, LocalEnvironmentBuilder};

use crate::errors::ErrorCode;
use crate::{decoded_instructions::DecodedWhirlpoolInstruction, types::AccountMap};
use crate::util_replay;

use crate::programs;
use crate::replay_instructions;

pub struct WritableAccountSnapshot {
  pub pre_snapshot: AccountMap,
  pub post_snapshot: AccountMap,
}

pub struct ReplayInstructionResult {
  pub transaction_status: EncodedConfirmedTransactionWithStatusMeta,
  pub snapshot: WritableAccountSnapshot,
}

pub struct ReplayInstructionParams<'info, T> {
  pub env_builder: &'info mut LocalEnvironmentBuilder,
  pub decoded_instruction: &'info T,
  pub account_map: &'info AccountMap,
}

const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey = solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn replay_whirlpool_instruction(
  instruction: DecodedWhirlpoolInstruction,
  account_map: &AccountMap, // readonly
  clock_unixtimestamp: i64,
  whirlpool_program_so: &[u8],
  token_metadata_program_so: &[u8],
) -> Result<ReplayInstructionResult, ErrorCode> {
  let mut builder = LocalEnvironment::builder();

  // emulate SYSVAR/Clock
  builder.set_creation_time(clock_unixtimestamp);

  // deploy programs: SPL Token & SPL Associated Token Account
  // TODO: receive as params
  util_replay::add_upgradable_program(&mut builder, SPL_TOKEN_PROGRAM_ID, programs::SPL_TOKEN);
  util_replay::add_upgradable_program(&mut builder, SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, programs::SPL_ASSOCIATED_TOKEN_ACCOUNT);
  // deploy programs: Orca Whirlpool & Metaplex Token Metadata
  util_replay::add_upgradable_program(&mut builder, ORCA_WHIRLPOOL_PROGRAM_ID, whirlpool_program_so);
  util_replay::add_upgradable_program(&mut builder, METAPLEX_METADATA_PROGRAM_ID, token_metadata_program_so);

  match instruction {
    // major instructions
    DecodedWhirlpoolInstruction::Swap(decoded) => Ok(replay_instructions::swap::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::TwoHopSwap(decoded) => Ok(replay_instructions::two_hop_swap::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::UpdateFeesAndRewards(decoded) => Ok(replay_instructions::update_fees_and_rewards::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::CollectFees(decoded) => Ok(replay_instructions::collect_fees::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::CollectReward(decoded) => Ok(replay_instructions::collect_reward::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::IncreaseLiquidity(decoded) => Ok(replay_instructions::increase_liquidity::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::DecreaseLiquidity(decoded) => Ok(replay_instructions::decrease_liquidity::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::OpenPosition(decoded) => Ok(replay_instructions::open_position::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::OpenPositionWithMetadata(decoded) => Ok(replay_instructions::open_position_with_metadata::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::ClosePosition(decoded) => Ok(replay_instructions::close_position::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    DecodedWhirlpoolInstruction::CollectProtocolFees(decoded) => Ok(replay_instructions::collect_protocol_fees::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    // InitializeTickArray
    // minor instructions
    // InitializePool
    DecodedWhirlpoolInstruction::InitializeReward(decoded) => Ok(replay_instructions::initialize_reward::replay(ReplayInstructionParams { env_builder: &mut builder, decoded_instruction: &decoded, account_map: &account_map })),
    // SetRewardEmissions

    // InitializePositionBundle
    // InitializePositionBundleWithMetadata
    // OpenBundledPosition
    // CloseBundledPosition


    // ---------------------------------
    // DeletePositionBundle
    // InitializeConfig
    // InitializeFeeTier
    // SetCollectProtocolFeesAuthority
    // SetDefaultFeeRate
    // SetDefaultProtocolFeeRate
    // SetFeeAuthority
    // SetFeeRate
    // SetProtocolFeeRate
    // SetRewardAuthority
    // SetRewardAuthorityBySuperAuthority
    // SetRewardEmissionsSuperAuthority
    // AdminIncreaseLiquidity
    _ => {
      
      Err(ErrorCode::UnknownWhirlpoolInstruction("not implemented yet".to_string()))
    }
  }
}

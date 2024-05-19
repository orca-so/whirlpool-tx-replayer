use anyhow::Result;
use solana_transaction_status::ConfirmedTransactionWithStatusMeta;

use std::str::FromStr;

use crate::account_data_store::AccountDataStore;
use crate::errors::ErrorCode;
use crate::{decoded_instructions::DecodedWhirlpoolInstruction, types::AccountSnapshot};
use solana_sdk::{pubkey::Pubkey, transaction::Transaction, instruction::{Instruction, AccountMeta}, message::Message};
use solana_sdk::signer::Signer;

use anchor_lang::{InstructionData, ToAccountMetas};

use crate::replay_instructions;
use crate::replay_environment;
use crate::replay_environment::ReplayEnvironment;

use crate::pubkeys;

#[derive(Clone)]
pub struct WritableAccountSnapshot {
  pub pre_snapshot: AccountSnapshot,
  pub post_snapshot: AccountSnapshot,
}

#[derive(Clone)]
pub struct ReplayInstructionResult {
  pub transaction_status: ConfirmedTransactionWithStatusMeta,
  pub snapshot: WritableAccountSnapshot,
}

pub struct ReplayInstructionParams<'info, T> {
  pub replayer: &'info mut replay_environment::ReplayEnvironment,
  pub decoded_instruction: &'info T,
  pub accounts: &'info AccountDataStore,
}

pub fn replay_whirlpool_instruction(
  replayer: &mut replay_environment::ReplayEnvironment,
  instruction: &DecodedWhirlpoolInstruction,
  accounts: &AccountDataStore, // readonly
) -> Result<ReplayInstructionResult, ErrorCode> {
  match instruction {
    // major instructions
    DecodedWhirlpoolInstruction::Swap(decoded) => Ok(replay_instructions::swap::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::TwoHopSwap(decoded) => Ok(replay_instructions::two_hop_swap::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::UpdateFeesAndRewards(decoded) => Ok(replay_instructions::update_fees_and_rewards::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CollectFees(decoded) => Ok(replay_instructions::collect_fees::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CollectReward(decoded) => Ok(replay_instructions::collect_reward::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CollectProtocolFees(decoded) => Ok(replay_instructions::collect_protocol_fees::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::IncreaseLiquidity(decoded) => Ok(replay_instructions::increase_liquidity::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::DecreaseLiquidity(decoded) => Ok(replay_instructions::decrease_liquidity::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::OpenPosition(decoded) => Ok(replay_instructions::open_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::OpenPositionWithMetadata(decoded) => Ok(replay_instructions::open_position_with_metadata::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::ClosePosition(decoded) => Ok(replay_instructions::close_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::OpenBundledPosition(decoded) => Ok(replay_instructions::open_bundled_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CloseBundledPosition(decoded) => Ok(replay_instructions::close_bundled_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeTickArray(decoded) => Ok(replay_instructions::initialize_tick_array::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // minor instructions
    DecodedWhirlpoolInstruction::InitializePool(decoded) => Ok(replay_instructions::initialize_pool::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeReward(decoded) => Ok(replay_instructions::initialize_reward::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetRewardEmissions(decoded) => Ok(replay_instructions::set_reward_emissions::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializePositionBundle(decoded) => Ok(replay_instructions::initialize_position_bundle::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializePositionBundleWithMetadata(decoded) => Ok(replay_instructions::initialize_position_bundle_with_metadata::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::DeletePositionBundle(decoded) => Ok(replay_instructions::delete_position_bundle::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // very rare instructions
    DecodedWhirlpoolInstruction::InitializeFeeTier(decoded) => Ok(replay_instructions::initialize_fee_tier::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetFeeRate(decoded) => Ok(replay_instructions::set_fee_rate::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeConfig(decoded) => Ok(replay_instructions::initialize_config::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetCollectProtocolFeesAuthority(decoded) => Ok(replay_instructions::set_collect_protocol_fees_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetDefaultFeeRate(decoded) => Ok(replay_instructions::set_default_fee_rate::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetDefaultProtocolFeeRate(decoded) => Ok(replay_instructions::set_default_protocol_fee_rate::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetFeeAuthority(decoded) => Ok(replay_instructions::set_fee_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetProtocolFeeRate(decoded) => Ok(replay_instructions::set_protocol_fee_rate::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetRewardAuthority(decoded) => Ok(replay_instructions::set_reward_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetRewardAuthorityBySuperAuthority(decoded) => Ok(replay_instructions::set_reward_authority_by_super_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetRewardEmissionsSuperAuthority(decoded) => Ok(replay_instructions::set_reward_emissions_super_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // temporary patch instructions
    DecodedWhirlpoolInstruction::AdminIncreaseLiquidity(decoded) => Ok(replay_instructions::admin_increase_liquidity::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    //_ => {
    //  Err(ErrorCode::UnknownWhirlpoolInstruction("not implemented yet".to_string()))
    //}
  }
}



impl ReplayEnvironment {
  pub fn set_whirlpool_account(&mut self, pubkey: &String, accounts: &AccountDataStore) {
    self.set_account_with_data(
      Pubkey::from_str(pubkey).unwrap(),
      pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
      &accounts.get(pubkey).unwrap().unwrap(),
      false
    );
  }

  pub fn set_funder_account(
    &mut self,
    pubkey: &String,
  ) {
    self.set_account_with_lamports(
      solana_program::pubkey::Pubkey::from_str(pubkey.as_str()).unwrap(),
      pubkeys::SYSTEM_PROGRAM_ID,
      10_000_000_000, // 10 SOL
    );
  }
  
  pub fn build_whirlpool_replay_transaction(
    &mut self,
    args: impl InstructionData,
    accounts: impl ToAccountMetas,
  ) -> Transaction {
    let payer = self.payer();
    let recent_blockhash = self.get_latest_blockhash();

    let whirlpool_instruction = Instruction {
      program_id: pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
      data: args.data(), // using Anchor, at least instruction code (8 bytes)
      accounts: accounts.to_account_metas(None),
    };

    // to avoid duplicated transaction signature for instructions with same args & accounts
    let nonce = format!("{:x}", self.get_next_nonce());
    let memo_instruction = Instruction {
      program_id: pubkeys::SPL_MEMO_PROGRAM_ID,
      data: nonce.as_bytes().to_vec(),
      accounts: vec![AccountMeta::new(payer.pubkey(), true)],
    };

    // create transaction with only sign of payer
    let message = Message::new(&[whirlpool_instruction, memo_instruction], Some(&payer.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    tx.partial_sign(&[&payer], recent_blockhash);

    return tx;
  }

  pub fn take_snapshot(
    &self,
    pubkeys: &[&String],
  ) -> AccountSnapshot {
    let mut snapshot = AccountSnapshot::new();
  
    for pubkey_string in pubkeys {
      let account = self.get_account(Pubkey::from_str(pubkey_string).unwrap()).unwrap();
      snapshot.insert((*pubkey_string).clone(), account.data);
    }
  
    return snapshot;
  }

}
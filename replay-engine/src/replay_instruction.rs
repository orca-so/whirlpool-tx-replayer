use anyhow::Result;
use solana_accounts_db::transaction_results::TransactionExecutionResult;
use solana_program::program_option::COption;
use std::str::FromStr;

use crate::account_data_store::AccountDataStore;
use crate::errors::ErrorCode;
use crate::types::WritableAccountSnapshot;
use crate::{decoded_instructions::DecodedWhirlpoolInstruction, types::AccountSnapshot};
use solana_sdk::{pubkey::Pubkey, transaction::Transaction, instruction::{Instruction, AccountMeta}, message::Message};
use solana_sdk::signer::Signer;

use anchor_lang::{InstructionData, ToAccountMetas, AnchorSerialize};

use crate::replay_instructions;
use crate::replay_environment;
use crate::replay_environment::ReplayEnvironment;

use crate::pubkeys;

#[derive(Clone)]
pub struct ReplayInstructionResult {
  pub execution_result: TransactionExecutionResult,
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
    // v2 instructions
    DecodedWhirlpoolInstruction::CollectFeesV2(decoded) => Ok(replay_instructions::collect_fees_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CollectProtocolFeesV2(decoded) => Ok(replay_instructions::collect_protocol_fees_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::CollectRewardV2(decoded) => Ok(replay_instructions::collect_reward_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::DecreaseLiquidityV2(decoded) => Ok(replay_instructions::decrease_liquidity_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::IncreaseLiquidityV2(decoded) => Ok(replay_instructions::increase_liquidity_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SwapV2(decoded) => Ok(replay_instructions::swap_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::TwoHopSwapV2(decoded) => Ok(replay_instructions::two_hop_swap_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializePoolV2(decoded) => Ok(replay_instructions::initialize_pool_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeRewardV2(decoded) => Ok(replay_instructions::initialize_reward_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetRewardEmissionsV2(decoded) => Ok(replay_instructions::set_reward_emissions_v2::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeConfigExtension(decoded) => Ok(replay_instructions::initialize_config_extension::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializeTokenBadge(decoded) => Ok(replay_instructions::initialize_token_badge::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::DeleteTokenBadge(decoded) => Ok(replay_instructions::delete_token_badge::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetConfigExtensionAuthority(decoded) => Ok(replay_instructions::set_config_extension_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetTokenBadgeAuthority(decoded) => Ok(replay_instructions::set_token_badge_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // TokenExtensions based Position NFT instructions
    DecodedWhirlpoolInstruction::OpenPositionWithTokenExtensions(decoded) => Ok(replay_instructions::open_position_with_token_extensions::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::ClosePositionWithTokenExtensions(decoded) => Ok(replay_instructions::close_position_with_token_extensions::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // Liquidity Lock
    DecodedWhirlpoolInstruction::LockPosition(decoded) => Ok(replay_instructions::lock_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // Reset Position Range
    DecodedWhirlpoolInstruction::ResetPositionRange(decoded) => Ok(replay_instructions::reset_position_range::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // Transfer Locked Position
    DecodedWhirlpoolInstruction::TransferLockedPosition(decoded) => Ok(replay_instructions::transfer_locked_position::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // Adaptive Fee instructions
    DecodedWhirlpoolInstruction::InitializeAdaptiveFeeTier(decoded) => Ok(replay_instructions::initialize_adaptive_fee_tier::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::InitializePoolWithAdaptiveFee(decoded) => Ok(replay_instructions::initialize_pool_with_adaptive_fee::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetInitializePoolAuthority(decoded) => Ok(replay_instructions::set_initialize_pool_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetDelegatedFeeAuthority(decoded) => Ok(replay_instructions::set_delegated_fee_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetDefaultBaseFeeRate(decoded) => Ok(replay_instructions::set_default_base_fee_rate::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetFeeRateByDelegatedFeeAuthority(decoded) => Ok(replay_instructions::set_fee_rate_by_delegated_fee_authority::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    DecodedWhirlpoolInstruction::SetPresetAdaptiveFeeConstants(decoded) => Ok(replay_instructions::set_preset_adaptive_fee_constants::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // Dynamic Tick Array
    DecodedWhirlpoolInstruction::InitializeDynamicTickArray(decoded) => Ok(replay_instructions::initialize_dynamic_tick_array::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    // temporary patch instructions
    DecodedWhirlpoolInstruction::AdminIncreaseLiquidity(decoded) => Ok(replay_instructions::admin_increase_liquidity::replay(ReplayInstructionParams { replayer, decoded_instruction: &decoded, accounts })),
    //_ => {
    //  Err(ErrorCode::UnknownWhirlpoolInstruction("not implemented yet".to_string()))
    //}
  }
}

impl ReplayInstructionResult {
  pub fn new(
    execution_result: TransactionExecutionResult,
    pre_snapshot: AccountSnapshot,
    post_snapshot: AccountSnapshot,
  ) -> Self {
    Self {
      execution_result,
      snapshot: WritableAccountSnapshot {
        pre_snapshot,
        post_snapshot,
      },
    }
  }
}

#[derive(Debug, Clone, Copy)]
pub enum TokenTrait {
  Token,
  TokenExtensions,
  TokenExtensionsWithTransferFee(u16, u64), // bps, max
  TokenExtensionsWithCloseAuthority(Pubkey), // close authority
}

impl ReplayEnvironment {
  pub fn set_token_mint(
    &mut self,
    pubkey: Pubkey,
    mint_authority: Option<Pubkey>,
    supply: u64,
    decimals: u8,
    freeze_authority: Option<Pubkey>,
  ) -> &mut Self {
      self.set_account_with_packable(
          pubkey,
          spl_token::ID,
          spl_token::state::Mint {
              mint_authority: COption::from(mint_authority.map(|c| c.clone())),
              supply,
              decimals,
              is_initialized: true,
              freeze_authority: COption::from(freeze_authority.map(|c| c.clone())),
          },
      )
  }

  pub fn set_token_mint_with_trait(
    &mut self,
    pubkey: Pubkey,
    token_trait: TokenTrait,
    mint_authority: Option<Pubkey>,
    supply: u64,
    decimals: u8,
    freeze_authority: Option<Pubkey>,
  ) -> &mut Self {
    match token_trait {
      TokenTrait::Token => { self.set_token_mint(pubkey, mint_authority, supply, decimals, freeze_authority) }
      TokenTrait::TokenExtensions => {
        self.set_account_with_packable(
          pubkey,
          spl_token_2022::ID,
          spl_token_2022::state::Mint {
              mint_authority: COption::from(mint_authority.map(|c| c.clone())),
              supply,
              decimals,
              is_initialized: true,
              freeze_authority: COption::from(freeze_authority.map(|c| c.clone())),
          },
        )
      }
      TokenTrait::TokenExtensionsWithTransferFee(transfer_fee_basis_point, maximum_fee) => {
        #[derive(Default, AnchorSerialize)]
        struct MintWithTransferFeeConfigLayout {
            // 82 for Mint
            pub coption_mint_authority: u32, // 4
            pub mint_authority: Pubkey, // 32
            pub supply: u64, // 8
            pub decimals: u8, // 1
            pub is_initialized: bool, // 1
            pub coption_freeze_authority: u32, // 4
            pub freeze_authority: Pubkey, // 32
    
            // 83 for padding
            pub padding1: [u8; 32],
            pub padding2: [u8; 32],
            pub padding3: [u8; 19],
    
            pub account_type: u8, // 1
    
            pub extension_type: u16, // 2
            pub extension_length: u16, // 2
            // 108 for TransferFeeConfig data
            pub transfer_fee_config_authority: Pubkey, // 32
            pub withdraw_withheld_authority: Pubkey, // 32
            pub withheld_amount: u64, // 8
            pub older_epoch: u64, // 8
            pub older_maximum_fee: u64, // 8
            pub older_transfer_fee_basis_point: u16, // 2
            pub newer_epoch: u64, // 8
            pub newer_maximum_fee: u64, // 8
            pub newer_transfer_fee_basis_point: u16, // 2
        }
        impl MintWithTransferFeeConfigLayout {
            pub const LEN: usize = 82 + 83 + 1 + 2 + 2 + 108; // 278
        }

        let (coption_mint_authority, mint_authority) = if let Some(authority) = mint_authority {
          (1u32, authority)
        } else {
          (0u32, Pubkey::default())
        };
        let (coption_freeze_authority, freeze_authority) = if let Some(authority) = freeze_authority {
          (1u32, authority)
        } else {
          (0u32, Pubkey::default())
        };

        let data = MintWithTransferFeeConfigLayout {
          coption_mint_authority,
          mint_authority,
          supply,
          decimals,
          is_initialized: true,
          coption_freeze_authority,
          freeze_authority,
          // extension part
          account_type: 1, // Mint
          extension_type: 1, // TransferFeeConfig
          extension_length: 108,
          older_epoch: 0,
          older_maximum_fee: maximum_fee,
          older_transfer_fee_basis_point: transfer_fee_basis_point,
          newer_epoch: 0,
          newer_maximum_fee: maximum_fee,
          newer_transfer_fee_basis_point: transfer_fee_basis_point,
          ..Default::default()
        };

        let mut packed = Vec::with_capacity(MintWithTransferFeeConfigLayout::LEN);
        data.serialize(&mut packed).unwrap();
        self.set_account_with_data(
          pubkey,
          spl_token_2022::ID,
          &packed,
          false
        )
      }
      TokenTrait::TokenExtensionsWithCloseAuthority(close_authority) => {
        #[derive(Default, AnchorSerialize)]
        struct MintWithMintCloseAuthorityLayout {
            // 82 for Mint
            pub coption_mint_authority: u32, // 4
            pub mint_authority: Pubkey, // 32
            pub supply: u64, // 8
            pub decimals: u8, // 1
            pub is_initialized: bool, // 1
            pub coption_freeze_authority: u32, // 4
            pub freeze_authority: Pubkey, // 32
    
            // 83 for padding
            pub padding1: [u8; 32],
            pub padding2: [u8; 32],
            pub padding3: [u8; 19],
    
            pub account_type: u8, // 1
    
            pub extension_type: u16, // 2
            pub extension_length: u16, // 2
            // 32 for MintCloseAuthority data
            pub close_authority: Pubkey, // 32
        }
        impl MintWithMintCloseAuthorityLayout {
            pub const LEN: usize = 82 + 83 + 1 + 2 + 2 + 32; // 202
        }

        let (coption_mint_authority, mint_authority) = if let Some(authority) = mint_authority {
          (1u32, authority)
        } else {
          (0u32, Pubkey::default())
        };
        let (coption_freeze_authority, freeze_authority) = if let Some(authority) = freeze_authority {
          (1u32, authority)
        } else {
          (0u32, Pubkey::default())
        };

        let data = MintWithMintCloseAuthorityLayout {
          coption_mint_authority,
          mint_authority,
          supply,
          decimals,
          is_initialized: true,
          coption_freeze_authority,
          freeze_authority,
          // extension part
          account_type: 1, // Mint
          extension_type: 3, // MintCloseAuthority
          extension_length: 32,
          close_authority,
          ..Default::default()
        };

        let mut packed = Vec::with_capacity(MintWithMintCloseAuthorityLayout::LEN);
        data.serialize(&mut packed).unwrap();
        self.set_account_with_data(
          pubkey,
          spl_token_2022::ID,
          &packed,
          false
        )
      }
    }
  }

  // Add a token-account into the environment.
  pub fn set_token_account(
      &mut self,
      pubkey: Pubkey,
      mint: Pubkey,
      owner: Pubkey,
      amount: u64,
  ) -> &mut Self {
      self.set_account_with_packable(
          pubkey,
          spl_token::ID,
          spl_token::state::Account {
              mint,
              owner,
              amount,
              delegate: COption::None,
              state: spl_token::state::AccountState::Initialized,
              is_native: COption::None,
              delegated_amount: 0,
              close_authority: COption::None,
          },
      )
  }

  // TODO: refactor (integrate with set_token_account_with_trait)
  pub fn set_frozen_token_account_2022(
    &mut self,
    pubkey: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
  ) -> &mut Self {
    self.set_account_with_packable(
      pubkey,
      spl_token_2022::ID,
      spl_token_2022::state::Account {
          mint,
          owner,
          amount,
          delegate: COption::None,
          state: spl_token_2022::state::AccountState::Frozen,
          is_native: COption::None,
          delegated_amount: 0,
          close_authority: COption::None,
      },
    )
  }

  // TODO: refactor (integrate with set_token_account_with_trait)
  pub fn set_delegated_token_account_2022(
    &mut self,
    pubkey: Pubkey,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
    delegate: Pubkey,
    delegated_amount: u64,
  ) -> &mut Self {
    self.set_account_with_packable(
      pubkey,
      spl_token_2022::ID,
      spl_token_2022::state::Account {
          mint,
          owner,
          amount,
          delegate: COption::Some(delegate),
          delegated_amount,
          state: spl_token_2022::state::AccountState::Initialized,
          is_native: COption::None,
          close_authority: COption::None,
      },
    )
  }
  
  pub fn set_token_account_with_trait(
    &mut self,
    pubkey: Pubkey,
    token_trait: TokenTrait,
    mint: Pubkey,
    owner: Pubkey,
    amount: u64,
  ) -> &mut Self {
    match token_trait {
      TokenTrait::Token => { self.set_token_account(pubkey, mint, owner, amount) }
      TokenTrait::TokenExtensions | TokenTrait::TokenExtensionsWithCloseAuthority(_) => {
        self.set_account_with_packable(
          pubkey,
          spl_token_2022::ID,
          spl_token_2022::state::Account {
              mint,
              owner,
              amount,
              delegate: COption::None,
              state: spl_token_2022::state::AccountState::Initialized,
              is_native: COption::None,
              delegated_amount: 0,
              close_authority: COption::None,
          },
        )
      }
      TokenTrait::TokenExtensionsWithTransferFee(_transfer_fee_basis_point, _maximum_fee) => {
        #[derive(Default, AnchorSerialize)]
        struct AccountWithTransferFeeAmountLayout {
            // 165 for Account
            pub mint: Pubkey, // 32
            pub owner: Pubkey, // 32
            pub amount: u64, // 8
            pub coption_delegate: u32, // 4
            pub delegate: Pubkey, // 32
            pub state: u8, // 1
            pub coption_is_native: u32, // 4
            pub is_native: u64, // 8
            pub delegated_amount: u64, // 8
            pub coption_close_authority: u32, // 4
            pub close_authority: Pubkey, // 32
    
            pub account_type: u8, // 1
    
            pub extension_type: u16, // 2
            pub extension_length: u16, // 2
            // 8 for TransferFeeAmount data
            pub withheld_amount: u64, // 8
        }
        impl AccountWithTransferFeeAmountLayout {
            pub const LEN: usize = 165 + 1 + 2 + 2 + 8; // 178
        }

        let data = AccountWithTransferFeeAmountLayout {
          mint,
          owner,
          amount,
          state: 1, // Initialized
          // extension part
          account_type: 2, // Account
          extension_type: 2, // TransferFeeAmount
          extension_length: 8,
          withheld_amount: 0,
          ..Default::default()
        };

        let mut packed = Vec::with_capacity(AccountWithTransferFeeAmountLayout::LEN);
        data.serialize(&mut packed).unwrap();
        self.set_account_with_data(
          pubkey,
          spl_token_2022::ID,
          &packed,
          false
        )
      }
    }
  }

  pub fn set_whirlpool_account(&mut self, pubkey: &String, accounts: &AccountDataStore) {
    self.set_account_with_data(
      Pubkey::from_str(pubkey).unwrap(),
      pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
      &accounts.get(pubkey).unwrap().unwrap(),
      false
    );
  }

  pub fn set_whirlpool_account_with_additional_lamports(&mut self, pubkey: &String, accounts: &AccountDataStore) {
    self.set_account_with_data_with_additional_lamports(
      Pubkey::from_str(pubkey).unwrap(),
      pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
      &accounts.get(pubkey).unwrap().unwrap(),
      false,
      1_000_000_000, // 1 SOL
    );
  }

  pub fn set_whirlpool_account_if_exists(&mut self, pubkey: &String, accounts: &AccountDataStore) -> bool {
    if let Some(data) = accounts.get(pubkey).unwrap() {
      self.set_account_with_data(
        Pubkey::from_str(pubkey).unwrap(),
        pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
        &data,
        false
      );

      return true;
    }

    return false;
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
    self.build_whirlpool_replay_transaction_with_remaining_accounts(args, accounts, vec![])
  }

  pub fn build_whirlpool_replay_transaction_with_remaining_accounts(
    &mut self,
    args: impl InstructionData,
    accounts: impl ToAccountMetas,
    remaining_accounts: Vec<AccountMeta>,
  ) -> Transaction {
    let payer = self.payer();
    let recent_blockhash = self.get_latest_blockhash();

    let mut whirlpool_instruction = Instruction {
      program_id: pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID,
      data: args.data(), // using Anchor, at least instruction code (8 bytes)
      accounts: accounts.to_account_metas(None),
    };

    // add remaining accounts
    whirlpool_instruction.accounts.extend(remaining_accounts);

    // to avoid duplicated transaction signature for instructions with same args & accounts
    let nonce = format!("{:x}", self.get_next_nonce());
    let memo_instruction = Instruction {
      program_id: pubkeys::SPL_MEMO_PROGRAM_ID,
      data: nonce.as_bytes().to_vec(),
      accounts: vec![AccountMeta::new(payer.pubkey(), true)],
    };

    // create transaction with only sign of payer --> setting blockhash only
    let message = Message::new(&[whirlpool_instruction, memo_instruction], Some(&payer.pubkey()));
    let mut tx = Transaction::new_unsigned(message);
    //tx.partial_sign(&[&payer], recent_blockhash);
    tx.message.recent_blockhash = recent_blockhash;

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
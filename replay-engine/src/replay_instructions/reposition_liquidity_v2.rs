use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

use anchor_lang::{InstructionData, ToAccountMetas, Discriminator, AnchorSerialize};
use crate::util;
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize)]
struct RepositionLiquidityV2InstructionArgs {
  pub new_tick_lower_index: i32,
  pub new_tick_upper_index: i32,
  pub method: RepositionLiquidityMethod,
  pub remaining_accounts_info: Option<u8>, // dummy, not used for replay (always None)
}
#[derive(AnchorSerialize)]
enum RepositionLiquidityMethod {
  ByLiquidity {
    new_liquidity_amount: u128,
    existing_range_token_min_a: u64,
    existing_range_token_min_b: u64,
    new_range_token_max_a: u64,
    new_range_token_max_b: u64,
  },
}
impl Discriminator for RepositionLiquidityV2InstructionArgs {
  const DISCRIMINATOR: [u8; 8] = [0xbf, 0xa9, 0xe0, 0x0b, 0x83, 0x13, 0x9e, 0xfd];
}
impl InstructionData for RepositionLiquidityV2InstructionArgs {}

struct RepositionLiquidityV2InstructionAccounts {
  pub whirlpool: Pubkey,
  pub token_program_a: Pubkey,
  pub token_program_b: Pubkey,
  pub memo_program: Pubkey,
  pub position_authority: Pubkey,
  pub funder: Pubkey,
  pub position: Pubkey,
  pub position_token_account: Pubkey,
  pub token_mint_a: Pubkey,
  pub token_mint_b: Pubkey,
  pub token_owner_account_a: Pubkey,
  pub token_owner_account_b: Pubkey,
  pub token_vault_a: Pubkey,
  pub token_vault_b: Pubkey,
  pub existing_tick_array_lower: Pubkey,
  pub existing_tick_array_upper: Pubkey,
  pub new_tick_array_lower: Pubkey,
  pub new_tick_array_upper: Pubkey,
  pub system_program: Pubkey,
}
impl ToAccountMetas for RepositionLiquidityV2InstructionAccounts {
  fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<solana_program::instruction::AccountMeta> {
    vec![
      solana_program::instruction::AccountMeta::new(self.whirlpool, false),
      solana_program::instruction::AccountMeta::new_readonly(self.token_program_a, false),
      solana_program::instruction::AccountMeta::new_readonly(self.token_program_b, false),
      solana_program::instruction::AccountMeta::new_readonly(self.memo_program, false),
      solana_program::instruction::AccountMeta::new_readonly(self.position_authority, is_signer.unwrap_or(true)),
      solana_program::instruction::AccountMeta::new(self.funder, is_signer.unwrap_or(true)),
      solana_program::instruction::AccountMeta::new(self.position, false),
      solana_program::instruction::AccountMeta::new_readonly(self.position_token_account, false),
      solana_program::instruction::AccountMeta::new_readonly(self.token_mint_a, false),
      solana_program::instruction::AccountMeta::new_readonly(self.token_mint_b, false),
      solana_program::instruction::AccountMeta::new(self.token_owner_account_a, false),
      solana_program::instruction::AccountMeta::new(self.token_owner_account_b, false),
      solana_program::instruction::AccountMeta::new(self.token_vault_a, false),
      solana_program::instruction::AccountMeta::new(self.token_vault_b, false),
      solana_program::instruction::AccountMeta::new(self.existing_tick_array_lower, false),
      solana_program::instruction::AccountMeta::new(self.existing_tick_array_upper, false),
      solana_program::instruction::AccountMeta::new(self.new_tick_array_lower, false),
      solana_program::instruction::AccountMeta::new(self.new_tick_array_upper, false),
      solana_program::instruction::AccountMeta::new_readonly(self.system_program, false),
    ]
  }
}

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedRepositionLiquidityV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  // This instruction transfer tokens depending on the method and the position state,
  // so we need to determine the transfer direction and amount based on the decoded instruction data in order to set up the correct pre-state for replay.
  let amount_a = ix.transfer_0.amount;
  let (vault_amount_a, owner_amount_a) = if ix.aux_data_is_token_a_transfer_from_owner {
    (0u64, amount_a)
  } else {
    (amount_a, 0u64)
  };
  let amount_b = ix.transfer_1.amount;
  let (vault_amount_b, owner_amount_b) = if ix.aux_data_is_token_b_transfer_from_owner {
    (0u64, amount_b)
  } else {
    (amount_b, 0u64)
  };

  let token_trait_a = util::determine_token_trait(&ix.key_token_program_a, &ix.transfer_0);
  let token_trait_b = util::determine_token_trait(&ix.key_token_program_b, &ix.transfer_1);

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // token_program_a
  // token_program_b
  // memo_program
  // position_authority
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // position
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_position, accounts); // add lamports to initialize 2 ticks if needed
  // position_token_amount
  replayer.set_token_account(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // token_mint_a
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_token_mint_a),
    token_trait_a,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_b
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_token_mint_b),
    token_trait_b,
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_owner_account_a
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_a),
    token_trait_a,
    mint_a,
    pubkey(&ix.key_position_authority),
    owner_amount_a
  );
  // token_owner_account_b
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_owner_account_b),
    token_trait_b,
    mint_b,
    pubkey(&ix.key_position_authority),
    owner_amount_b
  );
  // token_vault_a
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_vault_a),
    token_trait_a,
    mint_a,
    pubkey(&ix.key_whirlpool),
    vault_amount_a
  );
  // token_vault_b
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_token_vault_b),
    token_trait_b,
    mint_b,
    pubkey(&ix.key_whirlpool),
    vault_amount_b
  );
  // existing_tick_array_lower
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_existing_tick_array_lower, accounts); // add lamports to collect rent of 2 ticks
  // existing_tick_array_upper
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_existing_tick_array_upper, accounts); // add lamports to collect rent of 2 ticks
  // new_tick_array_lower
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_new_tick_array_lower, accounts); // add lamports to collect rent of 2 ticks
  // new_tick_array_upper
  replayer.set_whirlpool_account_with_additional_lamports(&ix.key_new_tick_array_upper, accounts); // add lamports to collect rent of 2 ticks
  // system_program

  let method = match ix.data_method {
    decoded_instructions::RepositionLiquidityMethod::ByLiquidity { new_liquidity_amount, existing_range_token_min_a, existing_range_token_min_b, new_range_token_max_a, new_range_token_max_b } => {
      RepositionLiquidityMethod::ByLiquidity {
        new_liquidity_amount,
        existing_range_token_min_a,
        existing_range_token_min_b,
        new_range_token_max_a,
        new_range_token_max_b,
      }
    },
  };

  let tx = replayer.build_whirlpool_replay_transaction(
     RepositionLiquidityV2InstructionArgs {
      new_tick_lower_index: ix.data_new_tick_lower_index,
      new_tick_upper_index: ix.data_new_tick_upper_index,
      method,
      // don't replay transfer hook
      remaining_accounts_info: None,
    },
    RepositionLiquidityV2InstructionAccounts {
      whirlpool: pubkey(&ix.key_whirlpool),
      token_program_a: pubkey(&ix.key_token_program_a),
      token_program_b: pubkey(&ix.key_token_program_b),
      memo_program: pubkey(&ix.key_memo_program),
      position_authority: pubkey(&ix.key_position_authority),
      funder: pubkey(&ix.key_funder),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      token_owner_account_a: pubkey(&ix.key_token_owner_account_a),
      token_owner_account_b: pubkey(&ix.key_token_owner_account_b),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      existing_tick_array_lower: pubkey(&ix.key_existing_tick_array_lower),
      existing_tick_array_upper: pubkey(&ix.key_existing_tick_array_upper),
      new_tick_array_lower: pubkey(&ix.key_new_tick_array_lower),
      new_tick_array_upper: pubkey(&ix.key_new_tick_array_upper),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_existing_tick_array_lower,
    &ix.key_existing_tick_array_upper,
    &ix.key_new_tick_array_lower,
    &ix.key_new_tick_array_upper,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_existing_tick_array_lower,
    &ix.key_existing_tick_array_upper,
    &ix.key_new_tick_array_lower,
    &ix.key_new_tick_array_upper,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

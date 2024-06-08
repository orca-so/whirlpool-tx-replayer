use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

use anchor_lang::{InstructionData, ToAccountMetas, Discriminator, AnchorSerialize};
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize)]
struct AdminIncreaseLiquidityInstructionArgs {
  pub liquidity: u128,
}
impl Discriminator for AdminIncreaseLiquidityInstructionArgs {
  const DISCRIMINATOR: [u8; 8] = [0xe5, 0x81, 0x99, 0x9d, 0x99, 0x6a, 0x61, 0xa0];
}
impl InstructionData for AdminIncreaseLiquidityInstructionArgs {}

struct AdminIncreaseLiquidityInstructionAccounts {
  pub whirlpools_config: Pubkey,
  pub whirlpool: Pubkey,
  pub authority: Pubkey,
}
impl ToAccountMetas for AdminIncreaseLiquidityInstructionAccounts {
  fn to_account_metas(&self, is_signer: Option<bool>) -> Vec<solana_program::instruction::AccountMeta> {
    let mut metas = Vec::new();
    metas.push(solana_program::instruction::AccountMeta::new_readonly(self.whirlpools_config, false));
    metas.push(solana_program::instruction::AccountMeta::new(self.whirlpool, false));
    metas.push(solana_program::instruction::AccountMeta::new_readonly(self.authority, is_signer.unwrap_or(true)));
    return metas;
  }
}

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedAdminIncreaseLiquidity>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // authority
    
  let tx = replayer.build_whirlpool_replay_transaction(
    AdminIncreaseLiquidityInstructionArgs {
      liquidity: ix.data_liquidity,
    },
    AdminIncreaseLiquidityInstructionAccounts {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      whirlpool: pubkey(&ix.key_whirlpool),
      authority: pubkey(&ix.key_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

use anchor_lang::{InstructionData, ToAccountMetas, Discriminator, AnchorSerialize};
use anchor_lang::solana_program::pubkey::Pubkey;

#[derive(AnchorSerialize)]
struct MigrateRepurposeRewardAuthoritySpaceInstructionArgs {
}
impl Discriminator for MigrateRepurposeRewardAuthoritySpaceInstructionArgs {  
  const DISCRIMINATOR: [u8; 8] = [0xd6, 0xa1, 0xf8, 0x4f, 0x98, 0x62, 0xac, 0xe7];
}
impl InstructionData for MigrateRepurposeRewardAuthoritySpaceInstructionArgs {}

struct MigrateRepurposeRewardAuthoritySpaceInstructionAccounts {
  pub whirlpool: Pubkey,
}
impl ToAccountMetas for MigrateRepurposeRewardAuthoritySpaceInstructionAccounts {
  fn to_account_metas(&self, _is_signer: Option<bool>) -> Vec<solana_program::instruction::AccountMeta> {
    let mut metas = Vec::new();
    metas.push(solana_program::instruction::AccountMeta::new(self.whirlpool, false));
    return metas;
  }
}

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedMigrateRepurposeRewardAuthoritySpace>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
    
  let tx = replayer.build_whirlpool_replay_transaction(
    MigrateRepurposeRewardAuthoritySpaceInstructionArgs {
    },
    MigrateRepurposeRewardAuthoritySpaceInstructionAccounts {
      whirlpool: pubkey(&ix.key_whirlpool),
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

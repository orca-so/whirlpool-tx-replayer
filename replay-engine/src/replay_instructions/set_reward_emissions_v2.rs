use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetRewardEmissionsV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_reward = whirlpool_data.reward_infos[ix.data_reward_index as usize].mint;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // reward_authority
  // reward_vault (no need to determine Token or TokenExtensions)
  replayer.set_token_account(
    pubkey(&ix.key_reward_vault),
    mint_reward,
    pubkey(&ix.key_whirlpool),
    u64::MAX // dummy
  );
    
  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetRewardEmissionsV2 {
      reward_index: ix.data_reward_index,
      emissions_per_second_x64: ix.data_emissions_per_second_x64,
    },
    whirlpool_ix_accounts::SetRewardEmissionsV2 {
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_authority: pubkey(&ix.key_reward_authority),
      reward_vault: pubkey(&ix.key_reward_vault),
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

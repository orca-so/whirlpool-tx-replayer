use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetRewardAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // reward_authority
  // new_reward_authority
    
  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetRewardAuthority {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::SetRewardAuthority {
      whirlpool: pubkey(&ix.key_whirlpool),
      reward_authority: pubkey(&ix.key_reward_authority),
      new_reward_authority: pubkey(&ix.key_new_reward_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let transaction_status = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(transaction_status, pre_snapshot, post_snapshot)
}

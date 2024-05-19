use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedUpdateFeesAndRewards>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // position
  replayer.set_whirlpool_account(&ix.key_position, accounts);
  // tick_array_lower
  replayer.set_whirlpool_account(&ix.key_tick_array_lower, accounts);
  // tick_array_upper
  replayer.set_whirlpool_account(&ix.key_tick_array_upper, accounts);

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::UpdateFeesAndRewards {
    },
    whirlpool_ix_accounts::UpdateFeesAndRewards {
      whirlpool: pubkey(&ix.key_whirlpool),
      position: pubkey(&ix.key_position),
      tick_array_lower: pubkey(&ix.key_tick_array_lower),
      tick_array_upper: pubkey(&ix.key_tick_array_upper),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
    &ix.key_position,
    &ix.key_tick_array_lower,
    &ix.key_tick_array_upper,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

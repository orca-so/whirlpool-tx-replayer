use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetCollectProtocolFeesAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, account_map);
  // collect_protocol_fees_authority
  // new_collect_protocol_fees_authority

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetCollectProtocolFeesAuthority {
    },
    whirlpool_ix_accounts::SetCollectProtocolFeesAuthority {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      collect_protocol_fees_authority: pubkey(&ix.key_collect_protocol_fees_authority),
      new_collect_protocol_fees_authority: pubkey(&ix.key_new_collect_protocol_fees_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

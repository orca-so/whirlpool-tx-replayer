use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetDefaultProtocolFeeRate>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // fee_authority

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetDefaultProtocolFeeRate {
      default_protocol_fee_rate: ix.data_default_protocol_fee_rate,
    },
    whirlpool_ix_accounts::SetDefaultProtocolFeeRate {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      fee_authority: pubkey(&ix.key_fee_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

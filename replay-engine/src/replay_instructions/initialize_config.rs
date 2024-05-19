use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeConfig>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let _accounts = req.accounts;

  // config
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeConfig {
      fee_authority: pubkey(&ix.data_fee_authority),
      collect_protocol_fees_authority: pubkey(&ix.data_collect_protocol_fees_authority),
      reward_emissions_super_authority: pubkey(&ix.data_reward_emissions_super_authority),
      default_protocol_fee_rate: ix.data_default_protocol_fee_rate,
    },
    whirlpool_ix_accounts::InitializeConfig {
      config: pubkey(&ix.key_whirlpools_config),
      funder: pubkey(&ix.key_funder),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config, // created
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

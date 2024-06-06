use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializeConfigExtension>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // config_extension
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // fee_authority
  // system_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializeConfigExtension {
    },
    whirlpool_ix_accounts::InitializeConfigExtension {
      config: pubkey(&ix.key_whirlpools_config),
      config_extension: pubkey(&ix.key_whirlpools_config_extension),
      funder: pubkey(&ix.key_funder),
      fee_authority: pubkey(&ix.key_fee_authority),
      system_program: pubkey(&ix.key_system_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_whirlpools_config_extension, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetTokenBadgeAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // whirlpools_config_extension
  replayer.set_whirlpool_account(&ix.key_whirlpools_config_extension, accounts);
  // config_extension_authority
  // new_token_badge_authority

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetTokenBadgeAuthority {
    },
    whirlpool_ix_accounts::SetTokenBadgeAuthority {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      whirlpools_config_extension: pubkey(&ix.key_whirlpools_config_extension),
      config_extension_authority: pubkey(&ix.key_config_extension_authority),
      new_token_badge_authority: pubkey(&ix.key_new_token_badge_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_whirlpools_config_extension,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_whirlpools_config_extension,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

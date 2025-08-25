use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::decoded_instructions::TokenBadgeAttribute;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetTokenBadgeAttribute>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let attribute = match ix.data_attribute {
    TokenBadgeAttribute::RequireNonTransferablePosition { required } => whirlpool_base::state::TokenBadgeAttribute::RequireNonTransferablePosition(required),
  };

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // whirlpools_config_extension
  replayer.set_whirlpool_account(&ix.key_whirlpools_config_extension, accounts);
  // token_badge_authority
  // token_mint (no need to determine Token or TokenExtensions)
  replayer.set_token_mint(
    pubkey(&ix.key_token_mint),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_badge
  replayer.set_whirlpool_account(&ix.key_token_badge, accounts);

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetTokenBadgeAttribute {
      attribute,
    },
    whirlpool_ix_accounts::SetTokenBadgeAttribute {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      whirlpools_config_extension: pubkey(&ix.key_whirlpools_config_extension),
      token_badge_authority: pubkey(&ix.key_token_badge_authority),
      token_mint: pubkey(&ix.key_token_mint),
      token_badge: pubkey(&ix.key_token_badge),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_token_badge,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_token_badge,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedDeletePositionBundle>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let position_bundle_data = util_replay::get_position_bundle_data(&ix.key_position_bundle, account_map);
  let position_bundle_mint = position_bundle_data.position_bundle_mint;

  // position_bundle
  replayer.set_whirlpool_account(&ix.key_position_bundle, account_map);
  // position_bundle_mint
  replayer.set_token_mint(
    pubkey(&ix.key_position_bundle_mint),
    None,
    1u64,
    0u8,
    None
  );
  // position_bundle_token_account
  replayer.set_token_account(
    pubkey(&ix.key_position_bundle_token_account),
    position_bundle_mint,
    pubkey(&ix.key_position_bundle_owner),
    1u64
  );
  // position_bundle_owner
  // receiver
  // token_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::DeletePositionBundle {
    },
    whirlpool_ix_accounts::DeletePositionBundle {
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_mint: pubkey(&ix.key_position_bundle_mint),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_owner: pubkey(&ix.key_position_bundle_owner),
      receiver: pubkey(&ix.key_receiver),
      token_program: pubkey(&ix.key_token_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_position_bundle,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    // closed
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedInitializePositionBundle>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let _accounts = req.accounts;

  // position_bundle
  // position_bundle_mint
  // position_bundle_token_account
  // position_bundle_owner
  // funder
  replayer.set_funder_account(&ix.key_funder);
  // token_program
  // system_program
  // rent
  // associated_token_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::InitializePositionBundle {
    },
    whirlpool_ix_accounts::InitializePositionBundle {
      position_bundle: pubkey(&ix.key_position_bundle),
      position_bundle_mint: pubkey(&ix.key_position_bundle_mint),
      position_bundle_token_account: pubkey(&ix.key_position_bundle_token_account),
      position_bundle_owner: pubkey(&ix.key_position_bundle_owner),
      funder: pubkey(&ix.key_funder),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
      associated_token_program: pubkey(&ix.key_associated_token_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_position_bundle, // created
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult, TokenTrait};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedClosePositionWithTokenExtensions>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let position_data = util::get_position_data(&ix.key_position, accounts);
  let position_mint = position_data.position_mint;

  let position_mint_token_trait = TokenTrait::TokenExtensionsWithCloseAuthority(
    pubkey(&ix.key_position)
  );


  // position_authority
  // receiver
  // position
  replayer.set_whirlpool_account(&ix.key_position, accounts);
  // position_mint
  replayer.set_token_mint_with_trait(
    pubkey(&ix.key_position_mint),
    position_mint_token_trait,
    None,
    1u64,
    0u8,
    Some(pubkey(&ix.key_position))
  );
  // position_token_amount
  replayer.set_token_account_with_trait(
    pubkey(&ix.key_position_token_account),
    position_mint_token_trait,
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // token_2022_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::ClosePositionWithTokenExtensions {
    },
    whirlpool_ix_accounts::ClosePositionWithTokenExtensions {
      position_authority: pubkey(&ix.key_position_authority),
      receiver: pubkey(&ix.key_receiver),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_token_account: pubkey(&ix.key_position_token_account),
      token_2022_program: pubkey(&ix.key_token_2022_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_position,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    // closed
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

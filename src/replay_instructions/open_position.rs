use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedOpenPosition>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  // funder
  util_replay::add_funder_account(builder, &ix.key_funder);
  // owner
  // position
  // position_mint
  // position_token_account
  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // token_program
  // system_program
  // rent
  // associated_token_program

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  let tx = util_replay::build_unsigned_whirlpool_transaction(
    whirlpool_ix_args::OpenPosition {
      bumps: whirlpool_base::state::OpenPositionBumps {
        position_bump: 0, // dummy
      },
      tick_lower_index: ix.data_tick_lower_index,
      tick_upper_index: ix.data_tick_upper_index,
    },
    whirlpool_ix_accounts::OpenPosition {
      funder: pubkey(&ix.key_funder),
      owner: pubkey(&ix.key_owner),
      position: pubkey(&ix.key_position),
      position_mint: pubkey(&ix.key_position_mint),
      position_token_account: pubkey(&ix.key_position_token_account),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_program: pubkey(&ix.key_token_program),
      system_program: pubkey(&ix.key_system_program),
      rent: pubkey(&ix.key_rent),
      associated_token_program: pubkey(&ix.key_associated_token_program),
    },
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
  ]);
  
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_position, // created
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

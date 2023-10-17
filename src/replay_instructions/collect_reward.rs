use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::replay_environment;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCollectReward>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_reward = whirlpool_data.reward_infos[ix.data_reward_index as usize].mint;

  let position_data = util_replay::get_position_data(&ix.key_position, account_map);
  let position_mint = position_data.position_mint;

  let amount_reward = ix.transfer_amount_0;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // whirlpool
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpool),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpool).unwrap(),
    false,
  );
  // position_authority
  // position
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_position, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_position),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_position).unwrap(),
    false,
  );
  // position_token_amount
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_position_token_account),
    position_mint,
    pubkey(&ix.key_position_authority),
    1u64
  );
  // reward_owner_account
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_reward_owner_account),
    mint_reward,
    pubkey(&ix.key_position_authority),
    0u64
  );
  // reward_vault
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_reward_vault),
    mint_reward,
    pubkey(&ix.key_whirlpool),
    amount_reward
  );
  // token_program


  //let mut env = builder.build();
  //let payer = env.payer();
  //let latest_blockhash = env.get_latest_blockhash();

  let payer = replayer.payer();
  let latest_blockhash = replayer.get_latest_blockhash();
  let nonce = replayer.get_next_nonce();

  //let tx = util_replay::build_unsigned_whirlpool_transaction(
  let tx = util_replay::build_unsigned_whirlpool_transaction_with_nonce(
    whirlpool_ix_args::CollectReward {
      reward_index: ix.data_reward_index,
    },
    whirlpool_ix_accounts::CollectReward {
      whirlpool: pubkey(&ix.key_whirlpool),
      position_authority: pubkey(&ix.key_position_authority),
      position: pubkey(&ix.key_position),
      position_token_account: pubkey(&ix.key_position_token_account),
      reward_owner_account: pubkey(&ix.key_reward_owner_account),
      reward_vault: pubkey(&ix.key_reward_vault),
      token_program: pubkey(&ix.key_token_program),
    },
    &payer,
    latest_blockhash,
    nonce,
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpool,
    &ix.key_position,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

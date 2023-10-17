use poc_framework::Environment;
use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_core::{ReplayInstructionParams, ReplayInstructionResult, WritableAccountSnapshot};
use crate::util_replay;
use crate::util_replay::pubkey; // abbr
use crate::replay_environment;

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCollectProtocolFees>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let amount_a = ix.transfer_amount_0;
  let amount_b = ix.transfer_amount_1;

  let ORCA_WHIRLPOOL_PROGRAM_ID = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  // whirlpools_config
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpools_config, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpools_config),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpools_config).unwrap(),
    false,
  );
  // whirlpool
  //util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  replayer.set_account_with_data(
    pubkey(&ix.key_whirlpool),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(&ix.key_whirlpool).unwrap(),
    false,
  );
  // collect_protocol_fees_authority
  // token_vault_a
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_token_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    amount_a
  );
  // token_vault_b
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_token_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    amount_b
  );
  // token_destination_a
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_token_destination_a),
    mint_a,
    pubkey(&ix.key_collect_protocol_fees_authority),
    0u64
  );
  // token_destination_b
  //builder.add_account_with_tokens(
  replayer.set_token_account(
    pubkey(&ix.key_token_destination_b),
    mint_b,
    pubkey(&ix.key_collect_protocol_fees_authority),
    0u64
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
    whirlpool_ix_args::CollectProtocolFees {
    },
    whirlpool_ix_accounts::CollectProtocolFees {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      whirlpool: pubkey(&ix.key_whirlpool),
      collect_protocol_fees_authority: pubkey(&ix.key_collect_protocol_fees_authority),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      token_destination_a: pubkey(&ix.key_token_destination_a),
      token_destination_b: pubkey(&ix.key_token_destination_b),
      token_program: pubkey(&ix.key_token_program),
    },
    &payer,
    latest_blockhash,
    nonce
  );

  let pre_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpools_config,
    &ix.key_whirlpool,
  ]);
  
  let replay_result = replayer.execute_transaction(tx);

  let post_snapshot = util_replay::replayer_take_snapshot(&replayer, &[
    &ix.key_whirlpools_config,
    &ix.key_whirlpool,
  ]);

  return ReplayInstructionResult {
    transaction_status: replay_result,
    snapshot: WritableAccountSnapshot {
      pre_snapshot,
      post_snapshot,
    }
  }
}

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util;
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedCollectProtocolFeesV2>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  let whirlpool_data = util::get_whirlpool_data(&ix.key_whirlpool, accounts);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let amount_a = ix.transfer_0.amount;
  let amount_b = ix.transfer_1.amount;

  let token_trait_a = util::determine_token_trait(&ix.key_token_program_a, &ix.transfer_0);
  let token_trait_b = util::determine_token_trait(&ix.key_token_program_b, &ix.transfer_1);

  // whirlpools_config
  replayer.set_whirlpool_account(&ix.key_whirlpools_config, accounts);
  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // collect_protocol_fees_authority
  // token_mint_a
  replayer.set_token_mint_with_trait(
    token_trait_a,
    pubkey(&ix.key_token_mint_a),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_mint_b
  replayer.set_token_mint_with_trait(
    token_trait_b,
    pubkey(&ix.key_token_mint_b),
    None,
    u64::MAX, // dummy
    6, // dummy
    None
  );
  // token_vault_a
  replayer.set_token_account_with_trait(
    token_trait_a,
    pubkey(&ix.key_token_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    amount_a
  );
  // token_vault_b
  replayer.set_token_account_with_trait(
    token_trait_b,
    pubkey(&ix.key_token_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    amount_b
  );
  // token_destination_a
  replayer.set_token_account_with_trait(
    token_trait_a,
    pubkey(&ix.key_token_destination_a),
    mint_a,
    pubkey(&ix.key_collect_protocol_fees_authority),
    0u64
  );
  // token_destination_b
  replayer.set_token_account_with_trait(
    token_trait_b,
    pubkey(&ix.key_token_destination_b),
    mint_b,
    pubkey(&ix.key_collect_protocol_fees_authority),
    0u64
  );
  // token_program_a
  // token_program_b
  // memo_program

  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::CollectProtocolFeesV2 {
      // don't replay transfer hook
      remaining_accounts_info: None,
    },
    whirlpool_ix_accounts::CollectProtocolFeesV2 {
      whirlpools_config: pubkey(&ix.key_whirlpools_config),
      whirlpool: pubkey(&ix.key_whirlpool),
      collect_protocol_fees_authority: pubkey(&ix.key_collect_protocol_fees_authority),
      token_mint_a: pubkey(&ix.key_token_mint_a),
      token_mint_b: pubkey(&ix.key_token_mint_b),
      token_vault_a: pubkey(&ix.key_token_vault_a),
      token_vault_b: pubkey(&ix.key_token_vault_b),
      token_destination_a: pubkey(&ix.key_token_destination_a),
      token_destination_b: pubkey(&ix.key_token_destination_b),
      token_program_a: pubkey(&ix.key_token_program_a),
      token_program_b: pubkey(&ix.key_token_program_b),
      memo_program: pubkey(&ix.key_memo_program),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_whirlpool,
  ]);
  
  let transaction_status = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpools_config,
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(transaction_status, pre_snapshot, post_snapshot)
}

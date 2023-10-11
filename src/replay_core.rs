use std::collections::HashMap;
////////////////////
use anchor_lang::AccountDeserialize;
use base64::prelude::{Engine as _, BASE64_STANDARD};
use mysql::LocalInfileHandler;
//use solana_program_test::*;
use solana_sdk::{signer::Signer, signature::Keypair, transaction::{Transaction, VersionedTransaction}};
use solana_sdk::pubkey::Pubkey;
use solana_program::{bpf_loader, bpf_loader_upgradeable};

use std::str::FromStr;

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use flate2::read::GzDecoder;

#[derive(Debug, Deserialize, Serialize)]
struct AccountString {
    pubkey: String,
    data_base64: String,
}

use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use poc_framework::{Environment, LocalEnvironment, LocalEnvironmentBuilder, PrintableTransaction, setup_logging, LogLevel};
////////////////////

use crate::{decoded_instructions::{DecodedWhirlpoolInstruction, DecodedSwap}, types::AccountMap};
use crate::util_replay;

use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::programs;

use crate::util_replay::pubkey as pubkey; // abbr

pub struct WritableAccountMap {
  pub pre_snapshot: AccountMap,
  pub post_snapshot: AccountMap,
}

const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn replay_whirlpool_instruction(
  instruction: DecodedWhirlpoolInstruction,
  account_map: &AccountMap, // readonly
  clock_unixtimestamp: i64,
  whirlpool_program_so: &[u8],
  token_metadata_program_so: &[u8],
) -> ReplayInstructionResult {
  let mut builder = LocalEnvironment::builder();

  // emulate SYSVAR/Clock
  builder.set_creation_time(clock_unixtimestamp);

  // deploy programs: SPL Token
  // TODO: ATA
  // TODO: receive as params
  util_replay::add_upgradable_program(&mut builder, SPL_TOKEN_PROGRAM_ID, programs::SPL_TOKEN);
  // deploy programs: Orca Whirlpool & Metaplex Token Metadata
  util_replay::add_upgradable_program(&mut builder, ORCA_WHIRLPOOL_PROGRAM_ID, whirlpool_program_so);
  util_replay::add_upgradable_program(&mut builder, METAPLEX_METADATA_PROGRAM_ID, token_metadata_program_so);

  match instruction {
    DecodedWhirlpoolInstruction::Swap(decoded) => {
      let req = ReplayInstructionParams {
        env_builder: &mut builder,
        decoded_instruction: &decoded,
        account_map: &account_map,
      };
      replay_swap(req)      
    }
    _ => {
      println!("IGNORE INSTRUCTION AT THE MOMENT: {:?}", instruction);
      panic!("instruction not supported yet");
    }
  }
}

pub struct ReplayInstructionResult {
  pub replay_result: EncodedConfirmedTransactionWithStatusMeta,
  pub writable_account_map: WritableAccountMap,
}

struct ReplayInstructionParams<'info, T> {
  pub env_builder: &'info mut LocalEnvironmentBuilder,
  pub decoded_instruction: &'info T,
  pub account_map: &'info AccountMap,
}

fn replay_swap(req: ReplayInstructionParams<DecodedSwap>) -> ReplayInstructionResult {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let account_map = req.account_map;

  let whirlpool_data = util_replay::get_whirlpool_data(&ix.key_whirlpool, account_map);
  let mint_a = whirlpool_data.token_mint_a;
  let mint_b = whirlpool_data.token_mint_b;

  let mint_a_is_input = ix.data_a_to_b;
  let mint_b_is_input = !mint_a_is_input;
  let input_amount = ix.transfer_amount0;
  let output_amount = ix.transfer_amount1;
/* 
  println!("replay swap: pool = {}, direction = {}, mode = {}, mint_a = {}, mint_b = {}",
    ix.key_whirlpool,
    if ix.data_a_to_b { "A->B" } else { "B->A" },
    if ix.data_amount_specified_is_input { "ExactIn" } else { "ExactOut" },
    mint_a.to_string()[0..10].to_string(),
    mint_b.to_string()[0..10].to_string(),
  );
  println!("{:?}", ix);
*/
  // token_program
  // token_authority
  // whirlpool
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_whirlpool, &account_map);
  // token_owner_account_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_a),
    mint_a,
    pubkey(&ix.key_token_authority),
    if mint_a_is_input { input_amount } else { 0u64 }
  );
  // token_vault_a
  builder.add_account_with_tokens(
    pubkey(&ix.key_vault_a),
    mint_a,
    pubkey(&ix.key_whirlpool),
    if mint_a_is_input { 0u64 } else { output_amount }
  );
  // token_owner_account_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_token_owner_account_b),
    mint_b,
    pubkey(&ix.key_token_authority),
    if mint_b_is_input { input_amount } else { 0u64 }
  );
  // token_vault_b
  builder.add_account_with_tokens(
    pubkey(&ix.key_vault_b),
    mint_b,
    pubkey(&ix.key_whirlpool),
    if mint_b_is_input { 0u64 } else { output_amount }
  );
  // tick_array_0
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array0, &account_map);
  // tick_array_1
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array1, &account_map);
  // tick_array_2
  util_replay::add_whirlpool_account_with_data(builder, &ix.key_tick_array2, &account_map);
  // oracle

  let mut env = builder.build();
  let payer = env.payer();
  let latest_blockhash = env.get_latest_blockhash();

  // build tx
  let tx = util_replay::build_unsigned_transaction(
    ORCA_WHIRLPOOL_PROGRAM_ID,
    whirlpool_ix_args::Swap {
      amount: ix.data_amount,
      other_amount_threshold: ix.data_other_amount_threshold,
      sqrt_price_limit: ix.data_sqrt_price_limit,
      amount_specified_is_input: ix.data_amount_specified_is_input,
      a_to_b: ix.data_a_to_b,
    },
    whirlpool_ix_accounts::Swap {
      token_program: pubkey(&ix.key_token_program),
      token_authority: pubkey(&ix.key_token_authority),
      whirlpool: pubkey(&ix.key_whirlpool),
      token_owner_account_a: pubkey(&ix.key_token_owner_account_a),
      token_vault_a: pubkey(&ix.key_vault_a),
      token_owner_account_b: pubkey(&ix.key_token_owner_account_b),
      token_vault_b: pubkey(&ix.key_vault_b),
      tick_array_0: pubkey(&ix.key_tick_array0),
      tick_array_1: pubkey(&ix.key_tick_array1),
      tick_array_2: pubkey(&ix.key_tick_array2),
      oracle: pubkey(&ix.key_oracle),
    },
    &payer,
    latest_blockhash);

  let pre_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_tick_array0,
    &ix.key_tick_array1,
    &ix.key_tick_array2,
  ]);
  
  // run
  let replay_result = env.execute_transaction(tx);

  let post_snapshot = util_replay::take_snapshot(&env, &[
    &ix.key_whirlpool,
    &ix.key_tick_array0,
    &ix.key_tick_array1,
    &ix.key_tick_array2,
  ]);

  return ReplayInstructionResult {
    replay_result,
    writable_account_map: WritableAccountMap {
      pre_snapshot,
      post_snapshot,
    }
  }
}

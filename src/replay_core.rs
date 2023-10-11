use std::collections::HashMap;
////////////////////
use anchor_lang::AccountDeserialize;
use base64::prelude::{Engine as _, BASE64_STANDARD};
//use solana_program_test::*;
use solana_sdk::{signer::Signer, signature::Keypair, transaction::{Transaction, VersionedTransaction}};
use solana_sdk::pubkey::Pubkey;
use solana_program::{bpf_loader, bpf_loader_upgradeable};

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use flate2::read::GzDecoder;

#[derive(Debug, Deserialize, Serialize)]
struct AccountString {
    pubkey: String,
    data_base64: String,
}

use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};

use poc_framework::{Environment, LocalEnvironment, LocalEnvironmentBuilder, PrintableTransaction, setup_logging, LogLevel};
////////////////////

use crate::decoded_instructions::{DecodedWhirlpoolInstruction, DecodedSwap};
use crate::util_replay;

pub struct WritableAccountMap {
  pub pre: HashMap<String, String>,
  pub post: HashMap<String, String>,
}

const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

pub fn replay_whirlpool_instruction(
  instruction: DecodedWhirlpoolInstruction,
  account_map: &HashMap<String, String>, // readonly
  clock_unixtimestamp: i64,
  whirlpool_program_so: &[u8],
  token_metadata_program_so: &[u8],
) -> WritableWhirlpoolAccountMap {
  let mut builder = LocalEnvironment::builder();

  // emulate SYSVAR/Clock
  builder.set_creation_time(clock_unixtimestamp);

  // deploy programs: Orca Whirlpool & Metaplex Token Metadata
  util_replay::add_upgradable_program(&mut builder, ORCA_WHIRLPOOL_PROGRAM_ID, whirlpool_program_so);
  util_replay::add_upgradable_program(&mut builder, METAPLEX_METADATA_PROGRAM_ID, token_metadata_program_so);

  match instruction {
    DecodedWhirlpoolInstruction::Swap(decoded) => {
      let req = ReplayInstructionRequest {
        env_builder: &mut builder,
        decoded_instruction: &decoded,
        account_map: &account_map,
      };
      let res = replay_swap(req);
      let writable_account_map = res.writable_account_map;
      return writable_account_map;
    }
    _ => {
      println!("IGNORE INSTRUCTION AT THE MOMENT: {:?}", instruction);
      //panic!("instruction not supported");
    }
  }
}

struct ReplayInstructionRequest<'info, T> {
  pub env_builder: &'info mut LocalEnvironmentBuilder,
  pub decoded_instruction: &'info T,
  pub account_map: &'info HashMap<String, String>,
}

struct ReplayInstructionResponse {
  pub writable_account_map: WritableAccountMap,
  // from poc_platform ?
}

fn replay_swap(req: ReplayInstructionRequest<DecodedSwap>) -> ReplayInstructionResponse {
  let builder = req.env_builder;
  let ix = req.decoded_instruction;
  let accounts = req.account_map;

  // token_program
  // token_authority
  // whirlpool
  builder.add_account_with_data(
    solana_program::pubkey!(ix.key_whirlpool),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &BASE64_STANDARD.decode(accounts.get(&ix.key_whirlpool).unwrap()).unwrap(),
    false,
  );

  // take snapshot

  //  let mut env = req.env_builder.build();

  // build tx

  // run

  // take snapshot


}

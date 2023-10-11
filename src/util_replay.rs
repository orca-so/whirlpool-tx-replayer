use std::collections::HashMap;

use anchor_lang::AccountDeserialize;
use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::hash::Hash;
use solana_sdk::instruction::Instruction;
use poc_framework::{LocalEnvironmentBuilder, LocalEnvironment, Environment};
use solana_program::bpf_loader_upgradeable;
use bincode;

use whirlpool_base::state::{Whirlpool, Position, PositionBundle};
use std::str::FromStr;

use anchor_lang::{InstructionData, ToAccountMetas};

use crate::types::AccountMap;

// LocalEnvironmentBuilder.add_program doesn't work for upgradeable programs
// https://github.com/solana-labs/solana/blob/170478924705c9c62dbeb475c5425b68ba61b375/sdk/program/src/bpf_loader_upgradeable.rs#L27-L53
pub fn add_upgradable_program(
    builder: &mut LocalEnvironmentBuilder,
    pubkey: Pubkey,
    data: &[u8],
) {
    let program_pubkey = pubkey;
    let programdata_pubkey = Keypair::new().pubkey();

    let program_data = bpf_loader_upgradeable::UpgradeableLoaderState::Program {
      programdata_address: programdata_pubkey
    };

    let programdata_header = bpf_loader_upgradeable::UpgradeableLoaderState::ProgramData {
      slot: 1, // 0 is not valid
      upgrade_authority_address: Some(Pubkey::default()), // None is not valid
    };

    let program_bytes = bincode::serialize(&program_data).unwrap();
    let mut programdata_bytes = bincode::serialize(&programdata_header).unwrap();
    programdata_bytes.extend_from_slice(data);

    builder.add_account_with_data(program_pubkey, bpf_loader_upgradeable::ID, &program_bytes, true);
    builder.add_account_with_data(programdata_pubkey, bpf_loader_upgradeable::ID, &programdata_bytes, false);
}

pub fn add_whirlpool_account_with_data(
  builder: &mut LocalEnvironmentBuilder,
  pubkey_string: &String,
  account_map: &AccountMap,
) {
  // TODO: refactor
  let ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

  builder.add_account_with_data(
    solana_program::pubkey::Pubkey::from_str(pubkey_string.as_str()).unwrap(),
    ORCA_WHIRLPOOL_PROGRAM_ID,
    &account_map.get(pubkey_string).unwrap(),
    false,
  );
}

pub fn add_funder_account(
  builder: &mut LocalEnvironmentBuilder,
  pubkey_string: &String,
) {
  // TODO: refactor
  let SYSTEM_PROGRAM_ID: Pubkey = solana_program::pubkey!("11111111111111111111111111111111");

  builder.add_account_with_lamports(
    solana_program::pubkey::Pubkey::from_str(pubkey_string.as_str()).unwrap(),
    SYSTEM_PROGRAM_ID,
    10_000_000_000 // 10 SOL
  );
}

pub fn get_whirlpool_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> Whirlpool {
  let data = account_map.get(pubkey_string).unwrap();
  let whirlpool_data = whirlpool_base::state::Whirlpool::try_deserialize(&mut data.as_slice()).unwrap();
  return whirlpool_data;
}

pub fn get_position_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> Position {
  let data = account_map.get(pubkey_string).unwrap();
  let position_data = whirlpool_base::state::Position::try_deserialize(&mut data.as_slice()).unwrap();
  return position_data;
}

pub fn get_position_bundle_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> PositionBundle {
  let data = account_map.get(pubkey_string).unwrap();
  let position_bundle_data = whirlpool_base::state::PositionBundle::try_deserialize(&mut data.as_slice()).unwrap();
  return position_bundle_data;
}

pub fn build_unsigned_whirlpool_transaction(
  args: impl InstructionData,
  accounts: impl ToAccountMetas,
  payer: &Keypair,
  recent_blockhash: Hash,
) -> Transaction {
  // TODO: refactor
  let ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
  return build_unsigned_transaction(ORCA_WHIRLPOOL_PROGRAM_ID, args, accounts, payer, recent_blockhash);
}

fn build_unsigned_transaction(
  program_id: Pubkey,
  args: impl InstructionData,
  accounts: impl ToAccountMetas,
  payer: &Keypair,
  recent_blockhash: Hash,
) -> Transaction {
  let instruction = Instruction {
    program_id,
    data: args.data(), // using Anchor, at least instruction code (8 bytes)
    accounts: accounts.to_account_metas(None),
  };

  // create transaction with only sign of payer
  let message = solana_sdk::message::Message::new(&[instruction], Some(&payer.pubkey()));
  let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);
  tx.partial_sign(&[payer], recent_blockhash);

  return tx;
}

pub fn pubkey(pubkey_string: &String) -> Pubkey {
  return Pubkey::from_str(pubkey_string).unwrap();
}

pub fn take_snapshot(
  env: &LocalEnvironment,
  pubkeys: &[&String],
) -> AccountMap {
  let mut snapshot = AccountMap::new();

  for pubkey_string in pubkeys {
    let account = env.get_account(pubkey(pubkey_string)).unwrap();
    snapshot.insert((*pubkey_string).clone(), account.data);
  }

  return snapshot;
}

pub fn update_account_map(
  account_map: &mut AccountMap,
  pre_snapshot: AccountMap,
  post_snapshot: AccountMap,
) {
  let closed_account_pubkeys: Vec<String> = pre_snapshot.keys()
    .filter(|k| !post_snapshot.contains_key(*k))
    .map(|k| k.clone())
    .collect();

  // add created & update accounts
  account_map.extend(post_snapshot);

  // remove closed accounts
  for pubkey_string in closed_account_pubkeys {
    account_map.remove(&pubkey_string);
  }
}
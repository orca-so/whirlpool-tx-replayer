use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::{BufRead, BufReader, BufWriter}};
use serde_json::Value;
use replay_engine::decoded_instructions::{deserialize_u64, deserialize_base64, serialize_base64};
use replay_engine::types::AccountMap;
use reqwest;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolState {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  pub accounts: Vec<WhirlpoolStateAccount>,
  #[serde(deserialize_with = "deserialize_base64", serialize_with = "serialize_base64")]
  pub program_data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolStateAccount {
  pub pubkey: String,
  #[serde(deserialize_with = "deserialize_base64", serialize_with = "serialize_base64")]
  pub data: Vec<u8>,
}

pub fn load_from_local_whirlpool_state_file(file_path: &String) -> WhirlpoolState {
  let file = File::open(file_path).unwrap();
  let decoder = GzDecoder::new(file);
  let reader = BufReader::new(decoder);
  return serde_json::from_reader(reader).unwrap();
}

pub fn save_to_whirlpool_state_file(file_path: &String, state: &WhirlpoolState) {
  let file = File::create(file_path).unwrap();
  let encoder = GzEncoder::new(file, flate2::Compression::default());
  let writer = BufWriter::new(encoder);
  serde_json::to_writer(writer, state).unwrap();
}

pub fn load_from_remote_whirlpool_state_file(url: &String) -> WhirlpoolState {
  let response = reqwest::blocking::get(url).unwrap();
  let decoder = GzDecoder::new(response);
  let reader = BufReader::new(decoder);
  return serde_json::from_reader(reader).unwrap();
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolTransaction {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  pub transactions: Vec<Transaction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
  pub index: u32,
  pub signature: String,
  pub payer: String,
  pub balances: Vec<TransactionBalance>,
  pub instructions: Vec<TransactionInstruction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionBalance {
  pub account: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub pre: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub post: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TransactionInstruction {
  pub name: String,
  pub payload: Value,
}

pub fn load_from_local_whirlpool_transaction_file(file_path: &String) -> impl Iterator<Item = WhirlpoolTransaction>
{
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let buf = BufReader::new(decoder);

    let iter = buf.lines()
      .map(|jsonl| jsonl.unwrap())
      .map(|jsonl| {
        let t: Result<WhirlpoolTransaction, serde_json::Error> = serde_json::from_str(jsonl.as_str());
        return t.unwrap();
      });

    return iter;
}

pub fn load_from_remote_whirlpool_transaction_file(url: &String) -> impl Iterator<Item = WhirlpoolTransaction>
{
    let response = reqwest::blocking::get(url).unwrap();

    let decoder = GzDecoder::new(response);
    let buf = BufReader::new(decoder);

    let iter = buf.lines()
      .map(|jsonl| jsonl.unwrap())
      .map(|jsonl| {
        let t: Result<WhirlpoolTransaction, serde_json::Error> = serde_json::from_str(jsonl.as_str());
        return t.unwrap();
      });

    return iter;
}

pub fn convert_accounts_to_account_map(accounts: &Vec<WhirlpoolStateAccount>) -> AccountMap {
  let mut account_map = AccountMap::new();
  for account in accounts {
    account_map.insert(account.pubkey.clone(), account.data.clone());
  }
  return account_map;
}

pub fn convert_account_map_to_accounts(account_map: &AccountMap) -> Vec<WhirlpoolStateAccount> {
  let mut accounts = Vec::<WhirlpoolStateAccount>::new();
  for (pubkey, data) in account_map {
    accounts.push(WhirlpoolStateAccount {
      pubkey: pubkey.clone(),
      data: data.clone(),
    });
  }
  return accounts;
}

pub fn download_from_remote_storage(url: &String, file_path: &String) {
  let mut response = reqwest::blocking::get(url).unwrap();
  std::fs::create_dir_all(std::path::Path::new(file_path).parent().unwrap()).unwrap();
  let mut file = File::create(file_path).unwrap();
  std::io::copy(&mut response, &mut file).unwrap();
}
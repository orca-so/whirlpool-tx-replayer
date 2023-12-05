use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::{BufRead, BufReader}};

use serde_json::Value;

use base64::prelude::{Engine as _, BASE64_STANDARD};

use replay_engine::types::AccountMap;
use replay_engine::decoded_instructions::deserialize_u64;

#[derive(Debug, Deserialize, Serialize)]
struct PubkeyAndDataBase64 {
    pubkey: String,
    data_base64: String,
}

// TODO: error handling
pub fn load_from_snapshot_file(file_path: &String) -> AccountMap {
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(decoder);

    let mut account_map = AccountMap::new();
    reader.deserialize::<PubkeyAndDataBase64>().for_each(|row| {
        let row = row.unwrap();
        let data = BASE64_STANDARD.decode(row.data_base64).unwrap();
        account_map.insert(row.pubkey, data);
    });

    account_map
}

pub fn save_to_snapshot_file(file_path: &String, account_map: &AccountMap) {
    let file = File::create(file_path).unwrap();

    let encoder = GzEncoder::new(file, flate2::Compression::default());
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(encoder);

    for (pubkey, data) in account_map {
        let data_base64 = BASE64_STANDARD.encode(data);
        let row = PubkeyAndDataBase64 {
            pubkey: pubkey.to_string(),
            data_base64,
        };
        writer.serialize(row).unwrap();
    }

    writer.flush().unwrap();
}

pub fn load_from_transaction_file(file_path: &String) -> Vec<String> {
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let buf = BufReader::new(decoder);
    let lines = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    return lines;
}





#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransactionBalance {
  pub account: String,
  #[serde(deserialize_with = "deserialize_u64")]
  pub pre: u64,
  #[serde(deserialize_with = "deserialize_u64")]
  pub post: u64,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransactionInstruction {
  pub name: String,
  pub payload: Value,
}


#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransaction {
  pub index: u32,
  pub signature: String,
  pub payer: String,
  pub balances: Vec<SlotTransactionBalance>,
  pub instructions: Vec<SlotTransactionInstruction>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SlotTransactions {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  pub transactions: Vec<SlotTransaction>,
}

pub fn json_to_slot_transactions(json: &str) -> Result<SlotTransactions, serde_json::Error> {
  serde_json::from_str(json)
}


use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use replay_engine::decoded_instructions::{deserialize_u64, deserialize_base64, serialize_base64};

pub use replay_engine::decoded_instructions::{DecodedInstruction, DecodedProgramDeployInstruction, DecodedWhirlpoolInstruction};

/*

Whirlpool State File JSON Schema

A whirlpool state file (whirlpool-state-yyyymmdd.json.gz) is GZIP compressed JSON file with the following schema:
 
{
  slot: u64,
  blockHeight: u64,
  blockTime: i64,
  accounts: [
    { pubkey: String(base58 encoding), data: String(base64 encoding) },
    { pubkey: String(base58 encoding), data: String(base64 encoding) },
    ...
  ],
  programData: String(base64 encoding)
}

*/

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

/*

Whirlpool Transaction File JSON Lines Format

A whirlpool transaction file (whirlpool-transaction-yyyymmdd.json.gz) is GZIP compressed text file.
Each line is a JSON object with the following schema:

{
  slot: u64,
  blockHeight: u64,
  blockTime: i64,
  transactions: [
    {
      index: u32,
      signature: String(base58 encoding),
      payer: String(base58 encoding),
      balances: [
        { account: String(base58 encoding), pre: String(u64 as string), post: String(u64 as string) },
        { account: String(base58 encoding), pre: String(u64 as string), post: String(u64 as string) },
        ...
      ],
      instructions: [
        { name: String, payload: Value },
        { name: String, payload: Value },
        ...
      ],
    },
    ...
  ]
}

*/

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

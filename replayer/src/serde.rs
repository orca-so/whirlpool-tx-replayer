use crate::schema::*;
use replay_engine::account_data_store::AccountDataStore;
use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
};
use std::fmt;
use serde_derive::{Deserialize, Serialize};
use replay_engine::decoded_instructions::{deserialize_base64, serialize_base64};

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolStateOnMemoryDeserializer {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  #[serde(deserialize_with = "deserialize_account_data_store_on_memory")]
  pub accounts: AccountDataStore,
  #[serde(deserialize_with = "deserialize_base64")]
  pub program_data: Vec<u8>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolStateOnDiskDeserializer {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  #[serde(deserialize_with = "deserialize_account_data_store_on_disk")]
  pub accounts: AccountDataStore,
  #[serde(deserialize_with = "deserialize_base64")]
  pub program_data: Vec<u8>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WhirlpoolStateSerializer<'a> {
  pub slot: u64,
  pub block_height: u64,
  pub block_time: i64,
  #[serde(serialize_with = "serialize_account_data_store")]
  pub accounts: &'a AccountDataStore,
  #[serde(serialize_with = "serialize_base64")]
  pub program_data: &'a Vec<u8>,
}

pub fn deserialize_account_data_store_on_memory<'de, D>(
    deserializer: D,
) -> Result<AccountDataStore, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct AccountDataStoreVisitor {}

    impl<'de> Visitor<'de> for AccountDataStoreVisitor {
        type Value = AccountDataStore;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of WhirlpoolStateAccount")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut result = AccountDataStore::new_on_memory();
            while let Some(value) = seq.next_element()? {
                let account: WhirlpoolStateAccount = value;
                result.upsert(&account.pubkey, &account.data).unwrap();
            }
            Ok(result)
        }
    }

    deserializer.deserialize_seq(AccountDataStoreVisitor {})
}

pub fn deserialize_account_data_store_on_disk<'de, D>(
    deserializer: D,
) -> Result<AccountDataStore, D::Error>
where
    D: de::Deserializer<'de>,
{
    struct AccountDataStoreVisitor {}

    impl<'de> Visitor<'de> for AccountDataStoreVisitor {
        type Value = AccountDataStore;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a sequence of WhirlpoolStateAccount")
        }

        fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: SeqAccess<'de>,
        {
            let mut result = AccountDataStore::new_on_disk();
            while let Some(value) = seq.next_element()? {
                let account: WhirlpoolStateAccount = value;
                result.upsert(&account.pubkey, &account.data).unwrap();
            }
            Ok(result)
        }
    }

    deserializer.deserialize_seq(AccountDataStoreVisitor {})
}

pub fn serialize_account_data_store<S>(
    accounts: &AccountDataStore,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut seq = serializer.serialize_seq(None)?;

    accounts
        .traverse(|pubkey, data| {
            let account = WhirlpoolStateAccount {
                pubkey: pubkey.to_string(),
                data: data.to_vec(),
            };
            seq.serialize_element(&account).unwrap();
            Ok(())
        })
        .unwrap();

    seq.end()
}

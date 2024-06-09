use crate::schema::*;
use replay_engine::account_data_store::AccountDataStore;
use serde::{
    de::{self, DeserializeSeed, MapAccess, Visitor},
    ser::SerializeSeq, Deserializer,
};
use std::fmt;
use serde_derive::Serialize;
use replay_engine::decoded_instructions::serialize_base64;
use base64::prelude::{Engine as _, BASE64_STANDARD};

pub fn deserialize_whirlpool_state_from_reader(
    reader: impl std::io::Read,
    config: AccountDataStoreConfig,
) -> WhirlpoolState {
    let deserializer = WhirlpoolStateDeserializer::new(config);
    let de = &mut serde_json::Deserializer::from_reader(reader);
    deserializer.deserialize(de).unwrap()
}

#[derive(Clone)]
pub enum AccountDataStoreConfig {
    OnMemory,
    OnDisk(Option<String>),
}

pub struct WhirlpoolStateDeserializer{
    config: AccountDataStoreConfig,
}

pub struct AccountsDeserializeConfig {
    config: AccountDataStoreConfig,
}

impl WhirlpoolStateDeserializer {
    pub fn new(config: AccountDataStoreConfig) -> Self {
        Self { config }
    }
}

impl<'de> DeserializeSeed<'de> for WhirlpoolStateDeserializer {
    type Value = WhirlpoolState;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        const FIELD_SLOT: &'static str = "slot";
        const FIELD_BLOCK_HEIGHT: &'static str = "blockHeight";
        const FIELD_BLOCK_TIME: &'static str = "blockTime";
        const FIELD_PROGRAM_DATA: &'static str = "programData";
        const FIELD_ACCOUNTS: &'static str = "accounts";

        struct LocalVisitor {
            config: AccountDataStoreConfig,
        }

        impl<'de> Visitor<'de> for LocalVisitor {
            type Value = WhirlpoolState;
    
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct WhirlpoolState")
            }

            fn visit_map<V>(self, mut map: V) -> Result<WhirlpoolState, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut slot: Option<u64> = None;
                let mut block_height: Option<u64> = None;
                let mut block_time: Option<i64> = None;
                let mut program_data: Option<Vec<u8>> = None;
                let mut accounts: Option<AccountDataStore> = None;
                while let Some(key) = map.next_key::<String>()? {
                    match key.as_str() {
                        FIELD_SLOT => {
                            if slot.is_some() {
                                return Err(de::Error::duplicate_field(FIELD_SLOT));
                            }
                            slot = Some(map.next_value()?);
                        }
                        FIELD_BLOCK_HEIGHT => {
                            if block_height.is_some() {
                                return Err(de::Error::duplicate_field(FIELD_BLOCK_HEIGHT));
                            }
                            block_height = Some(map.next_value()?);
                        }
                        FIELD_BLOCK_TIME => {
                            if block_time.is_some() {
                                return Err(de::Error::duplicate_field(FIELD_BLOCK_TIME));
                            }
                            block_time = Some(map.next_value()?);
                        }
                        FIELD_PROGRAM_DATA => {
                            if program_data.is_some() {
                                return Err(de::Error::duplicate_field(FIELD_PROGRAM_DATA));
                            }
                            let program_data_base64: String = map.next_value()?;
                            program_data = Some(match BASE64_STANDARD.decode(program_data_base64).ok() {
                                Some(data) => Ok(data),
                                None => Err(de::Error::custom("expected base64 string")),
                            }?);
                        }
                        FIELD_ACCOUNTS => {
                            if accounts.is_some() {
                                return Err(de::Error::duplicate_field(FIELD_ACCOUNTS));
                            }
                            accounts = Some(map.next_value_seed(AccountsDeserializeConfig { config: self.config.clone() })?);                        
                        }
                        _ => {
                            map.next_value::<serde::de::IgnoredAny>()?;
                        }
                    }
                }
                let slot = slot.ok_or_else(|| de::Error::missing_field(FIELD_SLOT))?;
                let block_height = block_height.ok_or_else(|| de::Error::missing_field(FIELD_BLOCK_HEIGHT))?;
                let block_time = block_time.ok_or_else(|| de::Error::missing_field(FIELD_BLOCK_TIME))?;
                let program_data = program_data.ok_or_else(|| de::Error::missing_field(FIELD_PROGRAM_DATA))?;
                let accounts = accounts.ok_or_else(|| de::Error::missing_field(FIELD_ACCOUNTS))?;
                Ok(WhirlpoolState {
                    slot,
                    block_height,
                    block_time,
                    program_data,
                    accounts,
                })
            }
        }

        const FIELDS: &'static [&'static str] = &[
            FIELD_SLOT,
            FIELD_BLOCK_HEIGHT,
            FIELD_BLOCK_TIME,
            FIELD_PROGRAM_DATA,
            FIELD_ACCOUNTS,
        ];
        deserializer.deserialize_struct("WhirlpoolState", FIELDS, LocalVisitor { config: self.config.clone() })
    }
}

impl<'de> DeserializeSeed<'de> for AccountsDeserializeConfig {
    type Value = AccountDataStore;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct LocalVisitor {
            config: AccountDataStoreConfig,
        }

        impl<'de> Visitor<'de> for LocalVisitor {
            type Value = AccountDataStore;
    
            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct AccountDataStore")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<AccountDataStore, V::Error>
            where
                V: serde::de::SeqAccess<'de>,
            {
                let mut store = match self.config {
                    AccountDataStoreConfig::OnMemory => AccountDataStore::new_on_memory(),
                    AccountDataStoreConfig::OnDisk(ref path) => AccountDataStore::new_on_disk(path.clone()),
                };
                while let Some(value) = seq.next_element()? {
                    let account: WhirlpoolStateAccount = value;
                    store.upsert(&account.pubkey, &account.data).unwrap();
                }
                Ok(store)
            }
        }

        deserializer.deserialize_seq(LocalVisitor { config: self.config.clone() })
    }
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

fn serialize_account_data_store<S>(
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

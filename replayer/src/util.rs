use crate::schema::*;
use replay_engine::account_data_store::AccountDataStore;
use serde::{
    de::{self, SeqAccess, Visitor},
    ser::SerializeSeq,
};
use std::fmt;

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

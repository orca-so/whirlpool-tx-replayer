use std::collections::HashMap;
use anyhow::Result;
use super::AccountDataStoreInnerTrait;
use crate::types::AccountData;

#[derive(Debug)]
pub struct MemoryAccountDataStore {
    data: HashMap<String, AccountData>,
}

impl MemoryAccountDataStore {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl AccountDataStoreInnerTrait for MemoryAccountDataStore {
    fn get(&self, pubkey: &String) -> Result<Option<AccountData>> {
        Ok(self.data.get(pubkey).map(|data| data.clone()))
    }

    fn upsert(&mut self, pubkey: &String, data: &AccountData) -> Result<()> {
        self.data.insert(pubkey.clone(), data.clone());
        Ok(())
    }

    fn delete(&mut self, pubkey: &String) -> Result<()> {
      self.data.remove(pubkey);
      Ok(())
    }

    fn traverse<F: FnMut(&String, &AccountData) -> Result<()>>(&self, mut callback: F) -> Result<()> {
        for (pubkey, data) in self.data.iter() {
            callback(pubkey, data)?;
        }
        Ok(())
    }
}

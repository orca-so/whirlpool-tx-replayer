use anyhow::Result;
use crate::types::AccountData;

mod memory;
mod rocksdb;

trait AccountDataStoreInnerTrait {
  fn get(&self, pubkey: &String) -> Result<Option<AccountData>>;
  fn upsert(&mut self, pubkey: &String, data: &AccountData) -> Result<()>;
  fn delete(&mut self, pubkey: &String) -> Result<()>;
  fn traverse<F: FnMut(&String, &AccountData) -> Result<()>>(&self, callback: F) -> Result<()>;
}

enum AccountDataStoreInner {
  Memory(memory::MemoryAccountDataStore),
  RocksDB(rocksdb::RocksDBAccountDataStore),
}

pub struct AccountDataStore {
  inner: AccountDataStoreInner,
}

impl AccountDataStore {
  pub fn new_on_memory() -> Self {
    Self {
      inner: AccountDataStoreInner::Memory(memory::MemoryAccountDataStore::new())
    }
  }

  pub fn new_on_disk() -> Self {
    Self {
      inner: AccountDataStoreInner::RocksDB(rocksdb::RocksDBAccountDataStore::new())
    }
  }

  pub fn get(&self, pubkey: &String) -> Result<Option<AccountData>> {
      match &self.inner {
          AccountDataStoreInner::Memory(store) => store.get(pubkey),
          AccountDataStoreInner::RocksDB(store) => store.get(pubkey),
      }
  }

  pub fn upsert(&mut self, pubkey: &String, data: &AccountData) -> Result<()> {
      match &mut self.inner {
          AccountDataStoreInner::Memory(store) => store.upsert(pubkey, data),
          AccountDataStoreInner::RocksDB(store) => store.upsert(pubkey, data),
      }
  }

  pub fn delete(&mut self, pubkey: &String) -> Result<()> {
    match &mut self.inner {
      AccountDataStoreInner::Memory(store) => store.delete(pubkey),
      AccountDataStoreInner::RocksDB(store) => store.delete(pubkey),
    }
  }

  pub fn traverse<F: FnMut(&String, &AccountData) -> Result<()>>(&self, callback: F) -> Result<()> {
      match &self.inner {
          AccountDataStoreInner::Memory(store) => store.traverse(callback),
          AccountDataStoreInner::RocksDB(store) => store.traverse(callback),
      }
  }
}

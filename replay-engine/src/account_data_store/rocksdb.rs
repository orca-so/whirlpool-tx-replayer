use anyhow::Result;
use super::AccountDataStoreInnerTrait;
use crate::types::AccountData;

#[derive(Debug)]
pub struct RocksDBAccountDataStore {
    data: rocksdb::DB,
    #[allow(dead_code)]
    rocksdb_temp_dir: tempfile::TempDir,
}

impl RocksDBAccountDataStore {
    pub fn new<P: AsRef<std::path::Path>>(dir: Option<P>) -> Self {
        let rocksdb_temp_dir = if let Some(dir) = dir {
            tempfile::tempdir_in(dir).unwrap()
        } else {
            tempfile::tempdir().unwrap()
        };
        let path = rocksdb_temp_dir.path().to_str().unwrap();
        let db = rocksdb::DB::open_default(path).unwrap();
        Self {
            data: db,
            rocksdb_temp_dir,
        }
    }
}

impl AccountDataStoreInnerTrait for RocksDBAccountDataStore {
    fn get(&self, pubkey: &String) -> Result<Option<AccountData>> {
        self.data.get(pubkey.as_bytes()).map_err(|e| anyhow::anyhow!(e))
    }

    fn upsert(&mut self, pubkey: &String, data: &AccountData) -> Result<()> {
        self.data.put(pubkey.as_bytes(), data.as_slice())?;
        Ok(())
    }

    fn delete(&mut self, pubkey: &String) -> Result<()> {
      self.data.delete(pubkey.as_bytes())?;
      Ok(())
    }

    fn traverse<F: FnMut(&String, &AccountData) -> Result<()>>(&self, mut callback: F) -> Result<()> {
        let iter = self.data.iterator(rocksdb::IteratorMode::Start);
        for item in iter {
            let (key, value) = item?;
            let pubkey = String::from_utf8(key.to_vec()).unwrap();
            let data = value.to_vec();
            callback(&pubkey, &data)?;
        }
        Ok(())
    }
}

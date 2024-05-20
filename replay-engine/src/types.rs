use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Slot {
    pub slot: u64,
    pub block_height: u64,
    pub block_time: i64,
}

impl Slot {
    pub fn new(slot: u64, block_height: u64, block_time: i64) -> Self {
        Self { slot, block_height, block_time }
    }
}

pub type ProgramData = Vec<u8>;
pub type AccountData = Vec<u8>;

pub type AccountSnapshot = HashMap<String, AccountData>;

#[derive(Clone)]
pub struct WritableAccountSnapshot {
  pub pre_snapshot: AccountSnapshot,
  pub post_snapshot: AccountSnapshot,
}

pub enum AccountUpdate {
    Created(AccountData),
    Updated(AccountData),
    Deleted,
}

pub struct AccountUpdates {
    inner: HashMap<String, AccountUpdate>,
}

impl AccountUpdates {
    pub fn new() -> Self {
        Self { inner: HashMap::new() }
    }

    pub fn merge(&mut self, other: Self) {
        for (pubkey, update) in other.inner {
            self.inner.insert(pubkey, update);
        }
    }

    pub fn get(&self, pubkey: &String) -> Option<&AccountUpdate> {
        self.inner.get(pubkey)
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, AccountUpdate> {
        self.inner.iter()
    }
}

impl From<&WritableAccountSnapshot> for AccountUpdates {
    fn from(snapshot: &WritableAccountSnapshot) -> AccountUpdates {
      let pre_snapshot = &snapshot.pre_snapshot;
      let post_snapshot = &snapshot.post_snapshot;
      let pre_pubkeys = pre_snapshot.keys().cloned().collect::<std::collections::HashSet<_>>();
      let post_pubkeys = post_snapshot.keys().cloned().collect::<std::collections::HashSet<_>>();
  
      let mut updates = HashMap::new();
  
      let created_pubkeys = post_pubkeys.difference(&pre_pubkeys).collect::<std::collections::HashSet<_>>();
      for pubkey in created_pubkeys {
        updates.insert(pubkey.clone(), crate::types::AccountUpdate::Created(post_snapshot.get(pubkey).unwrap().clone()));
      }
  
      let updated_pubkeys = post_pubkeys.intersection(&pre_pubkeys).collect::<std::collections::HashSet<_>>();
      for pubkey in updated_pubkeys {
        updates.insert(pubkey.clone(), crate::types::AccountUpdate::Updated(post_snapshot.get(pubkey).unwrap().clone()));
      }
  
      let deleted_pubkeys = pre_pubkeys.difference(&post_pubkeys).collect::<std::collections::HashSet<_>>();
      for pubkey in deleted_pubkeys {
        updates.insert(pubkey.clone(), crate::types::AccountUpdate::Deleted);
      }
    
      return AccountUpdates { inner: updates };
    }
}

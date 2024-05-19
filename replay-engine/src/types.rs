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

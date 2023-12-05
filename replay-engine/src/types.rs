use std::collections::HashMap;

pub type AccountMap = HashMap<String, Vec<u8>>;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Slot {
    pub slot: u64,
    pub block_height: u64,
    pub block_time: i64,
}

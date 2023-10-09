use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use csv;
use serde_derive::{Deserialize, Serialize};
use base64::{Engine, prelude::BASE64_STANDARD};

use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Slot {
    slot: u64,
    blockHeight: u64,
    blockTime: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct WhirlpoolInstruction {
    txid: u64,
    order: u32,
    signature: String,
    ix: String,
    json: String,
}




fn main() {
    let url = "mysql://root:password@localhost:3306/localtest";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let selected_slots = conn.query_map(
        "SELECT slot, blockHeight, blockTime FROM slots WHERE slot > 215135999 ORDER BY slot ASC LIMIT 10",
        |(slot, blockHeight, blockTime)| {
            Slot {
                slot,
                blockHeight,
                blockTime,
            }
        },
    ).unwrap();

    for slot in selected_slots {
        println!("{:?}", slot);

        let start = slot.slot << 24;
        let end = ((slot.slot + 1) << 24) - 1;

        println!("  start: {:?}, end: {:?}", start, end);
        let selected_ixs = conn.exec_map(
            //"SELECT txid, order, signature, ix, json FROM vwixsSwap t WHERE txid BETWEEN :txidMin AND :txidMax ORDER BY t.txid ASC, t.order ASC",
            "SELECT t.txid, t.order, t.signature, t.ix, t.json FROM vwixsSwap t WHERE txid BETWEEN :s and :e ORDER BY t.txid ASC, t.order ASC",
            params! {
                "s" => start,
                "e" => end,
            },
            |(txid, order, signature, ix, json)| {
                WhirlpoolInstruction {
                    txid,
                    order,
                    signature,
                    ix,
                    json,
                }
            },
        );

        for ix in selected_ixs.unwrap() {
            println!("  {:?}", ix);
        }
    }
}

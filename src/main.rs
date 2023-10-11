use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use csv;
use serde_derive::{Deserialize, Serialize};
use serde::de;
use serde_json;
use base64::{Engine, prelude::BASE64_STANDARD};

use mysql::*;
use mysql::prelude::*;

mod errors;
mod decoded_instructions;
mod util_database_io;
mod util_file_io;
mod util_replay;
mod replay_core;
mod programs;
mod types;

mod replay_instructions;

use decoded_instructions::{from_json, DecodedWhirlpoolInstruction};

use poc_framework::{Environment, LocalEnvironment, PrintableTransaction, setup_logging, LogLevel};

#[derive(Debug, PartialEq, Eq)]
struct Slot {
    slot: u64,
    block_height: u64,
    block_time: i64,
}


fn main() {
    let url = "mysql://root:password@localhost:3306/localtest";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let start_snapshot_slot = 215135999;
    let start_snapshot_file = "data/whirlpool-snapshot-215135999.csv.gz";

    // TODO: protect account_map (stop using HashMap directly)
    let mut account_map = util_file_io::load_from_snapshot_file(&start_snapshot_file.to_string());
    println!("loaded {} accounts", account_map.len());

    let last_processed_slot = util_database_io::fetch_slot_info(start_snapshot_slot, &mut conn);

    let mut next_slots = util_database_io::fetch_next_slot_infos(last_processed_slot.slot, 2048, &mut conn);

    assert_eq!(next_slots[0].slot, last_processed_slot.slot);
    next_slots.pop();

    for slot in next_slots {
        println!("processing slot = {:?} ...", slot);

        let ixs_in_slot = util_database_io::fetch_instructions_in_slot(slot.slot, &mut conn);
        for ix in ixs_in_slot {
            println!("replaying instruction = {} ...", ix.ix_name);

            let result = replay_core::replay_whirlpool_instruction(
                ix.ix,
                &account_map,
                slot.block_time,
                programs::ORCA_WHIRLPOOL_20230901_A574AE5,
                programs::METAPLEX_TOKEN_METADATA_20230903_1_13_3
            );
            
            match result {
                Ok(result) => {
                    if let Some(meta) = result.transaction_status.transaction.clone().meta {
                        if meta.err.is_some() {
                            result.transaction_status.print_named("swap");
                            println!("🔥REPLAY TRANSACTION FAILED!!!");
                            //panic!("🔥REPLAY TRANSACTION FAILED!!!");
                        }
                    }

                    // write back
                    util_replay::update_account_map(&mut account_map, result.snapshot.post_snapshot);
                },
                Err(err) => {
                    println!("🤦‍REPLAY INSTRUCTION FAILED!!! {:?}", err);
                }
            }
        }
    }
}

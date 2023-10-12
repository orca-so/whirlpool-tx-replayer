use mysql::*;
//use mysql::prelude::*;

mod errors;
mod decoded_instructions;
mod util_database_io;
mod util_file_io;
mod util_replay;
mod replay_core;
mod programs;
mod types;
mod replay_instructions;

use poc_framework::PrintableTransaction; // setup_logging, LogLevel};

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

    let start_snapshot_slot = 215135999u64;
    let target_snapshot_slot = start_snapshot_slot + 150;
    let snapshot_interval = 100u64;

    let start_snapshot_file = format!("data/test/whirlpool-snapshot-{start_snapshot_slot}.csv.gz");

    // TODO: protect account_map (stop using HashMap directly)
    let mut account_map = util_file_io::load_from_snapshot_file(&start_snapshot_file.to_string());
    println!("loaded {} accounts", account_map.len());

    let last_processed_slot = util_database_io::fetch_slot_info(start_snapshot_slot, &mut conn);

    let mut next_slots = util_database_io::fetch_next_slot_infos(last_processed_slot.slot, u8::MAX, &mut conn);

    assert_eq!(next_slots[0].slot, last_processed_slot.slot);
    next_slots.pop();

    for slot in next_slots {
        println!("processing slot = {:?} ...", slot);

        let ixs_in_slot = util_database_io::fetch_instructions_in_slot(slot.slot, &mut conn);
        for ix in ixs_in_slot {
            println!("  replaying instruction = {} ...", ix.ix_name);

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
                    util_replay::update_account_map(
                        &mut account_map,
                        result.snapshot.pre_snapshot,
                        result.snapshot.post_snapshot
                    );
                },
                Err(err) => {
                    println!("🤦‍REPLAY INSTRUCTION FAILED!!! {:?}", err);
                }
            }
        }

        let should_save_snapshot = slot.slot == target_snapshot_slot || slot.slot % snapshot_interval == 0;
        if should_save_snapshot {
            println!("saving snapshot ...");
            let snapshot_file = format!("data/test/whirlpool-snapshot-{}.csv.gz", slot.slot);
            util_file_io::save_to_snapshot_file(&snapshot_file.to_string(), &account_map);
            println!("saved snapshot to {}", snapshot_file);
        }
    }
}

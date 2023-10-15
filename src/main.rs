use mysql::*;
//use mysql::prelude::*;
use chrono::Local;

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


fn main() {
    let url = "mysql://root:password@localhost:3306/localtest";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let start_snapshot_slot = 215135999u64;
    let target_snapshot_slot = 215150000u64;// 215567999u64; // start_snapshot_slot + 100;
    let save_snapshot_interval_slot = 10000u64;
    let need_to_process = target_snapshot_slot - start_snapshot_slot;

    let start_snapshot_file = format!("data/test/whirlpool-snapshot-{start_snapshot_slot}.csv.gz");

    // TODO: protect account_map (stop using HashMap directly)
    let mut account_map = util_file_io::load_from_snapshot_file(&start_snapshot_file.to_string());
    println!("loaded {} accounts", account_map.len());

    let mut last_processed_slot = util_database_io::fetch_slot_info(start_snapshot_slot, &mut conn);

    while last_processed_slot.slot < target_snapshot_slot {
        println!("fetching next slots start_slot = {} ...", last_processed_slot.slot);
        let mut next_slots = util_database_io::fetch_next_slot_infos(last_processed_slot.slot, u8::MAX, &mut conn);

        assert_eq!(next_slots[0].slot, last_processed_slot.slot);
        next_slots.remove(0);

        if next_slots.len() == 0 {
            println!("😭NO MORE SLOTS TO PROCESS!! exiting ...");
            break;
        }

        for slot in next_slots {
            if slot.block_height != last_processed_slot.block_height + 1 {
                panic!(
                    "🤮SLOT GAP DETECTED!! block height is not sequential! last_processed_block_height = {}, slot.block_height = {}",
                    last_processed_slot.block_height,
                    slot.block_height
                );
            }

            // print progress
            let now = Local::now();
            let processed = slot.slot - start_snapshot_slot - 1;
            let processed_percent = (processed as f64 / need_to_process as f64) * 100.0;
            println!("[{}, {:.2}%] processing slot = {:?} ...", now.format("%H:%M:%S"), processed_percent, slot);

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
                                result.transaction_status.print_named("instruction");
                                panic!("🔥REPLAY TRANSACTION FAILED!!");
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
                        panic!("🤦‍SOMETHING WENT WRONG!! {:?}", err);
                    }
                }
            }

            // take snapshot at the specific slot
            let should_save_snapshot = slot.slot == target_snapshot_slot || slot.slot % save_snapshot_interval_slot == 0;
            if should_save_snapshot {
                println!("saving snapshot ...");
                let snapshot_file = format!("data/test/whirlpool-snapshot-{}.csv.gz", slot.slot);
                util_file_io::save_to_snapshot_file(&snapshot_file.to_string(), &account_map);
                println!("saved snapshot to {}", snapshot_file);
            }

            // advance
            last_processed_slot = slot;
        }
    }
}

use mysql::*;
//use mysql::prelude::*;
use chrono::Local;

//mod errors;
use replay_engine::decoded_instructions;
mod util_database_io;
mod util_file_io;
mod util;
use replay_engine::programs;
use replay_engine::replay_engine::ReplayEngine;

use solana_transaction_status::UiTransactionEncoding;
use util::PrintableTransaction;

fn main() {
    let start_snapshot_slot = 226394687u64;
    let start_snapshot_block_height = 208_390_198u64;
    let start_snapshot_block_time = 1698451199i64;
    let target_snapshot_slot = 226606335u64;
    let save_snapshot_interval_slot = 100_000u64;
    let need_to_process = target_snapshot_slot - start_snapshot_slot;

    let start_snapshot_file = format!("../data/tmp/whirlpool-snapshot-{start_snapshot_slot}.csv.gz");
    
    // TODO: protect account_map (stop using HashMap directly)
    let mut account_map = util_file_io::load_from_snapshot_file(&start_snapshot_file.to_string());
    println!("loaded {} accounts", account_map.len());

    let mut last_processed_slot = replay_engine::types::Slot {
        slot: start_snapshot_slot,
        block_height: start_snapshot_block_height,
        block_time: start_snapshot_block_time,
    };

    let txs = util_file_io::load_from_transaction_file(&"../data/tmp/transaction-1698451200.jsonl.gz".to_string());
/* 
    println!("txs0 = {}", txs[0]);
    let x = decoded_instructions::json_to_slot_transactions(&txs[0]).unwrap();
    println!("x = {:?}", x);
    let ix = decoded_instructions::from_json(&x.transactions[0].instructions[0].name, &x.transactions[0].instructions[0].payload.to_string()).unwrap();
    println!("ix = {:?}", ix);
*/

    let mut replay_engine = ReplayEngine::new(
        start_snapshot_slot,
        start_snapshot_block_height,
        start_snapshot_block_time,
        programs::ORCA_WHIRLPOOL_20230901_A574AE5.to_vec(),
        account_map,
    );

    for slot_transactions_string in txs {
        let slot_transactions = util_file_io::json_to_slot_transactions(&slot_transactions_string).unwrap();

        // print progress
        let now = Local::now();
        let processed = slot_transactions.slot - start_snapshot_slot - 1;
        let processed_percent = (processed as f64 / need_to_process as f64) * 100.0;
        println!("[{}, {:.2}%] processing slot = {:?} ...", now.format("%H:%M:%S"), processed_percent, slot_transactions.slot);
        
        replay_engine.update_slot(slot_transactions.slot, slot_transactions.block_height, slot_transactions.block_time);

        for tx in slot_transactions.transactions {
            for ix in tx.instructions {
                let name = ix.name;
                let payload = ix.payload.to_string();

                // TODO: handle deploy program
                let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                println!("  replaying instruction = {} ...", name);

                match decoded {
                    decoded_instructions::DecodedInstruction::ProgramDeployInstruction(ix) => {
                        replay_engine.update_program_data(ix.program_data);
                    },
                    decoded_instructions::DecodedInstruction::WhirlpoolInstruction(ix) => {
                        let result = replay_engine.replay_instruction(ix);
                        match result {
                            Ok(result) => {
                                // TODO: refactor, use util ?
                                let meta = result.transaction_status.tx_with_meta.get_status_meta().unwrap();
                                if meta.status.is_err() {
                                    let encoded = result.transaction_status
                                        .encode(UiTransactionEncoding::Binary, Some(0))
                                        .expect("Failed to encode transaction");
                            
                                    encoded.print_named("instruction");
                                    panic!("🔥REPLAY TRANSACTION FAILED!!");
                                }
                            },
                            Err(err) => {
                                panic!("🤦‍SOMETHING WENT WRONG!! {:?}", err);
                            }
                        }        
                    }
                }
            }
        }

        // take snapshot at the specific slot
        let should_save_snapshot = slot_transactions.slot == target_snapshot_slot || slot_transactions.slot % save_snapshot_interval_slot == 0;
        if should_save_snapshot {
            println!("saving snapshot ...");
            let snapshot_file = format!("../tests/output-snapshot/whirlpool-snapshot-{}.csv.gz", slot_transactions.slot);
            util_file_io::save_to_snapshot_file(&snapshot_file.to_string(), &replay_engine.get_accounts());
            println!("saved snapshot to {}", snapshot_file);
        }
        
    }
}

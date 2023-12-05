use mysql::*;
//use mysql::prelude::*;
use chrono::Local;

//mod errors;
use replay_engine::decoded_instructions;
mod util_database_io;
mod util_file_io;
use replay_engine::util_replay;
use replay_engine::replay_environment;
use replay_engine::replay_core;
mod programs;
//mod types;
//mod replay_instructions;

use replay_engine::util_replay::PrintableTransaction;

use solana_program::pubkey::Pubkey;
const SPL_MEMO_PROGRAM_ID: Pubkey = solana_program::pubkey!("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr");
const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID: Pubkey = solana_program::pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");


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

    let mut last_processed_slot = util_database_io::Slot {
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



    // The environment should be rebuilt periodically to avoid processing too many transactions in a single environment.
    // Since Solana is capable of handling 50,000 TPS, it should theoretically be able to safely handle 20,000 txs per bank, haha.
    let mut builder = replay_environment::ReplayEnvironment::builder();

    // deploy programs
    builder.add_upgradable_program(SPL_TOKEN_PROGRAM_ID, programs::SPL_TOKEN);
    builder.add_upgradable_program(SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, programs::SPL_ASSOCIATED_TOKEN_ACCOUNT);
    builder.add_upgradable_program(SPL_MEMO_PROGRAM_ID, programs::SPL_MEMO);
    builder.add_upgradable_program(ORCA_WHIRLPOOL_PROGRAM_ID, programs::ORCA_WHIRLPOOL_20230901_A574AE5);
    // DEV_NULL_PROGRAM will do nothing for every instruction.  It will always succeed.
    //
    // The work of Metaplex Token Program is to create Metadata account,
    // and it does NOT affect the state of Whirlpool accounts, so it can be ignored in replay context.
    // If we handle this program, we need to pay attention to swith V2/V3.
    // I think Metaplex removed V2 instructions at slot 196,112,106.
    // https://solscan.io/tx/5hKy1aL5Si4ymFvUGX7DAhAhDCEWBgpRUdQJNXYC5d4qKfD2xEEAnGfBJpQKRQQt9cZeQ4EZpze5PQjxj5SMBeiP
    // https://github.com/metaplex-foundation/mpl-token-metadata/commit/28f8410f67ce364798f5c36c1dcb244a206b4371
    //builder.add_upgradable_program(METAPLEX_METADATA_PROGRAM_ID, programs::METAPLEX_TOKEN_METADATA_20230903_1_13_3);
    builder.add_upgradable_program(METAPLEX_METADATA_PROGRAM_ID, programs::DEV_NULL_PROGRAM);

    let mut replayer = builder.build();

    for slotTransactionsString in txs {
        let slotTransactions = decoded_instructions::json_to_slot_transactions(&slotTransactionsString).unwrap();

        // print progress
        let now = Local::now();
        let processed = slotTransactions.slot - start_snapshot_slot - 1;
        let processed_percent = (processed as f64 / need_to_process as f64) * 100.0;
        println!("[{}, {:.2}%] processing slot = {:?} ...", now.format("%H:%M:%S"), processed_percent, slotTransactions.slot);
        
        replayer.set_sysvar_clock_unix_timestamp(slotTransactions.block_time);

        for tx in slotTransactions.transactions {
            for ix in tx.instructions {
                let name = ix.name;
                let payload = ix.payload.to_string();
                let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                println!("  replaying instruction = {} ...", name);

                let result = replay_core::replay_whirlpool_instruction(
                    &mut replayer,
                    decoded,
                    &account_map,
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
        }

        // take snapshot at the specific slot
        let should_save_snapshot = slotTransactions.slot == target_snapshot_slot || slotTransactions.slot % save_snapshot_interval_slot == 0;
        if should_save_snapshot {
            println!("saving snapshot ...");
            let snapshot_file = format!("../tests/output-snapshot/whirlpool-snapshot-{}.csv.gz", slotTransactions.slot);
            util_file_io::save_to_snapshot_file(&snapshot_file.to_string(), &account_map);
            println!("saved snapshot to {}", snapshot_file);
        }
        
    }

/* 
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
            if ixs_in_slot.len() > 0 {
                replayer.set_sysvar_clock_unix_timestamp(slot.block_time);

                for ix in ixs_in_slot {
                    println!("  replaying instruction = {} ...", ix.ix_name);

                    let result = replay_core::replay_whirlpool_instruction(
                        &mut replayer,
                        ix.ix,
                        &account_map,
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
            }

            // take snapshot at the specific slot
            let should_save_snapshot = slot.slot == target_snapshot_slot || slot.slot % save_snapshot_interval_slot == 0;
            if should_save_snapshot {
                println!("saving snapshot ...");
                let snapshot_file = format!("tests/output-snapshot/whirlpool-snapshot-{}.csv.gz", slot.slot);
                util_file_io::save_to_snapshot_file(&snapshot_file.to_string(), &account_map);
                println!("saved snapshot to {}", snapshot_file);
            }

            // advance
            last_processed_slot = slot;
        }
//    }
*/
  //  */
}

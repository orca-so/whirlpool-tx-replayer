use chrono::Local;
use replay_engine::decoded_instructions;
mod file_io;
mod util;
use replay_engine::replay_engine::ReplayEngine;
use util::PrintableTransaction;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_path = args[1].to_string();
    println!("base_path = {}", base_path);
    let yyyymmdd = args[2].to_string();
    println!("yyyymmdd = {}", yyyymmdd);

    let yyyymmdd_date = chrono::NaiveDate::parse_from_str(&yyyymmdd, "%Y%m%d").unwrap();
    let previous_yyyymmdd_date = yyyymmdd_date.pred();
    println!("previous_yyyymmdd = {}", previous_yyyymmdd_date.format("%Y%m%d"));

    // snapshot of the previous day
    let state_file_path = format!(
        "{}/{}/{}/whirlpool-state-{}.json.gz",
        base_path,
        previous_yyyymmdd_date.format("%Y"),
        previous_yyyymmdd_date.format("%m%d"),
        previous_yyyymmdd_date.format("%Y%m%d"),
    );
    println!("snapshot_file_path = {}", state_file_path);

    // transactions of the day
    let transaction_file_path = format!(
        "{}/{}/{}/whirlpool-transaction-{}.jsonl.gz",
        base_path,
        yyyymmdd_date.format("%Y"),
        yyyymmdd_date.format("%m%d"),
        yyyymmdd_date.format("%Y%m%d"),
    );
    println!("transaction_file_path = {}", transaction_file_path);

    let state = file_io::load_from_whirlpool_state_file(&state_file_path);
    let transaction_iter = file_io::load_from_whirlpool_transaction_file(&transaction_file_path);

    let mut replay_engine = ReplayEngine::new(
        state.slot,
        state.block_height,
        state.block_time,
        state.program_data,
        file_io::convert_accounts_to_account_map(&state.accounts),
    );

    for tx in transaction_iter {
        // print progress
        let now = Local::now();
        println!("[{}] processing slot = {:?} ...", now.format("%H:%M:%S"), tx.slot);
        
        replay_engine.update_slot(tx.slot, tx.block_height, tx.block_time);

        for tx in tx.transactions {
            for ix in tx.instructions {
                let name = ix.name;
                let payload = ix.payload.to_string();
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
                                if let Some(meta) = result.transaction_status.transaction.clone().meta {
                                    if meta.err.is_some() {
                                        result.transaction_status.print_named("instruction");
                                        panic!("🔥REPLAY TRANSACTION FAILED!!");
                                    }
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
    }

    // save snapshot
    let snapshot_file = "next-whirlpool-state.json.gz";
    let latest_slot = replay_engine.get_slot();
    let latest_accounts = file_io::convert_account_map_to_accounts(replay_engine.get_accounts());
    file_io::save_to_whirlpool_state_file(
        &snapshot_file.to_string(),
        &file_io::WhirlpoolState {
            slot: latest_slot.slot,
            block_height: latest_slot.block_height,
            block_time: latest_slot.block_time,
            program_data: replay_engine.get_program_data().clone(),
            accounts: latest_accounts,
        }
    );
}

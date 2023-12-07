//use chrono::Local;
//use replay_engine::decoded_instructions;
mod file_io;
mod util;
//use replay_engine::replay_engine::ReplayEngine;
//use solana_transaction_status::UiTransactionEncoding;
//use util::PrintableTransaction;
use std::env;

use replayer::{WhirlpoolReplayer, ReplayUntil, SlotCallback, InstructionCallback};

fn main() {
    let args: Vec<String> = env::args().collect();

    let base_path = args[1].to_string();
    println!("base_path = {}", base_path);
    let yyyymmdd = args[2].to_string();
    println!("yyyymmdd = {}", yyyymmdd);

    let until_slot = if args.len() > 3 {
        ReplayUntil::Slot(args[3].parse::<u64>().unwrap())
    } else {
        ReplayUntil::End
    };

    let yyyymmdd_date = chrono::NaiveDate::parse_from_str(&yyyymmdd, "%Y%m%d").unwrap();
    let previous_yyyymmdd_date = yyyymmdd_date.pred();
    println!("previous_yyyymmdd = {}", previous_yyyymmdd_date.format("%Y%m%d"));

    let mut replayer = WhirlpoolReplayer::build_with_local_file_storage(&base_path, &yyyymmdd);

    let slot_callback: Option<SlotCallback> = Some(|slot| {
        println!("processing slot: {} ({}) ...", slot.slot, slot.block_height);
    });

    let instruction_callback: Option<InstructionCallback> = Some(|slot, transaction, name, instruction, accounts, result| {
        println!("  replayed instruction: {}", name);
    });

    replayer.replay(until_slot, slot_callback, instruction_callback);
    
    // save snapshot
    let snapshot_file = "next-whirlpool-state.json.gz";
    let latest_slot = replayer.get_slot();
    let latest_accounts = file_io::convert_account_map_to_accounts(replayer.get_accounts());
    file_io::save_to_whirlpool_state_file(
        &snapshot_file.to_string(),
        &file_io::WhirlpoolState {
            slot: latest_slot.slot,
            block_height: latest_slot.block_height,
            block_time: latest_slot.block_time,
            program_data: replayer.get_program_data().clone(),
            accounts: latest_accounts,
        }
    );
}

use std::env;
use replayer::{file_io, WhirlpoolReplayer, ReplayUntil, SlotCallback, InstructionCallback};

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

    let instruction_callback: Option<InstructionCallback> = Some(|_slot, _transaction, name, _instruction, _accounts, _result| {
        println!("  replayed instruction: {}", name);
    });

    replayer.replay(until_slot, slot_callback, instruction_callback);

    // save snapshot
    let latest_slot = replayer.get_slot();
    let latest_accounts = file_io::convert_account_map_to_accounts(replayer.get_accounts());
    let snapshot_file = format!("whirlpool-state-slot-{}.json.gz", latest_slot.slot);
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

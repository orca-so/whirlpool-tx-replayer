use clap::Parser;
use replayer::{file_io, InstructionCallback, ReplayUntil, SlotCallback, WhirlpoolReplayer};

#[derive(Parser, Debug)]
struct Args {
    #[clap(long, id = "directory")]
    cache_dir: Option<String>,

    #[clap(short, long, id = "filename")]
    save_as: Option<String>,

    #[clap(long, id = "slot")]
    stop_slot: Option<u64>,

    #[clap(long, id = "blockHeight")]
    stop_block_height: Option<u64>,

    #[clap(long, id = "blockTime")]
    stop_block_time: Option<i64>,

    #[clap(id = "path|url")]
    storage: String,

    #[clap(id = "yyyymmdd")]
    yyyymmdd: String,
}

fn main() {
    let args = Args::parse();

    let base_path_or_url: String = args.storage;
    let yyyymmdd: String = args.yyyymmdd;

    let until_condition = if args.stop_slot.is_some() {
        ReplayUntil::Slot(args.stop_slot.unwrap())
    } else if args.stop_block_height.is_some() {
        ReplayUntil::BlockHeight(args.stop_block_height.unwrap())
    } else if args.stop_block_time.is_some() {
        ReplayUntil::BlockTime(args.stop_block_time.unwrap())
    } else {
        ReplayUntil::End
    };

    //let mut replayer = WhirlpoolReplayer::build_with_local_file_storage(&base_path, &yyyymmdd);
    let mut replayer = if base_path_or_url.starts_with("https://") {
        if args.cache_dir.is_some() {
            let cache_dir = args.cache_dir.unwrap();
            WhirlpoolReplayer::build_with_remote_file_storage_with_local_cache(
                &base_path_or_url,
                &yyyymmdd,
                &cache_dir,
                false,
            )
        } else {
            WhirlpoolReplayer::build_with_remote_file_storage(&base_path_or_url, &yyyymmdd)
        }
    } else {
        WhirlpoolReplayer::build_with_local_file_storage(&base_path_or_url, &yyyymmdd)
    };

    let slot_callback: Option<SlotCallback> = Some(|slot| {
        println!("processing slot: {} ({}) ...", slot.slot, slot.block_height);
    });

    let instruction_callback: Option<InstructionCallback> = Some(
        |_slot, _transaction, name, _instruction, _accounts, _result| {
            println!("  replayed instruction: {}", name);
        },
    );

    replayer.replay(until_condition, slot_callback, instruction_callback);

    // save state
    if args.save_as.is_some() {
        let state_file = args.save_as.unwrap();

        let latest_slot = replayer.get_slot();
        let latest_program_data = replayer.get_program_data().clone();
        let latest_accounts = file_io::convert_account_map_to_accounts(replayer.get_accounts());
        file_io::save_to_whirlpool_state_file(
            &state_file.to_string(),
            &file_io::WhirlpoolState {
                slot: latest_slot.slot,
                block_height: latest_slot.block_height,
                block_time: latest_slot.block_time,
                program_data: latest_program_data,
                accounts: latest_accounts,
            },
        );
    }
}

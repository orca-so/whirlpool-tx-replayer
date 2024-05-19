use clap::Parser;
use whirlpool_replayer::{io, schema, InstructionCallback, ReplayUntil, SlotCallback, WhirlpoolReplayer};

use anchor_lang::prelude::*;
use whirlpool_base::state::Whirlpool;

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

    #[clap(short, long, id = "memory")]
    memory: bool,

    #[clap(id = "path|url")]
    storage: String,

    #[clap(id = "yyyymmdd")]
    yyyymmdd: String,
}

fn main() {
    let args = Args::parse();

    let on_memory = args.memory;

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

    let mut replayer = if base_path_or_url.starts_with("https://") {
        if args.cache_dir.is_some() {
            let cache_dir = args.cache_dir.unwrap();
            WhirlpoolReplayer::build_with_remote_file_storage_with_local_cache(
                &base_path_or_url,
                &yyyymmdd,
                on_memory,
                &cache_dir,
                false,
            )
        } else {
            WhirlpoolReplayer::build_with_remote_file_storage(&base_path_or_url, &yyyymmdd, on_memory)
        }
    } else {
        WhirlpoolReplayer::build_with_local_file_storage(&base_path_or_url, &yyyymmdd, on_memory)
    };

    let slot_callback: Option<SlotCallback> = Some(|slot| {
        println!("processing slot: {} (block_height={} block_time={}) ...", slot.slot, slot.block_height, slot.block_time);
    });

    let instruction_callback: Option<InstructionCallback> = Some(
        |_slot, transaction, name, instruction, accounts, result| {
            println!("  replayed instruction: {}", name);

            // callback will receive various data to implement various data processing!
            // For example, print the details of swap instruction with pre/post account state info.
            match instruction 
            {
                schema::DecodedWhirlpoolInstruction::Swap(params) => {
                    // accounts provides "post" state
                    // note: accounts contains all whirlpool accounts at the end of the instruction
                    let post_data = accounts.get(&params.key_whirlpool).unwrap().unwrap();
                    let post_whirlpool = Whirlpool::try_deserialize(&mut post_data.as_slice()).unwrap();

                    // we can get "pre" state from result
                    // note: snapshot only contains whirlpool accounts mentioned in the instruction
                    let pre_data = result.snapshot.pre_snapshot.get(&params.key_whirlpool).unwrap();
                    let pre_whirlpool = Whirlpool::try_deserialize(&mut pre_data.as_slice()).unwrap();

                    println!("    tx signature: {}", transaction.signature);
                    println!("    pool: {} (ts={}, fee_rate={})", params.key_whirlpool, pre_whirlpool.tick_spacing, pre_whirlpool.fee_rate);
                    println!("      direction: {}", if params.data_a_to_b { "A to B"} else { "B to A"});
                    println!("      in/out: in={} out={}", params.transfer_amount_0, params.transfer_amount_1);
                    println!("      sqrt_price: pre={} post={}", pre_whirlpool.sqrt_price, post_whirlpool.sqrt_price);
                },
                _ => {},
            }
        },
    );

    replayer.replay(until_condition, slot_callback, instruction_callback);

    // save state
    if args.save_as.is_some() {
        let state_file = args.save_as.unwrap();

        let latest_slot = replayer.get_slot();
        let latest_program_data = replayer.get_program_data();
        let latest_accounts = replayer.get_accounts();
        io::save_to_whirlpool_state_file(
            &state_file.to_string(),
            latest_slot,
            latest_program_data,
            latest_accounts,
        );
    }
}

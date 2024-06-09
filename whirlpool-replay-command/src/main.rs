use std::{cell::RefCell, collections::HashMap, rc::Rc};

use clap::Parser;
use itertools::Itertools;

use whirlpool_replayer::{
    io,
    schema,
    serde,
    WhirlpoolReplayer,
    ReplayUntil,
    SyncInstructionCallback,
    SyncSlotCallback
};

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

    let base_path_or_url: String = args.storage;
    let yyyymmdd: String = args.yyyymmdd;

    let account_data_store_config = if args.memory {
        // account data will be stored on memory
        serde::AccountDataStoreConfig::OnMemory
    } else {
        // account data will be stored on RocksDB in system default temporary directory (e.g. /tmp)
        // very small memory footprint, but (10% ~ 20%) slower than on-memory
        serde::AccountDataStoreConfig::OnDisk(None)
    };

    let until_condition = if args.stop_slot.is_some() {
        ReplayUntil::Slot(args.stop_slot.unwrap())
    } else if args.stop_block_height.is_some() {
        ReplayUntil::BlockHeight(args.stop_block_height.unwrap())
    } else if args.stop_block_time.is_some() {
        ReplayUntil::BlockTime(args.stop_block_time.unwrap())
    } else {
        ReplayUntil::End
    };

    // build replayer
    let mut replayer = if base_path_or_url.starts_with("https://") {
        if args.cache_dir.is_some() {
            let cache_dir = args.cache_dir.unwrap();
            WhirlpoolReplayer::build_with_remote_file_storage_with_local_cache(
                &base_path_or_url,
                &yyyymmdd,
                &account_data_store_config,
                &cache_dir,
                false,
            )
        } else {
            WhirlpoolReplayer::build_with_remote_file_storage(&base_path_or_url, &yyyymmdd, &account_data_store_config)
        }
    } else {
        WhirlpoolReplayer::build_with_local_file_storage(&base_path_or_url, &yyyymmdd, &account_data_store_config)
    };

    // define callbacks
    let slot_pre_callback: SyncSlotCallback = Rc::new(|slot, _accounts| {
        println!("processing slot: {} (block_height={} block_time={}) ...", slot.slot, slot.block_height, slot.block_time);

        // We can use accounts to access all whirlpool accounts at the beginning of the slot
    });

    // how to extract data from replayer
    let instruction_counter = Rc::new(RefCell::new(HashMap::<String, u64>::new()));
    let instruction_counter_clone = Rc::clone(&instruction_counter);

    let instruction_callback: SyncInstructionCallback = Rc::new(
        move |_slot, transaction, name, instruction, accounts, snapshot| {
            println!("  replayed instruction: {}", name);

            // callback will receive various data to implement various data processing!
            // For example, print the details of swap instruction with pre/post writable account state info.
            match instruction 
            {
                schema::DecodedWhirlpoolInstruction::Swap(params) => {
                    // accounts provides "post" state
                    // note: accounts contains all whirlpool accounts at the end of the instruction
                    let post_data = accounts.get(&params.key_whirlpool).unwrap().unwrap();
                    let post_whirlpool = Whirlpool::try_deserialize(&mut post_data.as_slice()).unwrap();

                    // we can get "pre" state from result
                    // note: snapshot only contains whirlpool accounts mentioned in the instruction
                    let pre_data = snapshot.pre_snapshot.get(&params.key_whirlpool).unwrap();
                    let pre_whirlpool = Whirlpool::try_deserialize(&mut pre_data.as_slice()).unwrap();

                    println!("    tx signature: {}", transaction.signature);
                    println!("    pool: {} (ts={}, fee_rate={})", params.key_whirlpool, pre_whirlpool.tick_spacing, pre_whirlpool.fee_rate);
                    println!("      direction: {}", if params.data_a_to_b { "A to B"} else { "B to A"});
                    println!("      in/out: in={} out={}", params.transfer_amount_0, params.transfer_amount_1);
                    println!("      sqrt_price: pre={} post={}", pre_whirlpool.sqrt_price, post_whirlpool.sqrt_price);
                },
                _ => {},
            }

            // update var (out of callback closure)
            let mut counter = instruction_counter_clone.borrow_mut();
            let count = counter.entry(name.clone()).or_insert(0u64);
            *count += 1;
        },
    );

    replayer.replay(
        until_condition,
        Some(instruction_callback),
        Some(slot_pre_callback),
        None // no slot_post_callback
    );

    // show instruction count
    println!("\n\nReplayed instructions\n");
    for (ix, count) in instruction_counter.borrow().iter().sorted() {
        println!("  {:8} : {}", count, ix);
    }

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

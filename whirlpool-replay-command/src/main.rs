use clap::Parser;
use whirlpool_replayer::{io, schema, util, ReplayUntil, WhirlpoolReplayer, Slot, AccountMap, Transaction, DecodedWhirlpoolInstruction, ReplayInstructionResult};

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

    let mut slot_callback = |slot: &Slot, _accounts: &AccountMap| {
        println!("processing slot: {} (block_height={} block_time={}) ...", slot.slot, slot.block_height, slot.block_time);
    };

    let mut swap_counter = 0u32;
    let mut two_hop_swap_counter = 0u32;
    let mut tx_signature_sample_containing_two_hop_swap = Vec::<String>::new();
    let mut instruction_callback =
        |_slot: &Slot, transaction: &Transaction, name: &String, instruction: &DecodedWhirlpoolInstruction, accounts: &AccountMap, result: &ReplayInstructionResult| {
            println!("  replayed instruction: {}", name);

            // callback will receive various data to implement various data processing!
            // For example, print the details of Swap and TwoHopSwap instruction with pre/post account state info.
            match instruction 
            {
                DecodedWhirlpoolInstruction::Swap(params) => {
                    swap_counter = swap_counter + 1;

                    // accounts provides "post" state
                    // note: accounts contains all whirlpool accounts at the end of the instruction
                    let post_data = accounts.get(&params.key_whirlpool).unwrap();
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
                DecodedWhirlpoolInstruction::TwoHopSwap(params) => {
                    two_hop_swap_counter = two_hop_swap_counter + 1;
                    if tx_signature_sample_containing_two_hop_swap.len() < 10 {
                        tx_signature_sample_containing_two_hop_swap.push(transaction.signature.clone());
                    }

                    println!("    tx signature: {}", transaction.signature);

                    let post_data_one = accounts.get(&params.key_whirlpool_one).unwrap();
                    let post_whirlpool_one = Whirlpool::try_deserialize(&mut post_data_one.as_slice()).unwrap();
                    let pre_data_one = result.snapshot.pre_snapshot.get(&params.key_whirlpool_one).unwrap();
                    let pre_whirlpool_one = Whirlpool::try_deserialize(&mut pre_data_one.as_slice()).unwrap();

                    println!("    pool: {} (ts={}, fee_rate={})", params.key_whirlpool_one, pre_whirlpool_one.tick_spacing, pre_whirlpool_one.fee_rate);
                    println!("      direction: {}", if params.data_a_to_b_one { "A to B"} else { "B to A"});
                    println!("      in/out: in={} out={}", params.transfer_amount_0, params.transfer_amount_1);
                    println!("      sqrt_price: pre={} post={}", pre_whirlpool_one.sqrt_price, post_whirlpool_one.sqrt_price);

                    let post_data_two = accounts.get(&params.key_whirlpool_two).unwrap();
                    let post_whirlpool_two = Whirlpool::try_deserialize(&mut post_data_two.as_slice()).unwrap();
                    let pre_data_two = result.snapshot.pre_snapshot.get(&params.key_whirlpool_two).unwrap();
                    let pre_whirlpool_two = Whirlpool::try_deserialize(&mut pre_data_two.as_slice()).unwrap();

                    println!("    pool: {} (ts={}, fee_rate={})", params.key_whirlpool_two, pre_whirlpool_two.tick_spacing, pre_whirlpool_two.fee_rate);
                    println!("      direction: {}", if params.data_a_to_b_two { "A to B"} else { "B to A"});
                    println!("      in/out: in={} out={}", params.transfer_amount_2, params.transfer_amount_3);
                    println!("      sqrt_price: pre={} post={}", pre_whirlpool_two.sqrt_price, post_whirlpool_two.sqrt_price);
                },
                _ => {},
            }
        };

    replayer.replay(until_condition, &mut slot_callback, &mut instruction_callback);

    println!("swap_counter: {}", swap_counter);
    println!("two_hop_swap_counter: {}", two_hop_swap_counter);
    println!("tx_signatures_containing_two_hop_swap: {:?}", tx_signature_sample_containing_two_hop_swap);

    // save state
    if args.save_as.is_some() {
        let state_file = args.save_as.unwrap();

        let latest_slot = replayer.get_slot();
        let latest_program_data = replayer.get_program_data().clone();
        let latest_accounts = util::convert_account_map_to_accounts(replayer.get_accounts());
        io::save_to_whirlpool_state_file(
            &state_file.to_string(),
            &schema::WhirlpoolState {
                slot: latest_slot.slot,
                block_height: latest_slot.block_height,
                block_time: latest_slot.block_time,
                program_data: latest_program_data,
                accounts: latest_accounts,
            },
        );
    }
}

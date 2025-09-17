use std::vec;
use clap::Parser;

use replay_engine::{decoded_instructions, replay_engine::ReplayEngine, types::{ProgramData, WritableAccountSnapshot}};
use whirlpool_replayer::{
    io, schema::{WhirlpoolTransaction}, serde::{self, AccountDataStoreConfig}, Slot,
};

#[derive(Parser, Debug)]
struct Args {
    #[clap(id = "path")]
    storage: String,

    #[clap(id = "yyyymmdd")]
    yyyymmdd: String,

    #[clap(id = "program so (left)")]
    program_path_left: String,

    #[clap(id = "program so (right)")]
    program_path_right: String,
}

fn main() {
    let args = Args::parse();

    let base_path: String = args.storage;
    let yyyymmdd: String = args.yyyymmdd;
    let account_data_store_config = serde::AccountDataStoreConfig::OnMemory;

    if base_path.starts_with("https://") {
        println!("Error: only local file storage is supported");
        std::process::exit(1);
    }

    // load program data from binary .so file
    let program_data_left = std::fs::read(&args.program_path_left).unwrap();
    let program_data_right = std::fs::read(&args.program_path_right).unwrap();
    
    // build replayer
    let mut replayer_left = WhirlpoolReplayerStep::build_with_local_file_storage(&base_path, &yyyymmdd, &account_data_store_config);
    let mut replayer_right = WhirlpoolReplayerStep::build_with_local_file_storage(&base_path, &yyyymmdd, &account_data_store_config);

    // override program data
    replayer_left.override_program_data(program_data_left);
    replayer_right.override_program_data(program_data_right);

    loop {
        println!("left replayer...");
        let result_left = replayer_left.replay_one_slot();
        println!("right replayer...");
        let result_right = replayer_right.replay_one_slot();

        assert_eq!(result_left.is_some(), result_right.is_some());
        if result_left.is_none() {
            break;
        }

        let result_left = result_left.unwrap();
        let result_right = result_right.unwrap();
        assert_eq!(result_left.len(), result_right.len());

        // foreach zipped
        for (left, right) in result_left.iter().zip(result_right.iter()) {
            let (slot_left, signature_left, name_left, payload_left, snapshot_left) = left;
            let (slot_right, signature_right, name_right, payload_right, snapshot_right) = right;

            assert_eq!(slot_left, slot_right);
            assert_eq!(signature_left, signature_right);
            assert_eq!(name_left, name_right);
            assert_eq!(payload_left, payload_right);

            // compare snapshots
            let snapshot_left = snapshot_left.post_snapshot.clone();
            let snapshot_right = snapshot_right.post_snapshot.clone();
            assert_eq!(snapshot_left.len(), snapshot_right.len());
            for (key_left, account_left) in snapshot_left.iter() {
                let account_right = snapshot_right.get(key_left).unwrap();
                if account_left != account_right {
                    println!("Account mismatch: slot={}, signature={}, name={}, payload={}", slot_left, signature_left, name_left, payload_left);
                    println!("Left: {} => {:?}", key_left, account_left);
                    println!("Right: {} => {:?}", key_left, account_right);
                    panic!("Fatal: Account mismatch");
                }
            }

            println!("ok: slot={}, signature={}, name={}", slot_left, signature_left, name_left);
        }        
    }

    println!("Replay finished successfully (no regression detected)");
}

pub struct WhirlpoolReplayerStep {
    replay_engine: ReplayEngine,
    transaction_iter: Box<dyn Iterator<Item = WhirlpoolTransaction> + Send>,
}

impl WhirlpoolReplayerStep {
    pub fn build_with_local_file_storage(
        base_path: &String,
        yyyymmdd: &String,
        account_data_store_config: &AccountDataStoreConfig,
    ) -> WhirlpoolReplayerStep {
        let current = chrono::NaiveDate::parse_from_str(yyyymmdd, "%Y%m%d").unwrap();
        let previous = current.pred_opt().unwrap();

        // snapshot of the previous day
        let state_file_relative_path = io::get_whirlpool_state_file_relative_path(&previous);
        let state_file_path = format!("{}/{}", base_path, state_file_relative_path);
        // transactions of the day
        let transaction_file_relative_path =
            io::get_whirlpool_transaction_file_relative_path(&current);
        let transaction_file_path = format!("{}/{}", base_path, transaction_file_relative_path);

        let state = io::load_from_local_whirlpool_state_file(&state_file_path, account_data_store_config);
        let transaction_iter =
            io::load_from_local_whirlpool_transaction_file(&transaction_file_path);

        let replay_engine = ReplayEngine::new(
            Slot::new(state.slot, state.block_height, state.block_time),
            state.program_data,
            state.accounts,
        );

        return WhirlpoolReplayerStep {
            replay_engine,
            transaction_iter: Box::new(transaction_iter),
        };
    }

    pub fn override_program_data(&mut self, program_data: ProgramData) {
        self.replay_engine.update_program_data(program_data);
    }

    pub fn replay_one_slot(
        &mut self,
    ) -> Option<Vec<(u64, String, String, String, WritableAccountSnapshot)>> {
        let next_whirlpool_transaction = self.transaction_iter.next();
        if next_whirlpool_transaction.is_none() {
            return None;
        }

        let whirlpool_transaction = next_whirlpool_transaction.unwrap();

        let slot = Slot {
            slot: whirlpool_transaction.slot,
            block_height: whirlpool_transaction.block_height,
            block_time: whirlpool_transaction.block_time,
        };

        self.replay_engine
            .update_slot(slot.slot, slot.block_height, slot.block_time);

        let mut writable_account_snapshots: Vec<(u64, String, String, String, WritableAccountSnapshot)> = vec![];

        for transaction in whirlpool_transaction.transactions {
            let signature = transaction.signature.clone();
            for instruction in transaction.clone().instructions {
                let name = instruction.name;
                let payload = instruction.payload.to_string();
                let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                match decoded {
                    decoded_instructions::DecodedInstruction::ProgramDeployInstruction(
                        _deploy_instruction,
                    ) => {
                        // self.replay_engine
                        //    .update_program_data(deploy_instruction.program_data);
                    }
                    decoded_instructions::DecodedInstruction::WhirlpoolInstruction(
                        whirlpool_instruction,
                    ) => {
                        let result = self
                            .replay_engine
                            .replay_instruction(&whirlpool_instruction);
                        if result.is_err() {
                            let e = result.err().unwrap();
                            println!("Error: {:?}", e);
                            println!("REPLAY: slot={}, signature={}, name={}, payload={}", slot.slot, signature, name, payload);
                            panic!("Fatal: Error during replay");
                        }
                        let result = result.unwrap();

                        writable_account_snapshots.push((
                            slot.slot,
                            signature.clone(),
                            name.clone(),
                            payload.clone(),
                            result.snapshot.clone(),
                        ));
                    }
                }
            }
        }

        Some(writable_account_snapshots)
    }
}

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::rc::Rc;

use replay_engine::{account_data_store::AccountDataStore, types::WritableAccountSnapshot};
use replay_engine::decoded_instructions;
use replay_engine::decoded_instructions::DecodedWhirlpoolInstruction;
use replay_engine::replay_engine::ReplayEngine;

pub use replay_engine::replay_instruction::ReplayInstructionResult;
use replay_engine::types::ProgramData;
pub use replay_engine::types::{AccountSnapshot, Slot};

pub mod io;
pub mod schema;
pub mod serde;

use schema::{Transaction, WhirlpoolTransaction};
use serde::AccountDataStoreConfig;
use tokio::sync::Mutex;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReplayUntil {
    End,
    Slot(u64),
    BlockHeight(u64),
    BlockTime(i64),
}

pub type SyncSlotCallback = Rc<
    dyn Fn(
        &Slot,
        &AccountDataStore
    )
>;
pub type SyncInstructionCallback = Rc<
    dyn Fn(
        &Slot,
        &Transaction,
        &String,
        &DecodedWhirlpoolInstruction,
        &AccountDataStore,
        &WritableAccountSnapshot,
    )
>;

pub type AsyncSlotCallback = Arc<Mutex<
    dyn Fn(
        &Slot,
        &AccountDataStore
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send
>>;
pub type AsyncInstructionCallback = Arc<Mutex<
    dyn Fn(
        &Slot,
        &Transaction,
        &String,
        &DecodedWhirlpoolInstruction,
        &AccountDataStore,
        &WritableAccountSnapshot,
    ) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send
>>;

pub struct WhirlpoolReplayer {
    replay_engine: ReplayEngine,
    transaction_iter: Box<dyn Iterator<Item = WhirlpoolTransaction> + Send>,
}

impl WhirlpoolReplayer {
    // TODO: build_with_mysql_database

    pub fn build_with_local_file_storage(
        base_path: &String,
        yyyymmdd: &String,
        account_data_store_config: &AccountDataStoreConfig,
    ) -> WhirlpoolReplayer {
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

        return WhirlpoolReplayer {
            replay_engine,
            transaction_iter: Box::new(transaction_iter),
        };
    }

    pub fn build_with_remote_file_storage(
        base_url: &String,
        yyyymmdd: &String,
        account_data_store_config: &AccountDataStoreConfig,
    ) -> WhirlpoolReplayer {
        let current = chrono::NaiveDate::parse_from_str(yyyymmdd, "%Y%m%d").unwrap();
        let previous = current.pred_opt().unwrap();

        // snapshot of the previous day
        let state_file_relative_path = io::get_whirlpool_state_file_relative_path(&previous);
        let state_file_url = format!("{}/{}", base_url, state_file_relative_path);
        // transactions of the day
        let transaction_file_relative_path =
            io::get_whirlpool_transaction_file_relative_path(&current);
        let transaction_file_url = format!("{}/{}", base_url, transaction_file_relative_path);

        let state = io::load_from_remote_whirlpool_state_file(&state_file_url, account_data_store_config);
        let transaction_iter =
            io::load_from_remote_whirlpool_transaction_file(&transaction_file_url);

        let replay_engine = ReplayEngine::new(
            Slot::new(state.slot, state.block_height, state.block_time),
            state.program_data,
            state.accounts,
        );

        return WhirlpoolReplayer {
            replay_engine,
            transaction_iter: Box::new(transaction_iter),
        };
    }

    pub fn build_with_remote_file_storage_with_local_cache(
        base_url: &String,
        yyyymmdd: &String,
        account_data_store_config: &AccountDataStoreConfig,
        cache_dir_path: &String,
        refresh: bool,
    ) -> WhirlpoolReplayer {
        let current = chrono::NaiveDate::parse_from_str(yyyymmdd, "%Y%m%d").unwrap();
        let previous = current.pred_opt().unwrap();

        // snapshot of the previous day
        let state_file_relative_path = io::get_whirlpool_state_file_relative_path(&previous);
        let state_file_url = format!("{}/{}", base_url, state_file_relative_path);
        // transactions of the day
        let transaction_file_relative_path =
            io::get_whirlpool_transaction_file_relative_path(&current);
        let transaction_file_url = format!("{}/{}", base_url, transaction_file_relative_path);

        let state_file_path = format!("{}/{}", cache_dir_path, state_file_relative_path);
        if refresh || !std::path::Path::new(&state_file_path).exists() {
            io::download_from_remote_storage(&state_file_url, &state_file_path);
        }

        let transaction_file_path =
            format!("{}/{}", cache_dir_path, transaction_file_relative_path);
        if refresh || !std::path::Path::new(&transaction_file_path).exists() {
            io::download_from_remote_storage(&transaction_file_url, &transaction_file_path);
        }

        let state = io::load_from_local_whirlpool_state_file(&state_file_path, account_data_store_config);
        let transaction_iter =
            io::load_from_local_whirlpool_transaction_file(&transaction_file_path);

        let replay_engine = ReplayEngine::new(
            Slot::new(state.slot, state.block_height, state.block_time),
            state.program_data,
            state.accounts,
        );

        return WhirlpoolReplayer {
            replay_engine,
            transaction_iter: Box::new(transaction_iter),
        };
    }

    pub fn get_slot(&self) -> &Slot {
        return self.replay_engine.get_slot();
    }

    pub fn get_program_data(&self) -> &ProgramData {
        return self.replay_engine.get_program_data();
    }

    pub fn get_accounts(&self) -> &AccountDataStore {
        return self.replay_engine.get_accounts();
    }

    pub fn replay(
        &mut self,
        cond: ReplayUntil,
        instruction_callback: Option<SyncInstructionCallback>,
        slot_pre_callback: Option<SyncSlotCallback>,
        slot_post_callback: Option<SyncSlotCallback>,
    ) {
        let mut next_whirlpool_transaction = self.transaction_iter.next();
        while next_whirlpool_transaction.is_some() {
            let whirlpool_transaction = next_whirlpool_transaction.unwrap();

            let slot = Slot {
                slot: whirlpool_transaction.slot,
                block_height: whirlpool_transaction.block_height,
                block_time: whirlpool_transaction.block_time,
            };

            if has_reached_until_condition(&cond, slot) {
                break;
            }

            self.replay_engine
                .update_slot(slot.slot, slot.block_height, slot.block_time);

            if let Some(callback) = slot_pre_callback.as_ref() {
                callback(self.replay_engine.get_slot(), self.replay_engine.get_accounts());
            }

            for transaction in whirlpool_transaction.transactions {
                for instruction in transaction.clone().instructions {
                    let name = instruction.name;
                    let payload = instruction.payload.to_string();
                    let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                    match decoded {
                        decoded_instructions::DecodedInstruction::ProgramDeployInstruction(
                            deploy_instruction,
                        ) => {
                            self.replay_engine
                                .update_program_data(deploy_instruction.program_data);
                        }
                        decoded_instructions::DecodedInstruction::WhirlpoolInstruction(
                            whirlpool_instruction,
                        ) => {
                            let result = self
                                .replay_engine
                                .replay_instruction(&whirlpool_instruction)
                                .unwrap();

                            if let Some(callback) = instruction_callback.as_ref() {
                                callback(
                                    &slot,
                                    &transaction,
                                    &name,
                                    &whirlpool_instruction,
                                    self.replay_engine.get_accounts(),
                                    &result.snapshot,
                                );
                            }
                        }
                    }
                }
            }

            if let Some(callback) = slot_post_callback.as_ref() {
                callback(self.replay_engine.get_slot(), self.replay_engine.get_accounts());
            }

            next_whirlpool_transaction = self.transaction_iter.next();
        }
    }

    pub async fn replay_async(
        &mut self,
        cond: ReplayUntil,
        instruction_callback: Option<AsyncInstructionCallback>,
        slot_pre_callback: Option<AsyncSlotCallback>,
        slot_post_callback: Option<AsyncSlotCallback>,
    ) {
        let mut next_whirlpool_transaction = self.transaction_iter.next();
        while next_whirlpool_transaction.is_some() {
            let whirlpool_transaction = next_whirlpool_transaction.unwrap();

            let slot = Slot {
                slot: whirlpool_transaction.slot,
                block_height: whirlpool_transaction.block_height,
                block_time: whirlpool_transaction.block_time,
            };

            if has_reached_until_condition(&cond, slot) {
                break;
            }

            self.replay_engine
                .update_slot(slot.slot, slot.block_height, slot.block_time);

            if let Some(callback) = slot_pre_callback.as_ref() {
                let callback_guard = callback.lock().await;
                callback_guard(self.replay_engine.get_slot(), self.replay_engine.get_accounts()).await;
            }

            for transaction in whirlpool_transaction.transactions {
                for instruction in transaction.clone().instructions {
                    let name = instruction.name.clone();
                    let payload = instruction.payload.to_string().clone();
                    let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                    match decoded {
                        decoded_instructions::DecodedInstruction::ProgramDeployInstruction(
                            deploy_instruction,
                        ) => {
                            self.replay_engine
                                .update_program_data(deploy_instruction.program_data);
                        }
                        decoded_instructions::DecodedInstruction::WhirlpoolInstruction(
                            whirlpool_instruction,
                        ) => {
                            let result = self
                                .replay_engine
                                .replay_instruction(&whirlpool_instruction)
                                .unwrap();

                            let accounts = self.replay_engine.get_accounts();

                            if let Some(callback) = instruction_callback.as_ref() {
                                let callback_guard = callback.lock().await;
                                callback_guard(
                                    &slot,
                                    &transaction,
                                    &name,
                                    &whirlpool_instruction,
                                    &accounts,
                                    &result.snapshot,
                                ).await;
                            }
                        }
                    }
                }
            }

            if let Some(callback) = slot_post_callback.as_ref() {
                let callback_guard = callback.lock().await;
                callback_guard(self.replay_engine.get_slot(), self.replay_engine.get_accounts()).await;
            }

            next_whirlpool_transaction = self.transaction_iter.next();
        }
    }

}

fn has_reached_until_condition(cond: &ReplayUntil, slot: Slot) -> bool {
    match cond {
        ReplayUntil::End => false,
        ReplayUntil::Slot(until_slot) => slot.slot > *until_slot,
        ReplayUntil::BlockHeight(until_block_height) => slot.block_height > *until_block_height,
        ReplayUntil::BlockTime(until_block_time) => slot.block_time > *until_block_time,
    }
}
use replay_engine::decoded_instructions;
use replay_engine::decoded_instructions::DecodedWhirlpoolInstruction;
use replay_engine::replay_engine::ReplayEngine;
use replay_engine::replay_instruction::ReplayInstructionResult;
use replay_engine::types::{AccountMap, Slot};

pub mod file_io;
use file_io::{WhirlpoolTransaction, Transaction};
mod util;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ReplayUntil {
    End,
    Slot(u64),
    BlockHeight(u64),
    BlockTime(i64),
}

pub type SlotCallback = fn(&Slot);

pub type InstructionCallback = fn(&Slot, &Transaction, &String, &DecodedWhirlpoolInstruction, &AccountMap, &ReplayInstructionResult);

pub struct WhirlpoolReplayer {
    replay_engine: ReplayEngine,
    transaction_iter: Box<dyn Iterator<Item = WhirlpoolTransaction>>,
}

impl WhirlpoolReplayer {
    // build_with_local_file_storage
    // build_with_remote_file_storage
    // TODO: build_with_mysql_database

    pub fn build_with_local_file_storage(
        base_path: &String,
        yyyymmdd: &String,
    ) -> WhirlpoolReplayer {
        let yyyymmdd_date = chrono::NaiveDate::parse_from_str(yyyymmdd, "%Y%m%d").unwrap();
        let previous_yyyymmdd_date = yyyymmdd_date.pred_opt().unwrap();

        // snapshot of the previous day
        let state_file_path = format!(
            "{}/{}/{}/whirlpool-state-{}.json.gz",
            base_path,
            previous_yyyymmdd_date.format("%Y"),
            previous_yyyymmdd_date.format("%m%d"),
            previous_yyyymmdd_date.format("%Y%m%d"),
        );

        // transactions of the day
        let transaction_file_path = format!(
            "{}/{}/{}/whirlpool-transaction-{}.jsonl.gz",
            base_path,
            yyyymmdd_date.format("%Y"),
            yyyymmdd_date.format("%m%d"),
            yyyymmdd_date.format("%Y%m%d"),
        );

        let state = file_io::load_from_whirlpool_state_file(&state_file_path);
        let transaction_iter =
            file_io::load_from_whirlpool_transaction_file(&transaction_file_path);

        let replay_engine = ReplayEngine::new(
            state.slot,
            state.block_height,
            state.block_time,
            state.program_data,
            file_io::convert_accounts_to_account_map(&state.accounts),
        );

        return WhirlpoolReplayer {
            replay_engine,
            transaction_iter: Box::new(transaction_iter),
        };
    }

    pub fn get_slot(&self) -> Slot {
        return self.replay_engine.get_slot();
    }

    pub fn get_program_data(&self) -> &Vec<u8> {
        return self.replay_engine.get_program_data();
    }

    pub fn get_accounts(&self) -> &AccountMap {
        return self.replay_engine.get_accounts();
    }

    pub fn replay(&mut self, cond: ReplayUntil, slot_callback: Option<SlotCallback>, instruction_callback: Option<InstructionCallback>) {
        let mut next_whirlpool_transaction = self.transaction_iter.next();
        while next_whirlpool_transaction.is_some() {
            let whirlpool_transaction = next_whirlpool_transaction.unwrap();

            let slot = Slot {
                slot: whirlpool_transaction.slot,
                block_height: whirlpool_transaction.block_height,
                block_time: whirlpool_transaction.block_time,
            };

            match cond {
                ReplayUntil::End => {}
                ReplayUntil::Slot(until_slot) => {
                    if slot.slot > until_slot {
                        break;
                    }
                }
                ReplayUntil::BlockHeight(until_block_height) => {
                    if slot.block_height > until_block_height {
                        break;
                    }
                }
                ReplayUntil::BlockTime(until_block_time) => {
                    if slot.block_time > until_block_time {
                        break;
                    }
                }
            }

            self.replay_engine.update_slot(slot.slot, slot.block_height, slot.block_time);

            if let Some(callback) = slot_callback {
                callback(&slot);
            }

            for transaction in whirlpool_transaction.transactions {
                for instruction in transaction.clone().instructions {
                    let name = instruction.name;
                    let payload = instruction.payload.to_string();
                    let decoded = decoded_instructions::from_json(&name, &payload).unwrap();

                    match decoded {
                        decoded_instructions::DecodedInstruction::ProgramDeployInstruction(deploy_instruction) => {
                            self.replay_engine.update_program_data(deploy_instruction.program_data);
                        }
                        decoded_instructions::DecodedInstruction::WhirlpoolInstruction(whirlpool_instruction) => {
                            let result = self.replay_engine.replay_instruction(&whirlpool_instruction).unwrap();

                            if let Some(callback) = instruction_callback {
                                callback(
                                    &slot,
                                    &transaction,
                                    &name,
                                    &whirlpool_instruction,
                                    self.replay_engine.get_accounts(),
                                    &result,
                                );
                            }
                        }
                    }
                }
            }

            next_whirlpool_transaction = self.transaction_iter.next();
        }
    }
}

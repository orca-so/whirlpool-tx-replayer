use crate::decoded_instructions::DecodedWhirlpoolInstruction;
use crate::replay_environment::ReplayEnvironment;
use crate::replay_instruction::{replay_whirlpool_instruction, ReplayInstructionResult};
use crate::types::Slot;
use crate::types::AccountMap;
use crate::programs;
use crate::errors::ErrorCode;
use crate::util;
use crate::pubkeys;

pub struct ReplayEngine {
  slot: Slot,
  program_data: Vec<u8>,
  accounts: AccountMap,
  environment: ReplayEnvironment,
  replay_execution_counter: u64,
}

impl ReplayEngine {
  pub fn new(
    slot: u64,
    block_height: u64,
    block_time: i64,
    program_data: Vec<u8>,
    accounts: AccountMap,
  ) -> ReplayEngine {
    let slot = Slot { slot, block_height, block_time };
    let environment = ReplayEngine::build_environment(block_time, &program_data);
    let replay_execution_counter = 0u64;
    return ReplayEngine {
      slot,
      program_data,
      accounts,
      environment,
      replay_execution_counter,
    };
  }

  fn build_environment(block_time: i64, program_data: &Vec<u8>) -> ReplayEnvironment {
    // The environment should be rebuilt periodically to avoid processing too many transactions in a single environment.
    // Since Solana is capable of handling 50,000 TPS, it should theoretically be able to safely handle 20,000 txs per bank, haha.
    let mut builder = ReplayEnvironment::builder();

    // initial clock state
    builder.set_creation_time(block_time);

    // deploy programs
    builder.add_upgradable_program(pubkeys::SPL_TOKEN_PROGRAM_ID, programs::SPL_TOKEN);
    builder.add_upgradable_program(pubkeys::SPL_ASSOCIATED_TOKEN_ACCOUNT_PROGRAM_ID, programs::SPL_ASSOCIATED_TOKEN_ACCOUNT);
    builder.add_upgradable_program(pubkeys::SPL_MEMO_PROGRAM_ID, programs::SPL_MEMO);

    // DEV_NULL_PROGRAM will do nothing for every instruction.  It will always succeed.
    //
    // The work of Metaplex Token Program is to create Metadata account,
    // and it does NOT affect the state of Whirlpool accounts, so it can be ignored in replay context.
    // If we handle this program, we need to pay attention to swith V2/V3.
    // I think Metaplex removed V2 instructions at slot 196,112,106.
    // https://solscan.io/tx/5hKy1aL5Si4ymFvUGX7DAhAhDCEWBgpRUdQJNXYC5d4qKfD2xEEAnGfBJpQKRQQt9cZeQ4EZpze5PQjxj5SMBeiP
    // https://github.com/metaplex-foundation/mpl-token-metadata/commit/28f8410f67ce364798f5c36c1dcb244a206b4371
    //builder.add_upgradable_program(METAPLEX_METADATA_PROGRAM_ID, programs::METAPLEX_TOKEN_METADATA_20230903_1_13_3);
    builder.add_upgradable_program(pubkeys::METAPLEX_METADATA_PROGRAM_ID, programs::DEV_NULL_PROGRAM);

    // whirlpool program
    builder.add_upgradable_program(pubkeys::ORCA_WHIRLPOOL_PROGRAM_ID, &program_data);

    return builder.build();
  }

  pub fn get_slot(&self) -> Slot {
    return self.slot;
  }

  pub fn get_program_data(&self) -> &Vec<u8> {
    return &self.program_data;
  }

  pub fn get_accounts(&self) -> &AccountMap {
    return &self.accounts;
  }

  pub fn update_slot(&mut self, slot: u64, block_height: u64, block_time: i64) {
    self.slot = Slot { slot, block_height, block_time };
    self.environment.set_sysvar_clock_unix_timestamp(self.slot.block_time);
  }

  pub fn update_program_data(&mut self, program_data: Vec<u8>) {
    self.program_data = program_data;
    self.environment = ReplayEngine::build_environment(self.slot.block_time, &self.program_data);
    self.replay_execution_counter = 0u64;
  }

  pub fn replay_instruction(&mut self, ix: &DecodedWhirlpoolInstruction) -> Result<ReplayInstructionResult, ErrorCode> {
    // rebuild periodically to avoid processing too many transactions in a single environment
    // TODO: threshold tuning if needed
    if self.replay_execution_counter > 20000 {
      self.environment = ReplayEngine::build_environment(self.slot.block_time, &self.program_data);
      self.replay_execution_counter = 0u64;
    }

    self.replay_execution_counter += 1;
    let result = replay_whirlpool_instruction(
      &mut self.environment,
      &ix,
      &self.accounts,
    );

    match result {
      Ok(result) => {
        // unwrap is safe because it is TransactionWithStatusMeta::Complete.
        // https://docs.rs/solana-transaction-status/latest/src/solana_transaction_status/lib.rs.html#812-817
        let meta = result.transaction_status.tx_with_meta.get_status_meta().unwrap();

        if meta.status.is_ok() {
          // write back
          util::update_account_map(
            &mut self.accounts,
            result.snapshot.pre_snapshot.clone(),
            result.snapshot.post_snapshot.clone()
          );
        }

        return Ok(result);
      },
      Err(err) => {
        return Err(err);
      },
    }
  }
}

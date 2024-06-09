use std::{
    collections::{/*HashMap,*/ HashSet},
    //convert::TryInto,
    path::Path,
    sync::{atomic::AtomicBool, Arc},
    //time::{SystemTime, UNIX_EPOCH},
};

//use itertools::izip;
use solana_program::{
    bpf_loader, bpf_loader_upgradeable,
    hash::Hash,
//    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    system_program,
    sysvar,
};
use solana_accounts_db::{
    accounts_db::AccountShrinkThreshold,
    accounts_index::AccountSecondaryIndexes,
    transaction_results::{TransactionExecutionResult, TransactionResults},
};
use solana_program_runtime::timings::ExecuteTimings;
use solana_runtime::{
    bank::{Bank, /*TransactionBalancesSet*/},
    genesis_utils,
    runtime_config::RuntimeConfig,
};
use solana_sdk::{
    account::{Account, AccountSharedData},
    feature_set,
    genesis_config::GenesisConfig,
    //packet,
    clock::UnixTimestamp,
    signature::Keypair,
    signature::Signer,
    transaction::VersionedTransaction,
};
/* 
use solana_transaction_status::{
    ConfirmedTransactionWithStatusMeta,
    InnerInstructions, TransactionStatusMeta, TransactionWithStatusMeta,
    VersionedTransactionWithStatusMeta,
    //EncodedConfirmedTransactionWithStatusMeta, UiTransactionEncoding,
};
*/

pub use bincode;
pub use serde;
pub use solana_client;
pub use solana_program;
pub use solana_sdk;
pub use solana_transaction_status;

pub struct ReplayEnvironment {
    bank: Bank,
    faucet: Keypair,
    config: GenesisConfig,
    nonce: u64,
}

impl ReplayEnvironment {
    /// Constructs a builder for a replay environment
    pub fn builder() -> ReplayEnvironmentBuilder {
        ReplayEnvironmentBuilder::new()
    }

    pub fn bank(&mut self) -> &mut Bank {
        &mut self.bank
    }

    pub fn payer(&self) -> Keypair {
        self.faucet.insecure_clone()
    }

    // to prevent generating same transaction signature
    pub fn get_next_nonce(&mut self) -> u64 {
        let nonce = self.nonce;
        self.nonce += 1;
        nonce
    }

    // https://github.com/neodyme-labs/solana-poc-framework/blob/c08d95c209f580b8e828860d73284a22e596277c/src/lib.rs#L443
    // TODO: think removing unnecessary processing
    /* 
    pub fn execute_transaction<T>(&mut self, tx: T) -> ConfirmedTransactionWithStatusMeta
    where
        VersionedTransaction: From<T>,
    {
        let tx = tx.into();
        let len = bincode::serialize(&tx).unwrap().len();
        if len > packet::PACKET_DATA_SIZE {
            panic!(
                "tx {:?} of size {} is {} too large",
                tx,
                len,
                len - packet::PACKET_DATA_SIZE
            )
        }
        let txs = vec![tx];

        let batch = self.bank.prepare_entry_batch(txs.clone()).unwrap();
        let tx_sanitized = batch.sanitized_transactions()[0].clone();

        let mut mint_decimals = HashMap::new();
        let tx_pre_token_balances = solana_ledger::token_balances::collect_token_balances(
            &self.bank,
            &batch,
            &mut mint_decimals,
        );
        let slot = self.bank.slot();
        let mut timings = Default::default();
        let (
            TransactionResults {
                execution_results, ..
            },
            TransactionBalancesSet {
                pre_balances,
                post_balances,
                ..
            },
        ) = self.bank.load_execute_and_commit_transactions(
            &batch,
            usize::MAX,
            true,
            true,
            true,
            true,
            &mut timings,
            None,
        );

        let tx_post_token_balances = solana_ledger::token_balances::collect_token_balances(
            &self.bank,
            &batch,
            &mut mint_decimals,
        );
        let (
          tx,
          execution_result,
          pre_balances,
          post_balances,
          pre_token_balances,
          post_token_balances,
      ) = izip!(
          txs.iter(),
          execution_results.into_iter(),
          pre_balances.into_iter(),
          post_balances.into_iter(),
          tx_pre_token_balances.into_iter(),
          tx_post_token_balances.into_iter(),
      ).next().expect("transaction could not be executed. Enable debug logging to get more information on why");

        let fee = self
            .bank
            .get_fee_for_message(tx_sanitized.message())
            .expect("Fee calculation must succeed");

        let status;
        let inner_instructions;
        let log_messages;
        let return_data;
        let compute_units_consumed;

        match execution_result {
            TransactionExecutionResult::Executed { details, .. } => {
                status = details.status;
                inner_instructions = details.inner_instructions;
                log_messages = details.log_messages;
                return_data = details.return_data;
                compute_units_consumed = Some(details.executed_units);
            }
            TransactionExecutionResult::NotExecuted(err) => {
                status = Err(err);
                inner_instructions = None;
                log_messages = None;
                return_data = None;
                compute_units_consumed = None;
            }
        }

        let inner_instructions = inner_instructions.map(|inner_instructions| {
            inner_instructions
                .into_iter()
                .enumerate()
                .map(|(index, instructions)| {
                    let inner_ixs_mapped = instructions
                        .into_iter()
                        .map(|x| solana_transaction_status::InnerInstruction {
                            instruction: x.instruction,
                            stack_height: Some(x.stack_height as u32),
                        })
                        .collect();
                    InnerInstructions {
                        index: index as u8,
                        instructions: inner_ixs_mapped,
                    }
                })
                .filter(|i| !i.instructions.is_empty())
                .collect()
        });

        let tx_status_meta = TransactionStatusMeta {
            status,
            fee,
            pre_balances,
            post_balances,
            pre_token_balances: Some(pre_token_balances),
            post_token_balances: Some(post_token_balances),
            inner_instructions,
            log_messages,
            rewards: None,
            loaded_addresses: tx_sanitized.get_loaded_addresses(),
            return_data,
            compute_units_consumed,
        };

        // https://docs.rs/solana-transaction-status/latest/solana_transaction_status/
        ConfirmedTransactionWithStatusMeta {
            slot,
            tx_with_meta: TransactionWithStatusMeta::Complete(VersionedTransactionWithStatusMeta {
                transaction: tx.clone(),
                meta: tx_status_meta,
            }),
            block_time: Some(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .try_into()
                    .unwrap(),
            ),
        }
        // Based on profiler analysis, .encode is too slow.
        // So we return ConfirmedTransactionWithStatusMeta instead of EncodedConfirmedTransactionWithStatusMeta.
        // Caller can encode it if needed.
        
        //.encode(UiTransactionEncoding::Binary, Some(0))
        //.expect("Failed to encode transaction")
        
    }
    */

    pub fn execute_transaction<T>(&mut self, tx: T) -> TransactionExecutionResult
    where
        VersionedTransaction: From<T>,
    {
        let txs = vec![tx.into()];
        let batch = self.bank.prepare_entry_batch(txs.clone()).unwrap();
        let (
            TransactionResults {
                mut execution_results,
                ..
            },
            ..
        ) = self.bank.load_execute_and_commit_transactions(
            &batch,
            16usize,
            false, // collect_balances
            false, // enable_cpi_recording
            false, // enable_log_recording
            false, // enable_return_data_recording
            &mut ExecuteTimings::default(),
            None,
        );

        execution_results.remove(0)
    }

    pub fn get_latest_blockhash(&self) -> Hash {
        self.bank.last_blockhash()
    }

    pub fn get_rent_exemption(&self, data: usize) -> u64 {
        self.bank.get_minimum_balance_for_rent_exemption(data)
    }

    pub fn set_sysvar_clock_unix_timestamp(&mut self, unix_timestamp: i64) {
        let clock = self.bank.get_sysvar_cache_for_tests().get_clock().unwrap();
        let new_clock = sysvar::clock::Clock {
            slot: clock.slot,
            epoch_start_timestamp: clock.epoch_start_timestamp,
            epoch: clock.epoch,
            leader_schedule_epoch: clock.leader_schedule_epoch,
            unix_timestamp,
        };
        self.bank.set_sysvar_for_tests(&new_clock);
    }

    pub fn get_account(&self, pubkey: Pubkey) -> Option<Account> {
        self.bank.get_account(&pubkey).map(|acc| acc.into())
    }

    pub fn set_account(&mut self, pubkey: Pubkey, account: &Account) -> &mut Self {
        self.bank.store_account(&pubkey, account);
        self
    }

    pub fn set_account_with_data(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: &[u8],
        executable: bool,
    ) -> &mut Self {
        self.set_account(
            pubkey,
            &Account {
                lamports: self.config.rent.minimum_balance(data.len()),
                data: data.to_vec(),
                executable,
                owner,
                rent_epoch: 0,
            },
        )
    }

    pub fn set_account_with_lamports(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        lamports: u64,
    ) -> &mut Self {
        self.set_account(
            pubkey,
            &Account {
                lamports,
                data: vec![],
                executable: false,
                owner,
                rent_epoch: 0,
            },
        )
    }

    pub fn set_account_with_packable<P: Pack>(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: P,
    ) -> &mut Self {
        let data = {
            let mut buf = vec![0u8; P::LEN];
            data.pack_into_slice(&mut buf[..]);
            buf
        };
        self.set_account_with_data(pubkey, owner, &data, false)
    }

    /// Advance the bank to the next blockhash.
    pub fn advance_blockhash(&self) -> Hash {
        let parent_distance = if self.bank.slot() == 0 {
            1
        } else {
            self.bank.slot() - self.bank.parent_slot()
        };

        for _ in 0..parent_distance {
            let last_blockhash = self.bank.last_blockhash();
            while self.bank.last_blockhash() == last_blockhash {
                self.bank.register_tick(&Hash::new_unique())
            }
        }

        self.get_latest_blockhash()
    }    
}

pub struct ReplayEnvironmentBuilder {
    config: GenesisConfig,
    faucet: Keypair,
}

impl ReplayEnvironmentBuilder {
    fn new() -> Self {
        let faucet = Keypair::new();
        let mut config = GenesisConfig::new(
            &[(
                faucet.pubkey(),
                AccountSharedData::new(1u64 << 48, 0, &system_program::id()),
            )],
            &[],
        );
        genesis_utils::activate_all_features(&mut config);
        // Deactivate fix_recent_blockhashes feature to allow for advancing blockhashes without creating new banks
        config
            .accounts
            .remove(&feature_set::fix_recent_blockhashes::id());

        let mut builder = ReplayEnvironmentBuilder { faucet, config };

/* 
        builder.add_account_with_data(
            spl_associated_token_account::ID,
            bpf_loader::ID,
            programs::SPL_ASSOCIATED_TOKEN,
            true,
        );
        builder.add_account_with_data(
            "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo"
                .parse()
                .unwrap(),
            bpf_loader::ID,
            programs::SPL_MEMO1,
            true,
        );
        builder.add_account_with_data(spl_memo::ID, bpf_loader::ID, programs::SPL_MEMO3, true);
        builder.add_account_with_data(spl_token::ID, bpf_loader::ID, programs::SPL_TOKEN, true);
        */
        builder.add_account_with_lamports(sysvar::rent::ID, sysvar::ID, 1);
        builder
    }

    /// Sets the creation time of the network
    pub fn set_creation_time(&mut self, unix_timestamp: UnixTimestamp) -> &mut Self {
        self.config.creation_time = unix_timestamp as UnixTimestamp;
        self
    }

    /// Adds the account into the environment.
    pub fn add_account(&mut self, pubkey: Pubkey, account: Account) -> &mut Self {
        self.config.add_account(pubkey, account.into());
        self
    }

    /// Reads the program from the path and add it at the address into the environment.
    pub fn add_program<P: AsRef<Path>>(&mut self, pubkey: Pubkey, path: P) -> &mut Self {
        self.add_account_with_data(pubkey, bpf_loader::ID, &std::fs::read(path).unwrap(), true);
        self
    }

    pub fn add_upgradable_program(
        &mut self,
        pubkey: Pubkey,
        data: &[u8],
    ) {
        let program_pubkey = pubkey;
        let programdata_pubkey = Keypair::new().pubkey();
    
        let program_data = bpf_loader_upgradeable::UpgradeableLoaderState::Program {
          programdata_address: programdata_pubkey
        };
    
        let programdata_header = bpf_loader_upgradeable::UpgradeableLoaderState::ProgramData {
          slot: 1, // 0 is not valid
          upgrade_authority_address: Some(Pubkey::default()), // None is not valid
        };
    
        let program_bytes = bincode::serialize(&program_data).unwrap();
        let mut programdata_bytes = bincode::serialize(&programdata_header).unwrap();
        programdata_bytes.extend_from_slice(data);
    
        self.add_account_with_data(program_pubkey, bpf_loader_upgradeable::ID, &program_bytes, true);
        self.add_account_with_data(programdata_pubkey, bpf_loader_upgradeable::ID, &programdata_bytes, false);
    }
    

    // Adds a rent-excempt account into the environment.
    pub fn add_account_with_data(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        data: &[u8],
        executable: bool,
    ) -> &mut Self {
        self.add_account(
            pubkey,
            Account {
                lamports: self.config.rent.minimum_balance(data.len()),
                data: data.to_vec(),
                executable,
                owner,
                rent_epoch: 0,
            },
        )
    }

    // Adds an account with the given balance into the environment.
    pub fn add_account_with_lamports(
        &mut self,
        pubkey: Pubkey,
        owner: Pubkey,
        lamports: u64,
    ) -> &mut Self {
        self.add_account(
            pubkey,
            Account {
                lamports,
                data: vec![],
                executable: false,
                owner,
                rent_epoch: 0,
            },
        )
    }


    /// Finalizes the environment.
    pub fn build(&mut self) -> ReplayEnvironment {
        //let tmpdir = Path::new("/tmp/");
        // /Volumes/RAMDisk
        // let tmpdir = Path::new("/tmp/");
        let exit = Arc::new(AtomicBool::new(false));

        /*

        #[derive(Serialize, Deserialize, Debug, Clone, AbiExample, PartialEq)]
        pub struct GenesisConfig {
            /// when the network (bootstrap validator) was started relative to the UNIX Epoch
            pub creation_time: UnixTimestamp,
            /// initial accounts
            pub accounts: BTreeMap<Pubkey, Account>,
            /// built-in programs
            pub native_instruction_processors: Vec<(String, Pubkey)>,
            /// accounts for network rewards, these do not count towards capitalization
            pub rewards_pools: BTreeMap<Pubkey, Account>,
            pub ticks_per_slot: u64,
            pub unused: u64,
            /// network speed configuration
            pub poh_config: PohConfig,
            /// this field exists only to ensure that the binary layout of GenesisConfig remains compatible
            /// with the Solana v0.23 release line
            pub __backwards_compat_with_v0_23: u64,
            /// transaction fee config
            pub fee_rate_governor: FeeRateGovernor,
            /// rent config
            pub rent: Rent,
            /// inflation config
            pub inflation: Inflation,
            /// how slots map to epochs
            pub epoch_schedule: EpochSchedule,
            /// network runlevel
            pub cluster_type: ClusterType,
        }

        pub struct RuntimeConfig {
            pub compute_budget: Option<ComputeBudget>,
            pub log_messages_bytes_limit: Option<usize>,
            pub transaction_account_lock_limit: Option<usize>,
        }

        pub const ITER_BATCH_SIZE: usize = 1000;
        pub const BINS_DEFAULT: usize = 8192;
        pub const BINS_FOR_TESTING: usize = 2; // we want > 1, but each bin is a few disk files with a disk based index, so fewer is better
        pub const BINS_FOR_BENCHMARKS: usize = 8192;
        pub const FLUSH_THREADS_TESTING: usize = 1;

        // TESTING /////////////////////////////////////////////////////////////////////////////////////////////////////////
        pub const ACCOUNTS_INDEX_CONFIG_FOR_TESTING: AccountsIndexConfig = AccountsIndexConfig {
            bins: Some(BINS_FOR_TESTING),
            flush_threads: Some(FLUSH_THREADS_TESTING),
            drives: None,
            index_limit_mb: IndexLimitMb::Unspecified,
            ages_to_stay_in_cache: None,
            scan_results_limit_bytes: None,
            started_from_validator: false,
        };

        pub const ACCOUNTS_DB_CONFIG_FOR_TESTING: AccountsDbConfig = AccountsDbConfig {
            index: Some(ACCOUNTS_INDEX_CONFIG_FOR_TESTING),
            accounts_hash_cache_path: None,
            filler_accounts_config: FillerAccountsConfig::const_default(),
            write_cache_limit_bytes: None,
            ancient_append_vec_offset: None,
            skip_initial_hash_calc: false,
            exhaustively_verify_refcounts: false,
            create_ancient_storage: CreateAncientStorage::Pack,
            test_partitioned_epoch_rewards: TestPartitionedEpochRewards::CompareResults,
        };

        // BENCHMARKS /////////////////////////////////////////////////////////////////////////////////////////////////////////
        pub const ACCOUNTS_INDEX_CONFIG_FOR_BENCHMARKS: AccountsIndexConfig = AccountsIndexConfig {
            bins: Some(BINS_FOR_BENCHMARKS),
            flush_threads: Some(FLUSH_THREADS_TESTING),
            drives: None,
            index_limit_mb: IndexLimitMb::Unspecified,
            ages_to_stay_in_cache: None,
            scan_results_limit_bytes: None,
            started_from_validator: false,
        };

        pub const ACCOUNTS_DB_CONFIG_FOR_BENCHMARKS: AccountsDbConfig = AccountsDbConfig {
            index: Some(ACCOUNTS_INDEX_CONFIG_FOR_BENCHMARKS),
            accounts_hash_cache_path: None,
            filler_accounts_config: FillerAccountsConfig::const_default(),
            write_cache_limit_bytes: None,
            ancient_append_vec_offset: None,
            skip_initial_hash_calc: false,
            exhaustively_verify_refcounts: false,
            create_ancient_storage: CreateAncientStorage::Pack,
            test_partitioned_epoch_rewards: TestPartitionedEpochRewards::None,
        };
        */
        let mut accounts_index_config =
            solana_accounts_db::accounts_index::ACCOUNTS_INDEX_CONFIG_FOR_TESTING;
        accounts_index_config.index_limit_mb =
            solana_accounts_db::accounts_index::IndexLimitMb::InMemOnly;
        //accounts_index_config.flush_threads = Some(5);

        let mut accounts_db_config = solana_accounts_db::accounts_db::ACCOUNTS_DB_CONFIG_FOR_TESTING;
        accounts_db_config.index = Some(accounts_index_config);

        // set compute budget to max
        let mut runtime_config = RuntimeConfig::default();
        runtime_config.compute_budget = Some(solana_program_runtime::compute_budget::ComputeBudget::new(1_400_000u64));

        let bank_slot0 = Bank::new_with_paths(
            &self.config,
            Arc::new(runtime_config),
            vec![/*tmpdir.to_path_buf()*/],
            None,
            None,
            AccountSecondaryIndexes {
                keys: None,
                indexes: HashSet::new(),
            },
            AccountShrinkThreshold::default(),
            false,
            Some(accounts_db_config), //None,
            None,
            exit,
        );
        /*
        let bank = Bank::new_with_paths_for_tests(
            &self.config,
            Arc::new(RuntimeConfig::default()),
            vec![tmpdir.to_path_buf()],
            AccountSecondaryIndexes {
                keys: None,
                indexes: HashSet::new(),
            },
            AccountShrinkThreshold::default(),
        );*/

        // advance to slot2
        // to avoid loading program every time, slot using for transaction processing > slot using for program deployment
        // - slot0: genesis (always slot 0)
        // - slot1: slot for program deployment
        // - slot2: slot for transaction processing
        let bank_slot1 = Bank::new_from_parent(Arc::new(bank_slot0), &Pubkey::default(), 1u64);
        let bank_slot2: Bank = Bank::new_from_parent(Arc::new(bank_slot1), &Pubkey::default(), 2u64);

        let env = ReplayEnvironment {
            bank: bank_slot2,
            faucet: self.faucet.insecure_clone(),
            config: self.config.clone(),
            nonce: 0,
        };
        env.advance_blockhash();

        env
    }
}

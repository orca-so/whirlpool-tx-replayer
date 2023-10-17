use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use csv;
use serde_derive::{Deserialize, Serialize};
use base64::{Engine, prelude::BASE64_STANDARD};

use anchor_lang::prelude::*;
use whirlpool_base::{
    self,
    state::Whirlpool,
    instructions,
//    ID as ORCA_WHIRLPOOL_PROGRAM_ID,
};

// use solana_program_test;

use std::cell::RefCell;
use std::rc::Rc;


// use solana_program::program::invoke_signed;
// use spl_token::instruction::{burn_checked, close_account, mint_to, set_authority, AuthorityType};

use solana_program;

#[derive(Debug, Deserialize, Serialize)]
struct AccountString {
    pubkey: String,
    data_base64: String,
}

/*
option
- mariadb-database
- mariadb-host
- mariadb-password
- mariadb-port
- mariadb-user

- snapshotdir
- start slot <指定しなければ snapshot dir から最も大きい slot を探す>
*/



fn create_account_info<'info>(data: &'info mut [u8], lamports: &'info mut u64) -> solana_program::account_info::AccountInfo<'info> {
    let account_info = solana_program::account_info::AccountInfo {
        executable: true, // always executable (^^;
        is_signer: true, // always siner (^^;
        is_writable: true, // always writable (^^;
        owner: &ORCA_WHIRLPOOL_PROGRAM_ID,
        rent_epoch: 0,
        key: &ORCA_WHIRLPOOL_PROGRAM_ID, // dummy
        //data: Rc::new(RefCell::new(&mut [0u8; 1000])),
        data: Rc::new(RefCell::new(&mut data[..])),
        lamports: Rc::new(RefCell::new(lamports)),
    };
    return account_info;
}

fn create_account_info_with_pubkey<'info>(data: &'info mut [u8], lamports: &'info mut u64, pubkey: &'info Pubkey) -> solana_program::account_info::AccountInfo<'info> {
    let account_info = solana_program::account_info::AccountInfo {
        executable: true, // always executable (^^;
        is_signer: true, // always siner (^^;
        is_writable: true, // always writable (^^;
        owner: &ORCA_WHIRLPOOL_PROGRAM_ID,
        rent_epoch: 0,
        key: &pubkey, // dummy
        //data: Rc::new(RefCell::new(&mut [0u8; 1000])),
        data: Rc::new(RefCell::new(&mut data[..])),
        lamports: Rc::new(RefCell::new(lamports)),
    };
    return account_info;
}

// BPFLoader2111111111111111111111111111111111
// BPFLoaderUpgradeab1e11111111111111111111111

const BPF_LOADER_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("BPFLoader2111111111111111111111111111111111");
const BPF_LOADER_UPGRADABLE_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");
const SYSTEM_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("11111111111111111111111111111111");
const SPL_TOKEN_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = anchor_lang::solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

fn create_account_info_with_owner_pubkey<'info>(data: &'info mut [u8], lamports: &'info mut u64, owner: &'info Pubkey, pubkey: &'info Pubkey) -> solana_program::account_info::AccountInfo<'info> {
    let account_info = solana_program::account_info::AccountInfo {
        executable: true, // always executable (^^;
        is_signer: true, // always siner (^^;
        is_writable: true, // always writable (^^;
        owner: &owner,
        rent_epoch: 0,
        key: &pubkey, // dummy
        //data: Rc::new(RefCell::new(&mut [0u8; 1000])),
        data: Rc::new(RefCell::new(&mut data[..])),
        lamports: Rc::new(RefCell::new(lamports)),
    };
    return account_info;
}

fn create_account_info_core<'info>(
    data: &'info mut [u8],
    lamports: &'info mut u64,
    owner: &'info Pubkey,
    pubkey: &'info Pubkey,
    executable: bool,
    is_signer: bool,
    is_writable: bool,
) -> solana_program::account_info::AccountInfo<'info> {
    return solana_program::account_info::AccountInfo {
        executable,
        is_signer,
        is_writable,
        owner: &owner,
        rent_epoch: 0,
        key: &pubkey,
        data: Rc::new(RefCell::new(&mut data[..])),
        lamports: Rc::new(RefCell::new(lamports)),
    };
}

fn create_program_account_info<'info>(data: &'info mut [u8], lamports: &'info mut u64, program_id: &'info Pubkey) -> solana_program::account_info::AccountInfo<'info> {
    return create_account_info_core(data, lamports, &BPF_LOADER_PROGRAM_ID, program_id, true, false, false);
}

fn create_whirlpool_account_info<'info>(data: &'info mut [u8], lamports: &'info mut u64, pubkey: &'info Pubkey) -> solana_program::account_info::AccountInfo<'info> {
    return create_account_info_core(data, lamports, &ORCA_WHIRLPOOL_PROGRAM_ID, pubkey, false, true, true);
}

fn create_signer_account_info<'info>(data: &'info mut [u8], lamports: &'info mut u64, pubkey: &'info Pubkey) -> solana_program::account_info::AccountInfo<'info> {
    return create_account_info_core(data, lamports, &SYSTEM_PROGRAM_ID, pubkey, false, true, true);
}

// token_account
// mint_account
// rent sysvar
// clock sysvar

fn main() {
    let gzcsvfile = "data/whirlpool-snapshot-215135999.csv.gz";
    let copiedfile = "data/whirlpool-snapshot-215135999.csv.2.gz";

    ////////////////////////////////////////////////////////////////////////////////
    // LOAD
    ////////////////////////////////////////////////////////////////////////////////
    let mut file = File::open(gzcsvfile).unwrap();
    let decoder = GzDecoder::new(file);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(decoder);

    let mut account_map = std::collections::HashMap::<String, String>::new();

    let mut total_lines = 0;
    reader.deserialize::<AccountString>().for_each(|row| {
        let row = row.unwrap();
        total_lines += 1;
        //println!("{:}", row.pubkey);
        account_map.insert(row.pubkey, row.data_base64);
    });

    println!("account_map.len(): {}", account_map.len());
    println!("SOL/USDC(64): {}", account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap());

    ////////////////////////////////////////////////////////////////////////////////
    // PROCESS
    ////////////////////////////////////////////////////////////////////////////////

    let mut in_memory_account_map = std::collections::HashMap::<String, Vec<u8>>::new();
    for (pubkey, data_base64) in account_map.iter() {
        let data = BASE64_STANDARD.decode(data_base64).unwrap();
        //in_memory_account_map.insert(pubkey.clone(), Rc::new(RefCell::new(data)));
        in_memory_account_map.insert(pubkey.clone(), data);
    }

    // https://github.com/solana-labs/solana-program-library/blob/master/token-swap/program/src/processor.rs#L1364

    struct TestSyscallStubs {
        pub stub_clock_sysvar: i64,
    }
    impl solana_program::program_stubs::SyscallStubs for TestSyscallStubs {
        fn sol_invoke_signed(
            &self,
            instruction: &solana_program::instruction::Instruction,
            account_infos: &[AccountInfo],
            signers_seeds: &[&[&[u8]]],
        ) -> solana_program::entrypoint::ProgramResult {
            msg!("TestSyscallStubs::sol_invoke_signed() silently ignored");
            Ok(())
        }

        fn sol_get_clock_sysvar(&self, var_addr: *mut u8) -> u64 {
            let mut clock = Clock::default();
            clock.unix_timestamp = self.stub_clock_sysvar;// 500i64;
            unsafe {
                *(var_addr as *mut _ as *mut Clock) = clock;
            }
            solana_program::entrypoint::SUCCESS
        }
    }

    solana_program::program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs { stub_clock_sysvar: 1696838023 }));

    /* 
    // https://github.com/solana-labs/example-helloworld/blob/master/src/program-rust/tests/lib.rs

    let mut program_test = solana_program_test::ProgramTest::new(
        "whirlpool",
        ORCA_WHIRLPOOL_PROGRAM_ID,
        solana_program_test::processor!(whirlpool_base::entry),
    );

    program_test.add
*/
    /* 
    let mut whirlpool_lamports = 1_000_000_000u64;
    let mut whirlpool_data = in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone();
    let whirlpool_account_info = create_account_info(&mut whirlpool_data, &mut whirlpool_lamports);
    let whirlpool = Account::try_from(&whirlpool_account_info).unwrap();

    let mut whirlpools_config_lamports = 1_000_000_000u64;
    let mut whirlpools_config_data = in_memory_account_map.get("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ").unwrap().clone();
    let whirlpools_config_account_info = create_account_info(&mut whirlpools_config_data, &mut whirlpools_config_lamports);
    let whirlpools_config = Account::try_from(&whirlpools_config_account_info).unwrap();

    let mut fee_authority_lamports = 1_000_000_000u64;
    let mut fee_authority_data: Vec<u8> = vec![];
    let fee_authority_account_info = create_account_info(&mut fee_authority_data, &mut fee_authority_lamports);
    let fee_authority = Signer::try_from(&fee_authority_account_info).unwrap();

    let mut accounts = instructions::set_fee_rate::SetFeeRate {
        whirlpool,
        whirlpools_config,
        fee_authority,
    };

    let bumps = BTreeMap::new();
    let remaining_accounts = [];
    let ctx = Context::new(
        &ORCA_WHIRLPOOL_PROGRAM_ID,
        &mut accounts,
        &remaining_accounts,
        bumps,
    );

    instructions::set_fee_rate::handler(ctx, 2000).unwrap();

    print!("after feerate {}", accounts.whirlpool.fee_rate);
*/

    test_set_fee_rate(&in_memory_account_map);
    test_set_fee_rate2(&mut in_memory_account_map);

    test_collect_reward(&in_memory_account_map);

    test_set_fee_rate2(&mut in_memory_account_map);

    test_update_fees_and_rewards(&mut in_memory_account_map);

    let clock = Clock::get().unwrap();
    println!("clock: {}", clock.unix_timestamp);

    solana_program::program_stubs::set_syscall_stubs(Box::new(TestSyscallStubs { stub_clock_sysvar: 10000i64 }));

    let clock2 = Clock::get().unwrap();
    println!("clock2: {}", clock2.unix_timestamp);

/* 
   let mut x_lamports = 1_000_000_000u64;
    let mut x_data = in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone();
    let x_account_info = create_account_info(&mut x_data, &mut x_lamports);

    
    invoke_signed(
        &mint_to(
            &spl_token::ID,
            &ORCA_WHIRLPOOL_PROGRAM_ID,
            &ORCA_WHIRLPOOL_PROGRAM_ID,
            &ORCA_WHIRLPOOL_PROGRAM_ID,
            &[&ORCA_WHIRLPOOL_PROGRAM_ID],
            1,
        ).unwrap(),
        &[
            x_account_info.clone().to_account_info(),
            x_account_info.clone().to_account_info(),
            x_account_info.clone().to_account_info(),
            x_account_info.clone().to_account_info(),
        ],
        &[],
    ).unwrap();

    print!("live!");
*/

    //let account_info2 = create_account_info(&mut data[..], &mut lamports);

    //let account_info = create_account_info(&mut in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone()[..], &mut lamports);

    //print!("before: {}", in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap()[45] as i32 + in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap()[46] as i32 * 256i32);

    //print!("before data: {} {}", data[45], data[46]);
//    let mut y = Account::<whirlpool_base::state::Whirlpool>::try_from(&account_info).unwrap();
//    y.update_fee_rate(1000).unwrap();

    //let yy = y.to_account_info();
    //let yy = y.into_inner();

    //print!("fee rate: {}", yy.fee_rate);

    //whirlpool_base::state::Whirlpool::try_serialize(yy, )
//    let mut back_data = in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone();
    //y.try_serialize(&mut &mut back_data[..]).unwrap();

//    let ai : &AccountInfo = y.as_ref();

    //drop(y);

    //print!("after data: {}", data[45] as i32 + data[46] as i32 * 256);

  //  in_memory_account_map.get_mut("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().copy_from_slice(&ai.data.borrow_mut()[..]);

    //in_memory_account_map.get_mut("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().copy_from_slice(&data[..]);


    //print!("AFTER: {}", in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap()[45] as i32 + in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap()[46] as i32 * 256i32);

    //print!("after yy: {}", yy.data.borrow()[45] as i32 + yy.data.borrow()[46] as i32 * 256);
    

    //print!("after: {}", in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap()[0]);

    /* 
    let account_data_base64 = account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap();
    let account_data = BASE64_STANDARD.decode(account_data_base64).unwrap();
    let mut x = &account_data[..];
    let pool_data = whirlpool_base::state::Whirlpool::try_deserialize(&mut x).unwrap();

    let config_data = BASE64_STANDARD.decode(account_map.get("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ").unwrap()).unwrap();
    let config = whirlpool_base::state::WhirlpoolsConfig::try_deserialize(&mut &config_data[..]).unwrap();

    print!("pool_data: tick_spacing = {:?}", pool_data.tick_spacing);

    let account_info = solana_program::account_info::AccountInfo {
        executable: false,
        is_signer: false,
        is_writable: true,
        owner: &ORCA_WHIRLPOOL_PROGRAM_ID,
        rent_epoch: 0,
        key: &ORCA_WHIRLPOOL_PROGRAM_ID, // dummy
        data: Rc::new(RefCell::new(&mut [0u8; 1000])),
        lamports: Rc::new(RefCell::new(&mut 0)),
    };

    let y = Account::<whirlpool_base::state::Whirlpool>::try_from(&account_info.clone()).unwrap();

    let accounts = instructions::set_fee_rate::SetFeeRate {
        whirlpool: y,
        whirlpools_config: config,
        fee_authority: Pubkey::new_from_array([0u8; 32]),
    };

    let ctx = anchor_lang::context::Context {
        accounts: accounts,
        bumps: BTreeMap::new(),
        remaining_accounts: &[],
        program_id: &ORCA_WHIRLPOOL_PROGRAM_ID,
    }

    //instructions::set_fee_rate::handler();
*/
    ////////////////////////////////////////////////////////////////////////////////
    // SAVE
    ////////////////////////////////////////////////////////////////////////////////
    let outfile = File::create(copiedfile).unwrap();
    let encoder = GzEncoder::new(outfile, flate2::Compression::default());
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(encoder);
        
    writer.serialize(AccountString {
        pubkey: "pubkey1".to_string(),
        data_base64: "data_base64".to_string(),
    }).unwrap();
    writer.serialize(AccountString {
        pubkey: "pubkey2".to_string(),
        data_base64: "data_base64".to_string(),
    }).unwrap();
    writer.serialize(AccountString {
        pubkey: "pubkey3".to_string(),
        data_base64: "data_base64".to_string(),
    }).unwrap();

    writer.flush().unwrap();
}

fn test_set_fee_rate(in_memory_account_map: &std::collections::HashMap::<String, Vec<u8>>) {
    let mut whirlpool_lamports = 1_000_000_000u64;
    let mut whirlpool_data = in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone();
    let whirlpool_account_info = create_account_info(&mut whirlpool_data, &mut whirlpool_lamports);
    let whirlpool = Account::try_from(&whirlpool_account_info).unwrap();

    let mut whirlpools_config_lamports = 1_000_000_000u64;
    let mut whirlpools_config_data = in_memory_account_map.get("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ").unwrap().clone();
    let whirlpools_config_account_info = create_account_info(&mut whirlpools_config_data, &mut whirlpools_config_lamports);
    let whirlpools_config = Account::try_from(&whirlpools_config_account_info).unwrap();

    let mut fee_authority_lamports = 1_000_000_000u64;
    let mut fee_authority_data: Vec<u8> = vec![];
    let fee_authority_account_info = create_account_info(&mut fee_authority_data, &mut fee_authority_lamports);
    let fee_authority = Signer::try_from(&fee_authority_account_info).unwrap();

    let mut accounts = instructions::set_fee_rate::SetFeeRate {
        whirlpool,
        whirlpools_config,
        fee_authority,
    };

    let bumps = BTreeMap::new();
    let remaining_accounts = [];
    let ctx = Context::new(
        &ORCA_WHIRLPOOL_PROGRAM_ID,
        &mut accounts,
        &remaining_accounts,
        bumps,
    );

    instructions::set_fee_rate::handler(ctx, 2000).unwrap();

    println!("after feerate {}", accounts.whirlpool.fee_rate);
}

fn test_collect_reward(in_memory_account_map: &std::collections::HashMap<String, Vec<u8>>) {
    let mut whirlpool_lamports = 1_000_000_000u64;
    let mut whirlpool_data = in_memory_account_map.get("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ").unwrap().clone();
    let whirlpool_account_info = create_account_info(&mut whirlpool_data, &mut whirlpool_lamports);
    let whirlpool = Account::try_from(&whirlpool_account_info).unwrap();

    // authority: 8KLXsmgjPY1xkGU9tfz5YeP26hxemWTwiN9WhkSrufFZ
    let mut position_authority_pubkey = anchor_lang::solana_program::pubkey!("8KLXsmgjPY1xkGU9tfz5YeP26hxemWTwiN9WhkSrufFZ");
    let mut position_authority_lamports = 1_000_000_000u64;
    let mut position_authority_data: Vec<u8> = vec![];
    let position_authority_account_info = create_account_info_with_pubkey(&mut position_authority_data, &mut position_authority_lamports, &position_authority_pubkey);
    let position_authority = Signer::try_from(&position_authority_account_info).unwrap();

    // position: 7MXHNKLmetpi1qh69bcda5aRDXysKWi7yvywfih2XCw3
    let mut position_lamports = 1_000_000_000u64;
    let mut position_data = in_memory_account_map.get("7MXHNKLmetpi1qh69bcda5aRDXysKWi7yvywfih2XCw3").unwrap().clone();
    let position_account_info = create_account_info(&mut position_data, &mut position_lamports);
    let position = Account::try_from(&position_account_info).unwrap();

    // ATA: H8omqqRQVePUxTdq1L9MetJcpWckuCd2UE7UaHtjvhBn
    let mut position_token_account_pubkey = anchor_lang::solana_program::pubkey!("H8omqqRQVePUxTdq1L9MetJcpWckuCd2UE7UaHtjvhBn");
    let mut position_token_account_lamports = 1_000_000_000u64;
    let mut position_token_account_data = BASE64_STANDARD.decode("SYksBBmzFFTG48kMplfHqJAtk/eyGKcwxyMiCV7PiY9stFksBFcs+AxnuABuYLnvRqpnqwvuDmSIGHzVo7T9NAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let position_token_account_account_info = create_account_info_with_owner_pubkey(&mut position_token_account_data, &mut position_token_account_lamports, &spl_token::ID, &position_token_account_pubkey);
    let position_token_account = Account::try_from(&position_token_account_account_info).unwrap();

    // 2tU3tKvj7RBxEatryyMYTUxBoLSSWCQXsdv1X6yce4T2
    let mut reward_owner_account_pubkey = anchor_lang::solana_program::pubkey!("2tU3tKvj7RBxEatryyMYTUxBoLSSWCQXsdv1X6yce4T2"); // dummy vault
    let mut reward_owner_account_lamports = 1_000_000_000u64;
    let mut reward_owner_account_data: Vec<u8> = BASE64_STANDARD.decode("DADQr+uGFNp/GaugLUDxjGklhfZQIN/O09Xl+anAxOHyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNd6HixEFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let reward_owner_account_account_info = create_account_info_with_owner_pubkey(&mut reward_owner_account_data, &mut reward_owner_account_lamports, &spl_token::ID, &reward_owner_account_pubkey);
    let reward_owner_account = Account::try_from(&reward_owner_account_account_info).unwrap();

    let mut reward_vault_pubkey = anchor_lang::solana_program::pubkey!("2tU3tKvj7RBxEatryyMYTUxBoLSSWCQXsdv1X6yce4T2"); // dummy vault
    let mut reward_vault_lamports = 1_000_000_000u64;
    let mut reward_vault_data: Vec<u8> = BASE64_STANDARD.decode("DADQr+uGFNp/GaugLUDxjGklhfZQIN/O09Xl+anAxOHyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNd6HixEFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA").unwrap();
    let reward_vault_account_info = create_account_info_with_owner_pubkey(&mut reward_vault_data, &mut reward_vault_lamports, &spl_token::ID, &reward_vault_pubkey);
    let reward_vault = Account::try_from(&reward_vault_account_info).unwrap();

    let mut token_program_pubkey = spl_token::ID;
    let mut token_program_lamports = 1_000_000_000u64;
    let mut token_program_data: Vec<u8> = vec![];
    let token_program_account_info = create_account_info_with_pubkey(&mut token_program_data, &mut token_program_lamports, &token_program_pubkey);
    let token_program = Program::try_from(&token_program_account_info).unwrap();
/*
    let mut d = [0u8; 1000];
    let mut t = Rc::new(RefCell::new(&mut d));
    let mut t2 = Rc::new(RefCell::new(&mut 1_000_000_000u64));

    let mut tcopy = t.clone();
    t.borrow_mut()[0] = 1;
    println!("t: {}", tcopy.borrow()[0]);
    tcopy.borrow_mut()[0] = 2;
    println!("t: {}", t.borrow()[0]);

    let mut x = t.borrow_mut();
    x[5] = 2;
    drop(x);
    let mut y = tcopy.borrow_mut();
    y[5] = 3;
 */
    let mut accounts = instructions::collect_reward::CollectReward {
        whirlpool: Box::new(whirlpool),
        position_authority,
        position: Box::new(position),
        position_token_account: Box::new(position_token_account),
        reward_owner_account: Box::new(reward_owner_account),
        reward_vault: Box::new(reward_vault),
        token_program,
    };

    let bumps = BTreeMap::new();
    let remaining_accounts = [];
    let ctx = Context::new(
        &ORCA_WHIRLPOOL_PROGRAM_ID,
        &mut accounts,
        &remaining_accounts,
        bumps,
    );

    instructions::collect_reward::handler(ctx, 0).unwrap();
}


fn test_set_fee_rate2(in_memory_account_map: &mut std::collections::HashMap::<String, Vec<u8>>) {
/* 
    let mut account_info_map = std::collections::HashMap::<Pubkey, AccountInfo>::new();
    let keys = ["HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ", "2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ"];

    let mut lamports = 1_000_000_000u64;
    let mut data_map = std::collections::HashMap::<String, Vec<u8>>::new();
    for key in keys.iter() {
        let mut data = in_memory_account_map.get(*key).unwrap().clone();
        let account_info = create_account_info(&mut data, &mut lamports);
        account_info_map.insert(account_info.key.clone(), account_info);
    }

*/
let LAMPORTS = 1_000_000_000u64;

    let key_whirlpool = "HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ";
    let key_whirlpools_config = "2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ";

// to use same account info for same address (especially for TickArray)
    let mut updating_account_info = std::collections::HashMap::<String, AccountInfo>::new();

    let mut whirlpool_lamports = LAMPORTS;
    let mut whirlpool_data = in_memory_account_map.get(key_whirlpool).unwrap().clone();
    updating_account_info.insert(key_whirlpool.to_string(), create_account_info(&mut whirlpool_data, &mut whirlpool_lamports));

    let mut whirlpools_config_lamports = LAMPORTS;
    let mut whirlpools_config_data = in_memory_account_map.get(key_whirlpools_config).unwrap().clone();
    updating_account_info.insert(key_whirlpools_config.to_string(), create_account_info(&mut whirlpools_config_data, &mut whirlpools_config_lamports));

    let mut fee_authority_lamports = LAMPORTS;
    let mut fee_authority_data: Vec<u8> = vec![];
    let fee_authority_account_info = create_account_info(&mut fee_authority_data, &mut fee_authority_lamports);

    //    let whirlpools_config = Account::try_from(&whirlpools_config_account_info).unwrap();

//    let mut fee_authority_lamports = 1_000_000_000u64;
    //let fee_authority = Signer::try_from(&fee_authority_account_info).unwrap();

    let mut accounts = instructions::set_fee_rate::SetFeeRate {
        whirlpool: Account::try_from(&updating_account_info.get(key_whirlpool).unwrap()).unwrap(),
        whirlpools_config: Account::try_from(&updating_account_info.get(key_whirlpools_config).unwrap()).unwrap(),
        fee_authority: Signer::try_from(&fee_authority_account_info).unwrap(),
    };

    let next_fee_rate = accounts.whirlpool.fee_rate + 1000;

    //let bumps = BTreeMap::new();
    //let remaining_accounts = [];
    let ctx = Context::new(
        &ORCA_WHIRLPOOL_PROGRAM_ID,
        &mut accounts,
        &[],//remaining_accounts,
        BTreeMap::new(),//bumps,
    );

    for (key, account_info) in updating_account_info.iter() {
        println!("updating {}: {}", key, account_info.data.borrow()[45]as i32 + account_info.data.borrow()[46] as i32 *256);
    }

    instructions::set_fee_rate::handler(ctx, next_fee_rate).unwrap();

    println!("after feerate {}", accounts.whirlpool.fee_rate);

    // persistence Account (AccountLoader はダイレクト書き込み)
    accounts.whirlpool.exit(&ORCA_WHIRLPOOL_PROGRAM_ID).unwrap();
    accounts.whirlpools_config.exit(&ORCA_WHIRLPOOL_PROGRAM_ID).unwrap();

    for (key, account_info) in updating_account_info.iter() {
        println!("updated {}: {}", key, account_info.data.borrow()[45]as i32 + account_info.data.borrow()[46] as i32 *256);
    }

    for (key, account_info) in updating_account_info.iter() {
        in_memory_account_map.insert(key.to_string(), account_info.data.borrow().to_vec());
    }

}




fn test_update_fees_and_rewards(in_memory_account_map: &mut std::collections::HashMap::<String, Vec<u8>>) {
    let LAMPORTS = 1_000_000_000u64;
    
    let key_whirlpool = "9vqYJjDUFecLL2xPUC4Rc7hyCtZ6iJ4mDiVZX7aFXoAe";
    let key_position = "ELVPibaoLYDyzSXQCiELdLgYTrB4zVr2RzuBtJGjuhJC";
    let key_lower_tick_array = "7SAU5FgSFsDV2fBVNfzAtSP7DcXwH174jjxzePQm11WD";
    let key_upper_tick_array = "8C7RSksyUbS3SmCUFpuKtY413Yswqh5HNmzHQ7c5TCNK";
    
    let mut updating_account_info = std::collections::HashMap::<String, AccountInfo>::new();

    let mut whirlpool_lamports = LAMPORTS;
    let mut whirlpool_data = in_memory_account_map.get(key_whirlpool).unwrap().clone();
    updating_account_info.insert(key_whirlpool.to_string(), create_account_info(&mut whirlpool_data, &mut whirlpool_lamports));

    let mut position_lamports = LAMPORTS;
    let mut position_data = in_memory_account_map.get(key_position).unwrap().clone();
    updating_account_info.insert(key_position.to_string(), create_account_info(&mut position_data, &mut position_lamports));

    let mut lower_tick_array_lamports = LAMPORTS;
    let mut lower_tick_array_data: Vec<u8> = in_memory_account_map.get(key_lower_tick_array).unwrap().clone();
    updating_account_info.insert(key_lower_tick_array.to_string(), create_account_info(&mut lower_tick_array_data, &mut lower_tick_array_lamports));

    let mut upper_tick_array_lamports = LAMPORTS;
    let mut upper_tick_array_data: Vec<u8> = in_memory_account_map.get(key_upper_tick_array).unwrap().clone();
    updating_account_info.insert(key_upper_tick_array.to_string(), create_account_info(&mut upper_tick_array_data, &mut upper_tick_array_lamports));

    
    let tick_array_lower = AccountLoader::try_from(&updating_account_info.get(key_lower_tick_array).unwrap()).unwrap();
    let tick_array_upper = AccountLoader::try_from(&updating_account_info.get(key_upper_tick_array).unwrap()).unwrap();
    tick_array_lower.load().unwrap();
    tick_array_upper.load().unwrap();

    let mut accounts = instructions::update_fees_and_rewards::UpdateFeesAndRewards {
        whirlpool: Account::try_from(&updating_account_info.get(key_whirlpool).unwrap()).unwrap(),
        position: Account::try_from(&updating_account_info.get(key_position).unwrap()).unwrap(),
        tick_array_lower,
        tick_array_upper,
    };

    let ctx = Context::new(
        &ORCA_WHIRLPOOL_PROGRAM_ID,
        &mut accounts,
        &[],
        BTreeMap::new(),
    );
    
    instructions::update_fees_and_rewards::handler(ctx).unwrap();

    println!("after feerate {}", accounts.whirlpool.fee_rate);

    // persistence Account (AccountLoader はダイレクト書き込み)
    accounts.whirlpool.exit(&ORCA_WHIRLPOOL_PROGRAM_ID).unwrap();
    accounts.position.exit(&ORCA_WHIRLPOOL_PROGRAM_ID).unwrap();


    for (key, account_info) in updating_account_info.iter() {
        in_memory_account_map.insert(key.to_string(), account_info.data.borrow().to_vec());
    }
    
}
    
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
    ID as ORCA_WHIRLPOOL_PROGRAM_ID,
};

use std::cell::RefCell;
use std::rc::Rc;

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

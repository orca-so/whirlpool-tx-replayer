use solana_program_test::*;
use solana_sdk::signer::Signer;

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use flate2::read::GzDecoder;

#[derive(Debug, Deserialize, Serialize)]
struct AccountString {
    pubkey: String,
    data_base64: String,
}

use anchor_client;


#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let gzcsvfile = "data/whirlpool-snapshot-215135999.csv.gz";

    ////////////////////////////////////////////////////////////////////////////////
    // LOAD
    ////////////////////////////////////////////////////////////////////////////////
    let file = File::open(gzcsvfile).unwrap();
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
    // REPLAY
    ////////////////////////////////////////////////////////////////////////////////
    let mut test = ProgramTest::new("program", whirlpool_base::ID, processor!(whirlpool_base::entry));

    let ORCA_WHIRLPOOL_PROGRAM_ID = whirlpool_base::ID;

    let whirlpool = solana_program::pubkey!("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ");
    test.add_account_with_base64_data(
        whirlpool,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&whirlpool.to_string()).unwrap(),
    );

    let whirlpools_config = solana_program::pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ");
    test.add_account_with_base64_data(
        whirlpools_config,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&whirlpools_config.to_string()).unwrap(),
    );

    let fee_authority = solana_program::pubkey!("3Pi4tc4SxZyKZivKxWnYfGNxeqFJJxPc8xRw1VnvXpbb");

    let mut context = test.start_with_context().await;

    let payer = std::rc::Rc::new(context.payer.insecure_clone());
    let anchor = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Localnet,
        payer.clone(),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );
    let program = anchor.program(ORCA_WHIRLPOOL_PROGRAM_ID);

    let ixs = program
        .request()
        .accounts(whirlpool_base::accounts::SetFeeRate {
            whirlpool,
            whirlpools_config,
            fee_authority,
        })
        .args(whirlpool_base::instruction::SetFeeRate {
            fee_rate: 10000,
        })
        .instructions().unwrap();

    let message = solana_sdk::message::Message::new(&ixs, Some(&payer.pubkey()));
    let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);

    tx.partial_sign(&[&context.payer], context.last_blockhash);
    context.banks_client.process_transaction(tx).await.unwrap();


    

    /*
    let tx = solana_sdk::transaction::Transaction::new_with_payer(
        &ixs,
        Some(&payer.pubkey()),
    );
     */

    //println!("tx: {:?}", tx);

    /* 
    println!("payer {}", context.payer.pubkey());
    let payer_account = context.banks_client.get_account(context.payer.pubkey()).await.unwrap();

    match payer_account {
        Some(v) => {
            println!("payer: {}", v.lamports);
            println!("payer: {}", v.owner);
            println!("payer: {}", v.executable);
            println!("payer: {}", v.rent_epoch);
            println!("payer: {}", v.data.len());
        },
        None => {
            println!("payer: no account found");
        }
    }

*/
/* 
    let v = context.banks_client.get_account(whirlpool).await.unwrap();

    match v {
        Some(v) => {
            println!("SOL/USDC(64): {}", v.lamports);
            println!("SOL/USDC(64): {}", v.owner);
            println!("SOL/USDC(64): {}", v.executable);
            println!("SOL/USDC(64): {}", v.rent_epoch);
            println!("SOL/USDC(64): {}", v.data.len());
        },
        None => {
            println!("SOL/USDC(64): no account found");
        }
    }
   */ 
}

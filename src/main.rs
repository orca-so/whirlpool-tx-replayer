use anchor_lang::AccountDeserialize;
use base64::prelude::{Engine as _, BASE64_STANDARD};
//use solana_program_test::*;
use solana_sdk::{signer::Signer, signature::Keypair, transaction::{Transaction, VersionedTransaction}};
use solana_sdk::pubkey::Pubkey;
use solana_program::{bpf_loader, bpf_loader_upgradeable};

use serde_derive::{Deserialize, Serialize};
use std::fs::File;
use flate2::read::GzDecoder;

#[derive(Debug, Deserialize, Serialize)]
struct AccountString {
    pubkey: String,
    data_base64: String,
}

use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};

use poc_framework::{Environment, LocalEnvironment, PrintableTransaction, setup_logging, LogLevel};

mod programs;
mod util;

use anchor_client;

const BPF_LOADER_PROGRAM_ID: Pubkey = solana_program::pubkey!("BPFLoader2111111111111111111111111111111111");
const BPF_LOADER_UPGRADABLE_PROGRAM_ID: Pubkey = solana_program::pubkey!("BPFLoaderUpgradeab1e11111111111111111111111");
const SYSTEM_PROGRAM_ID: Pubkey = solana_program::pubkey!("11111111111111111111111111111111");
const SPL_TOKEN_PROGRAM_ID: Pubkey = solana_program::pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");
const METAPLEX_METADATA_PROGRAM_ID: Pubkey = solana_program::pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");







#[tokio::main]
async fn main() {
    println!("Hello, world!");

    let gzcsvfile = "data/whirlpool-snapshot-215135999.csv.gz";
/* 
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

    setup_logging(LogLevel::DEBUG);

    println!("replay: set_fee_rate");
    set_fee_rate(&account_map).await;
*/








}

async fn set_fee_rate(account_map: &std::collections::HashMap::<String, String>) {
    //let mut replayer = ProgramTest::new("program", whirlpool_base::ID, processor!(whirlpool_base::entry));

    //let client = RpcClient::new("<RPC>");


    let mut builder = LocalEnvironment::builder();

    /* 
    // deploy whirlpool
    builder.add_account_with_data(
        ORCA_WHIRLPOOL_PROGRAM_ID,
        bpf_loader_upgradeable::ID,
        &BASE64_STANDARD.decode("AgAAALCj2NteOsPf+tyRpPc+hRFSbDKg2sRwo9znup2STWeS").unwrap(),
        true);
    builder.add_account_with_data(
        solana_program::pubkey!("CtXfPzz36dH5Ws4UYKZvrQ1Xqzn42ecDW6y8NKuiN8nD"),
        bpf_loader_upgradeable::ID,
        programs::ORCA_WHIRLPOOL_WITH_ANCHOR_DEBUG,
        false);
*/

    //builder.clone_upgradable_program_from_cluster(&client, ORCA_WHIRLPOOL_PROGRAM_ID);

    util::add_upgradable_program(&mut builder, ORCA_WHIRLPOOL_PROGRAM_ID, programs::ORCA_WHIRLPOOL_20230901_A574AE5);

    //println!("deployed whirlpool len: {}", programs::ORCA_WHIRLPOOL_20230823_a574ae5.len());

    // replayer.set_creation_time(unix_timestamp);

    let whirlpool = solana_program::pubkey!("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ");
    builder.add_account_with_data(
        whirlpool,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        &BASE64_STANDARD.decode(&account_map.get(&whirlpool.to_string()).unwrap()).unwrap(),
        false);

    let whirlpools_config = solana_program::pubkey!("2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ");
    builder.add_account_with_data(
        whirlpools_config,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        &BASE64_STANDARD.decode(&account_map.get(&whirlpools_config.to_string()).unwrap()).unwrap(),
        false);

    let fee_authority = solana_program::pubkey!("3Pi4tc4SxZyKZivKxWnYfGNxeqFJJxPc8xRw1VnvXpbb");

    let mut replayer = builder.build();

    //let dummy = Keypair::new();
    //let payer = std::rc::Rc::new(dummy);
    let anchor = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Localnet,
        std::rc::Rc::new(Keypair::new()),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );
    let program = anchor.program(ORCA_WHIRLPOOL_PROGRAM_ID);

    let ixs = program
        .request()
        .accounts(whirlpool_base::accounts::SetFeeRate {
            whirlpools_config,
            whirlpool,
            fee_authority,
        })
        .args(whirlpool_base::instruction::SetFeeRate {
            fee_rate: 10000,
        })
        .instructions().unwrap();

    let payer = replayer.payer();

    // create transaction with only sign of payer
    let message = solana_sdk::message::Message::new(&ixs, Some(&payer.pubkey()));
    let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);
    tx.partial_sign(&[&payer], replayer.get_latest_blockhash());

    let pre_account = replayer.get_account(whirlpool).unwrap();
    let pre_data = whirlpool_base::state::Whirlpool::try_deserialize(&mut pre_account.data.as_slice()).unwrap();
    println!("pre fee rate = {}", pre_data.fee_rate);

    // no signature verification
    let result = replayer.execute_transaction(tx);

    result.print_named("set_fee_rate");

    let post_account = replayer.get_account(whirlpool).unwrap();
    let post_data = whirlpool_base::state::Whirlpool::try_deserialize(&mut post_account.data.as_slice()).unwrap();
    println!("post fee rate = {}", post_data.fee_rate);

    
}
/* 
async fn collect_reward(account_map: &std::collections::HashMap::<String, String>) {
    let mut replayer = ProgramTest::new("program", whirlpool_base::ID, processor!(whirlpool_base::entry));

    let whirlpool = solana_program::pubkey!("HJPjoWUrhoZzkNfRpHuieeFk9WcZWjwy6PBjZ81ngndJ");
    replayer.add_account_with_base64_data(
        whirlpool,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&whirlpool.to_string()).unwrap(),
    );

    let position_authority = solana_program::pubkey!("8KLXsmgjPY1xkGU9tfz5YeP26hxemWTwiN9WhkSrufFZ");

    let position = solana_program::pubkey!("7MXHNKLmetpi1qh69bcda5aRDXysKWi7yvywfih2XCw3");
    replayer.add_account_with_base64_data(
        position,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&position.to_string()).unwrap(),
    );

    let position_token_account = solana_program::pubkey!("H8omqqRQVePUxTdq1L9MetJcpWckuCd2UE7UaHtjvhBn");
    replayer.add_account_with_base64_data(
        position_token_account,
        1_000_000_000u64,
        SPL_TOKEN_PROGRAM_ID,
        "SYksBBmzFFTG48kMplfHqJAtk/eyGKcwxyMiCV7PiY9stFksBFcs+AxnuABuYLnvRqpnqwvuDmSIGHzVo7T9NAEAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    );

    let reward_owner_account = solana_program::pubkey!("2tU3tKvj7RBxEatryyMYTUxBoLSSWCQXsdv1X6yce4T2");
    replayer.add_account_with_base64_data(
        reward_owner_account,
        1_000_000_000u64,
        SPL_TOKEN_PROGRAM_ID,
        "DADQr+uGFNp/GaugLUDxjGklhfZQIN/O09Xl+anAxOHyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNd6HixEFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    );

    // TODO: fix vault or owner_account (same)
    let reward_vault = solana_program::pubkey!("2tU3tKvj7RBxEatryyMYTUxBoLSSWCQXsdv1X6yce4T2");
    replayer.add_account_with_base64_data(
        reward_vault,
        1_000_000_000u64,
        SPL_TOKEN_PROGRAM_ID,
        "DADQr+uGFNp/GaugLUDxjGklhfZQIN/O09Xl+anAxOHyL5MQp9RFVK38fpgddAeEPkj5Nf7TAl/qe/yaQ/yJNd6HixEFAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAQAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA",
    );

    let token_program = SPL_TOKEN_PROGRAM_ID;

    let mut context = replayer.start_with_context().await;

    let payer = std::rc::Rc::new(context.payer.insecure_clone());
    let anchor = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Localnet,
        payer.clone(),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );
    let program = anchor.program(ORCA_WHIRLPOOL_PROGRAM_ID);

    let ixs = program
        .request()
        .accounts(whirlpool_base::accounts::CollectReward {
            whirlpool,
            position_authority,
            position,
            position_token_account,
            reward_owner_account,
            reward_vault,
            token_program,
        })
        .args(whirlpool_base::instruction::CollectReward {
            reward_index: 0,
        })
        .instructions().unwrap();

    let message = solana_sdk::message::Message::new(&ixs, Some(&payer.pubkey()));
    let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);

    // sign is not required, just to set the last_blockhash
    tx.partial_sign(&[&context.payer], context.last_blockhash);

    context.banks_client.process_transaction(tx).await.unwrap();   
}

async fn update_fees_and_rewards(account_map: &std::collections::HashMap::<String, String>) {
    let mut replayer = ProgramTest::new("program", whirlpool_base::ID, processor!(whirlpool_base::entry));

    let whirlpool = solana_program::pubkey!("9vqYJjDUFecLL2xPUC4Rc7hyCtZ6iJ4mDiVZX7aFXoAe");
    replayer.add_account_with_base64_data(
        whirlpool,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&whirlpool.to_string()).unwrap(),
    );

    let position = solana_program::pubkey!("ELVPibaoLYDyzSXQCiELdLgYTrB4zVr2RzuBtJGjuhJC");
    replayer.add_account_with_base64_data(
        position,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&position.to_string()).unwrap(),
    );

    let tick_array_lower = solana_program::pubkey!("7SAU5FgSFsDV2fBVNfzAtSP7DcXwH174jjxzePQm11WD");
    replayer.add_account_with_base64_data(
        tick_array_lower,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&tick_array_lower.to_string()).unwrap(),
    );

    let tick_array_upper = solana_program::pubkey!("8C7RSksyUbS3SmCUFpuKtY413Yswqh5HNmzHQ7c5TCNK");
    replayer.add_account_with_base64_data(
        tick_array_upper,
        1_000_000_000u64,
        ORCA_WHIRLPOOL_PROGRAM_ID,
        account_map.get(&tick_array_upper.to_string()).unwrap(),
    );

    let mut context = replayer.start_with_context().await;

    let payer = std::rc::Rc::new(context.payer.insecure_clone());
    let anchor = anchor_client::Client::new_with_options(
        anchor_client::Cluster::Localnet,
        payer.clone(),
        solana_sdk::commitment_config::CommitmentConfig::confirmed(),
    );
    let program = anchor.program(ORCA_WHIRLPOOL_PROGRAM_ID);

    let ixs = program
        .request()
        .accounts(whirlpool_base::accounts::UpdateFeesAndRewards {
            whirlpool,
            position,
            tick_array_lower,
            tick_array_upper,
        })
        .args(whirlpool_base::instruction::UpdateFeesAndRewards {})
        .instructions().unwrap();

    let message = solana_sdk::message::Message::new(&ixs, Some(&payer.pubkey()));
    let mut tx = solana_sdk::transaction::Transaction::new_unsigned(message);

    // sign is not required, just to set the last_blockhash
    tx.partial_sign(&[&context.payer], context.last_blockhash);

    context.banks_client.process_transaction(tx).await.unwrap();  
}

*/
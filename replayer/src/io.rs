use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use replay_engine::{account_data_store::AccountDataStore, types::Slot};
use reqwest;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

use crate::schema::*;
use crate::serde::*;

pub fn get_whirlpool_state_file_relative_path(date: &chrono::NaiveDate) -> String {
    format!(
        "{}/{}/whirlpool-state-{}.json.gz",
        date.format("%Y"),
        date.format("%m%d"),
        date.format("%Y%m%d"),
    )
}

pub fn get_whirlpool_transaction_file_relative_path(date: &chrono::NaiveDate) -> String {
    format!(
        "{}/{}/whirlpool-transaction-{}.jsonl.gz",
        date.format("%Y"),
        date.format("%m%d"),
        date.format("%Y%m%d"),
    )
}

pub fn load_from_local_whirlpool_state_file(file_path: &String, on_memory: bool) -> WhirlpoolState {
    let file = File::open(file_path).unwrap();
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    if on_memory {
        let deserialized: WhirlpoolStateOnMemoryDeserializer = serde_json::from_reader(reader).unwrap();
        WhirlpoolState {
            slot: deserialized.slot,
            block_height: deserialized.block_height,
            block_time: deserialized.block_time,
            program_data: deserialized.program_data,
            accounts: deserialized.accounts,
        }
    } else {
        let deserialized: WhirlpoolStateOnDiskDeserializer = serde_json::from_reader(reader).unwrap();
        WhirlpoolState {
            slot: deserialized.slot,
            block_height: deserialized.block_height,
            block_time: deserialized.block_time,
            program_data: deserialized.program_data,
            accounts: deserialized.accounts,
        }
    }
}

pub fn load_from_remote_whirlpool_state_file(url: &String, on_memory: bool) -> WhirlpoolState {
    let response = reqwest::blocking::get(url).unwrap();
    let decoder = GzDecoder::new(response);
    let reader = BufReader::new(decoder);
    if on_memory {
        let deserialized: WhirlpoolStateOnMemoryDeserializer = serde_json::from_reader(reader).unwrap();
        WhirlpoolState {
            slot: deserialized.slot,
            block_height: deserialized.block_height,
            block_time: deserialized.block_time,
            program_data: deserialized.program_data,
            accounts: deserialized.accounts,
        }
    } else {
        let deserialized: WhirlpoolStateOnDiskDeserializer = serde_json::from_reader(reader).unwrap();
        WhirlpoolState {
            slot: deserialized.slot,
            block_height: deserialized.block_height,
            block_time: deserialized.block_time,
            program_data: deserialized.program_data,
            accounts: deserialized.accounts,
        }
    }
}

pub fn save_to_whirlpool_state_file(
    file_path: &String,
    slot: &Slot,
    program_data: &Vec<u8>,
    accounts: &AccountDataStore,
) {
    let file = File::create(file_path).unwrap();
    let encoder = GzEncoder::new(file, flate2::Compression::default());
    let writer = BufWriter::new(encoder);
    let serializer = WhirlpoolStateSerializer {
        slot: slot.slot,
        block_height: slot.block_height,
        block_time: slot.block_time,
        program_data,
        accounts,
    };
    serde_json::to_writer(writer, &serializer).unwrap();
}

pub fn load_from_local_whirlpool_transaction_file(
    file_path: &String,
) -> impl Iterator<Item = WhirlpoolTransaction> {
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let buf = BufReader::new(decoder);

    let iter = buf.lines().map(|jsonl| jsonl.unwrap()).map(|jsonl| {
        let t: Result<WhirlpoolTransaction, serde_json::Error> =
            serde_json::from_str(jsonl.as_str());
        return t.unwrap();
    });

    return iter;
}

pub fn load_from_remote_whirlpool_transaction_file(
    url: &String,
) -> impl Iterator<Item = WhirlpoolTransaction> {
    let response = reqwest::blocking::get(url).unwrap();

    let decoder = GzDecoder::new(response);
    let buf = BufReader::new(decoder);

    let iter = buf.lines().map(|jsonl| jsonl.unwrap()).map(|jsonl| {
        let t: Result<WhirlpoolTransaction, serde_json::Error> =
            serde_json::from_str(jsonl.as_str());
        return t.unwrap();
    });

    return iter;
}

pub fn download_from_remote_storage(url: &String, file_path: &String) {
    let mut response = reqwest::blocking::get(url).unwrap();
    std::fs::create_dir_all(std::path::Path::new(file_path).parent().unwrap()).unwrap();
    let mut file = File::create(file_path).unwrap();
    std::io::copy(&mut response, &mut file).unwrap();
}

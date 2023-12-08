use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use reqwest;
use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter},
};

use crate::schema::*;

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

pub fn load_from_local_whirlpool_state_file(file_path: &String) -> WhirlpoolState {
    let file = File::open(file_path).unwrap();
    let decoder = GzDecoder::new(file);
    let reader = BufReader::new(decoder);
    return serde_json::from_reader(reader).unwrap();
}

pub fn save_to_whirlpool_state_file(file_path: &String, state: &WhirlpoolState) {
    let file = File::create(file_path).unwrap();
    let encoder = GzEncoder::new(file, flate2::Compression::default());
    let writer = BufWriter::new(encoder);
    serde_json::to_writer(writer, state).unwrap();
}

pub fn load_from_remote_whirlpool_state_file(url: &String) -> WhirlpoolState {
    let response = reqwest::blocking::get(url).unwrap();
    let decoder = GzDecoder::new(response);
    let reader = BufReader::new(decoder);
    return serde_json::from_reader(reader).unwrap();
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

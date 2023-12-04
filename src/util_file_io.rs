use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use serde_derive::{Deserialize, Serialize};
use std::{fs::File, io::{BufRead, BufReader}};

use base64::prelude::{Engine as _, BASE64_STANDARD};

use crate::types::AccountMap;

#[derive(Debug, Deserialize, Serialize)]
struct PubkeyAndDataBase64 {
    pubkey: String,
    data_base64: String,
}

// TODO: error handling
pub fn load_from_snapshot_file(file_path: &String) -> AccountMap {
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(decoder);

    let mut account_map = AccountMap::new();
    reader.deserialize::<PubkeyAndDataBase64>().for_each(|row| {
        let row = row.unwrap();
        let data = BASE64_STANDARD.decode(row.data_base64).unwrap();
        account_map.insert(row.pubkey, data);
    });

    account_map
}

pub fn save_to_snapshot_file(file_path: &String, account_map: &AccountMap) {
    let file = File::create(file_path).unwrap();

    let encoder = GzEncoder::new(file, flate2::Compression::default());
    let mut writer = csv::WriterBuilder::new()
        .has_headers(false)
        .from_writer(encoder);

    for (pubkey, data) in account_map {
        let data_base64 = BASE64_STANDARD.encode(data);
        let row = PubkeyAndDataBase64 {
            pubkey: pubkey.to_string(),
            data_base64,
        };
        writer.serialize(row).unwrap();
    }

    writer.flush().unwrap();
}

pub fn load_from_transaction_file(file_path: &String) -> Vec<String> {
    let file = File::open(file_path).unwrap();

    let decoder = GzDecoder::new(file);
    let buf = BufReader::new(decoder);
    let lines = buf.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    return lines;
}

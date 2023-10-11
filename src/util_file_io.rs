use flate2::read::GzDecoder;
use serde_derive::{Deserialize, Serialize};
use std::fs::File;

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

// TODO: store_to_file

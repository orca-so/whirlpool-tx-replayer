use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use csv;
use serde_derive::{Deserialize, Serialize};
use serde::de;
use serde_json;
use base64::{Engine, prelude::BASE64_STANDARD};

use mysql::*;
use mysql::prelude::*;

mod errors;
mod decoded_instructions;

use decoded_instructions::{from_json, DecodedWhirlpoolInstruction};

#[derive(Debug, PartialEq, Eq)]
struct Slot {
    slot: u64,
    block_height: u64,
    block_time: i64,
}

#[derive(Debug, PartialEq, Eq)]
struct WhirlpoolInstruction {
    txid: u64,
    order: u32,
    ix: DecodedWhirlpoolInstruction,
}


fn main() {
    let url = "mysql://root:password@localhost:3306/localtest";
    let pool = Pool::new(url).unwrap();

    let mut conn = pool.get_conn().unwrap();

    let selected_slots = conn.query_map(
        "SELECT slot, blockHeight, blockTime FROM slots WHERE slot > 215135999 ORDER BY slot ASC LIMIT 10",
        |(slot, block_height, block_time)| {
            Slot {
                slot,
                block_height,
                block_time,
            }
        },
    ).unwrap();

    for slot in selected_slots {
        println!("{:?}", slot);

        let start = slot.slot << 24;
        let end = ((slot.slot + 1) << 24) - 1;

        println!("  start: {:?}, end: {:?}", start, end);

        let selected_ixs = conn.exec_map(
            //"SELECT txid, order, signature, ix, json FROM vwixsSwap t WHERE txid BETWEEN :txidMin AND :txidMax ORDER BY t.txid ASC, t.order ASC",
            //"SELECT t.txid, t.order, t.ix, t.json FROM vwixsSwap t WHERE txid BETWEEN :s and :e ORDER BY t.txid ASC, t.order ASC",
            // Since select for UNION ALL view of these views was too slow, I didn't use UNION ALL view.
            "
                SELECT * FROM vwixsAdminIncreaseLiquidity WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsCloseBundledPosition WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsClosePosition WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsCollectFees WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsCollectProtocolFees WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsCollectReward WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsDecreaseLiquidity WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsDeletePositionBundle WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsIncreaseLiquidity WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializeConfig WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializeFeeTier WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializePool WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializePositionBundle WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializePositionBundleWithMetadata WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializeReward WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsInitializeTickArray WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsOpenBundledPosition WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsOpenPosition WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsOpenPositionWithMetadata WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetCollectProtocolFeesAuthority WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetDefaultFeeRate WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetDefaultProtocolFeeRate WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetFeeAuthority WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetFeeRate WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetProtocolFeeRate WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetRewardAuthority WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetRewardAuthorityBySuperAuthority WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetRewardEmissions WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSetRewardEmissionsSuperAuthority WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsSwap WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsTwoHopSwap WHERE txid BETWEEN :s and :e
                UNION ALL SELECT * FROM vwixsUpdateFeesAndRewards WHERE txid BETWEEN :s and :e",
            params! {
                "s" => start,
                "e" => end,
            },
            |(txid, order, ix, json)| {
                WhirlpoolInstruction {
                    txid,
                    order,
                    ix: from_json(&ix, &json).unwrap(),
                }
            },
        );

        for ix in selected_ixs.unwrap() {
            match ix.ix {
            decoded_instructions::DecodedWhirlpoolInstruction::Swap(detail) => {
                println!("    {:?}", detail);
            },
            decoded_instructions::DecodedWhirlpoolInstruction::IncreaseLiquidity(detail) => {
                println!("    {:?}", detail);
            },
            _ => {},
            }
        }
    }
}

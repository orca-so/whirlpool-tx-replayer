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

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
struct Slot {
    slot: u64,
    block_height: u64,
    block_time: i64,
}

#[derive(Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
struct WhirlpoolInstruction {
    txid: u64,
    order: u32,
    ix: String,
    json: String,
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SwapInstruction {
    data_amount: u64,
    data_other_amount_threshold: u64,
    data_sqrt_price_limit: u128,
    #[serde(deserialize_with = "deserialize_bool")]
    data_amount_specified_is_input: bool,
    #[serde(deserialize_with = "deserialize_bool")]
    data_a_to_b: bool,
    key_token_program: String,
    key_token_authority: String,
    key_whirlpool: String,
    key_token_owner_account_a: String,
    key_vault_a: String,
    key_token_owner_account_b: String,
    key_vault_b: String,
    key_tick_array0: String,
    key_tick_array1: String,
    key_tick_array2: String,
    key_oracle: String,
    transfer_amount0: u64,
    transfer_amount1: u64,
}


fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let n: u8 = de::Deserialize::deserialize(deserializer)?;
    match n {
        0 => Ok(false),
        1 => Ok(true),
        _ => Err(de::Error::custom("expected 0 or 1")),
    }
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
                    ix,
                    json,
                }
            },
        );

        for ix in selected_ixs.unwrap() {
            println!("  {:?}", ix.ix);
            //let deserialized: SwapInstruction = serde_json::from_str(&ix.json).unwrap();
            //println!("  {:?}", deserialized);
        }
    }
}

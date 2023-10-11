use std::borrow::BorrowMut;
use std::{fs::File, collections::BTreeMap};
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use mysql::*;
use mysql::prelude::*;

use crate::decoded_instructions::{from_json, DecodedWhirlpoolInstruction};

#[derive(Debug, PartialEq, Eq)]
struct Slot {
    slot: u64,
    block_height: u64,
    block_time: i64,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WhirlpoolInstruction {
    pub txid: u64,
    pub order: u32,
    pub ix: DecodedWhirlpoolInstruction,
}



// TODO: error handling
pub fn fetch_ixs_in_slot(slot: u64, database: &mut PooledConn) -> Vec<WhirlpoolInstruction> {
  let txid_start = slot << 24;
  let txid_end = ((slot + 1) << 24) - 1;

  let mut ixs_in_slot = database.exec_map(
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
          // no ORDER BY clause, sort at the client side
      params! {
          "s" => txid_start,
          "e" => txid_end,
      },
      |(txid, order, ix, json)| {
          WhirlpoolInstruction {
              txid,
              order,
              ix: from_json(&ix, &json).unwrap(),
          }
      },
  ).unwrap();

  // order by txid, order
  ixs_in_slot.sort_by_key(|ix| (ix.txid, ix.order));

  ixs_in_slot
}
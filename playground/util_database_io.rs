use mysql::*;
use mysql::prelude::*;

use replay_engine::decoded_instructions::{from_json, DecodedInstruction};
use replay_engine::types::Slot;

#[derive(Debug, PartialEq, Eq)]
pub struct Instruction {
    pub txid: u64,
    pub order: u32,
    pub ix_name: String,
    pub ix: DecodedInstruction,
}

// TODO: error handling
pub fn fetch_slot_info(slot: u64, database: &mut PooledConn) -> Slot {
    let mut slots = database.exec_map(
        "SELECT slot, blockHeight, blockTime FROM slots WHERE slot = :s",
        params! {
            "s" => slot,
        },
        |(slot, block_height, block_time)| {
            Slot {
                slot,
                block_height,
                block_time,
            }
        },
    ).unwrap();

    assert_eq!(slots.len(), 1);
    return slots.pop().unwrap();
}

pub fn fetch_next_slot_infos(start_slot: u64, limit: u8, database: &mut PooledConn) -> Vec<Slot> {
  let slots = database.exec_map(
    "SELECT slot, blockHeight, blockTime FROM slots WHERE slot >= :s ORDER BY slot ASC LIMIT :l",
    params! {
        "s" => start_slot,
        "l" => limit,
    },
    |(slot, block_height, block_time)| {
        Slot {
            slot,
            block_height,
            block_time,
        }
    },
  ).unwrap();

  assert!(slots.len() >= 1); // at least start_slot shoud be returned
  return slots;
}

// TODO: error handling
pub fn fetch_instructions_in_slot(slot: u64, database: &mut PooledConn) -> Vec<Instruction> {
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
          let ix_name: String = ix;
          Instruction {
              txid,
              order,
              ix_name: ix_name.clone(),
              ix: from_json(&ix_name, &json).unwrap(),
          }
      },
  ).unwrap();

  // order by txid, order
  ixs_in_slot.sort_by_key(|ix| (ix.txid, ix.order));

  return ixs_in_slot;
}
use crate::{
    account_data_store::AccountDataStore,
    decoded_instructions::{from_json, DecodedInstruction, DecodedWhirlpoolInstruction},
    replay_engine::ReplayEngine,
    types::Slot,
};

use super::WHIRLPOOL_PROGRAM_FOR_TESTING;

pub fn create_engine() -> ReplayEngine {
    ReplayEngine::new(
        Slot::new(1, 1, 1),
        WHIRLPOOL_PROGRAM_FOR_TESTING.to_vec(),
        AccountDataStore::new_on_memory(),
    )
}

pub fn ix(name: &str, json: &str) -> DecodedWhirlpoolInstruction {
    let decoded = from_json(&name.to_string(), &json.to_string()).unwrap();
    match decoded {
        DecodedInstruction::WhirlpoolInstruction(ix) => ix,
        _ => panic!("Invalid instruction"),
    }
}

pub fn replay(replay_engine: &mut ReplayEngine, ix: &DecodedWhirlpoolInstruction) {
    replay_engine.replay_instruction(ix).unwrap();
}

pub fn assert_account_initialized(replay_engine: &ReplayEngine, pubkey: &str) {
    assert!(
        replay_engine
            .get_accounts()
            .get(&pubkey.to_string())
            .unwrap()
            .unwrap()
            .len()
            > 0
    );
}

pub fn assert_account_closed(replay_engine: &ReplayEngine, pubkey: &str) {
    assert!(replay_engine
        .get_accounts()
        .get(&pubkey.to_string())
        .unwrap()
        .is_none());
}

fn deserialize_whirlpool_account<T>(replay_engine: &ReplayEngine, pubkey: &str) -> T
where
    T: anchor_lang::AccountDeserialize
{
    let data = replay_engine.get_accounts().get(&pubkey.to_string()).unwrap().unwrap();
    T::try_deserialize(&mut data.as_slice()).unwrap()
}

pub fn deserialize_lock_config(replay_engine: &ReplayEngine, pubkey: &str) -> whirlpool_base::state::LockConfig {
    deserialize_whirlpool_account(replay_engine, pubkey)
}

pub fn deserialize_position(replay_engine: &ReplayEngine, pubkey: &str) -> whirlpool_base::state::Position {
    deserialize_whirlpool_account(replay_engine, pubkey)
}

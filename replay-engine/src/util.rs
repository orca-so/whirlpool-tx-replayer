use anchor_lang::AccountDeserialize;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

use std::str::FromStr;
use whirlpool_base::state::{Position, PositionBundle, Whirlpool};

use crate::account_data_store::AccountDataStore;
use crate::decoded_instructions::TransferAmountWithTransferFeeConfig;
use crate::pubkeys::{ORCA_WHIRLPOOL_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID};
use crate::replay_instruction::TokenTrait;
use crate::types::WritableAccountSnapshot;

pub fn get_whirlpool_data(pubkey_string: &String, accounts: &AccountDataStore) -> Whirlpool {
    let data = accounts.get(pubkey_string).unwrap().unwrap();
    let whirlpool_data =
        whirlpool_base::state::Whirlpool::try_deserialize(&mut data.as_slice()).unwrap();
    return whirlpool_data;
}

pub fn get_position_data(pubkey_string: &String, accounts: &AccountDataStore) -> Position {
    let data = accounts.get(pubkey_string).unwrap().unwrap();
    let position_data =
        whirlpool_base::state::Position::try_deserialize(&mut data.as_slice()).unwrap();
    return position_data;
}

pub fn get_position_bundle_data(
    pubkey_string: &String,
    accounts: &AccountDataStore,
) -> PositionBundle {
    let data = accounts.get(pubkey_string).unwrap().unwrap();
    let position_bundle_data =
        whirlpool_base::state::PositionBundle::try_deserialize(&mut data.as_slice()).unwrap();
    return position_bundle_data;
}

pub fn pubkey(pubkey_string: &String) -> Pubkey {
    return Pubkey::from_str(pubkey_string).unwrap();
}

pub fn derive_position_bump(position_mint: &Pubkey) -> u8 {
    let (_pubkey, bump) = Pubkey::find_program_address(
        &[b"position", position_mint.as_ref()],
        &ORCA_WHIRLPOOL_PROGRAM_ID,
    );
    return bump;
}

pub fn derive_whirlpool_bump(
    whirlpools_config: &Pubkey,
    token_mint_a: &Pubkey,
    token_mint_b: &Pubkey,
    tick_spacing: u16,
) -> u8 {
    let (_pubkey, bump) = Pubkey::find_program_address(
        &[
            b"whirlpool",
            whirlpools_config.as_ref(),
            token_mint_a.as_ref(),
            token_mint_b.as_ref(),
            &tick_spacing.to_le_bytes(),
        ],
        &ORCA_WHIRLPOOL_PROGRAM_ID,
    );
    return bump;
}

pub fn determine_token_trait(
    token_program_pubkey_string: &String,
    transfer: &TransferAmountWithTransferFeeConfig,
) -> TokenTrait {
    if SPL_TOKEN_PROGRAM_ID.eq(&pubkey(token_program_pubkey_string)) {
        TokenTrait::Token
    } else {
        if transfer.transfer_fee_config_opt {
            TokenTrait::TokenExtensionsWithTransferFee(
                transfer.transfer_fee_config_bps,
                transfer.transfer_fee_config_max
            )
        } else {
            TokenTrait::TokenExtensions
        }
    }
}

pub fn update_accounts(
    accounts: &mut AccountDataStore,
    snapshot: &WritableAccountSnapshot,
) -> Result<()> {
    let pre_snapshot = &snapshot.pre_snapshot;
    let post_snapshot = &snapshot.post_snapshot;

    let closed_account_pubkeys: Vec<String> = pre_snapshot
        .keys()
        .filter(|k| !post_snapshot.contains_key(*k))
        .cloned()
        .collect();

    // insert created & update accounts
    for (pubkey, data) in post_snapshot {
        accounts.upsert(pubkey, data)?;
    }

    // delete closed accounts
    for pubkey in closed_account_pubkeys {
        accounts.delete(&pubkey)?;
    }

    Ok(())
}

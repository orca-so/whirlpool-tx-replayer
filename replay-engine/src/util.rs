use anchor_lang::AccountDeserialize;
use anyhow::Result;
use solana_sdk::pubkey::Pubkey;

use std::str::FromStr;
use whirlpool_base::state::{Position, PositionBundle, Whirlpool};

use crate::account_data_store::AccountDataStore;
use crate::decoded_instructions::{RemainingAccountsInfo, RemainingAccountsKeys, TransferAmountWithTransferFeeConfig};
use crate::pubkeys::{ORCA_WHIRLPOOL_PROGRAM_ID, SPL_TOKEN_PROGRAM_ID, SPL_TOKEN_2022_PROGRAM_ID};
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

pub fn get_remaining_accounts(
    remaining_accounts_info: &RemainingAccountsInfo,
    remaining_accounts_keys: &RemainingAccountsKeys,
    accounts_type: whirlpool_base::util::remaining_accounts_utils::AccountsType,
) -> Vec<String> {
    let accounts_type_u8 = accounts_type as u8;

    let mut offset = 0;
    for i in 0..remaining_accounts_info.len() {
        let slice = remaining_accounts_info[i];
        let slice_accounts_type = slice[0];
        let slice_length = slice[1];

        if slice_accounts_type == accounts_type_u8 {
            let slice_data = &remaining_accounts_keys[offset..offset + slice_length as usize];
            return slice_data.iter().map(|k| k.to_string()).collect();
        }

        offset += slice_length as usize;
    }

    vec![]
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
    if is_token_program(token_program_pubkey_string) {
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

pub fn is_token_program(pubkey_string: &String) -> bool {
    SPL_TOKEN_PROGRAM_ID.eq(&pubkey(pubkey_string))
}

pub fn is_token_2022_program(pubkey_string: &String) -> bool {
    SPL_TOKEN_2022_PROGRAM_ID.eq(&pubkey(pubkey_string))
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

#[cfg(test)]
mod tests {
    use whirlpool_base::util::remaining_accounts_utils::AccountsType;

    #[test]
    fn test_get_remaining_accounts_enum_u8_cast() {
        assert_eq!(AccountsType::TransferHookA as u8, 0);
        assert_eq!(AccountsType::TransferHookB as u8, 1);
        assert_eq!(AccountsType::TransferHookReward as u8, 2);
        assert_eq!(AccountsType::TransferHookInput as u8, 3);
        assert_eq!(AccountsType::TransferHookIntermediate as u8, 4);
        assert_eq!(AccountsType::TransferHookOutput as u8, 5);
        assert_eq!(AccountsType::SupplementalTickArrays as u8, 6);
        assert_eq!(AccountsType::SupplementalTickArraysOne as u8, 7);
        assert_eq!(AccountsType::SupplementalTickArraysTwo as u8, 8);
    }

    #[test]
    fn test_get_remaining_accounts_none() {
        let result = super::get_remaining_accounts(
            &vec![
                [AccountsType::TransferHookA as u8, 3],
                [AccountsType::TransferHookB as u8, 3],
            ],
            &vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
            ],
            AccountsType::SupplementalTickArrays,
        );

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_get_remaining_accounts_first() {
        let result = super::get_remaining_accounts(
            &vec![
                [AccountsType::SupplementalTickArrays as u8, 3],
                [AccountsType::TransferHookA as u8, 3],
                [AccountsType::TransferHookB as u8, 3],
            ],
            &vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
            ],
            AccountsType::SupplementalTickArrays,
        );

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "a");
        assert_eq!(result[1], "b");
        assert_eq!(result[2], "c");
    }

    #[test]
    fn test_get_remaining_accounts_last() {
        let result = super::get_remaining_accounts(
            &vec![
                [AccountsType::TransferHookA as u8, 3],
                [AccountsType::TransferHookB as u8, 3],
                [AccountsType::SupplementalTickArrays as u8, 3],
            ],
            &vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
                "d".to_string(),
                "e".to_string(),
                "f".to_string(),
                "g".to_string(),
                "h".to_string(),
                "i".to_string(),
            ],
            AccountsType::SupplementalTickArrays,
        );

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "g");
        assert_eq!(result[1], "h");
        assert_eq!(result[2], "i");
    }
}

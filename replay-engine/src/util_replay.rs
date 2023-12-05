use anchor_lang::AccountDeserialize;
use solana_sdk::pubkey::Pubkey;

use whirlpool_base::state::{Whirlpool, Position, PositionBundle};
use std::str::FromStr;

use solana_cli_output::display::println_transaction;
use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;

use crate::types::AccountMap;

// TODO: refactor (dedup definitions of pubkeys)
const ORCA_WHIRLPOOL_PROGRAM_ID: Pubkey = solana_program::pubkey!("whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc");

pub fn get_whirlpool_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> Whirlpool {
  let data = account_map.get(pubkey_string).unwrap();
  let whirlpool_data = whirlpool_base::state::Whirlpool::try_deserialize(&mut data.as_slice()).unwrap();
  return whirlpool_data;
}

pub fn get_position_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> Position {
  let data = account_map.get(pubkey_string).unwrap();
  let position_data = whirlpool_base::state::Position::try_deserialize(&mut data.as_slice()).unwrap();
  return position_data;
}

pub fn get_position_bundle_data(
  pubkey_string: &String,
  account_map: &AccountMap,
) -> PositionBundle {
  let data = account_map.get(pubkey_string).unwrap();
  let position_bundle_data = whirlpool_base::state::PositionBundle::try_deserialize(&mut data.as_slice()).unwrap();
  return position_bundle_data;
}


pub fn pubkey(pubkey_string: &String) -> Pubkey {
  return Pubkey::from_str(pubkey_string).unwrap();
}


// TODO: think to receive program_id
pub fn derive_position_bump(position_mint: &Pubkey) -> u8 {
  let (_pubkey, bump) = Pubkey::find_program_address(
    &[
      b"position",
      position_mint.as_ref(),
    ],
    &ORCA_WHIRLPOOL_PROGRAM_ID
  );
  return bump;
}

// TODO: same to derive_position_bump
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
    &ORCA_WHIRLPOOL_PROGRAM_ID
  );
  return bump;
}


pub fn update_account_map(
  account_map: &mut AccountMap,
  pre_snapshot: AccountMap,
  post_snapshot: AccountMap,
) {
  let closed_account_pubkeys: Vec<String> = pre_snapshot.keys()
    .filter(|k| !post_snapshot.contains_key(*k))
    .map(|k| k.clone())
    .collect();

  // add created & update accounts
  account_map.extend(post_snapshot);

  // remove closed accounts
  for pubkey_string in closed_account_pubkeys {
    account_map.remove(&pubkey_string);
  }
}




pub trait PrintableTransaction {
  /// Pretty print the transaction results, tagged with the given name for distinguishability.
  fn print_named(&self, name: &str);

  /// Pretty print the transaction results.
  fn print(&self) {
      self.print_named("");
  }

  /// Panic and print the transaction if it did not execute successfully
  fn assert_success(&self);
}

impl PrintableTransaction for EncodedConfirmedTransactionWithStatusMeta {
  fn print_named(&self, name: &str) {
      let tx = self.transaction.transaction.decode().unwrap();
      println!("EXECUTE {} (slot {})", name, self.slot);
      println_transaction(&tx, self.transaction.meta.as_ref(), "  ", None, None);
  }

  fn assert_success(&self) {
      match &self.transaction.meta {
          Some(meta) if meta.err.is_some() => {
              self.print();
              panic!("tx failed!")
          }
          _ => (),
      }
  }
}
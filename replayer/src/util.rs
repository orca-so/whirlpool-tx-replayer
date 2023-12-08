use crate::schema::*;
use replay_engine::types::AccountMap;

pub fn convert_accounts_to_account_map(accounts: &Vec<WhirlpoolStateAccount>) -> AccountMap {
  let mut account_map = AccountMap::new();
  for account in accounts {
      account_map.insert(account.pubkey.clone(), account.data.clone());
  }
  return account_map;
}

pub fn convert_account_map_to_accounts(account_map: &AccountMap) -> Vec<WhirlpoolStateAccount> {
  let mut accounts = Vec::<WhirlpoolStateAccount>::new();
  for (pubkey, data) in account_map {
      accounts.push(WhirlpoolStateAccount {
          pubkey: pubkey.clone(),
          data: data.clone(),
      });
  }
  return accounts;
}

use serde_derive::{Deserialize, Serialize};
use serde::de;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DecodedSwapInstruction {
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

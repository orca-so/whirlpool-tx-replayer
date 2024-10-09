pub const WHIRLPOOL_PROGRAM_FOR_TESTING: &[u8] = include_bytes!("whirlpool-for-testing.so");

mod test_token_extensions_based_position;
mod test_utils;

pub use test_utils::*;

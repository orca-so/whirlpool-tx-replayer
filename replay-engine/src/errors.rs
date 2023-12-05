use thiserror::Error;

#[derive(Error, Debug)]
pub enum ErrorCode {
  #[error("invalid whirlpool instruction json string")]
  InvalidWhirlpoolInstructionJsonString,

  #[error("unknown whirlpool instruction detected: {0}")]
  UnknownWhirlpoolInstruction(String),

}

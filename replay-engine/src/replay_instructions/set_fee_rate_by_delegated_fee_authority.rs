use whirlpool_base::accounts as whirlpool_ix_accounts;
use whirlpool_base::instruction as whirlpool_ix_args;

use crate::decoded_instructions;
use crate::replay_instruction::{ReplayInstructionParams, ReplayInstructionResult};
use crate::util::pubkey; // abbr

pub fn replay(req: ReplayInstructionParams<decoded_instructions::DecodedSetFeeRateByDelegatedFeeAuthority>) -> ReplayInstructionResult {
  let replayer = req.replayer;
  let ix = req.decoded_instruction;
  let accounts = req.accounts;

  // whirlpool
  replayer.set_whirlpool_account(&ix.key_whirlpool, accounts);
  // adaptive_fee_tier
  replayer.set_whirlpool_account(&ix.key_adaptive_fee_tier, accounts);
  // delegated_fee_authority
    
  let tx = replayer.build_whirlpool_replay_transaction(
    whirlpool_ix_args::SetFeeRateByDelegatedFeeAuthority {
      fee_rate: ix.data_fee_rate,
    },
    whirlpool_ix_accounts::SetFeeRateByDelegatedFeeAuthority {
      whirlpool: pubkey(&ix.key_whirlpool),
      adaptive_fee_tier: pubkey(&ix.key_adaptive_fee_tier),
      delegated_fee_authority: pubkey(&ix.key_delegated_fee_authority),
    },
  );

  let pre_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);
  
  let execution_result = replayer.execute_transaction(tx);

  let post_snapshot = replayer.take_snapshot(&[
    &ix.key_whirlpool,
  ]);

  ReplayInstructionResult::new(execution_result, pre_snapshot, post_snapshot)
}

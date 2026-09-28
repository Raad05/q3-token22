use anchor_lang::prelude::*;

#[error_code]
pub enum MintError {
    #[msg("Only the counter authority can update this counter")]
    Unauthorized,
    #[msg("Counter has reached the maximum value")]
    CounterOverflow,
    #[msg("mint carries an extension this program has not been written to handle")]
    UnsupportedExtension,
    #[msg("calculate_epoch_fee returned no fee for this epoch/amount")]
    FeeCalculationFailed,
    #[msg("caller-supplied fee does not match the fee computed from the mint's TransferFeeConfig")]
    FeeMismatch,
}

pub type ErrorCode = MintError;

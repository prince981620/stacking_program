use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
     #[msg("Freeze Periode Not Passed")]
    FreezePeriodeNotPassed,

    #[msg("Invalid admin")]
    InvalidAdmin,
}

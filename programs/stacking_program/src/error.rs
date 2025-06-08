use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
     #[msg("Freeze Periode Not Passed")]
    FreezePeriodeNotPassed,

    #[msg("Invalid admin")]
    InvalidAdmin,

    #[msg("Over Flow")]
    OverFlow,

    #[msg("Under Flow")]
    UnderFlow,

    #[msg("Stake Periode is too Low")]
    TooLessStakePeriod,

}

use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u32,
    pub amount_staked: u32,
    pub bump: u8,
}
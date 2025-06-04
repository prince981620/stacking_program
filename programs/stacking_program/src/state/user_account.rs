use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u64,
    pub nft_staked_amount: u64,
    pub spl_staked_amount: u64,
    pub sol_staked_amount: u64,
    pub bump: u8,
}
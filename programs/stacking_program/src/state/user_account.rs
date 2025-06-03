use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct UserAccount {
    pub points: u64,
    pub nft_staked_amount: u32,
    pub spl_staked_amount: u32,
    pub sol_staked_amount: u32,
    pub bump: u8,
}
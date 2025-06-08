use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StakeAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub staked_amt: u64,
    pub staked_at: i64,
    pub lock_period: i64,
    pub locked_stackers: bool,
    pub bump: u8,
    // pub vault_bump: u8,
    pub seed: u64,
}
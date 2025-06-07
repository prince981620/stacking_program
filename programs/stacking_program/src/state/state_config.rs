use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StateConfig {
    pub points_per_nft_stake: u8,
    pub points_per_spl_stake: u8,
    pub points_per_sol_stake: u8,
    pub min_freeze_period: i64,
    pub annaul_percentage_rate: u16,
    pub rewards_bump: u8,
    pub bump: u8,
}


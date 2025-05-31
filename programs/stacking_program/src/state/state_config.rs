use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct StateConfig {
    pub point_per_stake: u8,
    pub freeze_period: u32,
    pub rewards_bump: u8,
    pub bump: u8,
}


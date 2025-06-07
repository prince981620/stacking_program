use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token};

use crate::{error::ErrorCode, StateConfig, ADMIN};

#[derive(Accounts)]
pub struct InitializeConfig <'info> {
    #[account(
        mut,
        address = ADMIN @ ErrorCode::InvalidAdmin
    )]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        seeds = [b"config"],
        bump,
        space = 8 + StateConfig::INIT_SPACE
    )]
    pub config: Account<'info, StateConfig>,

    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub reward_mint: Account<'info, Mint>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

impl <'info> InitializeConfig <'info> {
    pub fn initialize_config(
        &mut self,
        points_per_nft_stake: u8,
        points_per_spl_stake: u8,
        points_per_sol_stake: u8,
        min_freeze_period: i64,
        annaul_percentage_rate: u16,
        bumps: &InitializeConfigBumps,
    ) -> Result<()> {
        self.config.set_inner(StateConfig {
            points_per_nft_stake,
            points_per_spl_stake,
            points_per_sol_stake,
            min_freeze_period,
            annaul_percentage_rate,
            rewards_bump: bumps.reward_mint, 
            bump: bumps.config
        });

        Ok(())
    }
}

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("CS3afP5HKjUkUCifygoRg57ecdzUqaNiGvgRgzwtzePP");

#[program]
pub mod stacking_program {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_nft_stake: u8,
        points_per_spl_stake: u8,
        points_per_sol_stake: u8,
        min_freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_nft_stake, points_per_spl_stake, points_per_sol_stake, min_freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        ctx.accounts.stake_nft(&ctx.bumps)
    }

    pub fn stake_sol(ctx: Context<StakeSOl>, amount: u64) -> Result<()> {
        ctx.accounts.stake_sol(amount, &ctx.bumps)
    }

    pub fn stake_spl(ctx: Context<StakeSPL>, amount: u64) -> Result<()> {
        ctx.accounts.stake_spl(amount, &ctx.bumps)
    }

    pub fn unstake_nft(ctx: Context<UnStakeNFT>) -> Result<()> {
        ctx.accounts.unstake_nft()
    }

    pub fn unstake_sol(ctx: Context<UnStakeSOl>) -> Result<()> {
        ctx.accounts.unstake_sol()
    }

    pub fn unstake_spl(ctx: Context<UnStakeSPL>) -> Result<()> {
        ctx.accounts.unstake_spl()
    }

}

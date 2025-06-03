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

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake:u8, freeze_period: u32) -> Result<()> {
        ctx.accounts.initialize_config(points_per_stake, freeze_period, &ctx.bumps)
    }

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps)
    }

    pub fn stake_nft(ctx: Context<StakeNFT>) -> Result<()> {
        ctx.accounts.stake_nft(&ctx.bumps)
    }
}

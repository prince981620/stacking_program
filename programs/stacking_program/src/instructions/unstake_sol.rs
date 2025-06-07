use anchor_lang::{prelude::*, solana_program::native_token::LAMPORTS_PER_SOL, system_program::{transfer, Transfer}};
use anchor_spl::token::{ Mint, Token, TokenAccount};

use crate::{error::ErrorCode, StakeAccount, StateConfig, UserAccount};

#[derive(Accounts)]
pub struct UnStakeSOl <'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"rewards", config.key().as_ref()],
        bump = config.rewards_bump,
        mint::authority = config,
    )]
    pub reward_mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = reward_mint,
        associated_token::authority = user,
    )]
    pub user_reward_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        close = user,
        seeds = [b"stake", config.key().as_ref(), user.key().as_ref(), stake_account.seed.to_le_bytes().as_ref()], // seed so that user can stake multiple ammounts
        bump = stake_account.bump,
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StateConfig>,

    // #[account(
    //     mut,
    //     seeds = [b"vault", stake_account.key().as_ref()],
    //     bump = stake_account.vault_bump,
    // )]
    // pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
} 

impl <'info> UnStakeSOl <'info> {
    pub fn unstake_sol(&mut self) -> Result<()> {

        let staked_at = self.stake_account.staked_at;
        let current = Clock::get()?.unix_timestamp;

        require!(current.checked_sub(staked_at).unwrap() >= self.config.min_freeze_period as i64, ErrorCode::FreezePeriodeNotPassed);

        // let seeds = &[
        //     b"vault",
        //     self.stake_account.to_account_info().key.as_ref(),
        //     &[self.stake_account.vault_bump],
        // ];

        // let signer_seeds = &[&seeds[..]];

        // let cpi_program = self.system_program.to_account_info();
        // let cpi_accounts = Transfer {
        //     from: self.vault.to_account_info(),
        //     to: self.user.to_account_info(),
        // };

        // let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        // transfer(cpi_ctx, self.vault.lamports())?;

        self.user_account.sol_staked_amount = self.user_account.sol_staked_amount.checked_sub(self.stake_account.get_lamports()).ok_or(ErrorCode::OverFlow)?;


        Ok(())

    }


}
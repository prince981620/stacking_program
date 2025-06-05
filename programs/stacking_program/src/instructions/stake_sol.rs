use anchor_lang::{prelude::*, system_program::{transfer, Transfer}};
use anchor_spl::token::{mint_to, spl_token::native_mint, Mint, MintTo, Token, TokenAccount};

use crate::{error::ErrorCode, StakeAccount, StateConfig, UserAccount};

#[derive(Accounts)]
pub struct StakeSOl <'info> {

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
        init,
        payer = user,
        seeds = [b"stake", config.key().as_ref(), user.key().as_ref()], // seed so that user can stake multiple ammounts
        bump,
        space = 8 + StakeAccount::INIT_SPACE
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StateConfig>,

    #[account(
        mut,
        seeds = [b"vault", stake_account.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,

    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
} 

impl <'info> StakeSOl <'info> {
    pub fn stake_sol(&mut self, amount: u64, bumps: &StakeSOlBumps) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: self.user.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_ctx, amount)?;

        let points_u64 = u64::try_from(self.config.points_per_sol_stake).or(Err(ErrorCode::OverFlow))?;

        let reward_amount = points_u64.checked_mul(amount).unwrap(); // amount is already in lamports

        self.user_account.points = self.user_account.points.checked_add(reward_amount).ok_or(ErrorCode::OverFlow)?;
        self.user_account.sol_staked_amount = self.user_account.sol_staked_amount.checked_add(amount).ok_or(ErrorCode::OverFlow)?;

        self.reward_user(reward_amount)?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: native_mint::id(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
            vault_bump: bumps.vault
        });

        Ok(())

    }

    pub fn reward_user(&mut self,amount: u64) -> Result<()> {
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo {
            mint: self.reward_mint.to_account_info(),
            to: self.user_reward_ata.to_account_info(),
            authority: self.config.to_account_info()
        };

        let seeds = &[
            &b"config"[..],
            &[self.config.bump]
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        mint_to(ctx, amount)
    }
}
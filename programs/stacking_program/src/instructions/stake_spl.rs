use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{mint_to, transfer_checked, Mint, MintTo, Token, TokenAccount, TransferChecked}};
use crate::{error::ErrorCode, StakeAccount, StateConfig, UserAccount};

#[derive(Accounts)]
pub struct StakeSPL <'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user
    )]
    pub mint_ata: Account<'info, TokenAccount>,

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
        seeds = [b"stake", config.key().as_ref(), user.key().as_ref(), mint.key().as_ref()], // seed so that user can stake multiple ammounts
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
        init,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = stake_account,
    )]
    pub vault_ata: Account<'info, TokenAccount>,


    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
} 

impl <'info> StakeSPL <'info> {

    pub fn stake_spl(&mut self, amount: u64, bumps: &StakeSPLBumps) -> Result<()> {

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from: self.mint_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.vault_ata.to_account_info(),
            authority: self.user.to_account_info()
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer_checked(cpi_ctx, amount, self.mint.decimals)?;

        let points_u64 = u64::try_from(self.config.points_per_spl_stake).or(Err(ErrorCode::OverFlow))?;

        let reward_amount = points_u64.checked_mul(amount).unwrap(); // amount is already in lamports

        self.user_account.points = self.user_account.points.checked_add(reward_amount).ok_or(ErrorCode::OverFlow)?;
        self.user_account.spl_staked_amount = self.user_account.spl_staked_amount.checked_add(amount).ok_or(ErrorCode::OverFlow)?;

        self.reward_user(reward_amount)?;

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            bump: bumps.stake_account,
            vault_bump: 0,
            seed: 0,
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
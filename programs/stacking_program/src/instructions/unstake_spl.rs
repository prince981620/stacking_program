use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{close_account, mint_to, transfer_checked, CloseAccount, Mint, MintTo, Token, TokenAccount, TransferChecked}};
use crate::{error::ErrorCode, StakeAccount, StateConfig, UserAccount};

#[derive(Accounts)]
pub struct UnStakeSPL <'info> {

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
        mut,
        close = user,
        seeds = [b"stake", config.key().as_ref(), user.key().as_ref(), mint.key().as_ref(), stake_account.seed.to_le_bytes().as_ref()], // seed so that user can stake multiple ammounts
        bump = stake_account.bump
    )]
    pub stake_account: Account<'info, StakeAccount>,
    
    #[account(
        seeds = [b"config"],
        bump = config.bump,
    )]
    pub config: Account<'info, StateConfig>,

    #[account(
        mut,
        close = user,
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
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>
} 

impl <'info> UnStakeSPL <'info> {

    pub fn unstake_spl(&mut self) -> Result<()> {

        let staked_at = self.stake_account.staked_at;
        let current = Clock::get()?.unix_timestamp;

        let time_passed = current.checked_sub(staked_at).unwrap();

        require!( time_passed >= self.stake_account.lock_period, ErrorCode::FreezePeriodeNotPassed);


        let seeds = &[
            b"stake",
            self.config.to_account_info().key.as_ref(),
            self.user.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &self.stake_account.seed.to_le_bytes(),
            &[self.stake_account.bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked {
            from: self.vault_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            to: self.mint_ata.to_account_info(),
            authority: self.stake_account.to_account_info()
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.vault_ata.amount, self.mint.decimals)?;

        
        let close_accounts = CloseAccount {
            account: self.vault_ata.to_account_info(),
            destination: self.user.to_account_info(),
            authority: self.stake_account.to_account_info()
        };
        
        let close_cpi_ctx = CpiContext::new_with_signer(self.token_program.to_account_info(), close_accounts, signer_seeds);
        
        close_account(close_cpi_ctx)?;

        let points_u64 = u64::try_from(self.config.points_per_spl_stake).or(Err(ErrorCode::OverFlow))?;
        let time_passed_u64 = u64::try_from(time_passed).or(Err(ErrorCode::OverFlow))?;

        let mut reward_amount: u64 = points_u64.checked_mul(time_passed_u64).ok_or(ErrorCode::OverFlow)?;

        if self.stake_account.locked_stackers {
            let annual_percentage_rate_u64 = u64::try_from(self.config.annaul_percentage_rate).or(Err(ErrorCode::OverFlow))?;
            let yield_time_u64 = u64::try_from(self.stake_account.lock_period).or(Err(ErrorCode::OverFlow))?;
            let yield_reward = yield_time_u64.checked_mul(points_u64).ok_or(ErrorCode::OverFlow)?;
            let product: u64 = yield_reward.checked_mul(annual_percentage_rate_u64).ok_or(ErrorCode::OverFlow)?;
            let yield_amt: u64 = product.checked_div(10_000u64).ok_or(ErrorCode::OverFlow)?;
            reward_amount = reward_amount.checked_add(yield_amt).ok_or(ErrorCode::OverFlow)?;
        }
        
        self.user_account.spl_staked_amount = self.user_account.spl_staked_amount.checked_add(self.vault_ata.amount).ok_or(ErrorCode::OverFlow)?;
        self.reward_user(reward_amount)?;

        Ok(())
    }

    pub fn reward_user(&mut self, amount: u64) -> Result<()> {
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

        mint_to(ctx, amount)?;

        self.user_account.points = self.user_account.points.checked_add(amount).ok_or(ErrorCode::OverFlow)?;

        Ok(())
    }


}
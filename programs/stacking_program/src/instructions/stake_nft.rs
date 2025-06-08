use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::{
        mpl_token_metadata::instructions::{
            FreezeDelegatedAccountCpi, FreezeDelegatedAccountCpiAccounts,
        },
        MasterEditionAccount, Metadata, MetadataAccount,
    },
    token::{approve, mint_to, Approve, Mint, MintTo, Token, TokenAccount},
};

use crate::{error::ErrorCode, StakeAccount, StateConfig, UserAccount};

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct StakeNFT<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: Account<'info, Mint>,

    pub collection_mint: Account<'info, Mint>,

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
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key(),
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true

    )]
    pub metadata: Account<'info, MetadataAccount>,

    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref(),
            b"edition"
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    pub master_edition: Account<'info, MasterEditionAccount>,

    #[account(
        init, // if we stake -> unstake and then stake again this may fail
        payer = user,
        seeds = [b"stake", config.key().as_ref(), mint.key().as_ref(), seed.to_le_bytes().as_ref()],
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
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, UserAccount>,

    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
}

impl<'info> StakeNFT<'info> {
    pub fn stake_nft(&mut self, seed:u64, locked_stakers: bool, lock_period: i64, bumps: &StakeNFTBumps) -> Result<()> {

        require!(lock_period >= self.config.min_freeze_period, ErrorCode::TooLessStakePeriod);

        let cpi_program = self.token_program.to_account_info();
        let cpi_account = Approve {
            to: self.mint_ata.to_account_info(),
            delegate: self.stake_account.to_account_info(),
            authority: self.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_account);

        approve(cpi_ctx, 1)?;

        let delegate = &self.stake_account.to_account_info();
        let token_account = &self.mint_ata.to_account_info();
        let edition = &self.master_edition.to_account_info();
        let mint = &self.mint.to_account_info();
        let token_program = &self.token_program.to_account_info();
        let metadata_program = &self.metadata_program.to_account_info();

        let seeds = &[
            b"stake",
            self.config.to_account_info().key.as_ref(),
            self.mint.to_account_info().key.as_ref(),
            &seed.to_le_bytes()[..],
            &[bumps.stake_account],
        ];

        let signer_seeds = &[&seeds[..]];
        FreezeDelegatedAccountCpi::new(
            metadata_program,
            FreezeDelegatedAccountCpiAccounts {
                delegate,
                token_account,
                edition,
                mint,
                token_program,
            },
        )
        .invoke_signed(signer_seeds)?;
        

        self.stake_account.set_inner(StakeAccount {
            owner: self.user.key(),
            mint: self.mint.key(),
            staked_at: Clock::get()?.unix_timestamp,
            lock_period: lock_period,
            locked_stackers: locked_stakers,
            bump: bumps.stake_account,
            // vault_bump: 0,
            seed,
        });

        // let points_u64 = u64::try_from(self.config.points_per_nft_stake).or(Err(ErrorCode::OverFlow))?;

        // let reward_amount = points_u64.checked_mul(1_000_000u64).unwrap();

        self.reward_user(100_000_000u64)?; // one time reward to stake NFT

        // self.user_account.points = self.user_account.points.checked_add(100_000_000u64).ok_or(ErrorCode::OverFlow)?;
        self.user_account.nft_staked_amount = self.user_account.nft_staked_amount.checked_add(1).ok_or(ErrorCode::OverFlow)?;

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

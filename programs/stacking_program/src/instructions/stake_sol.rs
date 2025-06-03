use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct StakeSOl <'info> {

    #[account(mut)]
    pub user: Signer<'info>,

    
} 
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::state::{Bank, User};

#[derive(Accounts)]
pub struct Borrow<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump = user_account.bump
    )]
    pub user_account: Account<'info, User>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::authority = user,
        associated_token::token_program = token_program
    )]
    pub user_token_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"bank", mint.key().as_ref()],
        bump 
    )]
    pub bank_account: Account<'info, Bank>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = bank_account,
        associated_token::token_program = token_program
    )]
    pub bank_token_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn borrow(ctx: Context<Borrow>, mint: Pubkey, amount: u64) -> Result<()> {

    Ok(())
}

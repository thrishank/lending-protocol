use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::state::{Bank, User};

#[derive(Accounts)]
pub struct InitUser<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init,
        payer = signer,
        space = 8 + User::INIT_SPACE,
        seeds = [b"user", signer.key().as_ref()],
        bump
    )]
    pub user_account: Account<'info, User>,

    pub system_program: Program<'info, System>,
}

pub fn init_user(ctx: Context<InitUser>) -> Result<()> {
    let now = Clock::get()?.unix_timestamp;

    let user = &mut ctx.accounts.user_account;
    user.owner = ctx.accounts.signer.key();
    user.bump = ctx.bumps.user_account;
    user.updated_at = now;

    Ok(())
}

#[derive(Accounts)]
pub struct InitBank<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        payer = signer,
        space = 8 + Bank::INIT_SPACE,
        seeds = [b"bank", mint.key().as_ref()],
        bump
    )]
    pub bank_account: Account<'info, Bank>,

    #[account(
        init,
        payer = signer,
        token::mint = mint,
        token::authority = bank_token_account,
        seeds = [b"bank:token", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

pub fn init_bank(ctx: Context<InitBank>, liquidation_threshold: u64, max_ltv: u64) -> Result<()> {
    let bank = &mut ctx.accounts.bank_account;

    bank.owner = ctx.accounts.signer.key();
    bank.mint = ctx.accounts.mint.key();
    bank.liquidation_threshold = liquidation_threshold;
    bank.max_ltv = max_ltv;
    bank.bump = ctx.bumps.bank_account;
    bank.bump = ctx.bumps.bank_token_account;

    Ok(())
}

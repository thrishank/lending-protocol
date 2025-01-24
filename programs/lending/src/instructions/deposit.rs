use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked,
};

use crate::state::{Bank, User, UserTokenAccount};

#[derive(Accounts)]
pub struct Deposit<'info> {
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
        mut,
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
        seeds = [b"bank:token", mint.key().as_ref()],
        bump
    )]
    pub bank_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.user_token_ata.to_account_info(),
        to: ctx.accounts.bank_token_account.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_transfer = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_accounts,
    );

    transfer_checked(cpi_transfer, amount, ctx.accounts.mint.decimals)?;

    let now = Clock::get()?.unix_timestamp;
    let bank = &mut ctx.accounts.bank_account;

    bank.last_updated = now;

    if bank.total_deposit == 0 {
        bank.total_deposit_shares = amount;
        bank.total_deposit = amount;
    }
    let deposit_ratio = amount.checked_div(bank.total_deposit).unwrap();
    let user_shares = bank
        .total_deposit_shares
        .checked_mul(deposit_ratio)
        .unwrap();

    let user = &mut ctx.accounts.user_account;

    user.updated_at = now;
    let user_token_account = user
        .token_account
        .iter_mut()
        .find(|account| account.mint == ctx.accounts.mint.to_account_info().key());

    if let Some(token_account) = user_token_account {
        token_account.deposited += amount;
        token_account.deposited_shares += user_shares;
    } else {
        user.token_account.push(UserTokenAccount {
            mint: ctx.accounts.mint.to_account_info().key(),
            deposited: amount,
            deposited_shares: user_shares,
            borrowed: 0,
            borrowed_shares: 0,
        })
    }

    bank.total_deposit += amount;
    bank.total_deposit_shares += user_shares;

    Ok(())
}

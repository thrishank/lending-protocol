use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface,TransferChecked, transfer_checked},
};
use crate::state::{Bank, User};
use crate::error::ErrorCode;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", user.key().as_ref()],
        bump 
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
        seeds = [b"bank:token", mint.key().as_ref()],
        bump 
   )]
    pub bank_token_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>, 
    pub system_program: Program<'info, System>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user_account;
    let user_token_account = user
        .token_account
        .iter_mut()
        .find(|account| account.mint == ctx.accounts.mint.to_account_info().key()).unwrap();

    if amount > user_token_account.deposited {
            return Err(ErrorCode::InsufficientFunds.into());
    }
    let transfer_accounts = TransferChecked {
        from: ctx.accounts.bank_token_ata.to_account_info(),
        to: ctx.accounts.user_token_ata.to_account_info(),
        mint: ctx.accounts.mint.to_account_info(),
        authority: ctx.accounts.bank_token_ata.to_account_info()
    };
    let mint_key = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[
        &[
            b"bank:token",
            mint_key.as_ref(),
            &[ctx.bumps.bank_token_ata],
        ],
    ];
    let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), transfer_accounts).with_signer(signer_seeds);

    transfer_checked(cpi, amount, ctx.accounts.mint.decimals)?;

    let bank = &mut ctx.accounts.bank_account;
    let shares_to_remove = (amount as f64 / bank.total_deposit as f64) * bank.total_deposit_shares as f64;

    user_token_account.deposited -= shares_to_remove as u64;
    bank.total_deposit -= amount;
    bank.total_deposit_shares -= shares_to_remove as u64;

    Ok(())
}

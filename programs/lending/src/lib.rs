pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;

declare_id!("2GD3pRrokGDHUKgbrrNGeGfgf1vuCvKPyds3uqkoYN9j");

#[program]
pub mod lending {

    use super::*;

    pub fn initalize_user(ctx: Context<InitUser>) -> Result<()> {
        instructions::init::init_user(ctx)
    }

    pub fn initalize_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        instructions::init::init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn user_deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::deposit::deposit(ctx, amount)
    }

    pub fn user_withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::withdraw::withdraw(ctx, amount)
    }
}

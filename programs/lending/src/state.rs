use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    // Pubkey of the users's wallet
    pub owner: Pubkey,
    // User's token accounts mapped by asset type
    #[max_len(100)]
    pub token_account: Vec<UserTokenAccount>,
    // health factor of the user
    pub health_factor: u64,
    // last update timestamp
    pub updated_at: i64,
    // bump used to derive the PDA
    pub bump: u8,
}

#[derive(Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct UserTokenAccount {
    pub mint: Pubkey,
    pub deposited: u64,
    pub borrowed: u64,
    pub deposited_shares: u64,
    pub borrowed_shares: u64,
}

#[account]
#[derive(InitSpace)]
pub struct Bank {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub total_deposit: u64,
    pub total_borrow: u64,
    pub total_deposit_shares: u64,
    pub total_borrowed_shares: u64,
    /// LTV at which the loan is defined as under collateralized and can be liquidated
    pub liquidation_threshold: u64,
    /// Bonus percentage of collateral that can be liquidated
    pub liquidation_bonus: u64,
    /// Percentage of collateral that can be liquidated
    pub liquidation_close_factor: u64,
    /// Max percentage of collateral that can be borrowed
    pub max_ltv: u64,
    /// Last updated timestamp
    pub last_updated: i64,
    pub interest_rate: u64,
    pub bump: u8,
    pub token_bump: u8,
}

use crate::*;

#[account]
#[derive(InitSpace)]
pub struct Treasury {
    pub treasurer: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct BetInfo {
    pub user: Pubkey,
    pub amount: u64,
    pub prediction_up: bool,
    pub deadline: u64,
    pub claimed: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct Round {
    pub start_ts: u64,
    pub end_ts: u64,
    pub price_open: u64,
    pub price_close: u64,
    pub total_up: u64,
    pub total_down: u64,
    pub settled: bool,
    pub bump: u8,
}

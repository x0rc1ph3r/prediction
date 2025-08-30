use crate::*;

#[error_code]
pub enum CustomError {
    #[msg("Negative Price")]
    NegativePrice,
    #[msg("Already Claimed")]
    AlreadyClaimed,
    #[msg("Insufficient Funds")]
    InsufficientFunds,
    #[msg("Invalid Amount")]
    InvalidAmount,
    #[msg("Bet Not Expired")]
    BetNotExpired,
    #[msg("Round Not Settled")]
    RoundNotSettled,
    #[msg("Overflow")]
    Overflow
}

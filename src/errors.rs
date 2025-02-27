use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,
    #[error("Insufficient balance")]
    InsufficientBalance,
    #[error("Bet not found")]
    BetNotFound,
    #[error("Invalid bet amount")]
    InvalidBetAmount,
}
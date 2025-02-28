use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("User not found")]
    UserNotFound,
    // This error variant will be implemented later
    //#[error("Insufficient balance")]
    //InsufficientBalance,
    #[error("Bet not found")]
    BetNotFound,
    #[error("Invalid bet amount")]
    InvalidBetAmount,
}
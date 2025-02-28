use crate::{models::User, errors::AppError};
//use uuid::Uuid;

pub struct PaymentService;

impl PaymentService {
    pub fn deposit(user: &mut User, amount: f64) -> Result<(), AppError> {
        if amount <= 0.0 {
            return Err(AppError::InvalidBetAmount);
        }
        user.balance += amount;
        Ok(())
    }
/* 
// Keeping this function for future enhancements
    pub fn withdraw(user: &mut User, amount: f64) -> Result<(), AppError> {
        if amount <= 0.0 {
            return Err(AppError::InvalidBetAmount);
        }
        if user.balance < amount {
            return Err(AppError::InsufficientBalance);
        }
        user.balance -= amount;
        Ok(())
    }
*/
}
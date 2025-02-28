use crate::{models::{Bet, BetStatus}, errors::AppError};
use uuid::Uuid;
use std::collections::HashMap;

pub struct BettingService {
    pub bets: HashMap<Uuid, Bet>,
}

impl BettingService {
    pub fn new() -> Self {
        Self {
            bets: HashMap::new(),
        }
    }

    pub fn place_bet(&mut self, user_id: Uuid, amount: f64, odds: f64) -> Result<Bet, AppError> {
        if amount <= 0.0 {
            return Err(AppError::InvalidBetAmount);
        }

        let bet = Bet {
            id: Uuid::new_v4(),
            user_id,
            amount,
            odds,
            status: BetStatus::Pending,
        };
        self.bets.insert(bet.id, bet.clone());
        Ok(bet)
    }

    pub fn settle_bet(&mut self, bet_id: Uuid, status: BetStatus) -> Result<(), AppError> {
        let bet = self.bets.get_mut(&bet_id).ok_or(AppError::BetNotFound)?;
        bet.status = status;
        Ok(())
    }
}
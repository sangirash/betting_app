use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub balance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bet {
    pub id: Uuid,
    pub user_id: Uuid,
    pub amount: f64,
    pub odds: f64,
    pub status: BetStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BetStatus {
    Pending,
    Won,
    Lost,
}
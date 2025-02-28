use crate::{models::User, errors::AppError};
use uuid::Uuid;
use std::collections::HashMap;

pub struct UserService {
    users: HashMap<Uuid, User>,
}

impl UserService {
    pub fn new() -> Self {
        Self {
            users: HashMap::new(),
        }
    }

    pub fn create_user(&mut self, username: String) -> User {
        let id = Uuid::new_v4();
        let user = User {
            id,
            username,
            balance: 0.0,
        };
        self.users.insert(id, user.clone());
        user
    }

    pub fn get_user(&self, user_id: Uuid) -> Result<&User, AppError> {
        self.users.get(&user_id).ok_or(AppError::UserNotFound)
    }

    pub fn update_balance(&mut self, user_id: Uuid, amount: f64) -> Result<(), AppError> {
        let user = self.users.get_mut(&user_id).ok_or(AppError::UserNotFound)?;
        user.balance += amount;
        Ok(())
    }
}
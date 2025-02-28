pub mod models;
pub mod errors;
pub mod user_service;
pub mod betting_service;
pub mod payment_service;

use user_service::UserService;
use betting_service::BettingService;
use payment_service::PaymentService;
use models::BetStatus;

pub fn run() {
    let mut user_service = UserService::new();
    let mut betting_service = BettingService::new();

    // Create a new user
    let user = user_service.create_user("john_doe".to_string());
    println!("Created user: {:?}", user);

    // Deposit money
    let mut user = user_service.get_user(user.id).unwrap().clone();
    PaymentService::deposit(&mut user, 100.0).unwrap();
    user_service.update_balance(user.id, 100.0).unwrap();
    println!("User after deposit: {:?}", user);

    // Place a bet
    let bet = betting_service.place_bet(user.id, 50.0, 2.0).unwrap();
    println!("Placed bet: {:?}", bet);

    // Settle bet
    betting_service.settle_bet(bet.id, BetStatus::Won).unwrap();
    println!("Settled bet: {:?}", betting_service.bets.get(&bet.id).unwrap());
}
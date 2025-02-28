use betting_app::betting_service::BettingService;
use uuid::Uuid;

#[test]
fn test_place_bet() {
    let mut service = BettingService::new();
    let user_id = Uuid::new_v4();
    let bet = service.place_bet(user_id, 50.0, 2.0).unwrap();
    assert_eq!(bet.amount, 50.0);
}

#[test]
fn test_settle_bet() {
    let mut service = BettingService::new();
    let user_id = Uuid::new_v4();
    let bet = service.place_bet(user_id, 50.0, 2.0).unwrap();
    service.settle_bet(bet.id, betting_app::models::BetStatus::Won).unwrap();
    let settled_bet = service.bets.get(&bet.id).unwrap();
    assert!(matches!(settled_bet.status, betting_app::models::BetStatus::Won));
}
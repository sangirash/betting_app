use betting_app::payment_service::PaymentService;
use betting_app::models::User;
use betting_app::errors::AppError;

#[test]
fn test_deposit() {
    let mut user = User {
        id: uuid::Uuid::new_v4(),
        username: "test_user".to_string(),
        balance: 0.0,
    };
    PaymentService::deposit(&mut user, 100.0).unwrap();
    assert_eq!(user.balance, 100.0);
}

/*
#[test] 
// Withdraw function will be implemented later
fn test_withdraw() {
    let mut user = User {
        id: uuid::Uuid::new_v4(),
        username: "test_user".to_string(),
        balance: 100.0,
    };
    PaymentService::withdraw(&mut user, 50.0).unwrap();
    assert_eq!(user.balance, 50.0);
}
    */

/* 
    #[test]
// Temporary block on this test, since this is related to withdraw
fn test_insufficient_balance() {
    let mut user = User {
        id: uuid::Uuid::new_v4(),
        username: "test_user".to_string(),
        balance: 10.0,
    };
    let result = PaymentService::withdraw(&mut user, 50.0);
    assert!(matches!(result, Err(AppError::InsufficientBalance)));
}
    */
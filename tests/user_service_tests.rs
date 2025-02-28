use betting_app::user_service::UserService;
use uuid::Uuid;

#[test]
fn test_create_user() {
    let mut service = UserService::new();
    let user = service.create_user("test_user".to_string());
    assert_eq!(user.username, "test_user");
}

#[test]
fn test_get_user() {
    let mut service = UserService::new();
    let user = service.create_user("test_user".to_string());
    let fetched_user = service.get_user(user.id).unwrap();
    assert_eq!(fetched_user.username, "test_user");
}
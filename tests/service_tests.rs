use std::sync::Arc;

use rust_async_api::domain::user::{CreateUserRequest, UpdateUserRequest};
use rust_async_api::error::app_error::ServiceError;
use rust_async_api::repository::in_memory_user_repository::InMemoryUserRepository;
use rust_async_api::service::user_service::UserService;

fn user_service() -> UserService {
    let repository = Arc::new(InMemoryUserRepository::new());
    UserService::new(repository)
}

#[test]
fn create_user_rejects_invalid_input() {
    let service = user_service();

    match service.create_user(CreateUserRequest {
        name: "".to_string(),
        description: "Has description".to_string(),
    }) {
        Ok(_) => panic!("create should fail"),
        Err(ServiceError::BadRequest(msg)) => assert_eq!(msg, "Invalid payload"),
        Err(_) => panic!("expected BadRequest"),
    }
}

#[test]
fn get_user_rejects_invalid_uuid() {
    let service = user_service();

    match service.get_user("not-a-uuid") {
        Ok(_) => panic!("get_user should fail"),
        Err(ServiceError::BadRequest(msg)) => assert_eq!(msg, "Invalid ID"),
        Err(_) => panic!("expected BadRequest"),
    }
}

#[test]
fn delete_user_returns_not_found_for_absent_user() {
    let service = user_service();

    match service.delete_user("11111111-1111-1111-1111-111111111111") {
        Ok(_) => panic!("delete_user should fail"),
        Err(ServiceError::NotFound(msg)) => assert_eq!(msg, "User not found"),
        Err(_) => panic!("expected NotFound"),
    }
}

#[test]
fn service_create_then_update_and_get_works() {
    let service = user_service();

    let created = match service.create_user(CreateUserRequest {
        name: "Alice".to_string(),
        description: "Admin".to_string(),
    }) {
        Ok(created) => created,
        Err(_) => panic!("create should succeed"),
    };

    let updated = match service.update_user(
        &created.id.to_string(),
        UpdateUserRequest {
            name: "Alice Updated".to_string(),
            description: "Admin Updated".to_string(),
        },
    ) {
        Ok(updated) => updated,
        Err(_) => panic!("update should succeed"),
    };

    assert_eq!(updated.name, "Alice Updated");

    let fetched = match service.get_user(&created.id.to_string()) {
        Ok(fetched) => fetched,
        Err(_) => panic!("get should succeed"),
    };
    assert_eq!(fetched.description, "Admin Updated");
}

use rust_async_api::domain::user::User;
use rust_async_api::repository::in_memory_user_repository::InMemoryUserRepository;
use rust_async_api::repository::UserRepository;

#[test]
fn in_memory_repository_crud_flow_works() {
    let repository = InMemoryUserRepository::new();
    let user = User {
        id: "11111111-1111-1111-1111-111111111111".to_string(),
        name: "Alice".to_string(),
        description: "Admin".to_string(),
    };

    let created = repository.create(user.clone()).expect("create should succeed");
    assert_eq!(created.id, user.id);

    let fetched = repository
        .get_by_id(&user.id)
        .expect("get should succeed")
        .expect("user should exist");
    assert_eq!(fetched.name, "Alice");

    let updated_user = User {
        id: user.id.clone(),
        name: "Alice Updated".to_string(),
        description: "Admin Updated".to_string(),
    };
    let updated = repository
        .update(updated_user.clone())
        .expect("update should succeed")
        .expect("user should exist");
    assert_eq!(updated.description, "Admin Updated");

    let all_users = repository.get_all().expect("get_all should succeed");
    assert_eq!(all_users.len(), 1);

    let deleted = repository
        .delete(&updated_user.id)
        .expect("delete should succeed");
    assert!(deleted);
    assert!(
        repository
            .get_by_id(&updated_user.id)
            .expect("get should succeed")
            .is_none()
    );
}

#[test]
fn in_memory_repository_update_and_delete_return_absent_for_missing_user() {
    let repository = InMemoryUserRepository::new();

    let updated = repository
        .update(User {
            id: "11111111-1111-1111-1111-111111111111".to_string(),
            name: "Ghost".to_string(),
            description: "Missing".to_string(),
        })
        .expect("update should succeed");
    assert!(updated.is_none());

    let deleted = repository
        .delete("11111111-1111-1111-1111-111111111111")
        .expect("delete should succeed");
    assert!(!deleted);
}

use rust_async_api::domain::user::User;
use rust_async_api::repository::in_memory_user_repository::InMemoryUserRepository;
use rust_async_api::repository::{RepositoryError, UserRepository};
use uuid::Uuid;

#[test]
fn in_memory_repository_crud_flow_works() {
    let repository = InMemoryUserRepository::new();
    let user = User {
        id: Uuid::new_v4(),
        name: "Alice".to_string(),
        description: "Admin".to_string(),
    };

    let created = repository
        .create(user.clone())
        .expect("create should succeed");
    assert_eq!(created.id, user.id);

    let fetched = repository
        .get_by_id(user.id)
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
        .delete(updated_user.id)
        .expect("delete should succeed");
    assert!(deleted);
    assert!(
        repository
            .get_by_id(updated_user.id)
            .expect("get should succeed")
            .is_none()
    );
}

#[test]
fn in_memory_repository_update_and_delete_return_absent_for_missing_user() {
    let repository = InMemoryUserRepository::new();

    let uuid = Uuid::new_v4();

    let updated = repository
        .update(User {
            id: uuid,
            name: "Ghost".to_string(),
            description: "Missing".to_string(),
        })
        .expect("update should succeed");
    assert!(updated.is_none());

    let deleted = repository.delete(uuid).expect("delete should succeed");
    assert!(!deleted);
}

#[test]
fn in_memory_repository_create_returns_duplicate_id_error() {
    let repository = InMemoryUserRepository::new();
    let user = User {
        id: Uuid::new_v4(),
        name: "Alice".to_string(),
        description: "Admin".to_string(),
    };

    repository
        .create(user.clone())
        .expect("first create should succeed");

    let error = repository
        .create(user)
        .expect_err("second create with same id should fail");
    assert!(matches!(error, RepositoryError::DuplicateId));
}

#[test]
fn in_memory_repository_returns_storage_failure_when_mutex_is_poisoned() {
    let repository = InMemoryUserRepository::new();
    poison_users_mutex(&repository);

    let error = repository
        .get_all()
        .expect_err("get_all should fail when storage mutex is poisoned");
    assert!(matches!(error, RepositoryError::StorageFailure));
}

fn poison_users_mutex(repository: &InMemoryUserRepository) {
    use std::collections::HashMap;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    use std::sync::Mutex;

    // SAFETY: InMemoryUserRepository currently contains a single field:
    // Mutex<HashMap<Uuid, User>>. This test-only helper accesses it to poison the lock.
    let users_mutex: &Mutex<HashMap<Uuid, User>> =
        unsafe { &*((repository as *const InMemoryUserRepository).cast()) };

    let panic_result = catch_unwind(AssertUnwindSafe(|| {
        let _guard = users_mutex.lock().expect("should lock before panic");
        panic!("poison users mutex");
    }));
    assert!(panic_result.is_err());
}

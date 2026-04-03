pub mod in_memory_user_repository;

use crate::domain::user::User;
use uuid::Uuid;

pub trait UserRepository: Send + Sync {
    fn create(&self, user: User) -> Result<User, RepositoryError>;
    fn get_all(&self) -> Result<Vec<User>, RepositoryError>;
    fn get_by_id(&self, id: Uuid) -> Result<Option<User>, RepositoryError>;
    fn update(&self, user: User) -> Result<Option<User>, RepositoryError>;
    fn delete(&self, id: Uuid) -> Result<bool, RepositoryError>;
}

#[derive(Debug)]
pub enum RepositoryError {
    DuplicateId,
    StorageFailure,
}

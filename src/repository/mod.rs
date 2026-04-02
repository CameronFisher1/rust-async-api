pub mod in_memory_user_repository;

use crate::domain::user::User;

pub trait UserRepository: Send + Sync {
    fn create(&self, user: User) -> Result<User, ()>;
    fn get_all(&self) -> Result<Vec<User>, ()>;
    fn get_by_id(&self, id: &str) -> Result<Option<User>, ()>;
    fn update(&self, user: User) -> Result<Option<User>, ()>;
    fn delete(&self, id: &str) -> Result<bool, ()>;
}

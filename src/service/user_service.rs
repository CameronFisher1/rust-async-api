use std::sync::Arc;

use uuid::Uuid;

use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::error::app_error::{ServiceError, convert_repo_error};
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub fn create_user(&self, payload: CreateUserRequest) -> Result<User, ServiceError> {
        if payload.name.is_empty() || payload.description.is_empty() {
            return Err(ServiceError::BadRequest("Invalid payload".into()));
        }

        let user = User {
            id: Uuid::new_v4(),
            name: payload.name,
            description: payload.description,
        };

        self.repository.create(user).map_err(convert_repo_error)
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, ServiceError> {
        self.repository.get_all().map_err(|e| convert_repo_error(e))
    }

    pub fn get_user(&self, id: &str) -> Result<User, ServiceError> {
        let user = self
            .repository
            .get_by_id(
                Uuid::parse_str(&id).map_err(|_| ServiceError::BadRequest("Invalid ID".into()))?,
            )
            .map_err(convert_repo_error)?;

        user.ok_or_else(|| ServiceError::NotFound("User not found".into()))
    }

    pub fn update_user(&self, id: &str, payload: UpdateUserRequest) -> Result<User, ServiceError> {
        if payload.name.is_empty() || payload.description.is_empty() {
            return Err(ServiceError::BadRequest("Invalid payload".into()));
        }

        let new_user = User {
            id: Uuid::parse_str(&id).map_err(|_| ServiceError::BadRequest("Invalid ID".into()))?,
            name: payload.name,
            description: payload.description,
        };

        let updated = self
            .repository
            .update(new_user)
            .map_err(convert_repo_error)?;

        updated.ok_or_else(|| ServiceError::NotFound("User not found".into()))
    }

    pub fn delete_user(&self, id: &str) -> Result<(), ServiceError> {
        let deleted = self
            .repository
            .delete(
                Uuid::parse_str(&id).map_err(|_| ServiceError::BadRequest("Invalid ID".into()))?,
            )
            .map_err(convert_repo_error)?;

        if deleted {
            Ok(())
        } else {
            Err(ServiceError::NotFound("User not found".into()))
        }
    }
}

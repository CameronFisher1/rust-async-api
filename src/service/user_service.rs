use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;

use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::error::app_error::{ApiError, api_error, convert_repo_error};
use crate::repository::UserRepository;

#[derive(Clone)]
pub struct UserService {
    repository: Arc<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub fn create_user(&self, payload: CreateUserRequest) -> Result<User, ApiError> {
        if payload.name.is_empty() || payload.description.is_empty() {
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid payload"));
        }

        let user = User {
            id: Uuid::new_v4(),
            name: payload.name,
            description: payload.description,
        };

        self.repository
            .create(user)
            .map_err(|e| convert_repo_error(e))
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, ApiError> {
        self.repository.get_all().map_err(|e| convert_repo_error(e))
    }

    pub fn get_user(&self, id: &str) -> Result<User, ApiError> {
        let user = self
            .repository
            .get_by_id(
                Uuid::parse_str(&id)
                    .map_err(|_| api_error(StatusCode::BAD_REQUEST, "Invalid ID"))?,
            )
            .map_err(|e| convert_repo_error(e))?;

        user.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "User not found"))
    }

    pub fn update_user(&self, id: &str, payload: UpdateUserRequest) -> Result<User, ApiError> {
        if payload.name.is_empty() || payload.description.is_empty() {
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid payload"));
        }

        let new_user = User {
            id: Uuid::parse_str(&id)
                .map_err(|_| api_error(StatusCode::BAD_REQUEST, "Invalid ID"))?,
            name: payload.name,
            description: payload.description,
        };

        let updated = self
            .repository
            .update(new_user)
            .map_err(|e| convert_repo_error(e))?;

        updated.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "User not found"))
    }

    pub fn delete_user(&self, id: &str) -> Result<(), ApiError> {
        let deleted = self
            .repository
            .delete(
                Uuid::parse_str(&id)
                    .map_err(|_| api_error(StatusCode::BAD_REQUEST, "Invalid ID"))?,
            )
            .map_err(|e| convert_repo_error(e))?;

        if deleted {
            Ok(())
        } else {
            Err(api_error(StatusCode::NOT_FOUND, "User not found"))
        }
    }
}

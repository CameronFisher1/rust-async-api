use std::sync::Arc;

use axum::http::StatusCode;
use uuid::Uuid;

use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::error::app_error::{api_error, ApiError};
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
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid input"));
        }

        let user = User {
            id: Uuid::new_v4().to_string(),
            name: payload.name,
            description: payload.description,
        };

        self.repository
            .create(user)
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
    }

    pub fn get_all_users(&self) -> Result<Vec<User>, ApiError> {
        self.repository
            .get_all()
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))
    }

    pub fn get_user(&self, id: &str) -> Result<User, ApiError> {
        if !is_valid_uuid(id) {
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid ID"));
        }

        let user = self
            .repository
            .get_by_id(id)
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

        user.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "User not found"))
    }

    pub fn update_user(&self, id: &str, payload: UpdateUserRequest) -> Result<User, ApiError> {
        if !is_valid_uuid(id) || payload.name.is_empty() || payload.description.is_empty() {
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid input"));
        }

        let new_user = User {
            id: id.to_string(),
            name: payload.name,
            description: payload.description,
        };

        let updated = self
            .repository
            .update(new_user)
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

        updated.ok_or_else(|| api_error(StatusCode::NOT_FOUND, "User not found"))
    }

    pub fn delete_user(&self, id: &str) -> Result<(), ApiError> {
        if !is_valid_uuid(id) {
            return Err(api_error(StatusCode::BAD_REQUEST, "Invalid ID"));
        }

        let deleted = self
            .repository
            .delete(id)
            .map_err(|_| api_error(StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error"))?;

        if deleted {
            Ok(())
        } else {
            Err(api_error(StatusCode::NOT_FOUND, "User not found"))
        }
    }
}

fn is_valid_uuid(uuid_str: &str) -> bool {
    Uuid::parse_str(uuid_str).is_ok()
}

use crate::repository::RepositoryError;
use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    pub error: String,
}

pub enum ServiceError {
    BadRequest(String),
    NotFound(String),
    Conflict(String),
    Internal(String),
}

pub type ApiError = (StatusCode, Json<AppError>);

pub fn convert_svc_error(error: ServiceError) -> ApiError {
    match error {
        ServiceError::BadRequest(e) => (
            StatusCode::BAD_REQUEST,
            Json(AppError {
                error: e.to_string(),
            }),
        ),
        ServiceError::NotFound(e) => (
            StatusCode::NOT_FOUND,
            Json(AppError {
                error: e.to_string(),
            }),
        ),
        ServiceError::Conflict(e) => (
            StatusCode::CONFLICT,
            Json(AppError {
                error: e.to_string(),
            }),
        ),
        ServiceError::Internal(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppError {
                error: e.to_string(),
            }),
        ),
    }
}

pub fn convert_repo_error(error: RepositoryError) -> ServiceError {
    match error {
        RepositoryError::DuplicateId => ServiceError::Conflict("Resource already exists".into()),
        RepositoryError::StorageFailure => ServiceError::Internal("Internal Server Error".into()),
    }
}

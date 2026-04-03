use crate::repository::RepositoryError;
use axum::Json;
use axum::http::StatusCode;
use serde::Serialize;

#[derive(Serialize)]
pub struct AppError {
    pub error: String,
}

pub type ApiError = (StatusCode, Json<AppError>);

pub fn api_error(status: StatusCode, message: impl Into<String>) -> ApiError {
    (
        status,
        Json(AppError {
            error: message.into(),
        }),
    )
}

pub fn convert_repo_error(error: RepositoryError) -> ApiError {
    match error {
        RepositoryError::DuplicateId => (
            StatusCode::CONFLICT,
            Json(AppError {
                error: "Resource already exists".into(),
            }),
        ),
        RepositoryError::StorageFailure => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(AppError {
                error: "Internal Server Error".into(),
            }),
        ),
    }
}

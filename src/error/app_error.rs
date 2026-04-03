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

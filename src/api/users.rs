use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;

use crate::domain::user::{CreateUserRequest, UpdateUserRequest, User};
use crate::error::app_error::{ApiError, convert_svc_error};
use crate::state::AppState;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let user = state
        .user_service
        .create_user(payload)
        .map_err(convert_svc_error)?;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<User>>), ApiError> {
    let users = state
        .user_service
        .get_all_users()
        .map_err(convert_svc_error)?;
    Ok((StatusCode::OK, Json(users)))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let user = state
        .user_service
        .get_user(&id)
        .map_err(convert_svc_error)?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<User>), ApiError> {
    let user = state
        .user_service
        .update_user(&id, payload)
        .map_err(convert_svc_error)?;
    Ok((StatusCode::OK, Json(user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, ApiError> {
    state
        .user_service
        .delete_user(&id)
        .map_err(convert_svc_error)?;
    Ok(StatusCode::NO_CONTENT)
}

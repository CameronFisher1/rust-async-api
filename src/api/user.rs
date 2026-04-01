use crate::AppState;
use crate::model::error::ErrorRes;
use crate::model::user::{CreateUserRequest, UpdateUserRequest, User};
use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use uuid::Uuid;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<ErrorRes>)> {
    let mut users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorRes {
                error: "Internal Server Error".to_string(),
            }),
        )
    })?;

    if payload.name.is_empty() || payload.description.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorRes {
                error: "Invalid input".to_string(),
            }),
        ));
    }

    let id = Uuid::new_v4().to_string();

    let user = User {
        id,
        name: payload.name,
        description: payload.description,
    };

    users.insert(user.id.clone(), user.clone());

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_all_users(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, Json<ErrorRes>)> {
    let users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorRes {
                error: "Internal Server Error".to_string(),
            }),
        )
    })?;

    Ok((StatusCode::OK, Json(users.values().cloned().collect())))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<ErrorRes>)> {
    let users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorRes {
                error: "Internal Server Error".to_string(),
            }),
        )
    })?;

    if !is_valid_uuid(&id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorRes {
                error: "Invalid ID".to_string(),
            }),
        ));
    }

    let user = users.get(&id);

    match user {
        Some(user) => Ok((StatusCode::OK, Json(user.clone()))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ErrorRes {
                error: "User not found".to_string(),
            }),
        )),
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<ErrorRes>)> {
    let mut users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorRes {
                error: "Internal Server Error".to_string(),
            }),
        )
    })?;

    if !is_valid_uuid(&id) || payload.name.is_empty() || payload.description.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorRes {
                error: "Invalid input".to_string(),
            }),
        ));
    }

    if !users.contains_key(&id) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(ErrorRes {
                error: "User not found".to_string(),
            }),
        ));
    }

    let new_user = User {
        id,
        name: payload.name,
        description: payload.description,
    };

    users.insert(new_user.id.clone(), new_user.clone());

    Ok((StatusCode::OK, Json(new_user)))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, (StatusCode, Json<ErrorRes>)> {
    let mut users = state.users.lock().map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorRes {
                error: "Internal Server Error".to_string(),
            }),
        )
    })?;

    if !is_valid_uuid(&id) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorRes {
                error: "Invalid ID".to_string(),
            }),
        ));
    }

    let initial_user_count = users.len();
    users.remove(&id);

    if users.len() == initial_user_count {
        Err((
            StatusCode::NOT_FOUND,
            Json(ErrorRes {
                error: "User not found".to_string(),
            }),
        ))
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

fn is_valid_uuid(uuid_str: &str) -> bool {
    Uuid::parse_str(uuid_str).is_ok()
}

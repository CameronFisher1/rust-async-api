use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::AppState;
use crate::model::user::{CreateUserRequest, User};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut users = state.users.lock().unwrap();

    let id = Uuid::new_v4().to_string();

    let user = User {
        id: id.clone(),
        name: payload.name,
        description: payload.description,
    };
    users.push(user.clone());

    println!("user {} created", id);

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_all_users(
    State(state): State<AppState>
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let users = state.users.lock().unwrap();

    Ok((StatusCode::OK, Json(users.clone())))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.users.lock().unwrap();

    users.retain(|u| u.id != id);

    Ok(StatusCode::NO_CONTENT)
}
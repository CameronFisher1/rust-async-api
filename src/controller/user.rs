use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::Json;
use uuid::Uuid;
use crate::AppState;
use crate::model::user::{CreateUserRequest, UpdateUserRequest, User};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut users = state.users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let id = Uuid::new_v4().to_string();

    let user = User {
        id: id.clone(),
        name: payload.name,
        description: payload.description,
    };
    // users.push(user.clone());
    users.insert(id.clone(), user.clone());

    println!("user {} created", id);

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_all_users(
    State(state): State<AppState>
) -> Result<(StatusCode, Json<Vec<User>>), StatusCode> {
    let users = state.users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::OK, Json(users.values().cloned().collect())))
}

pub async fn get_user(
    State(state): State<AppState>,
    Path(id): Path<String>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let users = state.users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = users.get(&id);

    if user.is_none() {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok((StatusCode::OK, Json(user.unwrap().clone())))
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateUserRequest>
) -> Result<(StatusCode, Json<User>), StatusCode> {
    let mut users = state.users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !users.contains_key(&id) {
        return Err(StatusCode::NOT_FOUND)
    }

    let new_user = User {
        id: id.clone(),
        name: payload.name,
        description: payload.description,
    };

    users.insert(id.clone(), new_user.clone());

    Ok((StatusCode::OK, Json(new_user.clone())))
}

pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let mut users = state.users.lock().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let initial_user_count = users.len();
    users.remove(&id);

    if users.len() == initial_user_count {
        Err(StatusCode::NOT_FOUND)
    } else {
        Ok(StatusCode::NO_CONTENT)
    }
}

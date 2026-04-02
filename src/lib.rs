pub mod api;
pub mod domain;
pub mod service;
pub mod repository;
pub mod error;
pub mod state;

use std::sync::Arc;

use axum::routing::{delete, get, post};
use axum::Router;

use crate::api::health::health_check;
use crate::api::users::{create_user, delete_user, get_all_users, get_user, update_user};
use crate::repository::in_memory_user_repository::InMemoryUserRepository;
use crate::service::user_service::UserService;
use crate::state::AppState;

pub fn app() -> Router {
    let repository = Arc::new(InMemoryUserRepository::new());
    let service = Arc::new(UserService::new(repository));
    let state = AppState::new(service);

    Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(get_all_users))
        .route(
            "/users/:id",
            delete(delete_user).get(get_user).put(update_user),
        )
        .with_state(state)
}

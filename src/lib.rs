pub mod api;
pub mod model;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::routing::{delete, get, post};
use axum::Router;

use crate::api::health::health_check;
use crate::api::user::{create_user, delete_user, get_all_users, get_user, update_user};
use crate::model::user::User;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<String, User>>>,
}

pub fn app() -> Router {
    let state = AppState {
        users: Arc::new(Mutex::new(HashMap::new())),
    };

    Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(get_all_users))
        .route("/users/:id", delete(delete_user).get(get_user).put(update_user))
        .with_state(state)
}

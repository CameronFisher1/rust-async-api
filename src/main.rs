mod controller;
mod model;

use std::sync::{Arc, Mutex};
use axum::{Router, routing::get};
use axum::routing::{delete, post};
use crate::controller::health::health_check;
use crate::controller::user::{create_user, delete_user, get_all_users, get_user, update_user};
use model::user::User;

#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<Vec<User>>>,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        users: Arc::new(Mutex::new(Vec::new())),
    };

    let app = Router::new()
        .route("/health", get(health_check))
        .route("/users", post(create_user).get(get_all_users))
        .route("/users/:id", delete(delete_user).get(get_user).put(update_user))
        .with_state(state);

    let addr = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("could not start server");

    println!("listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}


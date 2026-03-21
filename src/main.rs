mod controller;

use axum::{Router, routing::get};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use crate::controller::health::health_check;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let addr = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("could not start server");

    println!("listening on http://{}", addr);

    axum::serve(listener, app)
        .await
        .expect("server error");
}


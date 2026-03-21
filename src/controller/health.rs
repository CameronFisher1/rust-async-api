use axum::{Router, routing::{get, delete}, extract::Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;

pub async fn health_check() -> impl IntoResponse {
    println!("health check");
    (StatusCode::OK, "ok")
}

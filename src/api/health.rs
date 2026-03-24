use axum::http::StatusCode;

pub async fn health_check() -> Result<(StatusCode, String), StatusCode> {
    Ok((StatusCode::OK, "ok".to_string()))
}

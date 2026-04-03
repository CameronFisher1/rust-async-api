use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use rust_async_api::app;
use serde_json::{Value, json};
use tower::util::ServiceExt;

async fn response_json(response: axum::response::Response) -> Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("body should be readable")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("body should be valid json")
}

#[tokio::test]
async fn create_user_returns_created() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Alice","description":"Admin"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::CREATED);
    let body = response_json(response).await;

    assert!(body["id"].as_str().is_some());
    assert_eq!(body["name"], "Alice");
    assert_eq!(body["description"], "Admin");
}

#[tokio::test]
async fn create_user_with_invalid_payload_returns_bad_request() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"","description":"missing name"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"Invalid payload"}));
}

#[tokio::test]
async fn get_all_users_returns_existing_users() {
    let app = app();

    let create_first = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Alice","description":"One"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");
    assert_eq!(create_first.status(), StatusCode::CREATED);

    let create_second = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Bob","description":"Two"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");
    assert_eq!(create_second.status(), StatusCode::CREATED);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    let users = body.as_array().expect("users should be an array");
    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn get_user_returns_user_when_exists() {
    let app = app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Charlie","description":"Reader"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");
    let create_body = response_json(create_response).await;
    let id = create_body["id"]
        .as_str()
        .expect("id should exist")
        .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/users/{id}"))
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body["id"], id);
    assert_eq!(body["name"], "Charlie");
    assert_eq!(body["description"], "Reader");
}

#[tokio::test]
async fn get_user_with_invalid_id_returns_bad_request() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/not-a-uuid")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"Invalid ID"}));
}

#[tokio::test]
async fn get_user_returns_not_found_when_missing() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/11111111-1111-1111-1111-111111111111")
                .method(Method::GET)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"User not found"}));
}

#[tokio::test]
async fn update_user_returns_updated_user() {
    let app = app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Diana","description":"Before"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");
    let create_body = response_json(create_response).await;
    let id = create_body["id"]
        .as_str()
        .expect("id should exist")
        .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/users/{id}"))
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Diana Updated","description":"After"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response_json(response).await;
    assert_eq!(body["id"], id);
    assert_eq!(body["name"], "Diana Updated");
    assert_eq!(body["description"], "After");
}

#[tokio::test]
async fn update_user_with_invalid_input_returns_bad_request() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/not-a-uuid")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Eve","description":"Editor"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"Invalid ID"}));
}

#[tokio::test]
async fn update_user_returns_not_found_when_missing() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/11111111-1111-1111-1111-111111111111")
                .method(Method::PUT)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Ghost","description":"Missing"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"User not found"}));
}

#[tokio::test]
async fn delete_user_returns_no_content_when_exists() {
    let app = app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/users")
                .method(Method::POST)
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({"name":"Frank","description":"To remove"}).to_string(),
                ))
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");
    let create_body = response_json(create_response).await;
    let id = create_body["id"]
        .as_str()
        .expect("id should exist")
        .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/users/{id}"))
                .method(Method::DELETE)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn delete_user_with_invalid_id_returns_bad_request() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/not-a-uuid")
                .method(Method::DELETE)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"Invalid ID"}));
}

#[tokio::test]
async fn delete_user_returns_not_found_when_missing() {
    let response = app()
        .oneshot(
            Request::builder()
                .uri("/users/11111111-1111-1111-1111-111111111111")
                .method(Method::DELETE)
                .body(Body::empty())
                .expect("request should build"),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    let body = response_json(response).await;
    assert_eq!(body, json!({"error":"User not found"}));
}

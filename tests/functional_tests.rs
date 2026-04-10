use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::Value;
use stage0::create_app;
use tower::ServiceExt;

#[tokio::test]
async fn test_classify_success() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=john")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["status"], "success");
    assert_eq!(body["data"]["name"], "john");
}

#[tokio::test]
async fn test_classify_with_spaces() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=john%20brown")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["status"], "success");
    assert_eq!(body["data"]["name"], "john brown");
}

#[tokio::test]
async fn test_classify_alphanumeric() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=r3r")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_numeric_name() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_json_boolean() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=true")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_json_array() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=%5B1,2,3%5D") // [1,2,3]
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_missing_name() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_classify_empty_name() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body["status"], "error");
    assert_eq!(body["message"], "Missing or empty name parameter");
}

#[tokio::test]
async fn test_classify_whitespace_name() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=%20%20%20")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_classify_json_object() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=%7B%22name%22%3A%22john%22%7D") // {"name":"john"}
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_json_null() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=null")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn test_classify_success_response_shape() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=john")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert!(body["data"]["name"].is_string());
    assert!(body["data"]["gender"].is_string());
    assert!(body["data"]["probability"].is_f64());
    assert!(body["data"]["sample_size"].is_u64());
    assert!(body["data"]["is_confident"].is_boolean());
    assert!(body["data"]["processed_at"].is_string());

    let processed_at = body["data"]["processed_at"].as_str().unwrap();
    assert!(processed_at.ends_with('Z'));
}

#[tokio::test]
async fn test_classify_missing_name_error_shape() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["status"], "error");
    assert!(body["message"].is_string());
}

#[tokio::test]
async fn test_classify_unknown_name() {
    let app = create_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/classify?name=xzqwvmblrpt")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body["status"], "error");
    assert_eq!(
        body["message"],
        "No prediction available for the provided name"
    );
}

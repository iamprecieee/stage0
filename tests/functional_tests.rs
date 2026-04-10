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

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::{json, Value};
use synthesizer::api;
use tower::ServiceExt; // for `oneshot` and `ready`

#[tokio::test]
async fn health_success() {
    let app = api::endpoints::create_pipeline_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(body, json!({ "data": "Feeling healthy!" }));
}

#[tokio::test]
async fn get_endpoints() {
    let app = api::endpoints::create_pipeline_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/endpoints")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_pipelines() {
    let app = api::endpoints::create_pipeline_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/endpoints")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);
}

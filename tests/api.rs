use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hyper::http;
use pretty_assertions::assert_eq;
use serde_json::{json, Value};
use synthesizer::{api, database};
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
async fn list_pipelines_success() {
    let app = api::endpoints::create_pipeline_router();
    database::run_migrations().await.unwrap();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/pipelines")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn create_pipeline_success() {
    let app = api::endpoints::create_pipeline_router();
    database::reset_database().await.unwrap();
    database::run_migrations().await.unwrap();

    // Build the request
    let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/pipelines")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"{"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"}"#,
        ))
        .unwrap();

    // Send the request and get the response
    let response = app.oneshot(request).await.unwrap();

    // Load the status and body as bytes for use in assertions
    let status = response.status();
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();

    assert_eq!(
        status,
        StatusCode::CREATED,
        "Assertion Failed due to: {:?}",
        String::from_utf8(body.to_vec()).unwrap()
    );

    let body: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(
        body,
        json!({ "data": {"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"} })
    );
}

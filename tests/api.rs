use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use hyper::http;
use serde_json::{json, Value};
use synthesizer::{api, database};
use tower::ServiceExt; // for `oneshot` and `ready`

pub async fn setupdb() {
    database::reset_database().await.unwrap();
    database::run_migrations().await.unwrap();
}

mod generic {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn health_success() {
        let app = api::endpoints::create_api_router();

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
        let app = api::endpoints::create_api_router();

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
}

mod pipelines {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn list_pipelines_success() {
        let app = api::endpoints::create_api_router();

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
    async fn create_one_pipeline_success() {
        let app = api::endpoints::create_api_router();
        setupdb().await;

        // Build the request
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/pipelines")
            .header("content-type", "application/json")
            // TODO: Switch the id to a UUID to prevent test collisions
            .body(Body::from(
                r#"[{"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"}]"#,
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
            json!({ "data": [{"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"}] })
        );
    }

    #[tokio::test]
    async fn create_multi_pipeline_success() {
        let app = api::endpoints::create_api_router();
        setupdb().await;

        // Build the request
        let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/pipelines")
        .header("content-type", "application/json")
        // TODO: Switch the id to a UUID to prevent test collisions
        .body(Body::from(
            r#"[{"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"},{"id": "testpipeline2", "name": "Test Pipeline 2", "schedule": "1 * * * *"}]"#,
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
            json!({ "data": [{"id": "testpipeline", "name": "Test Pipeline", "schedule": "1 * * * *"},{"id": "testpipeline2", "name": "Test Pipeline 2", "schedule": "1 * * * *"}] })
        );
    }
}

mod tasks {
    use super::*;
    use pretty_assertions::assert_eq;

    #[tokio::test]
    async fn create_one_task_success() {
        let app = api::endpoints::create_api_router();
        setupdb().await;

        // Build the request
        let request = Request::builder()
            .method(http::Method::POST)
            .uri("/api/tasks")
            .header("content-type", "application/json")
            // TODO: Switch the id to a UUID to prevent test collisions
            .body(Body::from(
                r#"[{"name": "testtask", "pipeline_id": "testpipeline", "command": "echo 1"}]"#,
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
            json!({ "data": [{"name": "testtask", "pipeline_id": "testpipeline", "command": "echo 1"}] })
        );
    }

    #[tokio::test]
    async fn create_multi_task_success() {
        let app = api::endpoints::create_api_router();
        setupdb().await;

        // Build the request
        let request = Request::builder()
        .method(http::Method::POST)
        .uri("/api/tasks")
        .header("content-type", "application/json")
        .body(Body::from(
            r#"[{"name": "testtask2", "pipeline_id": "testpipeline", "command": "echo 1"},{"name": "testtask3", "pipeline_id": "testpipeline", "command": "echo 2"}]"#,
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
            json!({ "data": [{"name": "testtask2", "pipeline_id": "testpipeline", "command": "echo 1"},{"name": "testtask3", "pipeline_id": "testpipeline", "command": "echo 2"}] })
        );
    }

    #[tokio::test]
    async fn list_tasks_success() {
        let app = api::endpoints::create_api_router();

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/api/tasks")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
}

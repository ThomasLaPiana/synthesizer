use axum::response::Response;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{json, Value};

/// Create a router with all of the endpoints used by the Games service
pub fn create_pipeline_router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/api/health", get(health))
        .route("/api/pipelines", get(list_pipelines))
        .route("/api/endpoints", get(list_endpoints))
}

async fn root() -> Json<Value> {
    Json(json!({
        "data": "You've reached the Synthesizer server!"
    }))
}

async fn health() -> Json<Value> {
    Json(json!({
        "data": "Feeling healthy!"
    }))
}

/// Show all of the available routes for the server
async fn list_endpoints() -> Json<Value> {
    let router = create_pipeline_router();

    Json(json!({"endpoints": "Some stuff"}))
}

/// Return a list of all pipelines
async fn list_pipelines() -> Response {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        http::{Request, StatusCode},
    };
    use serde_json::{json, Value};
    use tower::ServiceExt; // for `oneshot` and `ready`

    #[tokio::test]
    async fn test_health() {
        let app = create_pipeline_router();

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
}

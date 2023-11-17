use axum::response::Response;
use axum::routing::{get, method_routing::MethodRouter};
use axum::{Json, Router};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Explicitly list all routes in a map that is used to generate
/// the real router as well as documentation.
pub fn get_endpoint_map() -> HashMap<String, MethodRouter> {
    let mut endpoints = HashMap::new();

    endpoints.insert("/".to_owned(), get(root));
    endpoints.insert("/api/health".to_owned(), get(health));
    endpoints.insert("/api/pipelines".to_owned(), get(list_pipelines));
    endpoints.insert("/api/endpoints".to_owned(), get(list_endpoints));

    endpoints
}

/// Automatically generate a router based off of the endpoint map
pub fn create_pipeline_router() -> Router {
    let mut router = Router::new();

    for (path, handler) in get_endpoint_map() {
        router = router.route(&path, handler);
    }
    router
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
    let paths: Vec<String> = get_endpoint_map()
        .keys()
        .map(|path| path.to_owned())
        .collect();

    Json(json!({"endpoint_paths": paths}))
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

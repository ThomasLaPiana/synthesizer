use axum::response::Response;
use axum::routing::{get, method_routing::MethodRouter, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use std::collections::HashMap;

/// Explicitly list all routes in a map that is used to generate
/// the real router as well as documentation.
pub fn get_endpoint_map() -> HashMap<&'static str, MethodRouter> {
    let mut endpoints = HashMap::new();

    // Generic
    endpoints.insert("/", get(root));
    endpoints.insert("/api/health", get(health));
    endpoints.insert("/api/endpoints", get(list_endpoints));

    // Pipelines
    endpoints.insert("/api/pipelines", get(list_pipelines));
    endpoints.insert("/api/pipelines", post(create_pipelines));

    endpoints
}

/// Automatically generate a router based off of the endpoint map
pub fn create_pipeline_router() -> Router {
    let mut router = Router::new();

    for (path, handler) in get_endpoint_map() {
        router = router.route(path, handler);
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
    let paths: Vec<&str> = get_endpoint_map().keys().cloned().collect();

    Json(json!({"endpoint_paths": paths}))
}

/// Return a list of all pipelines
async fn list_pipelines() -> Response {
    todo!()
}

/// Return a list of all pipelines
async fn create_pipelines() -> Response {
    todo!()
}

use crate::database;
use crate::models::Pipeline;
use axum::response::{IntoResponse, Response};
use axum::routing::{get, method_routing::MethodRouter, post};
use axum::{extract, Json, Router};
use hyper::http::Method;
use hyper::StatusCode;
use serde_json::{json, Value};

#[derive(Debug, Default)]
pub struct Endpoint {
    path: &'static str,
    method: Method,
    handler: MethodRouter,
}
pub trait CustomDisplay {
    fn concise_display(&self) -> String;
}
impl CustomDisplay for Endpoint {
    fn concise_display(&self) -> String {
        format!("{} {}", self.method.as_str(), self.path)
    }
}

/// Explicitly list all routes in a map that is used to generate
/// the real router as well as documentation.
pub fn get_endpoints() -> Vec<Endpoint> {
    vec![
        Endpoint {
            path: "/",
            method: Method::GET,
            handler: get(root),
        },
        Endpoint {
            path: "/api/health",
            method: Method::GET,
            handler: get(health),
        },
        Endpoint {
            path: "/api/endpoints",
            method: Method::GET,
            handler: get(list_endpoints),
        },
        Endpoint {
            path: "/api/pipelines",
            method: Method::GET,
            handler: get(list_pipelines),
        },
        Endpoint {
            path: "/api/pipelines",
            method: Method::POST,
            handler: post(create_pipeline),
        },
    ]
}

/// Automatically generate a router based off of the endpoint map
pub fn create_pipeline_router() -> Router {
    let mut router = Router::new();

    for endpoint in get_endpoints() {
        router = router.route(endpoint.path, endpoint.handler);
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
    let endpoints: Vec<String> = get_endpoints()
        .iter()
        .map(|endpoint| endpoint.concise_display())
        .collect();

    Json(json!({"endpoint_paths": endpoints}))
}

/// Return a list of all pipelines
async fn list_pipelines() -> Response {
    let mut db = database::get_db_connection().await.unwrap();
    let pipelines = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
        .fetch_all(&mut db)
        .await
        .unwrap();

    (StatusCode::OK, Json(json!({ "data": pipelines}))).into_response()
}

/// Return a list of all pipelines
async fn create_pipeline(extract::Json(payload): extract::Json<Pipeline>) -> Response {
    let mut db = database::get_db_connection().await.unwrap();
    sqlx::query!(
        "INSERT INTO pipelines VALUES(?, ?, ?)",
        payload.id,
        payload.name,
        payload.schedule,
    )
    .execute(&mut db)
    .await
    .unwrap();

    (StatusCode::CREATED, Json(json!({ "data": payload }))).into_response()
}

use crate::models::{Pipeline, Task};
use serde_json::json;
use sqlx::SqlitePool;
use std::net::TcpListener;

use actix_web::dev::Server;
use actix_web::{http::Method, web, App, HttpResponse, HttpServer, Route};

/// Struct used to describe available endpoints.
pub struct Endpoint {
    path: &'static str,
    method: Method,
    route: Route,
}

pub trait CustomDisplay {
    fn concise_display(&self) -> String;
}
/// TODO: Rename to Display and implement as the standard trait
impl CustomDisplay for Endpoint {
    fn concise_display(&self) -> String {
        format!("{} {}", self.method.as_str(), self.path)
    }
}

/// Explicitly list all routes in the application.
///
/// This is used as a way to transparently document available endpoints.
pub fn get_endpoints() -> Vec<Endpoint> {
    vec![
        // Generic
        Endpoint {
            path: "/api/health",
            method: Method::GET,
            route: web::get().to(health),
        },
        Endpoint {
            path: "/api/endpoints",
            method: Method::GET,
            route: web::get().to(list_endpoints),
        },
        // Pipelines
        Endpoint {
            path: "/api/pipelines",
            method: Method::GET,
            route: web::get().to(list_pipelines),
        },
        Endpoint {
            path: "/api/pipelines",
            method: Method::POST,
            route: web::post().to(create_pipeline),
        },
        // Tasks
        Endpoint {
            path: "/api/tasks",
            method: Method::GET,
            route: web::get().to(list_tasks),
        },
        Endpoint {
            path: "/api/tasks",
            method: Method::POST,
            route: web::post().to(create_task),
        },
    ]
}

/// Standard health endpoint
async fn health() -> HttpResponse {
    HttpResponse::Ok().finish()
}

/// Show all of the available routes for the server
async fn list_endpoints() -> HttpResponse {
    let endpoints: Vec<String> = get_endpoints()
        .iter()
        .map(|endpoint| endpoint.concise_display())
        .collect();

    let json_data = json!({"endpoint_paths": endpoints});
    HttpResponse::Ok().json(web::Json(json_data))
}

/// Return a list of all pipelines
async fn list_pipelines(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let pipelines = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let json_data = json!({"data": pipelines});
    HttpResponse::Ok().json(web::Json(json_data))
}

/// Create a Pipeline
async fn create_pipeline(
    pipeline: web::Json<Pipeline>,
    db_pool: web::Data<SqlitePool>,
) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO pipelines VALUES(?, ?, ?)",
        pipeline.id,
        pipeline.name,
        pipeline.schedule,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().json(web::Json(pipeline))
}

/// Return a list of all tasks
async fn list_tasks(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let json_data = json!({"data": tasks});
    HttpResponse::Ok().json(web::Json(json_data))
}

/// Create a Task
async fn create_task(task: web::Json<Task>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO tasks VALUES(?, ?, ?)",
        task.pipeline_id,
        task.name,
        task.command,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().json(web::Json(task))
}

/// Configure and return a Server instance to be awaited
pub fn run(listener: TcpListener, pool: SqlitePool) -> Result<Server, anyhow::Error> {
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        // Build the App from the endpoint vector
        let mut app = App::new().app_data(pool.clone());
        for endpoint in get_endpoints() {
            app = app.route(endpoint.path, endpoint.route);
        }
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

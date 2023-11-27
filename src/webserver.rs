use crate::models::{Pipeline, Task};
use actix_web::dev::Server;
use actix_web::{http::Method, web, App, HttpResponse, HttpServer, Route};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::fmt;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, registry::Registry, EnvFilter};

/// Struct used to describe available endpoints.
pub struct Endpoint {
    path: &'static str,
    method: Method,
    route: Route,
}
impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.method.as_str(), self.path)
    }
}

/// Generic Struct used for API Responses
#[derive(Deserialize, Serialize, PartialEq, Debug)]
pub struct JSONResponse<T> {
    pub data: Option<Vec<T>>,
    pub errors: Option<Vec<String>>,
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
            path: "/api/pipelines/{id}",
            method: Method::GET,
            route: web::get().to(get_pipeline),
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
        Endpoint {
            path: "/api/tasks/{id}",
            method: Method::GET,
            route: web::get().to(get_task),
        },
    ]
}

/// Standard health endpoint
async fn health() -> HttpResponse {
    let response_data = JSONResponse::<String> {
        data: Some(vec!["Feeling healthy!".to_string()]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Show all of the available routes for the server
async fn list_endpoints() -> HttpResponse {
    let endpoints: Vec<String> = get_endpoints()
        .iter()
        .map(|endpoint| endpoint.to_string())
        .collect();

    let response_data = JSONResponse::<String> {
        data: Some(endpoints),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Return a list of all pipelines
async fn list_pipelines(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let pipelines = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Pipeline> {
        data: Some(pipelines),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Get a specific Pipeline
async fn get_pipeline(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let pipeline = sqlx::query_as!(Pipeline, "SELECT * FROM pipelines WHERE id = ?", id)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Pipeline> {
        data: Some(vec![pipeline]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Create a Pipeline
async fn create_pipeline(
    pipeline: web::Json<Pipeline>,
    db_pool: web::Data<SqlitePool>,
) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO pipelines (id, schedule) VALUES(?, ?)",
        pipeline.id,
        pipeline.schedule,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().finish()
}

/// Return a list of all Tasks
async fn list_tasks(db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let tasks = sqlx::query_as!(Task, "SELECT * FROM tasks")
        .fetch_all(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Task> {
        data: Some(tasks),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Get a specific Task
async fn get_task(path: web::Path<String>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    let id = path.to_string();
    let task = sqlx::query_as!(Task, "SELECT * FROM tasks WHERE id = ?", id)
        .fetch_one(db_pool.get_ref())
        .await
        .unwrap();

    let response_data = JSONResponse::<Task> {
        data: Some(vec![task]),
        errors: None,
    };
    HttpResponse::Ok().json(response_data)
}

/// Create a Task
async fn create_task(task: web::Json<Task>, db_pool: web::Data<SqlitePool>) -> HttpResponse {
    sqlx::query!(
        "INSERT INTO tasks (id, pipeline_id, command) VALUES(?, ?, ?)",
        task.id,
        task.pipeline_id,
        task.command,
    )
    .execute(db_pool.get_ref())
    .await
    .unwrap();

    HttpResponse::Created().finish()
}

/// Configure and return a Server instance to be awaited
pub fn run(listener: TcpListener, pool: SqlitePool) -> Result<Server, anyhow::Error> {
    // Configure the log Format
    let format_layer = tracing_subscriber::fmt::layer().with_target(false);
    // Make the logs configurable via the ENV
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    // Init logging and tracing
    let subscriber = Registry::default().with(filter_layer).with(format_layer);
    LogTracer::init().expect("Failed to set logger!");
    set_global_default(subscriber).expect("Failed to set subscriber!");

    // Configure DB Pool
    let pool = web::Data::new(pool);

    let server = HttpServer::new(move || {
        // Build the App from the endpoint vector
        let mut app = App::new()
            .app_data(pool.clone())
            // Enable tracing spans within handlers
            .wrap(TracingLogger::default());
        for endpoint in get_endpoints() {
            app = app.route(endpoint.path, endpoint.route);
        }
        app
    })
    .listen(listener)?
    .run();
    Ok(server)
}

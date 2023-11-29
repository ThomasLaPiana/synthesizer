use super::models::JSONResponse;
use super::pipeline_endpoints::{create_pipeline, get_pipeline, list_pipelines};
use super::task_endpoints::{create_task, get_task, list_tasks};
use actix_web::{http::Method, web, HttpResponse, Route};
use std::fmt;

/// Struct used to describe available endpoints.
pub struct Endpoint {
    pub path: &'static str,
    pub method: Method,
    pub route: Route,
}
impl fmt::Display for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.method.as_str(), self.path)
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

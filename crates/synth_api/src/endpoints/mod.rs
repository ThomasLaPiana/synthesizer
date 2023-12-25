mod pipelines;
mod task_instances;
mod tasks;
mod utility;

use crate::models::JSONResponse;
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
            route: web::get().to(utility::health),
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
            route: web::get().to(pipelines::list),
        },
        Endpoint {
            path: "/api/pipelines/{id}",
            method: Method::GET,
            route: web::get().to(pipelines::get),
        },
        Endpoint {
            path: "/api/pipelines",
            method: Method::POST,
            route: web::post().to(pipelines::create),
        },
        // Tasks
        Endpoint {
            path: "/api/tasks",
            method: Method::GET,
            route: web::get().to(tasks::list),
        },
        Endpoint {
            path: "/api/tasks",
            method: Method::POST,
            route: web::post().to(tasks::create),
        },
        Endpoint {
            path: "/api/tasks/{id}",
            method: Method::GET,
            route: web::get().to(tasks::get),
        },
        // Task Instances
        Endpoint {
            path: "/api/task_instances",
            method: Method::GET,
            route: web::get().to(task_instances::list),
        },
        Endpoint {
            path: "/api/task_instances/{id}",
            method: Method::GET,
            route: web::get().to(task_instances::get),
        },
    ]
}

/// Show all of the available routes for the server
pub async fn list_endpoints() -> HttpResponse {
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

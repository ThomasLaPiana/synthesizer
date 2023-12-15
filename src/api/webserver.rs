use super::endpoints::get_endpoints;
use crate::common::{database, telemetry};
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

/// Configure and return a Server instance to be awaited
pub fn run_webserver(listener: TcpListener, pool: SqlitePool) -> Result<Server, anyhow::Error> {
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

/// Setup for the webserver
pub async fn start_webserver() {
    telemetry::init_logging();
    let api_pool = database::get_db_pool().await;

    // Prepare values to configure the server
    let server_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(server_address).expect("Failed to bind port!");

    // Run the server
    println!("> Starting the webserver at address: {}", server_address);
    run_webserver(listener, api_pool).unwrap().await.unwrap();
}

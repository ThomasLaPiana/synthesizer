use super::endpoints::get_endpoints;
use super::scheduler::run_scheduler;
use crate::common::config;
use crate::common::config::BuildUrl;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use sqlx::SqlitePool;
use std::net::TcpListener;
use tracing::subscriber::set_global_default;
use tracing_actix_web::TracingLogger;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, registry::Registry, EnvFilter};

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

/// Initialize Logging and Tracing
fn init_logging() {
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
}

/// Setup for the webserver
pub async fn start_webserver() {
    let config = config::load_config("synth.toml").expect("Failed to load the config!");
    init_logging();

    // Init the db pool
    let scheduler_pool = SqlitePool::connect(&config.database.build_url())
        .await
        .expect("Failed to create the database pool!");
    let api_pool = scheduler_pool.clone();

    // Prepare values to configure the server
    let server_address = "127.0.0.1:8080";
    let listener = TcpListener::bind(server_address).expect("Failed to bind port!");

    // Start the scheduler
    println!("> Starting the Scheduler thread...");
    tokio::spawn(async {
        run_scheduler(scheduler_pool).await;
    });

    // Run the server
    println!("> Starting the webserver at address: {}", server_address);
    run_webserver(listener, api_pool).unwrap().await.unwrap();
}

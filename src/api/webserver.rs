use crate::api::endpoints;
use axum::Router;
use tokio;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

/// Create the full Application
pub fn create_app() -> Router {
    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    println!("> Building Router(s)...");
    let pipeline_api = endpoints::create_api_router()
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    Router::new().merge(pipeline_api)
}

async fn scheduler() {
    loop {
        // sleep
        let sleep_duration = tokio::time::Duration::from_secs(2);
        tokio::time::sleep(sleep_duration).await;
        println!("Hello from the scheduler!");
    }
}

/// Run all server setup logic and start the server
#[tokio::main(flavor = "current_thread")]
async fn start() -> anyhow::Result<()> {
    println!("> Checking Database...");
    // TODO: Use connection pooling

    // Spawn a thread for the Scheduler
    tokio::spawn(async {
        scheduler().await;
    });

    let app = create_app();
    let server_address = "127.0.0.1:8080";
    println!("> Starting server at address 'http://{server_address}'...");
    axum::Server::bind(&server_address.parse().unwrap())
        .serve(app.into_make_service())
        .await?;

    Ok(())
}

/// Wrapper function for starting the server
pub fn start_webserver() {
    let result = start();

    if let Some(err) = result.err() {
        eprintln!("Error: {err}")
    }
}

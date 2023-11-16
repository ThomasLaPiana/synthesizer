use crate::api::endpoints;
use crate::database;
use axum::Router;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

/// Run all server setup logic and start the server
#[tokio::main(flavor = "current_thread")]
async fn start() -> anyhow::Result<()> {
    println!("> Getting database...");

    let database = database::get_database();
    println!("Database: {:?}", database);
    database::add_to_database();
    println!("Database: {:?}", database);

    // TODO: Use connection pooling

    // Set tracing for logs
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    println!("> Building Routers...");
    let pipeline_api = endpoints::create_pipeline_router()
        // Add Logging
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let app = Router::new().merge(pipeline_api);

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
        println!("Error: {err}")
    }
}

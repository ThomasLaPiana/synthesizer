use sqlx::SqlitePool;
use std::net::TcpListener;
use synthesizer::common::config::{self, BuildUrl};
use synthesizer::server::{database, webserver};
use uuid::Uuid;

/// Spawn an application instance on a random, available
/// port and return the address. The application instance
/// will automatically be destroyed and cleaned when the
/// process ends.
pub async fn spawn_app() -> String {
    // Init values for configuration
    let mut config = config::load_config("synth.toml").expect("Failed to load configuration!");
    let listener =
        TcpListener::bind("127.0.0.1:0").expect("Failed to bind to a random, available port!");
    let port = listener.local_addr().unwrap().port();
    config.database.database = format!("test-{}", Uuid::new_v4());
    let db_url = &config.database.build_url();

    // Prepare the database and pool
    database::setupdb(db_url)
        .await
        .expect("Failed to setup test database!");
    let db_pool = SqlitePool::connect(db_url)
        .await
        .expect("Failed to create the database pool!");

    // Run the application instance
    let _ = tokio::spawn(webserver::run_webserver(listener, db_pool).unwrap());
    format!("http://127.0.0.1:{}", port)
}

use crate::common::config;
use crate::common::config::BuildUrl;
use sqlx::SqlitePool;
use sqlx::{migrate::MigrateDatabase, sqlite, ConnectOptions};
use std::str::FromStr;

/// Create a database pool and return it
pub async fn get_db_pool() -> SqlitePool {
    let config = config::load_config("synth.toml").expect("Failed to load the config!");
    SqlitePool::connect(&config.database.build_url())
        .await
        .expect("Failed to create the database pool!")
}

/// Create the Database if needed and run migrations
pub async fn setupdb(db_path: &str) -> Result<(), sqlx::Error> {
    // Create the database if it doesn't exist
    let database_exists = sqlx::Sqlite::database_exists(db_path)
        .await
        .unwrap_or(false);

    if !database_exists {
        match sqlx::Sqlite::create_database(db_path).await {
            Ok(_) => println!("> Database created."),
            Err(e) => println!("> Error creating database: {}", e),
        }
    } else {
        println!("> Database already exists! Exiting...");
    }

    // Run Migrations
    let mut db_conn = sqlite::SqliteConnectOptions::from_str(db_path)?
        .connect()
        .await?;

    sqlx::migrate!("src/migrations").run(&mut db_conn).await?;
    Ok(())
}

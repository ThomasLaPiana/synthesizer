use sqlx::{migrate::MigrateDatabase, sqlite, ConnectOptions};
use std::str::FromStr;

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

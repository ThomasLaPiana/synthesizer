pub mod cli;
pub mod commands;
pub mod config;
pub mod database;
pub mod manifests;
pub mod models;
pub mod utils;
pub mod webserver;

use sqlx::SqlitePool;
use std::net::TcpListener;

use crate::config::BuildUrl;

pub async fn synthesizer() {
    // Parse and validate the CLI
    let cli = cli::cli_builder().get_matches();
    let config_path = cli.get_one::<String>("config-path").unwrap();
    let config = config::load_config(config_path).expect("Failed to load the config!");
    let server_url = config.server.build_url();

    match cli.subcommand() {
        Some(("server", _)) => {
            // Init the db pool
            let db_pool = SqlitePool::connect(&config.database.url)
                .await
                .expect("Failed to create the database pool!");

            // Prepare values to configure the server
            let server_address = "127.0.0.1:8080";
            let listener = TcpListener::bind(server_address).expect("Failed to bind port!");

            // Run the server
            println!("> Starting the webserver at address: {}", server_address);
            webserver::run(listener, db_pool).unwrap().await.unwrap();
        }
        Some(("check", sub_matches)) => {
            let manifest = commands::check(sub_matches);
            println!(
                "> Successfully parsed {} pipeline(s)!",
                manifest.pipelines.len()
            );
        }
        Some(("config", _)) => println!("> Config Values:\n{:#?}", config),
        Some(("status", _)) => {
            if utils::check_url_reachable_and_success(&server_url) {
                println! {"Server is reachable!"}
            }
        }
        Some(("register", sub_matches)) => {
            let manifest = commands::check(sub_matches);
            cli::register(&server_url, manifest);
        }
        _ => unreachable!("'subcommand_required' prevents 'None'"),
    }
}

pub mod api;
pub mod cli;
pub mod commands;
pub mod config;
pub mod database;
pub mod manifests;
pub mod models;
pub mod utils;

use crate::config::BuildUrl;

pub fn synthesizer() {
    // Parse and validate the CLI
    let cli = cli::cli_builder().get_matches();
    let config_path = cli.get_one::<String>("config-path").unwrap();
    let config = config::load_config(config_path);
    let server_url = config.server.build_url();

    match cli.subcommand() {
        Some(("server", _)) => api::webserver::start_webserver(),
        Some(("check", sub_matches)) => {
            let manifest = commands::check(sub_matches);
            println!(
                "> Successfully parsed {} pipeline(s)!",
                manifest.pipelines.len()
            )
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
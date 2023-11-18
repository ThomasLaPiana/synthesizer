pub mod api;
pub mod cli;
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
        Some(("check", _)) => {
            let pipelines =
                manifests::parse_manifest_file(utils::load_file("./data/pipelines.yaml"));
            println!(
                "> Successfully parsed {} pipeline(s)!",
                pipelines.pipelines.len()
            )
        }
        Some(("config", _)) => println!("> Config Values:\n{:#?}", config),
        Some(("status", _)) => {
            if utils::check_url_reachable_and_success(&server_url) {
                println! {"Server is reachable!"}
            }
        }
        Some(("register", sub_matches)) => {
            let filepath = sub_matches.get_one::<String>("filepath").unwrap();
            let raw_manifest = utils::load_file(filepath);
            let manifest = manifests::parse_manifest_file(raw_manifest);
            cli::register(&server_url, manifest);
        }
        _ => unreachable!("'subcommand_required' prevents 'None'"),
    }
}

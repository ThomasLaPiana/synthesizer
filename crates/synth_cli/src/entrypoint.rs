use super::{commands, entrypoint, utils};
use clap::{crate_version, Arg, Command};
use reqwest::blocking::Client;
use serde_json::json;
use synth_common::config::{load_config, BuildUrl};
use synth_common::database;
use synth_common::models::Manifest;

/// Construct the CLI
pub fn cli_builder() -> Command {
    // Reusable arg for the path to a manifest file
    let manifest_filepath = Arg::new("filepath")
        .long("file")
        .short('f')
        .default_value("./data/synth_manifest.yml")
        .help("Path to a Synthesizer Pipelines manifest file.");

    // Build the CLI entrypoint and its subcommands.
    // For ease-of-use, alphabetize the commands!
    Command::new("Synthesizer")
        .propagate_version(true)
        .subcommand_required(true)
        .about("Synthesizer: The Sleek Workflow Orchestrator")
        .version(crate_version!())
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("config-path")
                .long("file")
                .short('f')
                .default_value("synth.toml")
                .help("Path to a Synthesizer Config File"),
        )
        // Add Subcommands
        .subcommand(
            Command::new("check")
                .about("Check that Synthesizer files are valid.")
                .arg(&manifest_filepath),
        )
        .subcommand(Command::new("config").about("Show the config values that are being used."))
        .subcommand(
            Command::new("ls")
                .about("List resources from the server.")
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("register")
                .about("Upsert pipeline(s) to the server.")
                .arg(&manifest_filepath),
        )
        .subcommand(Command::new("scheduler").about("Start the Synth scheduler."))
        .subcommand(Command::new("setupdb").about("Create the database and run migrations."))
        .subcommand(Command::new("status").about("Ping the webserver."))
        .subcommand(Command::new("webserver").about("Start the Synth API Webserver."))
}

/// Send the manifest to the server
pub fn register(url: &str, manifest: Manifest) -> bool {
    let json_data = json!(manifest);
    let client = Client::new();
    let res = client.post(url).json(&json_data).send().unwrap();
    println!("{:?}", res.status());
    true
}

pub async fn run() {
    // Parse and validate the CLI
    let cli = entrypoint::cli_builder().get_matches();
    let config_path = cli.get_one::<String>("config-path").unwrap();
    let config = load_config(config_path).expect("Failed to load the config!");
    let server_url = config.server.build_url();

    match cli.subcommand() {
        Some(("setupdb", _)) => {
            // Run the server
            database::setupdb(&config.database.build_url())
                .await
                .expect("Failed to setup the database!");
        }
        Some(("check", sub_matches)) => {
            let manifest = commands::check(sub_matches);
            println!(
                "> Successfully parsed {} pipeline(s)!",
                manifest.pipelines.len()
            );
        }
        Some(("config", _)) => println!("> Config Values:\n{:#?}", config),
        Some(("webserver", _)) => synth_api::start().await,
        Some(("scheduler", _)) => synth_scheduler::start().await,
        Some(("status", _)) => {
            if utils::check_url_reachable_and_success(&server_url) {
                println! {"Server is reachable!"}
            }
        }
        Some(("register", sub_matches)) => {
            let manifest = commands::check(sub_matches);
            entrypoint::register(&server_url, manifest);
        }
        _ => unreachable!("'subcommand_required' prevents 'None'"),
    }
}

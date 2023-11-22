use crate::models::Manifest;
use clap::{crate_version, Arg, Command};
use reqwest::blocking::Client;
use serde_json::json;

/// Construct the CLI
pub fn cli_builder() -> Command {
    // Reusable arg for the path to a manifest file
    let manifest_filepath = Arg::new("filepath")
        .long("file")
        .short('f')
        .default_value("./data/synth_manifest.yml")
        .help("Path to a Synthesizer Pipelines manifest file.");

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
        .subcommand(
            Command::new("check")
                .about("Check that Synthesizer files are valid.")
                .arg(&manifest_filepath),
        )
        .subcommand(Command::new("config").about("Show the config values that are being used."))
        .subcommand(Command::new("server").about("Run the server component."))
        .subcommand(Command::new("status").about("Ping the server."))
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
}

/// Send the manifest to the server
pub fn register(url: &str, manifest: Manifest) -> bool {
    let json_data = json!(manifest);
    let client = Client::new();
    let res = client.post(url).json(&json_data).send().unwrap();
    println!("{:?}", res.status());
    true
}

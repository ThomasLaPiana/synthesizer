use crate::models::Pipelines;
use clap::{crate_version, Arg, Command};
use reqwest::blocking::Client;
use serde_json::json;

/// Construct the CLI
pub fn cli_builder() -> Command {
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
        .subcommand(Command::new("check").about("Check that Synthesizer files are valid."))
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
                .arg(
                    Arg::new("filepath")
                        .long("file")
                        .short('f')
                        .default_value("./data/pipelines.yaml")
                        .help("Path to a Synthesizer Pipelines manifest file."),
                ),
        )
}

/// Send the manifest to the server
pub fn register(url: &str, manifest: Pipelines) -> bool {
    let json_data = json!(manifest);
    let client = Client::new();
    let res = client.post(url).json(&json_data).send().unwrap();
    println!("{:?}", res.status());
    true
}

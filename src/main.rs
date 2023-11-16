mod api;
mod cli;
mod config;
mod database;
mod manifests;
mod models;
mod utils;

fn main() {
    // Parse and validate the CLI
    let cli = cli::entrypoint::cli_builder().get_matches();
    let config_path = cli.get_one::<String>("config-path").unwrap();
    let config = config::load_config(config_path);

    match cli.subcommand_name().unwrap() {
        "server" => api::webserver::start_webserver(),
        "check" => {
            let pipelines =
                manifests::parse_manifest_file(utils::load_file("./data/pipelines.yaml"));
            println!(
                "> Successfully parsed {} pipeline(s)!",
                pipelines.pipelines.len()
            )
        }
        "config" => println!("> Config Values:\n{:#?}", config),
        command => {
            println!("> The command '{}' does not exist! Aborting...", command);
            std::process::exit(2);
        }
    }
}

mod api;
mod cli;
mod database;
mod models;

fn main() {
    let cli = cli::entrypoint::cli_builder().get_matches();
    match cli.subcommand_name().unwrap() {
        "server" => api::webserver::start_webserver(),
        command => {
            println!("'{}' is not a valid subcommand!", command);
            std::process::exit(2);
        }
    }
}

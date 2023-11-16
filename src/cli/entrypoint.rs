use clap::{crate_version, Arg, Command};

/// Construct the CLI
pub fn cli_builder() -> Command {
    Command::new("Synthesizer")
        .about("Synthesizer: The Sleek Workflow Orchestrator")
        .version(crate_version!())
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .arg(
            Arg::new("config-path")
                .long("file")
                .short('f')
                .default_value("synthesizer.toml")
                .help("Path to a Synthesizer Config File"),
        )
        .subcommand(Command::new("server").about("Run the server component."))
}

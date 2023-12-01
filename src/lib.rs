pub mod cli;
pub mod common;
pub mod server;

pub async fn synth() {
    cli::entrypoint::run().await;
}

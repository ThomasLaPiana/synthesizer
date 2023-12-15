use synthesizer::cli;

#[tokio::main]
async fn main() {
    cli::entrypoint::run().await;
}

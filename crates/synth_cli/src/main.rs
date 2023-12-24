use synth_cli::entrypoint;

#[tokio::main]
async fn main() {
    entrypoint::run().await;
}

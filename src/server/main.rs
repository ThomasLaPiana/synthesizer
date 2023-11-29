#[tokio::main]
pub async fn main() {
    synthesizer::server::webserver::start_webserver().await;
}

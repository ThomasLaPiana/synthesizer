use synthesizer::api;

#[tokio::main]
async fn main() {
    api::webserver::start_webserver().await;
}

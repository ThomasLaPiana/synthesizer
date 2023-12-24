use synth_api::webserver;

#[tokio::main]
async fn main() {
    webserver::start_webserver().await;
}

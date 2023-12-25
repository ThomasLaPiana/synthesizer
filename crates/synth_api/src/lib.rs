pub mod endpoints;
pub mod models;
pub mod webserver;

pub async fn start() {
    webserver::start_webserver().await;
}

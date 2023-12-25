pub mod api;
pub mod endpoints;
pub mod models;
pub mod views;
pub mod webserver;

pub async fn start() {
    webserver::start_webserver().await;
}

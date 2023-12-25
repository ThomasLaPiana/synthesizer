pub mod endpoints;
pub mod models;
pub mod pipeline_endpoints;
pub mod task_endpoints;
pub mod task_instance_endpoints;
pub mod webserver;

pub async fn start() {
    webserver::start_webserver().await;
}

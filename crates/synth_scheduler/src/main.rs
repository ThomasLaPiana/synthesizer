use synth_common::telemetry;
mod scheduler;

#[tokio::main]
async fn main() {
    telemetry::init_logging();
    scheduler::run_scheduler().await;
}

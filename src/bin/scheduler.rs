use synthesizer::common::telemetry;
use synthesizer::scheduler;

#[tokio::main]
async fn main() {
    telemetry::init_logging();
    scheduler::scheduler::run_scheduler().await;
}

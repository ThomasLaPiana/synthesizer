use synth_common::telemetry;
mod scheduler;

/// The Entrypoint for the Scheduler.
pub async fn start() {
    telemetry::init_logging();
    scheduler::run_scheduler().await;
}

use synthesizer::scheduler;

#[tokio::main]
async fn main() {
    scheduler::scheduler::run_scheduler().await;
}

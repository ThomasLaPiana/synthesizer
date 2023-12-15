use tracing::subscriber::set_global_default;
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, registry::Registry, EnvFilter};

/// Initialize Logging and Tracing
pub fn init_logging() {
    // Configure the log Format
    let format_layer = tracing_subscriber::fmt::layer().with_target(false);
    // Make the logs configurable via the ENV
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("info"))
        .unwrap();

    // Init logging and tracing
    let subscriber = Registry::default().with(filter_layer).with(format_layer);
    LogTracer::init().expect("Failed to set logger!");
    set_global_default(subscriber).expect("Failed to set subscriber!");
}

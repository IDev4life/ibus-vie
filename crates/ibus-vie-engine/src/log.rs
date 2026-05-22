use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the tracing subscriber with env filter (RUST_LOG).
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("debug"));

    let log_file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/ibus_vie_debug.log")
        .expect("failed to open log file");

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_writer(std::sync::Mutex::new(log_file))
        .init();
}

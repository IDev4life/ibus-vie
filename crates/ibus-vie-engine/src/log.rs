use tracing_subscriber::{fmt, EnvFilter};

/// Initialize the tracing subscriber with env filter (RUST_LOG).
pub fn init() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .with_target(true)
        .with_writer(std::io::stderr)
        .init();
}

pub mod connection;
pub mod engine_impl;
pub mod factory;
pub mod ibus_text;
pub mod signals;

use crate::config::Config;
use crate::error::EngineError;
use tracing::info;

/// Main entry point: connect to IBus daemon and register engine factory.
pub async fn run(config: Config) -> Result<(), EngineError> {
    let connection = connection::connect().await?;
    info!("connected to DBus session bus");

    factory::register(&connection, config).await?;
    info!("engine factory registered with IBus");

    // Keep running until terminated
    std::future::pending::<()>().await;
    Ok(())
}

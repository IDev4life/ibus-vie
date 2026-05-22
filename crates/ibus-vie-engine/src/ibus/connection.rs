use crate::error::EngineError;
use tracing::info;
use zbus::Connection;

/// Connect to the DBus session bus.
pub async fn connect() -> Result<Connection, EngineError> {
    let connection = Connection::session().await?;
    info!("DBus session connection established");
    Ok(connection)
}

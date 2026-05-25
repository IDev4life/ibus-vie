use std::sync::Mutex;

use crate::config::Config;
use crate::error::EngineError;
use crate::ibus::engine::IbusEngineImpl;
use crate::ibus::service::IbusService;
use tracing::info;
use zbus::Connection;

/// IBus Engine Factory interface path.
const FACTORY_PATH: &str = "/org/freedesktop/IBus/Factory";

/// Shared active method state across all engine instances.
/// Updated by property_activate, read by factory on CreateEngine.
static ACTIVE_METHOD: Mutex<Option<String>> = Mutex::new(None);


/// Get the current active method (returns None if never set).
pub fn get_active_method() -> Option<String> {
    ACTIVE_METHOD.lock().unwrap().clone()
}

/// Set the active method (called from property_activate).
pub fn set_active_method(method: &str) {
    *ACTIVE_METHOD.lock().unwrap() = Some(method.to_string());
}


/// Register the engine factory with the DBus connection.
///
/// IBus daemon will call CreateEngine on this factory when the user activates
/// one of our engines from Settings.
pub async fn register(connection: &Connection, config: Config) -> Result<(), EngineError> {
    let factory = EngineFactory::new(config);

    connection.object_server().at(FACTORY_PATH, factory).await?;

    info!("factory registered at {}", FACTORY_PATH);

    // Request the well-known bus name that IBus expects
    connection.request_name("org.freedesktop.IBus.Vie").await?;

    info!("bus name org.freedesktop.IBus.Vie acquired");
    Ok(())
}

/// The factory that IBus calls to create engine instances.
pub struct EngineFactory {
    #[allow(dead_code)]
    config: Config,
    engine_count: u32,
}

impl EngineFactory {
    fn new(config: Config) -> Self {
        Self {
            config,
            engine_count: 0,
        }
    }
}

/// DBus interface implementation for org.freedesktop.IBus.Factory.
#[zbus::interface(name = "org.freedesktop.IBus.Factory")]
impl EngineFactory {
    /// Called by ibus-daemon when it needs a new engine instance.
    async fn create_engine(
        &mut self,
        engine_name: &str,
        #[zbus(connection)] _connection: &Connection,
        #[zbus(object_server)] server: &zbus::ObjectServer,
    ) -> zbus::fdo::Result<zbus::zvariant::OwnedObjectPath> {
        self.engine_count += 1;
        let path = format!("/org/freedesktop/IBus/Engine/{}", self.engine_count);
        info!("creating engine '{}' at path {}", engine_name, path);

        let method = get_active_method().unwrap_or_else(|| self.config.method.clone());

        let engine = IbusEngineImpl::new(method);

        server
            .at(path.as_str(), engine)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(format!("failed to register engine: {}", e)))?;

        // IBus also calls Destroy on org.freedesktop.IBus.Service interface
        server
            .at(path.as_str(), IbusService)
            .await
            .map_err(|e| zbus::fdo::Error::Failed(format!("failed to register service: {}", e)))?;

        let object_path = zbus::zvariant::OwnedObjectPath::try_from(path)
            .map_err(|e| zbus::fdo::Error::Failed(format!("invalid path: {}", e)))?;

        Ok(object_path)
    }

    /// Destroy the factory.
    fn destroy(&self) {
        info!("factory destroy called");
    }
}

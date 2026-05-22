use thiserror::Error;

#[derive(Error, Debug)]
pub enum EngineError {
    #[error("DBus error: {0}")]
    Dbus(#[from] zbus::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Configuration error: {0}")]
    #[allow(dead_code)]
    Config(String),
}

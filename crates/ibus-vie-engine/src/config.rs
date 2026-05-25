use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tracing::warn;

/// Application configuration loaded from ~/.config/ibus-vie/config.toml
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct Config {
    /// Default input method: "telex" or "vni"
    pub method: String,
    /// Input mode: "preedit" (inline underline) or "popup" (floating candidate window)
    pub input_mode: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            method: "telex".to_string(),
            input_mode: "preedit".to_string(),
        }
    }
}

impl Config {
    /// Load config from the standard path, falling back to defaults.
    pub fn load() -> Self {
        let path = config_path();
        if !path.exists() {
            return Self::default();
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => match toml::from_str(&content) {
                Ok(config) => config,
                Err(e) => {
                    warn!("failed to parse config at {}: {}", path.display(), e);
                    Self::default()
                }
            },
            Err(e) => {
                warn!("failed to read config at {}: {}", path.display(), e);
                Self::default()
            }
        }
    }
}

/// Persist method and input_mode to config.toml.
/// Called when user switches via IBus property menu — updates default for future sessions.
pub fn save(method: &str, input_mode: &str) {
    let path = config_path();
    if let Some(parent) = path.parent() {
        if let Err(e) = std::fs::create_dir_all(parent) {
            warn!("failed to create config dir: {}", e);
            return;
        }
    }
    let cfg = Config { method: method.to_string(), input_mode: input_mode.to_string() };
    match toml::to_string_pretty(&cfg) {
        Ok(content) => {
            if let Err(e) = std::fs::write(&path, content) {
                warn!("failed to write config: {}", e);
            }
        }
        Err(e) => warn!("failed to serialize config: {}", e),
    }
}

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    config_dir.join("ibus-vie").join("config.toml")
}

use serde::Deserialize;
use std::path::PathBuf;
use tracing::warn;

/// Application configuration loaded from ~/.config/ibus-vie/config.toml
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default input method: "telex" or "vni"
    pub method: String,
    /// Tone placement style: "new" (hòa) or "old" (hoà)
    pub tone_style: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            method: "telex".to_string(),
            tone_style: "new".to_string(),
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

fn config_path() -> PathBuf {
    let config_dir = std::env::var("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
            PathBuf::from(home).join(".config")
        });
    config_dir.join("ibus-vie").join("config.toml")
}

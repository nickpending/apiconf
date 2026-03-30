use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::error::ConfigError;

#[derive(Clone, Serialize, Deserialize)]
pub struct Key {
    pub provider: String,
    pub value: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub env_var: Option<String>,
}

impl std::fmt::Debug for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Key")
            .field("provider", &self.provider)
            .field("value", &"[REDACTED]")
            .finish()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct App {
    #[serde(flatten)]
    pub providers: HashMap<String, String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub keys: HashMap<String, Key>,
    #[serde(default)]
    pub apps: HashMap<String, App>,
}

pub fn get_config_path() -> Result<PathBuf, ConfigError> {
    // Use ~/.config/apiconf/config.toml for cross-platform consistency
    let home = dirs::home_dir().ok_or(ConfigError::NoConfigDir)?;
    Ok(home.join(".config").join("apiconf").join("config.toml"))
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let path = get_config_path()?;

        if !path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&path).map_err(ConfigError::Read)?;
        let config: Config = toml::from_str(&content).map_err(ConfigError::Parse)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let path = get_config_path()?;

        // Create parent directories if missing
        if let Some(parent) = path.parent() {
            #[cfg(unix)]
            {
                use std::os::unix::fs::DirBuilderExt;
                fs::DirBuilder::new()
                    .recursive(true)
                    .mode(0o700)
                    .create(parent)
                    .map_err(ConfigError::Write)?;
            }
            #[cfg(not(unix))]
            {
                fs::create_dir_all(parent).map_err(ConfigError::Write)?;
            }
        }

        let content = toml::to_string_pretty(self).map_err(ConfigError::Serialize)?;

        // Atomic write: write to .tmp file, then rename
        let tmp_path = path.with_extension("toml.tmp");

        {
            // Create temp file with restricted permissions from the start (Unix)
            #[cfg(unix)]
            let mut file = {
                use std::os::unix::fs::OpenOptionsExt;
                fs::OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .mode(0o600)
                    .open(&tmp_path)
                    .map_err(ConfigError::Write)?
            };
            #[cfg(not(unix))]
            let mut file = fs::File::create(&tmp_path).map_err(ConfigError::Write)?;

            file.write_all(content.as_bytes())
                .map_err(ConfigError::Write)?;
            file.sync_all().map_err(ConfigError::Write)?;
        }

        // Atomic rename
        fs::rename(&tmp_path, &path).map_err(ConfigError::Write)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_path() {
        let path = get_config_path().unwrap();
        println!("Config path: {:?}", path);
        assert!(path.to_string_lossy().contains("apiconf"));
    }
}

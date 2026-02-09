use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

use crate::error::ConfigError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Key {
    pub provider: String,
    pub value: String,
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
            fs::create_dir_all(parent).map_err(ConfigError::Write)?;
        }

        let content = toml::to_string_pretty(self).map_err(ConfigError::Serialize)?;

        // Atomic write: write to .tmp file, then rename
        let tmp_path = path.with_extension("toml.tmp");

        {
            let mut file = File::create(&tmp_path).map_err(ConfigError::Write)?;
            file.write_all(content.as_bytes())
                .map_err(ConfigError::Write)?;
            file.sync_all().map_err(ConfigError::Write)?;
        }

        // Set permissions to 600 before rename (Unix only)
        #[cfg(unix)]
        {
            let permissions = fs::Permissions::from_mode(0o600);
            fs::set_permissions(&tmp_path, permissions).map_err(ConfigError::Write)?;
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

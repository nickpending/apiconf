use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file: {0}")]
    Read(#[source] std::io::Error),

    #[error("Failed to write config file: {0}")]
    Write(#[source] std::io::Error),

    #[error("Failed to parse config file: {0}")]
    Parse(#[source] toml::de::Error),

    #[error("Failed to serialize config: {0}")]
    Serialize(#[source] toml::ser::Error),

    #[error("Could not determine config directory")]
    NoConfigDir,
}

#[derive(Debug, Error)]
pub enum ApiconfError {
    #[error("Configuration error: {0}")]
    Config(#[from] ConfigError),

    #[error("Key '{0}' not found. Available keys: {1}")]
    KeyNotFound(String, String),

    #[error("App '{0}' not found. Available apps: {1}")]
    AppNotFound(String, String),

    #[error("Unknown provider '{0}'. Valid providers: {1}")]
    UnknownProvider(String, String),

    #[error("Key '{0}' already exists. Use --force to overwrite.")]
    KeyExists(String),

    #[error("App '{0}' already exists. Use --force to overwrite.")]
    AppExists(String),

    #[error("API key cannot be empty")]
    EmptyKey,

    #[error("API key contains invalid characters (NUL bytes)")]
    InvalidKey,
}

impl ApiconfError {
    pub fn exit_code(&self) -> i32 {
        match self {
            // User errors: exit code 1
            ApiconfError::KeyNotFound(_, _)
            | ApiconfError::AppNotFound(_, _)
            | ApiconfError::UnknownProvider(_, _)
            | ApiconfError::KeyExists(_)
            | ApiconfError::AppExists(_)
            | ApiconfError::EmptyKey
            | ApiconfError::InvalidKey => 1,

            // System errors: exit code 2
            ApiconfError::Config(_) => 2,
        }
    }
}

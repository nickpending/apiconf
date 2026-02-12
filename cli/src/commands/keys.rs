use crate::config::{Config, Key};
use crate::error::ApiconfError;
use crate::providers::{is_valid_provider, list_providers};

pub fn add(provider: &str, name: Option<&str>, force: bool) -> Result<(), ApiconfError> {
    // Validate provider
    if !is_valid_provider(provider) {
        let valid = list_providers().join(", ");
        return Err(ApiconfError::UnknownProvider(provider.to_string(), valid));
    }

    let key_name = name.unwrap_or(provider);
    let mut config = Config::load()?;

    // Check if key already exists
    if config.keys.contains_key(key_name) && !force {
        return Err(ApiconfError::KeyExists(key_name.to_string()));
    }

    // Prompt for the key value
    let value = rpassword::prompt_password(format!("Enter API key for {}: ", provider))
        .map_err(|e| ApiconfError::Config(crate::error::ConfigError::Read(e)))?;

    if value.is_empty() {
        return Err(ApiconfError::EmptyKey);
    }

    // Save the key
    config.keys.insert(
        key_name.to_string(),
        Key {
            provider: provider.to_string(),
            value,
        },
    );
    config.save()?;

    println!("Key '{}' added successfully.", key_name);
    Ok(())
}

pub fn list() -> Result<(), ApiconfError> {
    let config = Config::load()?;

    if config.keys.is_empty() {
        println!("No keys configured.");
        return Ok(());
    }

    for (name, key) in &config.keys {
        println!("{} ({})", name, key.provider);
    }

    Ok(())
}

pub fn remove(name: &str) -> Result<(), ApiconfError> {
    let mut config = Config::load()?;

    if !config.keys.contains_key(name) {
        let available = config.keys.keys().cloned().collect::<Vec<_>>().join(", ");
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available
        };
        return Err(ApiconfError::KeyNotFound(name.to_string(), available));
    }

    config.keys.remove(name);
    config.save()?;

    println!("Key '{}' removed successfully.", name);
    Ok(())
}

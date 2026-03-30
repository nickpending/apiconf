use crate::config::{App, Config};
use crate::error::ApiconfError;
use crate::providers::resolve_env_var;

pub fn create(name: &str) -> Result<(), ApiconfError> {
    let mut config = Config::load()?;

    if config.apps.contains_key(name) {
        return Err(ApiconfError::AppExists(name.to_string()));
    }

    config.apps.insert(name.to_string(), App::default());
    config.save()?;

    println!("App '{}' created successfully.", name);
    Ok(())
}

pub fn list() -> Result<(), ApiconfError> {
    let config = Config::load()?;

    if config.apps.is_empty() {
        println!("No apps configured.");
        return Ok(());
    }

    for (name, app) in &config.apps {
        let key_count = app.providers.len();
        let keys_word = if key_count == 1 { "key" } else { "keys" };
        println!("{} ({} {})", name, key_count, keys_word);
    }

    Ok(())
}

pub fn add(app_name: &str, provider: &str, key_name: Option<&str>) -> Result<(), ApiconfError> {
    let mut config = Config::load()?;

    // Validate app exists
    if !config.apps.contains_key(app_name) {
        let available = config.apps.keys().cloned().collect::<Vec<_>>().join(", ");
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available
        };
        return Err(ApiconfError::AppNotFound(app_name.to_string(), available));
    }

    // Determine key name
    let key = key_name.unwrap_or(provider);

    // Validate key exists
    if !config.keys.contains_key(key) {
        let available = config.keys.keys().cloned().collect::<Vec<_>>().join(", ");
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available
        };
        return Err(ApiconfError::KeyNotFound(key.to_string(), available));
    }

    // Add provider -> key mapping
    if let Some(app) = config.apps.get_mut(app_name) {
        app.providers.insert(provider.to_string(), key.to_string());
    }

    config.save()?;

    println!("Added {} -> {} to app '{}'.", provider, key, app_name);
    Ok(())
}

pub fn show(app_name: &str) -> Result<(), ApiconfError> {
    let config = Config::load()?;

    // Validate app exists
    if !config.apps.contains_key(app_name) {
        let available = config.apps.keys().cloned().collect::<Vec<_>>().join(", ");
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available
        };
        return Err(ApiconfError::AppNotFound(app_name.to_string(), available));
    }

    let app = &config.apps[app_name];

    println!("App: {}", app_name);

    if app.providers.is_empty() {
        println!("  (no keys configured)");
        return Ok(());
    }

    for (provider, key_name) in &app.providers {
        let explicit = config.keys.get(key_name).and_then(|k| k.env_var.as_deref());
        let env_var = resolve_env_var(provider, explicit);
        println!("  {} -> {} ({})", provider, key_name, env_var);
    }

    Ok(())
}

pub fn remove(app_name: &str) -> Result<(), ApiconfError> {
    let mut config = Config::load()?;

    if !config.apps.contains_key(app_name) {
        let available = config.apps.keys().cloned().collect::<Vec<_>>().join(", ");
        let available = if available.is_empty() {
            "none".to_string()
        } else {
            available
        };
        return Err(ApiconfError::AppNotFound(app_name.to_string(), available));
    }

    config.apps.remove(app_name);
    config.save()?;

    println!("App '{}' removed successfully.", app_name);
    Ok(())
}

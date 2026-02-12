use crate::config::Config;
use crate::error::ApiconfError;
use crate::providers::get_env_var;

/// Escape a string for safe use in shell single quotes.
/// Wraps value in single quotes and escapes internal single quotes with '\''
fn shell_escape(s: &str) -> String {
    let escaped = s.replace('\'', "'\\''");
    format!("'{}'", escaped)
}

pub fn export(app_name: &str) -> Result<(), ApiconfError> {
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

    for (provider, key_name) in &app.providers {
        // Look up the key value
        let key = match config.keys.get(key_name) {
            Some(k) => k,
            None => {
                eprintln!(
                    "Warning: Key '{}' referenced by app '{}' not found, skipping",
                    key_name, app_name
                );
                continue;
            }
        };

        // Get the env var name from the provider registry
        let env_var = match get_env_var(provider) {
            Some(v) => v,
            None => {
                eprintln!("Warning: No env var for provider '{}', skipping", provider);
                continue;
            }
        };

        // Output export statement
        println!("export {}={}", env_var, shell_escape(&key.value));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shell_escape_simple() {
        assert_eq!(shell_escape("sk-ant-xxx"), "'sk-ant-xxx'");
    }

    #[test]
    fn test_shell_escape_with_quotes() {
        assert_eq!(shell_escape("it's a key"), "'it'\\''s a key'");
    }

    #[test]
    fn test_shell_escape_empty() {
        assert_eq!(shell_escape(""), "''");
    }
}

use std::collections::HashMap;
use std::sync::LazyLock;

/// Known provider defaults — a hints table, not a validation gate.
/// Any string is a valid provider. Known providers get their conventional
/// env var from this table; unknown providers fall back to convention mapping.
static KNOWN_DEFAULTS: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("anthropic", "ANTHROPIC_API_KEY");
    m.insert("openai", "OPENAI_API_KEY");
    m.insert("google-gemini", "GOOGLE_API_KEY");
    m.insert("elevenlabs", "ELEVENLABS_API_KEY");
    m.insert("ollama", "OLLAMA_API_BASE");
    m
});

/// Resolve the environment variable name for a provider using three-tier resolution:
///   1. Explicit env_var (from TOML key config) — highest priority
///   2. Known defaults table
///   3. Convention: PROVIDER_NAME_API_KEY (hyphens become underscores)
pub fn resolve_env_var(provider: &str, explicit_env_var: Option<&str>) -> String {
    // Tier 1: explicit
    if let Some(env_var) = explicit_env_var {
        return env_var.to_string();
    }

    // Tier 2: known defaults
    if let Some(env_var) = KNOWN_DEFAULTS.get(provider) {
        return env_var.to_string();
    }

    // Tier 3: convention
    provider.replace('-', "_").to_uppercase() + "_API_KEY"
}

pub fn is_known_provider(name: &str) -> bool {
    KNOWN_DEFAULTS.contains_key(name)
}

pub fn list_known_providers() -> Vec<&'static str> {
    let mut providers: Vec<&'static str> = KNOWN_DEFAULTS.keys().copied().collect();
    providers.sort();
    providers
}

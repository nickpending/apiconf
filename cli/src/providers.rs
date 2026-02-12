use std::collections::HashMap;
use std::sync::LazyLock;

static PROVIDERS: LazyLock<HashMap<&'static str, Option<&'static str>>> = LazyLock::new(|| {
    let mut m = HashMap::new();
    m.insert("anthropic", Some("ANTHROPIC_API_KEY"));
    m.insert("openai", Some("OPENAI_API_KEY"));
    m.insert("google-gemini", Some("GOOGLE_API_KEY"));
    m.insert("elevenlabs", Some("ELEVENLABS_API_KEY"));
    m.insert("ollama", Some("OLLAMA_API_BASE"));
    m
});

pub fn get_env_var(provider: &str) -> Option<&'static str> {
    PROVIDERS.get(provider).copied().flatten()
}

pub fn is_valid_provider(name: &str) -> bool {
    PROVIDERS.contains_key(name)
}

pub fn list_providers() -> Vec<&'static str> {
    let mut providers: Vec<&'static str> = PROVIDERS.keys().copied().collect();
    providers.sort();
    providers
}

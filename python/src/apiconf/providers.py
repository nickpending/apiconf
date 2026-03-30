"""Provider registry mapping known provider names to default environment variables.

This is a hints table, not a validation gate. Any string is a valid provider.
Known providers get their conventional env var from this table; unknown
providers fall back to convention mapping (UPPERCASE + _API_KEY).
"""

from dataclasses import dataclass


@dataclass(frozen=True)
class ProviderInfo:
    """Information about a provider's environment variable."""

    env_var: str


KNOWN_DEFAULTS: dict[str, ProviderInfo] = {
    "anthropic": ProviderInfo(env_var="ANTHROPIC_API_KEY"),
    "openai": ProviderInfo(env_var="OPENAI_API_KEY"),
    "google-gemini": ProviderInfo(env_var="GOOGLE_API_KEY"),
    "elevenlabs": ProviderInfo(env_var="ELEVENLABS_API_KEY"),
    "ollama": ProviderInfo(env_var="OLLAMA_API_BASE"),
}


def resolve_env_var(provider: str, explicit_env_var: str | None = None) -> str:
    """Resolve the environment variable name for a provider using three-tier resolution.

    1. Explicit env_var (caller-provided) -- highest priority
    2. Known defaults table
    3. Convention: PROVIDER_NAME_API_KEY (hyphens become underscores)
    """
    # Tier 1: explicit
    if explicit_env_var:
        return explicit_env_var

    # Tier 2: known defaults
    info = KNOWN_DEFAULTS.get(provider)
    if info:
        return info.env_var

    # Tier 3: convention
    return provider.replace("-", "_").upper() + "_API_KEY"


def is_known_provider(name: str) -> bool:
    """Check if a provider name has a known default env var mapping."""
    return name in KNOWN_DEFAULTS


def list_providers() -> list[str]:
    """List all known provider names."""
    return sorted(KNOWN_DEFAULTS.keys())

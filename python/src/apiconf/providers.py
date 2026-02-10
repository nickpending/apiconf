"""Provider registry mapping provider names to environment variables."""

from dataclasses import dataclass


@dataclass(frozen=True)
class ProviderInfo:
    """Information about a provider's environment variable."""

    env_var: str | None


PROVIDERS: dict[str, ProviderInfo] = {
    "anthropic": ProviderInfo(env_var="ANTHROPIC_API_KEY"),
    "openai": ProviderInfo(env_var="OPENAI_API_KEY"),
    "google-gemini": ProviderInfo(env_var="GOOGLE_API_KEY"),
    "elevenlabs": ProviderInfo(env_var="ELEVENLABS_API_KEY"),
    "ollama": ProviderInfo(env_var=None),
}


def get_env_var(provider: str) -> str | None:
    """Get the environment variable name for a provider."""
    info = PROVIDERS.get(provider)
    return info.env_var if info else None


def is_valid_provider(name: str) -> bool:
    """Check if a provider name is valid."""
    return name in PROVIDERS


def list_providers() -> list[str]:
    """List all valid provider names."""
    return sorted(PROVIDERS.keys())

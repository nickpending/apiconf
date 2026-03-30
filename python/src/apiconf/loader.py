"""Config loader and API for apiconf."""

import tomllib
from pathlib import Path
from typing import Any

from .exceptions import AppNotFoundError, ConfigNotFoundError, KeyNotFoundError, ParseError


def get_config_path() -> Path:
    """Get the path to the config file."""
    return Path.home() / ".config" / "apiconf" / "config.toml"


def load_config() -> dict[str, Any]:
    """Load and parse the config file."""
    path = get_config_path()

    if not path.exists():
        raise ConfigNotFoundError(path)

    try:
        with path.open("rb") as f:
            return tomllib.load(f)
    except tomllib.TOMLDecodeError as e:
        raise ParseError(path, str(e)) from e


class AppConfig:
    """Configuration for an application with provider key access."""

    def __init__(
        self, app_name: str, providers: dict[str, str], keys: dict[str, dict[str, str]]
    ) -> None:
        self._app_name = app_name
        self._providers = providers
        self._keys = keys
        self._resolved: dict[str, str] = {}

        # Resolve all provider -> key -> value mappings
        for provider, key_name in providers.items():
            if key_name not in keys:
                import warnings

                warnings.warn(
                    f"Key '{key_name}' referenced by app '{app_name}' not found, skipping",
                    stacklevel=2,
                )
                continue

            key_info = keys[key_name]
            if "value" not in key_info:
                continue

            self._resolved[provider] = key_info["value"]

    def __getattr__(self, name: str) -> str:
        if name.startswith("_"):
            raise AttributeError(f"'{type(self).__name__}' has no attribute '{name}'")

        if name in self._resolved:
            return self._resolved[name]

        raise AttributeError(
            f"App '{self._app_name}' has no provider '{name}'. "
            f"Available: {', '.join(self._resolved.keys()) or 'none'}"
        )

    def __repr__(self) -> str:
        return f"AppConfig({self._app_name!r}, providers={list(self._resolved.keys())})"


def load(app_name: str) -> AppConfig:
    """Load configuration for an application.

    Args:
        app_name: The name of the application to load config for.

    Returns:
        An AppConfig object with provider key access.

    Raises:
        AppNotFoundError: If the app is not found in the config.
        ConfigNotFoundError: If the config file does not exist.
        ParseError: If the config file cannot be parsed.
    """
    config = load_config()

    apps = config.get("apps", {})
    if app_name not in apps:
        raise AppNotFoundError(app_name, list(apps.keys()))

    providers = apps[app_name]
    keys = config.get("keys", {})

    return AppConfig(app_name, providers, keys)


def get_key(key_name: str) -> str:
    """Get a key value directly by name.

    Args:
        key_name: The name of the key to retrieve.

    Returns:
        The key value.

    Raises:
        KeyNotFoundError: If the key is not found in the config.
        ConfigNotFoundError: If the config file does not exist.
        ParseError: If the config file cannot be parsed.
    """
    config = load_config()

    keys = config.get("keys", {})
    if key_name not in keys:
        raise KeyNotFoundError(key_name, list(keys.keys()))

    key_entry = keys[key_name]
    if "value" not in key_entry:
        raise ParseError(get_config_path(), f"Key '{key_name}' is missing 'value' field")

    return key_entry["value"]

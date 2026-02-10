"""Exceptions for apiconf."""

from pathlib import Path


class ApiconfError(Exception):
    """Base exception for apiconf errors."""


class ConfigNotFoundError(ApiconfError):
    """Raised when the config file is not found."""

    def __init__(self, path: Path) -> None:
        self.path = path
        super().__init__(f"Config file not found: {path}")


class ParseError(ApiconfError):
    """Raised when the config file cannot be parsed."""

    def __init__(self, path: Path, details: str) -> None:
        self.path = path
        self.details = details
        super().__init__(f"Failed to parse config file {path}: {details}")


class KeyNotFoundError(ApiconfError):
    """Raised when a key is not found in the config."""

    def __init__(self, key_name: str, available: list[str]) -> None:
        self.key_name = key_name
        self.available = available
        available_str = ", ".join(available) if available else "none"
        super().__init__(f"Key '{key_name}' not found. Available keys: {available_str}")


class AppNotFoundError(ApiconfError):
    """Raised when an app is not found in the config."""

    def __init__(self, app_name: str, available: list[str]) -> None:
        self.app_name = app_name
        self.available = available
        available_str = ", ".join(available) if available else "none"
        super().__init__(f"App '{app_name}' not found. Available apps: {available_str}")

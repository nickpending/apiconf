"""apiconf - Load API keys from shared config for your applications."""

from .exceptions import (
    ApiconfError,
    AppNotFoundError,
    ConfigNotFoundError,
    KeyNotFoundError,
    ParseError,
)
from .loader import get_key, load

__version__ = "0.1.0"

__all__ = [
    "ApiconfError",
    "AppNotFoundError",
    "ConfigNotFoundError",
    "KeyNotFoundError",
    "ParseError",
    "get_key",
    "load",
]

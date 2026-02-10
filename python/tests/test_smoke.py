"""Smoke tests for apiconf."""

from pathlib import Path
from typing import Generator
from unittest.mock import patch

import pytest

from apiconf import AppNotFoundError, KeyNotFoundError, get_key, load


@pytest.fixture
def mock_config(tmp_path: Path) -> Generator[Path, None, None]:
    """Create a temporary config directory with test data."""
    config_dir = tmp_path / ".config" / "apiconf"
    config_dir.mkdir(parents=True)

    config_file = config_dir / "config.toml"
    config_file.write_text("""
[keys.anthropic]
provider = "anthropic"
value = "sk-ant-test-key-12345"

[keys.openai]
provider = "openai"
value = "sk-openai-test"

[keys.ollama-local]
provider = "ollama"
value = "http://localhost:11434"

[apps.testapp]
anthropic = "anthropic"
openai = "openai"

[apps.localdev]
ollama = "ollama-local"
""")

    with patch.object(Path, "home", return_value=tmp_path):
        yield tmp_path


def test_load_returns_app_config(mock_config: Path) -> None:
    """Test that load() returns an AppConfig with correct values."""
    config = load("testapp")
    assert config._app_name == "testapp"


def test_config_anthropic_returns_key_value(mock_config: Path) -> None:
    """Test that config.anthropic returns the key value."""
    config = load("testapp")
    assert config.anthropic == "sk-ant-test-key-12345"


def test_config_openai_returns_key_value(mock_config: Path) -> None:
    """Test that config.openai returns the key value."""
    config = load("testapp")
    assert config.openai == "sk-openai-test"


def test_config_ollama_api_base_special_case(mock_config: Path) -> None:
    """Test that ollama sets ollama_api_base attribute."""
    config = load("localdev")
    assert config.ollama_api_base == "http://localhost:11434"
    assert config.ollama == "http://localhost:11434"


def test_load_nonexistent_raises_app_not_found(mock_config: Path) -> None:
    """Test that load() raises AppNotFoundError for unknown apps."""
    with pytest.raises(AppNotFoundError) as exc_info:
        load("nonexistent")

    assert exc_info.value.app_name == "nonexistent"
    assert "localdev" in exc_info.value.available
    assert "testapp" in exc_info.value.available


def test_config_unknown_raises_attribute_error(mock_config: Path) -> None:
    """Test that accessing unknown provider raises AttributeError."""
    config = load("testapp")
    with pytest.raises(AttributeError) as exc_info:
        _ = config.unknown

    assert "unknown" in str(exc_info.value)


def test_get_key_returns_value(mock_config: Path) -> None:
    """Test that get_key() returns the key value."""
    assert get_key("anthropic") == "sk-ant-test-key-12345"


def test_get_key_nonexistent_raises_key_not_found(mock_config: Path) -> None:
    """Test that get_key() raises KeyNotFoundError for unknown keys."""
    with pytest.raises(KeyNotFoundError) as exc_info:
        get_key("nonexistent")

    assert exc_info.value.key_name == "nonexistent"
    assert "anthropic" in exc_info.value.available


def test_app_config_repr(mock_config: Path) -> None:
    """Test AppConfig repr."""
    config = load("testapp")
    repr_str = repr(config)
    assert "testapp" in repr_str
    assert "AppConfig" in repr_str

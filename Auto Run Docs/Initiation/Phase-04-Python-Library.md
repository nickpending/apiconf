# Phase 04: Python Library

This phase creates the Python library that reads the same config file as the CLI. After this phase, Python applications can use `from apiconf import load; config = load("myapp"); print(config.anthropic)` to access their API keys. The library uses Python 3.11+ stdlib only (tomllib) with no external dependencies, making it lightweight and easy to install.

## Tasks

- [x] Create Python project structure with uv:
  - Create `python/` directory at project root
  - Create `python/pyproject.toml` with:
    - Name: `apiconf`, version `0.1.0`, requires-python `>=3.11`
    - No runtime dependencies (uses tomllib from stdlib)
    - Dev dependencies: `pytest>=8.0`, `ruff>=0.4`, `mypy>=1.10`
    - Build system: hatchling with src layout
    - Ruff config: target py311, line-length 100, select E/F/I/UP/B/SIM
    - Mypy config: python 3.11, strict mode
  - Create `python/src/apiconf/__init__.py` with version and exports placeholder
  - Create `python/src/apiconf/py.typed` (empty marker file for PEP 561)
  - Run `uv sync` to verify project setup
  - *Completed: Project structure created with hatchling build, all configs in place.*

- [x] Implement Python provider registry (`python/src/apiconf/providers.py`):
  - Create `ProviderInfo` dataclass with `env_var: str | None`
  - Create `PROVIDERS` dict with same 5 entries as Rust CLI:
    - `anthropic` → `ANTHROPIC_API_KEY`
    - `openai` → `OPENAI_API_KEY`
    - `google-gemini` → `GOOGLE_API_KEY`
    - `elevenlabs` → `ELEVENLABS_API_KEY`
    - `ollama` → `None`
  - Implement `get_env_var(provider: str) -> str | None`
  - Implement `is_valid_provider(name: str) -> bool`
  - Implement `list_providers() -> list[str]`
  - *Completed: providers.py with frozen dataclass and all functions.*

- [x] Implement Python exceptions (`python/src/apiconf/exceptions.py`):
  - Create `ApiconfError(Exception)` base class
  - Create `ConfigNotFoundError(ApiconfError)` with `path` attribute
  - Create `ParseError(ApiconfError)` with `path` and `details` attributes
  - Create `KeyNotFoundError(ApiconfError)` with `key_name` and `available` attributes
  - Create `AppNotFoundError(ApiconfError)` with `app_name` and `available` attributes
  - All errors should have clear, actionable messages
  - *Completed: exceptions.py with all error types and clear messages.*

- [x] Implement Python config loader and API (`python/src/apiconf/loader.py`):
  - Implement `get_config_path() -> Path` returning `~/.config/apiconf/config.toml`
  - Implement `load_config() -> dict` using tomllib to parse config
  - Create `AppConfig` class:
    - Constructor takes `app_name`, `providers` dict, and `keys` dict
    - Resolves provider → key_name → value mappings
    - Provides `__getattr__` for `config.anthropic` style access
    - Handles ollama special case: also sets `ollama_api_base` attribute
    - Raises `AttributeError` for unknown providers
  - Implement `load(app_name: str) -> AppConfig`:
    - Load config, validate app exists, return AppConfig
    - Raise `AppNotFoundError` with available apps if not found
  - Implement `get_key(key_name: str) -> str`:
    - Load config, return key value directly
    - Raise `KeyNotFoundError` with available keys if not found
  - *Completed: loader.py with AppConfig class and both load functions.*

- [x] Update Python package exports (`python/src/apiconf/__init__.py`):
  - Export `load`, `get_key` from loader module
  - Export `ApiconfError`, `KeyNotFoundError`, `AppNotFoundError`, `ConfigNotFoundError`, `ParseError`
  - Set `__version__ = "0.1.0"`
  - Set `__all__` with all public exports
  - *Completed: __init__.py exports all public API.*

- [x] Write Python smoke tests (`python/tests/test_smoke.py`):
  - Create `python/tests/__init__.py` (empty)
  - Create pytest fixture that:
    - Creates temp directory with `.config/apiconf/config.toml`
    - Patches `Path.home()` to return temp directory
    - Includes test keys (anthropic, openai, ollama) and test app
  - Test `load("testapp")` returns AppConfig with correct values
  - Test `config.anthropic` returns key value
  - Test `config.ollama_api_base` returns ollama value (special case)
  - Test `load("nonexistent")` raises `AppNotFoundError` with available apps
  - Test `config.unknown` raises `AttributeError`
  - Test `get_key("anthropic")` returns key value
  - Test `get_key("nonexistent")` raises `KeyNotFoundError` with available keys
  - *Completed: 9 tests covering all functionality with mock_config fixture.*

- [x] Verify Python library works end-to-end:
  - Run `uv run ruff check src/` - should pass with no errors
  - Run `uv run mypy src/` - should pass with strict mode
  - Run `uv run pytest -v` - all tests should pass
  - Manual test with real config (if keys added via CLI):
    - `uv run python -c "from apiconf import load; print(load('myapp').anthropic)"`
  - *Completed: ruff passes, mypy strict passes, pytest 9/9 pass, manual test works.*

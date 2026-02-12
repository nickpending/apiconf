# apiconf (Python)

Load API keys from shared config for your applications.

## Installation

Requires Python 3.11+ (uses stdlib `tomllib`). No external dependencies.

```bash
# From the python/ directory
uv sync

# Or with pip
pip install .
```

## Usage

```python
from apiconf import load

# Load an app's configuration
config = load("myapp")
config.anthropic  # Your ANTHROPIC_API_KEY value
config.openai     # Your OPENAI_API_KEY value

# Or get a key directly by name
from apiconf import get_key
key = get_key("anthropic")
```

## Configuration

Reads from `~/.config/apiconf/config.toml`, the same file managed by the `apiconf` CLI. See the [project README](../README.md) for config format details.

## Error handling

```python
from apiconf import load, AppNotFoundError, ConfigNotFoundError

try:
    config = load("myapp")
except ConfigNotFoundError:
    print("Run 'apiconf keys add' to set up your config first")
except AppNotFoundError as e:
    print(f"App not found. Available: {e.available}")
```

## License

MIT

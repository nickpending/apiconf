# apiconf

Load API keys from shared config for your applications.

## Installation

```bash
pip install apiconf
```

## Usage

```python
from apiconf import load

# Load an app's configuration
config = load("myapp")
print(config.anthropic)  # Your ANTHROPIC_API_KEY value
print(config.openai)     # Your OPENAI_API_KEY value

# Or get a key directly
from apiconf import get_key
key = get_key("anthropic")
```

## Configuration

apiconf reads from `~/.config/apiconf/config.toml`, the same file used by the CLI.

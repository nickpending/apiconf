# apiconf

Unified API key management. One config file, accessible from your shell and any language.

Store your API keys once in `~/.config/apiconf/config.toml`, then access them from the CLI, Python, or TypeScript. No more scattered `.env` files.

## How it works

**Keys** are named credentials tied to a provider:

```bash
apiconf keys add anthropic          # Prompts for your API key
apiconf keys add openai
```

**Apps** are profiles that select which keys to use:

```bash
apiconf apps create myapp
apiconf apps add myapp anthropic    # Links your anthropic key to myapp
apiconf apps add myapp openai
```

**Use them** from your shell or code:

```bash
# Shell: export as environment variables
eval $(apiconf env myapp)
# exports ANTHROPIC_API_KEY and OPENAI_API_KEY
```

```python
# Python
from apiconf import load
config = load("myapp")
config.anthropic  # your key value
```

```typescript
// TypeScript
import { load } from "apiconf";
const config = load("myapp");
config.anthropic; // your key value
```

## Install

### CLI (Rust)

Build from source (requires [Rust](https://rustup.rs/)):

```bash
cd cli && cargo install --path .
```

### Python library

Requires Python 3.11+. Install with [uv](https://docs.astral.sh/uv/):

```bash
cd python && uv sync
```

### TypeScript library

Requires Node 20+. Install from the local package:

```bash
cd typescript && npm install .
```

## Config format

All components read from `~/.config/apiconf/config.toml`, protected with `chmod 600` (same security model as SSH keys).

```toml
[keys.anthropic]
provider = "anthropic"
value = "sk-ant-xxx"

[keys.openai]
provider = "openai"
value = "sk-xxx"

[apps.myapp]
anthropic = "anthropic"
openai = "openai"
```

### Multiple keys per provider

Name keys to distinguish them:

```bash
apiconf keys add anthropic --name anthropic-personal
apiconf keys add anthropic --name anthropic-work
apiconf apps add myapp anthropic --key anthropic-work
```

## CLI reference

```
apiconf keys add <provider> [--name <name>] [--force]
apiconf keys list
apiconf keys remove <name>

apiconf apps create <name>
apiconf apps list
apiconf apps add <app> <provider> [--key <key>]
apiconf apps show <app>
apiconf apps remove <app>

apiconf env <app>
```

## Supported providers

| Provider | Environment variable |
|----------|---------------------|
| `anthropic` | `ANTHROPIC_API_KEY` |
| `openai` | `OPENAI_API_KEY` |
| `google-gemini` | `GOOGLE_API_KEY` |
| `elevenlabs` | `ELEVENLABS_API_KEY` |
| `ollama` | `OLLAMA_API_BASE` |

## License

MIT

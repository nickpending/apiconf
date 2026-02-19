# apiconf (TypeScript)

Load API keys from shared config for your applications.

## Installation

Requires [Bun](https://bun.sh/).

```bash
# From the typescript/ directory
bun install
```

## Usage

```typescript
import { load, getKey } from "apiconf";

// Load an app's configuration
const config = load("myapp");
config.anthropic; // Your ANTHROPIC_API_KEY value
config.openai;    // Your OPENAI_API_KEY value

// Or get a key directly by name
const key = getKey("anthropic");
```

## Configuration

Reads from `~/.config/apiconf/config.toml`, the same file managed by the `apiconf` CLI. See the [project README](../README.md) for config format details.

## Error handling

```typescript
import { load, AppNotFoundError, ConfigNotFoundError } from "apiconf";

try {
  const config = load("myapp");
} catch (error) {
  if (error instanceof ConfigNotFoundError) {
    console.error("Run 'apiconf keys add' to set up your config first");
  } else if (error instanceof AppNotFoundError) {
    console.error(`App not found. Available: ${error.available}`);
  }
}
```

## API

### `load(appName: string): AppConfig`

Returns an object mapping provider names to key values.

### `getKey(keyName: string): string`

Returns a key value directly by name.

### Exports

- `load`, `getKey`, `getConfigPath`, `loadConfig` — loader functions
- `AppConfig`, `RawConfig`, `RawKey`, `RawApp` — types
- `ApiconfError`, `AppNotFoundError`, `ConfigNotFoundError`, `KeyNotFoundError`, `ParseError` — errors
- `PROVIDERS`, `getEnvVar`, `isValidProvider`, `listProviders` — provider registry

## License

MIT

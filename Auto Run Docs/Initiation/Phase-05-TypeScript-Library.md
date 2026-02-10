# Phase 05: TypeScript Library

This phase creates the TypeScript library that provides the same API as Python but for Node.js applications. After this phase, TypeScript/JavaScript applications can use `import { load } from 'apiconf'; const config = load('myapp'); console.log(config.anthropic);` to access their API keys. The library uses smol-toml for TOML parsing and ships with full type definitions.

## Tasks

- [x] Create TypeScript project structure with pnpm:
  - Create `typescript/` directory at project root
  - Create `typescript/package.json` with:
    - Name: `apiconf`, version `0.1.0`, type `module`
    - Main: `dist/index.js`, types: `dist/index.d.ts`
    - Exports with types and import fields
    - Dependencies: `smol-toml: ^1.3.0`
    - Dev dependencies: `@types/node: ^22.0.0`, `typescript: ^5.6.0`, `vitest: ^2.1.0`
    - Scripts: build, test, test:watch, typecheck
    - Engines: node >=20
  - Create `typescript/tsconfig.json` with:
    - Target ES2022, module ESNext, moduleResolution bundler
    - Strict mode with noUncheckedIndexedAccess, noImplicitOverride, noPropertyAccessFromIndexSignature
    - Declaration output to dist/
  - Create `typescript/src/index.ts` with placeholder export
  - Run `pnpm install` to verify setup
  - *Completed: Project structure created with npm (pnpm not available). All configs in place.*

- [x] Implement TypeScript types (`typescript/src/types.ts`):
  - Define `RawConfig` interface with optional `keys` and `apps` records
  - Define `RawKey` interface with `provider: string` and `value: string`
  - Define `RawApp` type as `Record<string, string>`
  - Define `AppConfig` interface with string index signature for provider access
  - *Completed: types.ts with all interfaces.*

- [x] Implement TypeScript provider registry (`typescript/src/providers.ts`):
  - Define `ProviderInfo` interface with `envVar: string | null`
  - Create `PROVIDERS` const object with same 5 entries as Rust/Python:
    - `anthropic` → `ANTHROPIC_API_KEY`
    - `openai` → `OPENAI_API_KEY`
    - `google-gemini` → `GOOGLE_API_KEY`
    - `elevenlabs` → `ELEVENLABS_API_KEY`
    - `ollama` → `null`
  - Implement `getEnvVar(provider: string): string | null`
  - Implement `isValidProvider(name: string): boolean`
  - Implement `listProviders(): string[]`
  - *Completed: providers.ts with const object and all functions.*

- [x] Implement TypeScript errors (`typescript/src/errors.ts`):
  - Create `ApiconfError extends Error` base class
  - Create `ConfigNotFoundError` with `path` property
  - Create `ParseError` with `path` and `details` properties
  - Create `KeyNotFoundError` with `keyName` and `available` properties
  - Create `AppNotFoundError` with `appName` and `available` properties
  - Set `name` property on each error class for proper error identification
  - *Completed: errors.ts with all error classes and name properties.*

- [x] Implement TypeScript config loader and API (`typescript/src/loader.ts`):
  - Implement `getConfigPath(): string` returning `~/.config/apiconf/config.toml`
  - Implement `loadConfig(): RawConfig` using smol-toml parse
  - Implement `createAppConfig(appName, providers, keys): AppConfig`:
    - Build data object mapping provider names to key values
    - Handle ollama special case: also set `ollama_api_base`
    - Return as AppConfig
  - Implement `load(appName: string): AppConfig`:
    - Load config, validate app exists
    - Throw `AppNotFoundError` with available apps if not found
    - Return created AppConfig
  - Implement `getKey(keyName: string): string`:
    - Load config, validate key exists
    - Throw `KeyNotFoundError` with available keys if not found
    - Return key value
  - *Completed: loader.ts with createAppConfig, load, and getKey functions.*

- [x] Update TypeScript package exports (`typescript/src/index.ts`):
  - Export `load`, `getKey` from loader module
  - Export all error classes from errors module
  - Export types: `AppConfig`, `RawConfig`, `RawKey`, `RawApp`
  - Export `PROVIDERS`, `getEnvVar`, `isValidProvider`, `listProviders`
  - Export `VERSION = "0.1.0"`
  - *Completed: index.ts exports all public API.*

- [x] Write TypeScript tests (`typescript/tests/smoke.test.ts`):
  - Create test file with vitest imports
  - Test that `load` and `getKey` are exported as functions
  - Test that all error classes are exported and constructible
  - Test provider registry has all 5 providers
  - Test `getEnvVar("anthropic")` returns `"ANTHROPIC_API_KEY"`
  - Test `getEnvVar("ollama")` returns `null`
  - Note: Full integration tests require config path override (documented in test file)
  - *Completed: 16 tests covering exports, errors, and provider registry.*

- [x] Verify TypeScript library works end-to-end:
  - Run `pnpm run build` - should compile without errors
  - Run `pnpm run typecheck` - should pass strict mode checks
  - Run `pnpm run test` - all tests should pass
  - Verify `dist/` contains `index.js` and `index.d.ts`
  - Manual test with real config (if keys added via CLI):
    - `pnpm tsx -e "import { load } from './dist/index.js'; console.log(load('myapp').anthropic);"`
  - *Completed: tsc builds clean, vitest 16/16 pass, dist/ has .js and .d.ts, manual test returns key.*

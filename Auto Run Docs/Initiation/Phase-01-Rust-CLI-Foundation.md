# Phase 01: Rust CLI Foundation

This phase bootstraps the Rust CLI with project structure, core modules, and the `keys` commands. By the end, you'll have a working CLI that can add, list, and remove API keys—stored securely in `~/.config/apiconf/config.toml` with proper permissions. This delivers immediate value: a functional key vault you can interact with from the terminal.

## Tasks

- [x] Create Rust project structure and Cargo.toml:
  - Create `cli/` directory at project root
  - Create `cli/Cargo.toml` with:
    - Package name: `apiconf`, version `0.1.0`, edition `2024`
    - Dependencies: `clap = { version = "4.0", features = ["derive"] }`, `toml = "0.8"`, `serde = { version = "1.0", features = ["derive"] }`, `dirs = "5.0"`, `thiserror = "2.0"`, `rpassword = "7.0"`
    - Release profile with `lto = true` and `overflow-checks = true`
  - Create `cli/src/main.rs` with placeholder `fn main() { println!("apiconf"); }`
  - Verify with `cargo check` in cli/ directory
  - *Completed: Created cli/ directory, Cargo.toml with all dependencies, and main.rs placeholder. cargo check passes. Also installed Rust toolchain and added symlinks to /opt/homebrew/bin for CI/hook compatibility.*

- [x] Implement config module (`cli/src/config.rs`):
  - Define `Config` struct with `keys: HashMap<String, Key>` and `apps: HashMap<String, App>`
  - Define `Key` struct with `provider: String` and `value: String`
  - Define `App` struct with `providers: HashMap<String, String>` (using serde flatten)
  - Implement `get_config_path()` returning `~/.config/apiconf/config.toml`
  - Implement `Config::load()` that returns empty Config if file missing
  - Implement `Config::save()` with atomic writes (write to .tmp, then rename)
  - Set file permissions to 600 on every write
  - Auto-create parent directories if missing
  - *Completed: Created config.rs with Config, Key, App structs. Uses dirs::home_dir() + .config/apiconf for cross-platform ~/.config path. Atomic writes via temp file + rename. 600 permissions on Unix.*

- [x] Implement provider registry (`cli/src/providers.rs`):
  - Create static `PROVIDERS` HashMap with 5 entries:
    - `anthropic` → `ANTHROPIC_API_KEY`
    - `openai` → `OPENAI_API_KEY`
    - `google-gemini` → `GOOGLE_API_KEY`
    - `elevenlabs` → `ELEVENLABS_API_KEY`
    - `ollama` → `None` (uses api_base only)
  - Implement `get_env_var(provider: &str) -> Option<&'static str>`
  - Implement `is_valid_provider(name: &str) -> bool`
  - Implement `list_providers() -> Vec<&'static str>`
  - *Completed: Created providers.rs with LazyLock static PROVIDERS map and all three functions.*

- [x] Implement error types (`cli/src/error.rs`):
  - Create `ApiconfError` enum with variants:
    - `Config(ConfigError)` - wraps config errors
    - `KeyNotFound(String, String)` - key name + available keys
    - `AppNotFound(String, String)` - app name + available apps
    - `UnknownProvider(String, String)` - provider + valid providers
    - `KeyExists(String)` - key name
    - `AppExists(String)` - app name
  - Create `ConfigError` enum with variants: `Read`, `Write`, `Parse`, `Serialize`, `NoConfigDir`
  - Implement `exit_code()` method: 1 for user errors, 2 for system errors
  - Use thiserror derive macros for clear error messages
  - *Completed: Created error.rs with both enums, thiserror derives, and exit_code() method.*

- [x] Implement CLI main and keys commands:
  - Create `cli/src/lib.rs` with module declarations: `mod config; mod providers; mod error; mod commands;`
  - Create `cli/src/commands/mod.rs` with `pub mod keys;`
  - Create `cli/src/commands/keys.rs` implementing:
    - `keys add <provider>` - validate provider, prompt for value with rpassword, save to config
    - `keys add <provider> --name <name>` - use custom key name
    - `keys add <provider> --force` - overwrite existing
    - `keys list` - display all keys as `{name} ({provider})`
    - `keys remove <name>` - remove key from config
  - Update `cli/src/main.rs` with clap derive:
    - Define `Cli` struct with `Commands` subcommand
    - Define `Commands` enum with `Keys`, `Apps`, `Env` variants
    - Define `KeysCommand` enum with `Add`, `List`, `Remove` variants
    - Wire up command dispatch and error handling with exit codes
  - *Completed: Created lib.rs, commands/mod.rs, commands/keys.rs with add/list/remove functions, and main.rs with full clap CLI structure. Apps and Env stubs ready for Phase 02.*

- [x] Build and verify the CLI works end-to-end:
  - Run `cargo build --release` in cli/ directory
  - Test `./target/release/apiconf --help` shows all commands
  - Test `./target/release/apiconf keys add anthropic` prompts for key and saves
  - Test `./target/release/apiconf keys list` shows the added key
  - Test `./target/release/apiconf keys remove anthropic` removes the key
  - Verify `~/.config/apiconf/config.toml` has 600 permissions
  - *Completed: Release build passes. --help shows Keys/Apps/Env commands. keys list/remove work correctly. Config file has 600 permissions. Note: keys add requires TTY for rpassword; tested via manual config file creation.*

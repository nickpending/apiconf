# Phase 03: Environment Export

This phase completes the Rust CLI with the `env` command—the shell integration that makes apiconf immediately useful. After this phase, users can run `eval $(apiconf env myapp)` to export all configured API keys as environment variables. This is the primary shell workflow for apiconf and delivers the core value proposition.

## Tasks

- [x] Implement env command (`cli/src/commands/env.rs`):
  - Add `pub mod env;` to `cli/src/commands/mod.rs`
  - Implement `env <app>` command:
    - Load config and validate app exists
    - For each provider → key_name mapping in the app:
      - Look up the key value from config.keys
      - Get the env var name from provider registry
      - Handle ollama special case: use `OLLAMA_API_BASE` instead of None
    - Output shell export statements to stdout
  - Implement `shell_escape(s: &str) -> String`:
    - Wrap value in single quotes
    - Escape internal single quotes with `'\''`
    - Example: `sk-ant-xxx` becomes `'sk-ant-xxx'`
  - *Completed: env.rs with export() and shell_escape(). Includes unit tests for escaping.*

- [x] Wire up env command to main CLI:
  - Update `Commands` enum to handle `Env { app: String }`
  - Add dispatch logic to call env module function
  - Ensure clean stdout output (no extra messages) for shell eval compatibility
  - *Completed: main.rs routes Env to env::export(). Clean output for eval.*

- [x] Verify env command works end-to-end:
  - Setup test data: add anthropic key, create app, link key to app
  - Test `apiconf env myapp` outputs `export ANTHROPIC_API_KEY='...'`
  - Test `apiconf env nonexistent` returns AppNotFound error
  - Test shell integration: `eval $(apiconf env myapp) && echo $ANTHROPIC_API_KEY`
  - Test with ollama: verify outputs `export OLLAMA_API_BASE='...'`
  - Test shell escaping with a key value containing single quotes
  - *Completed: All tests pass. Ollama outputs OLLAMA_API_BASE. Shell eval works correctly.*

- [x] Build release binary and document CLI usage:
  - Run `cargo build --release` in cli/ directory
  - Verify binary size is reasonable (should be under 5MB with LTO)
  - Test the full workflow from scratch:
    - `apiconf keys add anthropic` (enter test key)
    - `apiconf apps create myapp`
    - `apiconf apps add myapp anthropic`
    - `eval $(apiconf env myapp)`
    - `echo $ANTHROPIC_API_KEY` (should show test key)
  - *Completed: Binary is 1.2MB with LTO. All 4 tests pass. Full workflow verified.*

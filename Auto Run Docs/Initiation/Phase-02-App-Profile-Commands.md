# Phase 02: App Profile Commands

This phase adds app profile management to the CLI. App profiles are the key abstraction that solves the multi-key-per-provider collision problem—each app (like "momentum" or "prismis") declares which keys it uses, enabling `eval $(apiconf env myapp)` to set exactly the right credentials. After this phase, users can create profiles and link keys to them.

## Tasks

- [x] Implement apps commands module (`cli/src/commands/apps.rs`):
  - Add `pub mod apps;` to `cli/src/commands/mod.rs`
  - Define `AppsCommand` enum with variants: `Create`, `Add`, `Show`, `List`, `Remove`
  - Implement `apps create <name>`:
    - Check app doesn't already exist (return `AppExists` error if so)
    - Insert empty `App` into config
    - Save config and print confirmation
  - Implement `apps list`:
    - Display all apps as `{name} ({key_count} keys)`
    - Show "(no apps)" if empty
  - *Completed: Created apps.rs with all 5 commands. Added to mod.rs.*

- [x] Implement apps add command with validation:
  - Command: `apps add <app> <provider>` with optional `--key <name>`
  - Validate app exists (return `AppNotFound` with available apps)
  - Validate provider is in registry (return `UnknownProvider` with valid providers)
  - Determine key name: use `--key` value or default to provider name
  - Validate key exists (return `KeyNotFound` with available keys)
  - Add provider → key_name mapping to app's providers HashMap
  - Save config and print confirmation
  - *Completed: Full validation chain implemented - app → provider → key existence checks.*

- [x] Implement apps show and remove commands:
  - `apps show <app>`:
    - Validate app exists
    - Print "App: {name}" header
    - For each provider mapping: print `  {provider} -> {key_name} ({env_var})`
    - Show "(no keys configured)" for empty app
    - Use `get_env_var()` from providers module for env var display
  - `apps remove <app>`:
    - Validate app exists
    - Remove from config.apps
    - Save and print confirmation
  - *Completed: Both commands implemented with proper validation and output formatting.*

- [x] Wire up apps commands to main CLI:
  - Update `Commands` enum in main.rs to include `Apps { command: AppsCommand }`
  - Add dispatch logic for `Commands::Apps` to route to apps module functions
  - Ensure all apps commands use consistent error handling and exit codes
  - *Completed: AppsCommand enum updated with all 5 variants. Dispatch routes to apps module.*

- [x] Verify apps commands work end-to-end:
  - Test `apiconf apps create momentum` creates empty profile
  - Test `apiconf apps create momentum` again returns AppExists error
  - Test `apiconf apps list` shows the created app
  - Test `apiconf apps add momentum anthropic` links the key (requires key from Phase 1)
  - Test `apiconf apps add momentum unknown` returns UnknownProvider error
  - Test `apiconf apps show momentum` displays the mapping with env var
  - Test `apiconf apps remove momentum` removes the profile
  - *Completed: All test cases pass. Proper error messages and exit codes (1 for user errors).*

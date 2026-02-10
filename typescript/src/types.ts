/**
 * Raw key configuration from TOML file.
 */
export interface RawKey {
  provider: string;
  value: string;
}

/**
 * Raw app configuration - maps provider names to key names.
 */
export type RawApp = Record<string, string>;

/**
 * Raw configuration structure from TOML file.
 */
export interface RawConfig {
  keys?: Record<string, RawKey>;
  apps?: Record<string, RawApp>;
}

/**
 * Resolved app configuration with provider key access.
 */
export interface AppConfig {
  [provider: string]: string;
}

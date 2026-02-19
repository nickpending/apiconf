/**
 * apiconf - Load API keys from shared config for your applications.
 */

// Loader API
export { load, getKey, getConfigPath, loadConfig, onWarn } from "./loader.js";

// Error classes
export {
  ApiconfError,
  AppNotFoundError,
  ConfigNotFoundError,
  KeyNotFoundError,
  ParseError,
} from "./errors.js";

// Types
export type { AppConfig, RawConfig, RawKey, RawApp } from "./types.js";

// Provider registry
export {
  PROVIDERS,
  getEnvVar,
  isValidProvider,
  listProviders,
  type ProviderInfo,
} from "./providers.js";

// Version
export const VERSION = "0.1.1";

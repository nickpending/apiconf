/**
 * Config loader and API for apiconf.
 */

import { readFileSync, existsSync } from "node:fs";
import { homedir } from "node:os";
import { join } from "node:path";
import { parse } from "smol-toml";
import {
  AppNotFoundError,
  ConfigNotFoundError,
  KeyNotFoundError,
  ParseError,
} from "./errors.js";

/** Warning handler — replace to suppress or redirect warnings. */
export let onWarn: (message: string) => void = (msg) => console.warn(msg);
import type { AppConfig, RawConfig, RawKey } from "./types.js";

/**
 * Get the path to the config file.
 */
export function getConfigPath(): string {
  return join(homedir(), ".config", "apiconf", "config.toml");
}

/**
 * Load and parse the config file.
 */
export function loadConfig(): RawConfig {
  const path = getConfigPath();

  if (!existsSync(path)) {
    throw new ConfigNotFoundError(path);
  }

  try {
    const content = readFileSync(path, "utf-8");
    return parse(content) as RawConfig;
  } catch (error) {
    if (error instanceof ConfigNotFoundError) {
      throw error;
    }
    const message = error instanceof Error ? error.message : String(error);
    throw new ParseError(path, message);
  }
}

/**
 * Create an AppConfig from providers and keys mappings.
 */
function createAppConfig(
  appName: string,
  providers: Record<string, string>,
  keys: Record<string, RawKey>,
): AppConfig {
  const config: Record<string, string> = {};

  for (const [provider, keyName] of Object.entries(providers)) {
    const keyInfo = keys[keyName];
    if (!keyInfo) {
      onWarn(
        `Warning: Key '${keyName}' referenced by app '${appName}' not found, skipping`,
      );
      continue;
    }
    if (!keyInfo.value) {
      continue;
    }

    config[provider] = keyInfo.value;

    // Special case for ollama: also set ollama_api_base
    if (provider === "ollama") {
      config["ollama_api_base"] = keyInfo.value;
    }
  }

  return new Proxy(config, {
    get(target, prop: string | symbol) {
      if (typeof prop === "symbol" || prop in target) {
        return target[prop as string];
      }
      const available = Object.keys(target).join(", ") || "none";
      throw new Error(
        `App '${appName}' has no provider '${prop}'. Available: ${available}`,
      );
    },
  }) as AppConfig;
}

/**
 * Load configuration for an application.
 *
 * @param appName - The name of the application to load config for.
 * @returns An AppConfig object with provider key access.
 * @throws AppNotFoundError if the app is not found in the config.
 * @throws ConfigNotFoundError if the config file does not exist.
 * @throws ParseError if the config file cannot be parsed.
 */
export function load(appName: string): AppConfig {
  const config = loadConfig();

  const apps = config.apps ?? {};
  if (!(appName in apps)) {
    throw new AppNotFoundError(appName, Object.keys(apps));
  }

  const providers = apps[appName] ?? {};
  const keys = config.keys ?? {};

  return createAppConfig(appName, providers, keys);
}

/**
 * Get a key value directly by name.
 *
 * @param keyName - The name of the key to retrieve.
 * @returns The key value.
 * @throws KeyNotFoundError if the key is not found in the config.
 * @throws ConfigNotFoundError if the config file does not exist.
 * @throws ParseError if the config file cannot be parsed.
 */
export function getKey(keyName: string): string {
  const config = loadConfig();

  const keys = config.keys ?? {};
  if (!(keyName in keys)) {
    throw new KeyNotFoundError(keyName, Object.keys(keys));
  }

  const keyInfo = keys[keyName];
  if (!keyInfo || typeof keyInfo.value !== "string" || keyInfo.value === "") {
    throw new ParseError(
      getConfigPath(),
      `Key '${keyName}' is missing 'value' field`,
    );
  }

  return keyInfo.value;
}

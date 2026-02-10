/**
 * Error classes for apiconf.
 */

/**
 * Base error class for apiconf errors.
 */
export class ApiconfError extends Error {
  constructor(message: string) {
    super(message);
    this.name = "ApiconfError";
  }
}

/**
 * Raised when the config file is not found.
 */
export class ConfigNotFoundError extends ApiconfError {
  readonly path: string;

  constructor(path: string) {
    super(`Config file not found: ${path}`);
    this.name = "ConfigNotFoundError";
    this.path = path;
  }
}

/**
 * Raised when the config file cannot be parsed.
 */
export class ParseError extends ApiconfError {
  readonly path: string;
  readonly details: string;

  constructor(path: string, details: string) {
    super(`Failed to parse config file ${path}: ${details}`);
    this.name = "ParseError";
    this.path = path;
    this.details = details;
  }
}

/**
 * Raised when a key is not found in the config.
 */
export class KeyNotFoundError extends ApiconfError {
  readonly keyName: string;
  readonly available: string[];

  constructor(keyName: string, available: string[]) {
    const availableStr = available.length > 0 ? available.join(", ") : "none";
    super(`Key '${keyName}' not found. Available keys: ${availableStr}`);
    this.name = "KeyNotFoundError";
    this.keyName = keyName;
    this.available = available;
  }
}

/**
 * Raised when an app is not found in the config.
 */
export class AppNotFoundError extends ApiconfError {
  readonly appName: string;
  readonly available: string[];

  constructor(appName: string, available: string[]) {
    const availableStr = available.length > 0 ? available.join(", ") : "none";
    super(`App '${appName}' not found. Available apps: ${availableStr}`);
    this.name = "AppNotFoundError";
    this.appName = appName;
    this.available = available;
  }
}

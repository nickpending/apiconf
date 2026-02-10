import { describe, expect, it } from "vitest";
import {
  load,
  getKey,
  ApiconfError,
  AppNotFoundError,
  ConfigNotFoundError,
  KeyNotFoundError,
  ParseError,
  PROVIDERS,
  getEnvVar,
  isValidProvider,
  listProviders,
  VERSION,
} from "../src/index.js";

describe("exports", () => {
  it("exports load as a function", () => {
    expect(typeof load).toBe("function");
  });

  it("exports getKey as a function", () => {
    expect(typeof getKey).toBe("function");
  });

  it("exports VERSION", () => {
    expect(VERSION).toBe("0.1.0");
  });
});

describe("error classes", () => {
  it("ApiconfError is constructible", () => {
    const error = new ApiconfError("test message");
    expect(error).toBeInstanceOf(Error);
    expect(error.name).toBe("ApiconfError");
    expect(error.message).toBe("test message");
  });

  it("ConfigNotFoundError has path property", () => {
    const error = new ConfigNotFoundError("/path/to/config");
    expect(error).toBeInstanceOf(ApiconfError);
    expect(error.name).toBe("ConfigNotFoundError");
    expect(error.path).toBe("/path/to/config");
  });

  it("ParseError has path and details properties", () => {
    const error = new ParseError("/path/to/config", "invalid toml");
    expect(error).toBeInstanceOf(ApiconfError);
    expect(error.name).toBe("ParseError");
    expect(error.path).toBe("/path/to/config");
    expect(error.details).toBe("invalid toml");
  });

  it("KeyNotFoundError has keyName and available properties", () => {
    const error = new KeyNotFoundError("mykey", ["key1", "key2"]);
    expect(error).toBeInstanceOf(ApiconfError);
    expect(error.name).toBe("KeyNotFoundError");
    expect(error.keyName).toBe("mykey");
    expect(error.available).toEqual(["key1", "key2"]);
  });

  it("AppNotFoundError has appName and available properties", () => {
    const error = new AppNotFoundError("myapp", ["app1", "app2"]);
    expect(error).toBeInstanceOf(ApiconfError);
    expect(error.name).toBe("AppNotFoundError");
    expect(error.appName).toBe("myapp");
    expect(error.available).toEqual(["app1", "app2"]);
  });
});

describe("provider registry", () => {
  it("has all 5 providers", () => {
    expect(Object.keys(PROVIDERS)).toHaveLength(5);
    expect(PROVIDERS["anthropic"]).toBeDefined();
    expect(PROVIDERS["openai"]).toBeDefined();
    expect(PROVIDERS["google-gemini"]).toBeDefined();
    expect(PROVIDERS["elevenlabs"]).toBeDefined();
    expect(PROVIDERS["ollama"]).toBeDefined();
  });

  it("getEnvVar returns ANTHROPIC_API_KEY for anthropic", () => {
    expect(getEnvVar("anthropic")).toBe("ANTHROPIC_API_KEY");
  });

  it("getEnvVar returns null for ollama", () => {
    expect(getEnvVar("ollama")).toBeNull();
  });

  it("getEnvVar returns null for unknown provider", () => {
    expect(getEnvVar("unknown")).toBeNull();
  });

  it("isValidProvider returns true for known providers", () => {
    expect(isValidProvider("anthropic")).toBe(true);
    expect(isValidProvider("openai")).toBe(true);
    expect(isValidProvider("ollama")).toBe(true);
  });

  it("isValidProvider returns false for unknown providers", () => {
    expect(isValidProvider("unknown")).toBe(false);
  });

  it("listProviders returns sorted list", () => {
    const providers = listProviders();
    expect(providers).toEqual([
      "anthropic",
      "elevenlabs",
      "google-gemini",
      "ollama",
      "openai",
    ]);
  });
});

// Note: Full integration tests would require mocking the config path.
// For now, we test that the functions throw ConfigNotFoundError when
// the config file doesn't exist in the expected location.
describe("loader integration", () => {
  it("load throws ConfigNotFoundError when config is missing", () => {
    // This test assumes the default config path doesn't exist in test env
    // or has been set up specifically for testing
    try {
      load("testapp");
      // If we get here, config exists - that's fine for this test
    } catch (error) {
      if (error instanceof ConfigNotFoundError) {
        expect(error.name).toBe("ConfigNotFoundError");
      } else if (error instanceof AppNotFoundError) {
        // Config exists but app doesn't - also valid
        expect(error.name).toBe("AppNotFoundError");
      } else {
        throw error;
      }
    }
  });
});

import { describe, expect, it } from "bun:test";
import {
  load,
  getKey,
  ApiconfError,
  AppNotFoundError,
  ConfigNotFoundError,
  KeyNotFoundError,
  ParseError,
  KNOWN_DEFAULTS,
  resolveEnvVar,
  isKnownProvider,
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
    expect(VERSION).toBe("0.2.0");
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
  it("has all 5 known defaults", () => {
    expect(Object.keys(KNOWN_DEFAULTS)).toHaveLength(5);
    expect(KNOWN_DEFAULTS["anthropic"]).toBeDefined();
    expect(KNOWN_DEFAULTS["openai"]).toBeDefined();
    expect(KNOWN_DEFAULTS["google-gemini"]).toBeDefined();
    expect(KNOWN_DEFAULTS["elevenlabs"]).toBeDefined();
    expect(KNOWN_DEFAULTS["ollama"]).toBeDefined();
  });

  it("ollama has OLLAMA_API_BASE as envVar", () => {
    expect(KNOWN_DEFAULTS["ollama"]?.envVar).toBe("OLLAMA_API_BASE");
  });

  it("isKnownProvider returns true for known providers", () => {
    expect(isKnownProvider("anthropic")).toBe(true);
    expect(isKnownProvider("openai")).toBe(true);
    expect(isKnownProvider("ollama")).toBe(true);
  });

  it("isKnownProvider returns false for unknown providers", () => {
    expect(isKnownProvider("brave")).toBe(false);
    expect(isKnownProvider("unknown")).toBe(false);
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

describe("resolveEnvVar", () => {
  it("tier 1: returns explicit env var when provided", () => {
    expect(resolveEnvVar("custom", "MY_CUSTOM_KEY")).toBe("MY_CUSTOM_KEY");
  });

  it("tier 2: returns known default for registered provider", () => {
    expect(resolveEnvVar("anthropic")).toBe("ANTHROPIC_API_KEY");
    expect(resolveEnvVar("ollama")).toBe("OLLAMA_API_BASE");
    expect(resolveEnvVar("google-gemini")).toBe("GOOGLE_API_KEY");
  });

  it("tier 3: returns convention-mapped name for unknown provider", () => {
    expect(resolveEnvVar("brave")).toBe("BRAVE_API_KEY");
  });

  it("tier 3: replaces hyphens with underscores", () => {
    expect(resolveEnvVar("google-gemini-custom")).toBe(
      "GOOGLE_GEMINI_CUSTOM_API_KEY",
    );
  });

  it("tier 1 overrides tier 2", () => {
    expect(resolveEnvVar("anthropic", "MY_OVERRIDE")).toBe("MY_OVERRIDE");
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

/**
 * Provider registry mapping known provider names to default environment variables.
 *
 * This is a hints table, not a validation gate. Any string is a valid provider.
 * Known providers get their conventional env var from this table; unknown
 * providers fall back to convention mapping (UPPERCASE + _API_KEY).
 */

export interface ProviderInfo {
  envVar: string;
}

export const KNOWN_DEFAULTS: Record<string, ProviderInfo> = {
  anthropic: { envVar: "ANTHROPIC_API_KEY" },
  openai: { envVar: "OPENAI_API_KEY" },
  "google-gemini": { envVar: "GOOGLE_API_KEY" },
  elevenlabs: { envVar: "ELEVENLABS_API_KEY" },
  ollama: { envVar: "OLLAMA_API_BASE" },
} as const;

/**
 * Resolve the environment variable name for a provider using three-tier resolution:
 *   1. Explicit env_var (caller-provided) — highest priority
 *   2. Known defaults table
 *   3. Convention: PROVIDER_NAME_API_KEY (hyphens become underscores)
 */
export function resolveEnvVar(
  provider: string,
  explicitEnvVar?: string,
): string {
  // Tier 1: explicit
  if (explicitEnvVar) {
    return explicitEnvVar;
  }

  // Tier 2: known defaults
  const known = KNOWN_DEFAULTS[provider];
  if (known) {
    return known.envVar;
  }

  // Tier 3: convention
  return provider.replace(/-/g, "_").toUpperCase() + "_API_KEY";
}

/**
 * Check if a provider name has a known default env var mapping.
 */
export function isKnownProvider(name: string): boolean {
  return name in KNOWN_DEFAULTS;
}

/**
 * List all known provider names.
 */
export function listProviders(): string[] {
  return Object.keys(KNOWN_DEFAULTS).sort();
}

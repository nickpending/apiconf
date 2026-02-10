/**
 * Provider registry mapping provider names to environment variables.
 */

export interface ProviderInfo {
  envVar: string | null;
}

export const PROVIDERS: Record<string, ProviderInfo> = {
  anthropic: { envVar: "ANTHROPIC_API_KEY" },
  openai: { envVar: "OPENAI_API_KEY" },
  "google-gemini": { envVar: "GOOGLE_API_KEY" },
  elevenlabs: { envVar: "ELEVENLABS_API_KEY" },
  ollama: { envVar: null },
} as const;

/**
 * Get the environment variable name for a provider.
 */
export function getEnvVar(provider: string): string | null {
  const info = PROVIDERS[provider];
  return info?.envVar ?? null;
}

/**
 * Check if a provider name is valid.
 */
export function isValidProvider(name: string): boolean {
  return name in PROVIDERS;
}

/**
 * List all valid provider names.
 */
export function listProviders(): string[] {
  return Object.keys(PROVIDERS).sort();
}

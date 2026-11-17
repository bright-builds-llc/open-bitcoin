export type ParitySurface = {
  id?: unknown;
  name?: unknown;
  requirements?: unknown;
  status?: unknown;
  upstream?: { sources?: unknown; tests?: unknown };
};
export type ParityIndex = { surfaces?: unknown; checklist?: { surfaces?: unknown } };

/** Parses `docs/parity/index.json`; records a failure and returns null when it is not JSON. */
export function parseParityIndex(raw: string, failures: string[]): ParityIndex | null {
  try {
    return JSON.parse(raw) as ParityIndex;
  } catch (error) {
    failures.push(`invalid parity index JSON: ${String(error)}`);
    return null;
  }
}

export function asSurfaceArray(value: unknown): ParitySurface[] {
  return Array.isArray(value)
    ? value.filter((item): item is ParitySurface => typeof item === "object" && item !== null)
    : [];
}

export function asStringArray(value: unknown): string[] {
  return Array.isArray(value)
    ? value.filter((item): item is string => typeof item === "string")
    : [];
}

/** Order-insensitive equality; assumes neither side repeats a value. */
export function sameMembers(actual: string[], expected: readonly string[]): boolean {
  return actual.length === expected.length && expected.every((value) => actual.includes(value));
}

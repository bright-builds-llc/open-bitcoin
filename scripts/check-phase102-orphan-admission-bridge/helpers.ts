import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { NO_CLAIM_MARKERS } from "./constants.ts";

export function requireExactRequirements(
  value: unknown,
  expected: readonly string[],
  label: string,
  failures: string[],
): void {
  if (!Array.isArray(value)) {
    failures.push(`${label} requirements must be an array`);
    return;
  }

  const actual = JSON.stringify(value);
  const wanted = JSON.stringify(expected);
  if (actual !== wanted) {
    failures.push(`${label} requirements mismatch: expected ${wanted}, got ${actual}`);
  }
}

export function requireArrayIncludes(
  value: unknown,
  needle: string,
  message: string,
  failures: string[],
): void {
  if (!Array.isArray(value) || !value.includes(needle)) {
    failures.push(message);
  }
}

export function requireContains(
  text: string,
  needle: string,
  message: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(message);
  }
}

export function verifyOrderedCommands(
  text: string,
  commands: readonly string[],
  label: string,
  failures: string[],
): void {
  let lastIndex = -1;
  for (const command of commands) {
    const index = text.indexOf(command);
    if (index < 0) {
      failures.push(`${label}: missing command ${command}`);
      return;
    }
    if (index <= lastIndex) {
      failures.push(`${label}: command out of order ${command}`);
      return;
    }
    lastIndex = index;
  }
}

export function executableVerifyText(text: string): string {
  return text.replace(/^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m, "");
}

export function hasNoClaimMarker(lowerUnit: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => lowerUnit.includes(marker));
}

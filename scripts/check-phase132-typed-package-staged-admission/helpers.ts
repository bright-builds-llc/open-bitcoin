import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export function requireAll(
  text: string,
  needles: readonly string[],
  failure: string,
  failures: string[],
): void {
  if (!needles.every((needle) => text.includes(needle))) {
    failures.push(failure);
  }
}

export function requireOrdered(
  text: string,
  needles: readonly string[],
  failure: string,
  failures: string[],
): void {
  if (!orderedOffsets(text, needles)) failures.push(failure);
}

export function orderedOffsets(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const next = text.indexOf(needle, cursor + 1);
    if (next === -1) return false;
    cursor = next;
  }
  return true;
}

export function hasPrivateFields(
  text: string,
  name: string,
  fields: readonly string[],
): boolean {
  const start = text.indexOf(`pub struct ${name} {`);
  if (start === -1) return false;
  const end = text.indexOf("\n}", start);
  if (end === -1) return false;
  const body = text.slice(start, end);
  return (
    fields.every(
      (field) => body.includes(`    ${field},`) && !body.includes(`pub ${field}`),
    ) && !/\n\s+pub(?:\([^)]*\))?\s+\w+\s*:/.test(body)
  );
}

export function near(
  text: string,
  anchor: string,
  needle: string,
  before: boolean,
): boolean {
  const anchorIndex = text.indexOf(anchor);
  const needleIndex = text.indexOf(needle);
  if (anchorIndex === -1 || needleIndex === -1) return false;
  return before ? needleIndex < anchorIndex : needleIndex > anchorIndex;
}

export function countMatches(text: string, pattern: RegExp): number {
  return Array.from(text.matchAll(pattern)).length;
}

export function sectionBetween(text: string, startNeedle: string, endNeedle: string): string {
  const start = text.indexOf(startNeedle);
  if (start === -1) return "";
  const end = text.indexOf(endNeedle, start + startNeedle.length);
  return end === -1 ? text.slice(start) : text.slice(start, end);
}

export function visibleCommandOrder(text: string): string {
  const marker = ": <<'VERIFY_COMMAND_ORDER'\n";
  const start = text.indexOf(marker);
  if (start === -1) return "";
  const bodyStart = start + marker.length;
  const end = text.indexOf("\nVERIFY_COMMAND_ORDER", bodyStart);
  return end === -1 ? "" : text.slice(bodyStart, end);
}

export function orderedLines(text: string, required: readonly string[]): boolean {
  const lines = text.split("\n").map((line) => line.trim());
  let cursor = -1;
  for (const line of required) {
    const index = lines.indexOf(line, cursor + 1);
    if (index === -1) return false;
    cursor = index;
  }
  return true;
}

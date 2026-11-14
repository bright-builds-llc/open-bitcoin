import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { NO_CLAIM_MARKERS, POSITIVE_CLAIM_PATTERNS } from "./constants.ts";

export function requireGateBeforeMutation(
  section: string,
  gateNeedle: string,
  mutationNeedles: readonly string[],
  label: string,
  failures: string[],
): void {
  if (section.length === 0) {
    failures.push(`scheduler-gate: missing ${label} section`);
    return;
  }
  const gateIndex = section.indexOf(gateNeedle);
  if (gateIndex === -1) {
    failures.push(`scheduler-gate: ${label} missing relay eligibility gate ${gateNeedle}`);
    return;
  }

  for (const mutationNeedle of mutationNeedles) {
    const mutationIndex = section.indexOf(mutationNeedle);
    if (mutationIndex !== -1 && mutationIndex < gateIndex) {
      failures.push(`scheduler-gate: ${label} eligibility gate must appear before insert_in_flight and insert_candidate`);
      return;
    }
  }
}

export function sectionBetween(text: string, startNeedle: string, endNeedle: string): string {
  const start = text.indexOf(startNeedle);
  if (start === -1) {
    return "";
  }
  const end = text.indexOf(endNeedle, start + startNeedle.length);
  return end === -1 ? text.slice(start) : text.slice(start, end);
}

export function requireContains(text: string, needle: string, message: string, failures: string[]): void {
  if (!text.includes(needle)) {
    failures.push(message);
  }
}

export function asStringArray(value: unknown): string[] {
  return Array.isArray(value) ? value.filter((item): item is string => typeof item === "string") : [];
}

export function sameMembers(actual: string[], expected: string[]): boolean {
  return actual.length === expected.length && expected.every((item) => actual.includes(item));
}

export function orderedIndexes(text: string, needles: readonly string[]): boolean {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) {
      return false;
    }
    cursor = index;
  }
  return true;
}

export function normalizeWhitespace(text: string): string {
  return text.replace(/\s+/g, " ").trim();
}

export function markdownParagraphs(text: string): Array<{ startLine: number; text: string }> {
  const paragraphs: Array<{ startLine: number; text: string }> = [];
  let startLine = 1;
  let current: string[] = [];
  for (const [index, line] of text.split("\n").entries()) {
    const trimmed = line.trim();
    if (trimmed === "" || (trimmed.startsWith("- ") && current.length > 0)) {
      if (current.length > 0) {
        paragraphs.push({ startLine, text: current.join(" ") });
        current = [];
      }
      startLine = index + 2;
      if (trimmed === "") {
        continue;
      }
    }
    if (current.length === 0) {
      startLine = index + 1;
    }
    current.push(line);
  }
  if (current.length > 0) {
    paragraphs.push({ startLine, text: current.join(" ") });
  }
  return paragraphs;
}

export function hasNoClaimMarker(line: string): boolean {
  return NO_CLAIM_MARKERS.some((marker) => line.includes(marker));
}

export function hasPositiveClaim(line: string): boolean {
  return POSITIVE_CLAIM_PATTERNS.some((patternValue) => patternValue.test(line));
}

export function isPublicEvidenceFile(file: string): boolean {
  return (
    file === "README.md" ||
    file.startsWith("docs/architecture/") ||
    file === "docs/operator/runtime-guide.md" ||
    file.startsWith("docs/parity/catalog/") ||
    file === "docs/parity/checklist.md" ||
    file === "packages/open-bitcoin-node/src/status/relay_evidence.rs"
  );
}

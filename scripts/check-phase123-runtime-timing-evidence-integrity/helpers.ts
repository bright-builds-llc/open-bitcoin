import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

export function normalizeWhitespace(text: string): string {
  return text.replaceAll(/\s+/g, " ").trim();
}

export function requireContains(text: string, needle: string, label: string, failures: string[]): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

export function requireAbsent(text: string, needle: string, label: string, failures: string[]): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

export function requireExactCount(
  text: string,
  needle: string,
  expected: number,
  label: string,
  failures: string[],
): void {
  const actual = text.split(needle).length - 1;
  if (actual !== expected) {
    failures.push(`${label} expected ${expected} occurrence(s) of ${needle}, found ${actual}`);
  }
}

export function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) failures.push(`${label} missing or out of order ${needle}`);
    else cursor = index;
  }
}

export function requireRepeatedOrder(
  text: string,
  needles: readonly string[],
  repetitions: number,
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (let repetition = 0; repetition < repetitions; repetition += 1) {
    for (const needle of needles) {
      const index = text.indexOf(needle, cursor + 1);
      if (index === -1) {
        failures.push(`${label} missing repetition ${repetition + 1}: ${needle}`);
        return;
      }
      cursor = index;
    }
  }
}

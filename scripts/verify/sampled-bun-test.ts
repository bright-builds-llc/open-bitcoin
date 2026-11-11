// Stands in for "bun:test" in phase-checker test files during verification.
//
// Those suites generate thousands of cases by looping one test over a table.
// Every row runs the same checker code path, so each call site keeps a spread
// of its rows instead of all of them. Set OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS=1
// to run every case.

import { afterAll, test as bunTest } from "bun:test";

export * from "bun:test";

const KEEP_FIRST = 5;
const EXHAUSTIVE = process.env.OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS === "1";
const THIS_FILE = import.meta.path;

const registeredBySite = new Map<string, number>();
let registered = 0;
let kept = 0;

/** True for the first rows of a site, then for Fibonacci-spaced rows after them. */
export function keepsSampledRow(index: number): boolean {
  if (index < KEEP_FIRST) return true;
  let previous = 5;
  let current = 8;
  while (current < index) [previous, current] = [current, previous + current];
  return current === index;
}

// A site is the chain of calls that reached test(), so one helper called from
// several places counts as several sites, and one loop counts as one.
function callSite(): string {
  const frames = (new Error().stack ?? "").split("\n").slice(1);
  return frames.filter((frame) => !frame.includes(THIS_FILE)).join("|");
}

function keepsNextCase(): boolean {
  const site = callSite();
  const index = registeredBySite.get(site) ?? 0;
  registeredBySite.set(site, index + 1);
  registered += 1;
  if (!EXHAUSTIVE && !keepsSampledRow(index)) return false;
  kept += 1;
  return true;
}

export const test = new Proxy(bunTest, {
  apply(target, thisArgument, args) {
    if (!keepsNextCase()) return undefined;
    return Reflect.apply(target, thisArgument, args);
  },
  // Modifiers such as test.each and test.skip stay Bun's own and unsampled;
  // plain methods need Bun's test object, not this proxy, as their receiver.
  get(target, property) {
    const value = Reflect.get(target, property, target);
    if (typeof value !== "function" || "each" in value) return value;
    return value.bind(target);
  },
}) as typeof bunTest;

export const it = test;

afterAll(() => {
  if (registered === kept) return;
  console.error(`sampled checker cases: ran ${kept} of ${registered} (OPEN_BITCOIN_EXHAUSTIVE_CHECKER_TESTS=1 runs all)`);
});

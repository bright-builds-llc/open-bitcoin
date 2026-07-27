import { afterEach, expect, test } from "bun:test";
import { mkdirSync, rmSync, writeFileSync } from "node:fs";
import path from "node:path";

import { readSourceCorpus } from "./source-corpus";

const repoRoot = path.resolve(import.meta.dir, "..");
const childDirectory = path.join(import.meta.dir, "check-phase73-uat-verification");
const untrackedPath = path.join(childDirectory, `source-corpus-untracked-${process.pid}.ts`);
const ignoredDirectory = path.join(childDirectory, `mutants.out-source-corpus-${process.pid}`);
const ignoredPath = path.join(ignoredDirectory, "sentinel.ts");

afterEach(() => {
  rmSync(untrackedPath, { force: true });
  rmSync(ignoredDirectory, { force: true, recursive: true });
});

test("reads tracked children while excluding untracked and ignored children", () => {
  // Arrange
  const untrackedToken = `untracked-token-${process.pid}`;
  const ignoredToken = `ignored-token-${process.pid}`;
  writeFileSync(untrackedPath, untrackedToken);
  mkdirSync(ignoredDirectory, { recursive: true });
  writeFileSync(ignoredPath, ignoredToken);

  // Act
  const corpus = readSourceCorpus(repoRoot, "scripts/check-phase73-uat-verification.ts");

  // Assert
  expect(corpus).toContain('export const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE73_REPO_ROOT"');
  expect(corpus).not.toContain(untrackedToken);
  expect(corpus).not.toContain(ignoredToken);
});

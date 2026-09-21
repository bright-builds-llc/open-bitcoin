import {
  mkdirSync,
  mkdtempSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { expect } from "bun:test";

import {
  PHASE134_TARGET_FILES,
  checkPhase134AuthoritativeLifecycle,
} from "../check-phase134-authoritative-lifecycle";
import { SCOPE_CLAIM_SOURCE_FILES } from "./scope-claims";
import {
  ARCHIVED_V22_REQUIREMENTS,
  V22_REQUIREMENTS_MILESTONE_NEEDLE,
  readPlanningRequirements,
  readSourceRoot,
} from "../source-corpus";

export const REPO_ROOT = path.resolve(import.meta.dir, "../..");
export const tempRoots: string[] = [];
export type Mutator = (files: Map<string, string>) => void;
export type MutationCase = readonly [string, string, Mutator];

export function assertExactFailure(expectedFailure: string, mutate: Mutator): void {
  // Arrange
  const root = createFixture(
    [...new Set([...PHASE134_TARGET_FILES, ...SCOPE_CLAIM_SOURCE_FILES])],
    mutate,
  );

  // Act
  const failures = checkPhase134AuthoritativeLifecycle(root);

  // Assert
  expect(failures).toEqual([expectedFailure]);
}

export function createFixture(
  relativePaths: readonly string[],
  maybeMutate?: Mutator,
): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase134-check-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  for (const relativePath of relativePaths) {
    files.set(
      relativePath,
      relativePath === ".planning/REQUIREMENTS.md"
        ? readPlanningRequirements(
            REPO_ROOT,
            ARCHIVED_V22_REQUIREMENTS,
            V22_REQUIREMENTS_MILESTONE_NEEDLE,
          )
        : readSourceRoot(REPO_ROOT, relativePath),
    );
  }
  maybeMutate?.(files);
  for (const [relativePath, contents] of files) {
    const destination = path.join(root, relativePath);
    mkdirSync(path.dirname(destination), { recursive: true });
    writeFileSync(destination, contents);
  }
  return root;
}

export function replace(relativePath: string, search: string, replacement: string): Mutator {
  return (files) => {
    const source = requireFile(files, relativePath);
    expect(source).toContain(search);
    files.set(relativePath, source.replace(search, replacement));
  };
}

export function replaceNth(
  relativePath: string,
  search: string,
  replacement: string,
  occurrence: number,
): Mutator {
  return (files) => {
    let source = requireFile(files, relativePath);
    let cursor = 0;
    for (let index = 1; index <= occurrence; index += 1) {
      const found = source.indexOf(search, cursor);
      expect(found).toBeGreaterThanOrEqual(0);
      if (index === occurrence) {
        source =
          source.slice(0, found) +
          replacement +
          source.slice(found + search.length);
        files.set(relativePath, source);
        return;
      }
      cursor = found + search.length;
    }
  };
}

export function append(relativePath: string, addition: string): Mutator {
  return (files) => {
    files.set(relativePath, requireFile(files, relativePath) + addition);
  };
}

export function insertAfter(relativePath: string, marker: string, addition: string): Mutator {
  return replace(relativePath, marker, marker + addition);
}

export function replaceInFirst(
  relativePaths: readonly string[],
  search: string,
  replacement: string,
): Mutator {
  return (files) => {
    const relativePath = relativePaths.find((candidate) =>
      requireFile(files, candidate).includes(search),
    );
    expect(relativePath).toBeDefined();
    replace(relativePath ?? "", search, replacement)(files);
  };
}

export function insertInFunction(
  files: Map<string, string>,
  relativePath: string,
  functionMarker: string,
  addition: string,
): void {
  const source = requireFile(files, relativePath);
  const functionStart = source.indexOf(functionMarker);
  expect(functionStart).toBeGreaterThanOrEqual(0);
  const brace = source.indexOf("{", functionStart);
  expect(brace).toBeGreaterThanOrEqual(0);
  files.set(
    relativePath,
    source.slice(0, brace + 1) + addition + source.slice(brace + 1),
  );
}

export function requireFile(files: Map<string, string>, relativePath: string): string {
  const maybeSource = files.get(relativePath);
  if (maybeSource === undefined) {
    throw new Error(`missing fixture file: ${relativePath}`);
  }
  return maybeSource;
}

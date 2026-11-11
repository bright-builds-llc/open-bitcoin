import { afterEach } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  CLOSEOUT_SURFACE,
  D14_SENTENCE,
  HISTORICAL_PHASE138_DIR,
  REQUIRED_BREADCRUMB_GROUPS,
  REQUIRED_DOC_FILES,
  REQUIRED_KNOTS_ANCHORS,
  REQUIRED_TOP_LEVEL_NAMES,
  REQUIRED_UAT_COMMANDS,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  STORED_BLOCK_PRESENCE_ANCHOR,
  STORED_BLOCK_PRESENCE_GROUP,
  UAT_PACKAGE,
  allV23RequirementIds,
  type RequiredDocFile,
} from "./constants.ts";

export type FixtureOptions = {
  maybeMutate?: (files: Map<string, string>) => void;
};

export const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

export function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase145-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  const scopedClaim = `Open Bitcoin provides ${D14_SENTENCE}.`;
  for (const file of REQUIRED_DOC_FILES) {
    files.set(file, scopedClaim);
  }
  files.set("docs/parity/index.json", JSON.stringify(createParityIndex(), null, 2));
  files.set("docs/parity/source-breadcrumbs.json", createBreadcrumbs());
  files.set("docs/operator/runtime-guide.md", createRuntimeGuide(scopedClaim));
  files.set(UAT_PACKAGE, createUatPackage(scopedClaim));
  files.set(REQUIREMENTS_FILE, createRequirements());
  files.set(
    "scripts/check-phase138-parity-uat-release-boundary.ts",
    `// historical verifier evidence ${HISTORICAL_PHASE138_DIR}\n`,
  );
  files.set(`${HISTORICAL_PHASE138_DIR}/138-CONTEXT.md`, "historical Phase 138 evidence\n");
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  return root;
}

export function createParityIndex(): {
  surfaces: Array<{ name: string; status: string }>;
  checklist: {
    surfaces: Array<{
      id: string;
      requirements: string[];
      status: string;
      upstream: { sources: string[]; tests: string[] };
    }>;
  };
} {
  return {
    surfaces: REQUIRED_TOP_LEVEL_NAMES.map((name) => ({ name, status: "done" })),
    checklist: {
      surfaces: Object.entries(REQUIREMENTS_BY_SURFACE).map(([id, requirements]) => ({
        id,
        requirements: [...requirements],
        status: "done",
        upstream:
          id === CLOSEOUT_SURFACE
            ? {
                sources: [...REQUIRED_KNOTS_ANCHORS],
                tests: ["packages/bitcoin-knots/src/test/blockmanager_tests.cpp"],
              }
            : { sources: [], tests: [] },
      })),
    },
  };
}

export function createRequirements(): string {
  return allV23RequirementIds()
    .map((id) => `- [x] **${id}**`)
    .join("\n");
}

export function createBreadcrumbs(): string {
  return JSON.stringify({
    groups: REQUIRED_BREADCRUMB_GROUPS.map((label) => ({
      label,
      files: [`${label}.rs`],
      breadcrumbs:
        label === STORED_BLOCK_PRESENCE_GROUP
          ? [STORED_BLOCK_PRESENCE_ANCHOR]
          : [...REQUIRED_KNOTS_ANCHORS],
    })),
  });
}

export function createRuntimeGuide(scopedClaim: string): string {
  return ["## Phase 145 Parity UAT And Release Boundary Review", scopedClaim, ...REQUIRED_UAT_COMMANDS].join(
    "\n",
  );
}

export function createUatPackage(scopedClaim: string): string {
  return [scopedClaim, ...REQUIRED_UAT_COMMANDS].join("\n");
}

export function mutateIndex(files: Map<string, string>, mutate: (index: any) => void): void {
  const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}") as any;
  mutate(index);
  files.set("docs/parity/index.json", JSON.stringify(index, null, 2));
}

export function replace(files: Map<string, string>, file: string, needle: string, value: string): void {
  files.set(file, (files.get(file) ?? "").replace(needle, value));
}

export function append(files: Map<string, string>, file: string, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n\n${value}`);
}

export type { RequiredDocFile };

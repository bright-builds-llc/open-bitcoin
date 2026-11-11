import { afterEach } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  BREADCRUMB_ANCHOR,
  BREADCRUMB_GROUP,
  CATALOG_FILES,
  CLOSEOUT_SURFACE,
  FJALL_DIFFERENCE_SENTENCE,
  GRD01_CHECKBOX,
  HISTORICAL_PHASE_DIR,
  PUBLIC_NETWORK_NOT_RUN_LINE,
  REQUIRED_CLOSEOUT_SOURCES,
  REQUIRED_DOC_FILES,
  REQUIRED_KNOTS_SYMBOLS,
  REQUIRED_TOP_LEVEL_NAMES,
  REQUIRED_UAT_COMMANDS,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  ROADMAP_FILE,
  UAT_PACKAGE,
  V23_D14_SENTENCE,
  V24_PRUNE_SENTENCE,
  checkedV24RequirementIds,
  type RequiredDocFile,
} from "./constants.ts";

export type FixtureOptions = {
  maybeMutate?: (files: Map<string, string>) => void;
};

type FixtureIndex = {
  surfaces: Array<{ name: string; status: string }>;
  checklist: {
    surfaces: Array<{
      id: string;
      requirements: string[];
      status: string;
      upstream: { sources: string[]; tests: string[] };
    }>;
  };
};

export const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

export function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase151-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  const foundation = `${V23_D14_SENTENCE}\n\n${V24_PRUNE_SENTENCE}`;
  for (const file of REQUIRED_DOC_FILES) {
    files.set(file, foundation);
  }
  const catalog = [foundation, FJALL_DIFFERENCE_SENTENCE, ...REQUIRED_KNOTS_SYMBOLS].join("\n");
  for (const file of CATALOG_FILES) {
    files.set(file, catalog);
  }
  files.set("docs/parity/index.json", JSON.stringify(createParityIndex(), null, 2));
  files.set("docs/parity/source-breadcrumbs.json", createBreadcrumbs());
  files.set(UAT_PACKAGE, createUatPackage());
  files.set(REQUIREMENTS_FILE, createRequirements());
  files.set(ROADMAP_FILE, createRoadmap());
  files.set(
    "scripts/check-phase151-parity-uat-release-boundary.ts",
    `// historical verifier evidence ${HISTORICAL_PHASE_DIR}\n`,
  );
  files.set(`${HISTORICAL_PHASE_DIR}/145-CONTEXT.md`, "historical Phase 145 evidence\n");
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  return root;
}

export function createParityIndex(): FixtureIndex {
  return {
    surfaces: REQUIRED_TOP_LEVEL_NAMES.map((name) => ({ name, status: "done" })),
    checklist: {
      surfaces: Object.entries(REQUIREMENTS_BY_SURFACE).map(([id, requirements]) => ({
        id,
        requirements: [...requirements],
        status: "done",
        upstream:
          id === CLOSEOUT_SURFACE
            ? { sources: [...REQUIRED_CLOSEOUT_SOURCES], tests: [] }
            : { sources: [], tests: [] },
      })),
    },
  };
}

export function createRequirements(): string {
  return ["**Milestone:** v2.4 Prune-Mode Product Behavior", ...checkedV24RequirementIds().map((id) => `- [x] **${id}**`), GRD01_CHECKBOX].join("\n");
}

export function createRoadmap(): string {
  return [
    "## Active Milestone: v2.4 Prune-Mode Product Behavior",
    "| OPER-01 | Phase 150 | Complete |",
    "| OPER-02 | Phase 150 | Complete |",
    "| OPER-03 | Phase 150 | Complete |",
    "| LOCK-02 | Phase 150 | Complete |",
    "| GRD-01 | Phase 151 | Complete |",
  ].join("\n");
}

export function createBreadcrumbs(): string {
  return JSON.stringify({
    groups: [
      {
        label: BREADCRUMB_GROUP,
        files: ["operator/prune.rs"],
        breadcrumbs: [BREADCRUMB_ANCHOR],
      },
    ],
  });
}

export function createUatPackage(): string {
  return [...REQUIRED_UAT_COMMANDS, PUBLIC_NETWORK_NOT_RUN_LINE].join("\n");
}

export function mutateIndex(files: Map<string, string>, mutate: (index: FixtureIndex) => void): void {
  const index = JSON.parse(files.get("docs/parity/index.json") ?? "{}") as FixtureIndex;
  mutate(index);
  files.set("docs/parity/index.json", JSON.stringify(index, null, 2));
}

export function replace(
  files: Map<string, string>,
  file: string,
  needle: string,
  value: string,
): void {
  files.set(file, (files.get(file) ?? "").replace(needle, value));
}

export function append(files: Map<string, string>, file: string, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n\n${value}`);
}

export type { RequiredDocFile };

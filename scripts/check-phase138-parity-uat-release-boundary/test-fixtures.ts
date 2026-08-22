import { afterEach } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import {
  CLOSEOUT_SURFACE,
  COMPOSITION_FILE,
  COMPOSITION_SYMBOL,
  D21_SENTENCE,
  PHASE117_CHECK,
  PHASE117_CHECK_STEP,
  PHASE117_TEST,
  PHASE117_TEST_STEP,
  PHASE138_CHECK,
  PHASE138_CHECK_STEP,
  PHASE138_TEST,
  PHASE138_TEST_STEP,
  RECONCILIATION_CHECK,
  RECONCILIATION_CHECK_STEP,
  RECONCILIATION_TEST,
  RECONCILIATION_TEST_STEP,
  REQUIRED_BENCH_TOKEN,
  REQUIRED_BREADCRUMB_GROUPS,
  REQUIRED_DOC_FILES,
  REQUIRED_KNOTS_ANCHORS,
  REQUIRED_TOP_LEVEL_NAMES,
  REQUIRED_UAT_COMMANDS,
  REQUIREMENTS_BY_SURFACE,
  REQUIREMENTS_FILE,
  RUNNABLE_CARGO_FILTERS,
  SNAPSHOT_CHECKLIST_ID,
  SNAPSHOT_TOP_LEVEL_NAME,
  THRESHOLD_FREE,
  UAT_PACKAGE,
  allV22RequirementIds,
  type RequiredDocFile,
} from "./constants.ts";
import { COMPOSITION_CELL, MATRIX_CELLS } from "./matrix.ts";

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
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase138-"));
  tempRoots.push(root);
  const files = new Map<string, string>();
  const scopedClaim = `Open Bitcoin provides ${D21_SENTENCE}.`;
  for (const file of REQUIRED_DOC_FILES) {
    files.set(file, scopedClaim);
  }
  files.set("docs/parity/index.json", JSON.stringify(createParityIndex(), null, 2));
  files.set(REQUIREMENTS_FILE, createRequirements());
  files.set("docs/parity/source-breadcrumbs.json", createBreadcrumbs());
  files.set("docs/operator/runtime-guide.md", createRuntimeGuide(scopedClaim));
  files.set(UAT_PACKAGE, createUatPackage(scopedClaim));
  files.set("scripts/verify.sh", createVerifyScript());
  files.set("scripts/check-benchmark-report.ts", `${THRESHOLD_FREE}\n`);
  files.set("packages/open-bitcoin-bench/src/cases/mempool.rs", `${REQUIRED_BENCH_TOKEN}\n`);
  for (const cell of MATRIX_CELLS) {
    const existing = files.get(cell.file) ?? "";
    files.set(cell.file, existing.includes(cell.symbol) ? existing : `${existing}${cell.symbol}\n`);
  }
  const compositionText = files.get(COMPOSITION_FILE) ?? "";
  files.set(
    COMPOSITION_FILE,
    compositionText.includes(COMPOSITION_SYMBOL)
      ? compositionText
      : `${compositionText}${COMPOSITION_SYMBOL}\n`,
  );
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
            ? { sources: [...REQUIRED_KNOTS_ANCHORS], tests: [] }
            : { sources: [], tests: [] },
      })),
    },
  };
}

export function createRequirements(): string {
  return allV22RequirementIds()
    .map((id) => `- [x] **${id}**`)
    .join("\n");
}

export function createBreadcrumbs(): string {
  return JSON.stringify({
    groups: REQUIRED_BREADCRUMB_GROUPS.map((label) => ({
      label,
      files: [`${label}.rs`],
      breadcrumbs: [...REQUIRED_KNOTS_ANCHORS],
    })),
  });
}

export function createRuntimeGuide(scopedClaim: string): string {
  return [
    "## Phase 138 Parity UAT And Release Boundary Review",
    scopedClaim,
    ...REQUIRED_UAT_COMMANDS,
    ...RUNNABLE_CARGO_FILTERS,
  ].join("\n");
}

export function createUatPackage(scopedClaim: string): string {
  return [scopedClaim, ...REQUIRED_UAT_COMMANDS, ...RUNNABLE_CARGO_FILTERS].join("\n");
}

export function createVerifyScript(options: { place138AfterReconciliation?: boolean } = {}): string {
  const visible138 = [PHASE138_TEST, PHASE138_CHECK];
  const visibleReconciliation = [RECONCILIATION_TEST, RECONCILIATION_CHECK];
  const visibleTail = options.place138AfterReconciliation
    ? [...visibleReconciliation, ...visible138]
    : [...visible138, ...visibleReconciliation];
  const executable138 = [PHASE138_TEST_STEP, PHASE138_CHECK_STEP];
  const executableReconciliation = [RECONCILIATION_TEST_STEP, RECONCILIATION_CHECK_STEP];
  const executableTail = options.place138AfterReconciliation
    ? [...executableReconciliation, ...executable138]
    : [...executable138, ...executableReconciliation];
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE117_TEST,
    PHASE117_CHECK,
    ...visibleTail,
    "VERIFY_COMMAND_ORDER",
    PHASE117_TEST_STEP,
    PHASE117_CHECK_STEP,
    ...executableTail,
  ].join("\n");
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

export function removeSymbol(files: Map<string, string>, file: string, symbol: string): void {
  files.set(file, (files.get(file) ?? "").replaceAll(symbol, ""));
}

export { COMPOSITION_CELL, COMPOSITION_FILE, COMPOSITION_SYMBOL, SNAPSHOT_CHECKLIST_ID };
export type { RequiredDocFile };

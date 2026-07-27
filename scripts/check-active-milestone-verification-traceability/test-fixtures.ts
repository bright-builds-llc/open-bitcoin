import { afterEach, expect, test } from "bun:test";
import {
  existsSync,
  mkdirSync,
  mkdtempSync,
  rmSync,
  writeFileSync,
} from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkActiveMilestoneVerificationTraceability } from "../check-active-milestone-verification-traceability";

export const PHASE115_DIR =
  ".planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff";
export const PHASE125_DIR =
  ".planning/phases/125-compact-download-verification-traceability-closure";
export const PHASE115_LIFECYCLE_ID = "115-fixture-lifecycle";
export const PHASE125_LIFECYCLE_ID = "125-fixture-lifecycle";
export const REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
export const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
export const PHASE115_SUMMARY = `${PHASE115_DIR}/115-01-SUMMARY.md`;
export const PHASE125_CONTEXT = `${PHASE125_DIR}/125-CONTEXT.md`;
export const PHASE125_VERIFICATION = `${PHASE125_DIR}/125-VERIFICATION.md`;

export type FixtureFile = string;
export type FixtureOptions = {
  maybeMutateFiles?: (files: Map<FixtureFile, string>) => void;
};

export const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

export function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(
    path.join(tmpdir(), "open-bitcoin-verification-traceability-"),
  );
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${contents}\n`);
  }

  return root;
}

export function fixtureFiles(): Map<FixtureFile, string> {
  return new Map([
    [".planning/ROADMAP.md", roadmapText()],
    [".planning/REQUIREMENTS.md", requirementsText()],
    [`${PHASE115_DIR}/115-CONTEXT.md`, contextText(PHASE115_LIFECYCLE_ID)],
    [
      `${PHASE115_DIR}/115-01-SUMMARY.md`,
      summaryText(REQUIREMENTS, PHASE115_LIFECYCLE_ID),
    ],
    [`${PHASE125_DIR}/125-CONTEXT.md`, contextText(PHASE125_LIFECYCLE_ID)],
    [
      `${PHASE125_DIR}/125-VERIFICATION.md`,
      verificationText(REQUIREMENTS, PHASE125_LIFECYCLE_ID),
    ],
  ]);
}

export function orphanFailure(requirementId: string): string {
  return `activated requirement ${requirementId} is missing lifecycle-valid active-phase verification coverage`;
}

export function replaceInFile(
  files: Map<FixtureFile, string>,
  relativePath: string,
  needle: string,
  replacement: string,
): void {
  files.set(
    relativePath,
    (files.get(relativePath) ?? "").replace(needle, replacement),
  );
}

export function appendToFile(
  files: Map<FixtureFile, string>,
  relativePath: string,
  value: string,
): void {
  files.set(relativePath, `${files.get(relativePath) ?? ""}\n${value}`);
}

export function insertBefore(
  files: Map<FixtureFile, string>,
  relativePath: string,
  marker: string,
  value: string,
): void {
  replaceInFile(files, relativePath, marker, `${value}\n${marker}`);
}

export function roadmapText(): string {
  return [
    "# Roadmap",
    "## Active Milestone: v2.1 Fixture",
    "### Phases",
    "- [x] **Phase 115: Missing Transaction Round Trip** - Historical implementation.",
    "- [ ] **Phase 125: Verification Traceability Closure** - Active closure.",
    "### Phase Details",
    "Fixture details.",
    "## Progress",
  ].join("\n");
}

export function requirementsText(): string {
  return [
    "# Requirements",
    "## v2.1 Requirements",
    ...REQUIREMENTS.map(
      (requirement) => `- [ ] **${requirement}**: Fixture requirement.`,
    ),
    "## Deferred Requirements",
    "- **FUT-01**: Deferred fixture requirement.",
    "## Traceability",
    "| Requirement | Phase | Status |",
    "| --- | --- | --- |",
    ...REQUIREMENTS.map(
      (requirement) => `| ${requirement} | Phase 125 | Pending |`,
    ),
  ].join("\n");
}

export function contextText(lifecycleId: string): string {
  return [
    "---",
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${lifecycleId}`,
    "---",
    "# Context",
  ].join("\n");
}

export function summaryText(
  requirements: readonly string[],
  lifecycleId: string,
): string {
  return [
    "---",
    `requirements-completed: [${requirements.join(", ")}]`,
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${lifecycleId}`,
    "---",
    "# Summary",
  ].join("\n");
}

export function verificationText(
  requirements: readonly string[],
  lifecycleId: string,
): string {
  return [
    "---",
    "status: passed",
    "lifecycle_validated: true",
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${lifecycleId}`,
    "---",
    "# Verification",
    ...requirements.map((requirement) => `Verified ${requirement}.`),
  ].join("\n");
}

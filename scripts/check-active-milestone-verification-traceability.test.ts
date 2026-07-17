import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkActiveMilestoneVerificationTraceability } from "./check-active-milestone-verification-traceability";

const PHASE115_DIR =
  ".planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff";
const PHASE125_DIR =
  ".planning/phases/125-compact-download-verification-traceability-closure";
const PHASE115_LIFECYCLE_ID = "115-fixture-lifecycle";
const PHASE125_LIFECYCLE_ID = "125-fixture-lifecycle";
const REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;

type FixtureFile = string;
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<FixtureFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("complete fixture succeeds", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

function createFixture(options: FixtureOptions = {}): string {
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

function fixtureFiles(): Map<FixtureFile, string> {
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

function roadmapText(): string {
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

function requirementsText(): string {
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

function contextText(lifecycleId: string): string {
  return [
    "---",
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${lifecycleId}`,
    "---",
    "# Context",
  ].join("\n");
}

function summaryText(
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

function verificationText(
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

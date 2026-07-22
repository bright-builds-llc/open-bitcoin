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

import { checkActiveMilestoneVerificationTraceability } from "./check-active-milestone-verification-traceability";

const PHASE115_DIR =
  ".planning/phases/115-missing-transaction-round-trip-fallback-and-validation-handoff";
const PHASE125_DIR =
  ".planning/phases/125-compact-download-verification-traceability-closure";
const PHASE115_LIFECYCLE_ID = "115-fixture-lifecycle";
const PHASE125_LIFECYCLE_ID = "125-fixture-lifecycle";
const REQUIREMENTS = ["RCN-04", "RCN-05", "RCN-06"] as const;
const REQUIREMENTS_FILE = ".planning/REQUIREMENTS.md";
const PHASE115_SUMMARY = `${PHASE115_DIR}/115-01-SUMMARY.md`;
const PHASE125_CONTEXT = `${PHASE125_DIR}/125-CONTEXT.md`;
const PHASE125_VERIFICATION = `${PHASE125_DIR}/125-VERIFICATION.md`;

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

test("current milestone headings and sibling phases section succeed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        ".planning/ROADMAP.md",
        [
          "# Roadmap",
          "## Active Milestone: v2.2 Fixture",
          "Fixture goal and boundary.",
          "## Phases",
          "- [x] **Phase 115: Missing Transaction Round Trip** - Historical implementation.",
          "- [ ] **Phase 125: Verification Traceability Closure** - Active closure.",
          "## Phase Details",
          "Fixture details.",
        ].join("\n"),
      );
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "## v2.1 Requirements",
        "## v2.2 Requirements",
      );
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "## Deferred Requirements",
        "## Future Requirements",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

test("completed milestone archive needs no active traceability corpus", () => {
  // Arrange
  const root = createFixture();
  writeFileSync(
    path.join(root, ".planning/ROADMAP.md"),
    "# Roadmap\n\n## Current Status\n\nNo active milestone.\n",
  );
  for (const name of [
    "v2.1-ROADMAP.md",
    "v2.1-REQUIREMENTS.md",
    "v2.1-MILESTONE-AUDIT.md",
  ]) {
    const directory = path.join(root, ".planning/milestones");
    mkdirSync(directory, { recursive: true });
    writeFileSync(path.join(directory, name), "archived\n");
  }

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

test("missing RCN-04 verification fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-04.",
        "Missing first requirement.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([orphanFailure("RCN-04")]);
});

test("missing RCN-05 verification fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-05.",
        "Missing second requirement.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([orphanFailure("RCN-05")]);
});

test("missing RCN-06 verification fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-06.",
        "Missing third requirement.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([orphanFailure("RCN-06")]);
});

test("pending unsummarized requirement remains excluded", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      insertBefore(
        files,
        REQUIREMENTS_FILE,
        "## Deferred Requirements",
        "- [ ] **PEND-01**: Pending and unsummarized.",
      );
      appendToFile(
        files,
        REQUIREMENTS_FILE,
        "| PEND-01 | Phase 125 | Pending |",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

test("completed unsummarized requirement fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "- [ ] **RCN-04**",
        "- [x] **RCN-04**",
      );
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "| RCN-04 | Phase 125 | Pending |",
        "| RCN-04 | Phase 125 | Complete |",
      );
      replaceInFile(
        files,
        PHASE115_SUMMARY,
        "requirements-completed: [RCN-04, RCN-05, RCN-06]",
        "requirements-completed: [RCN-05, RCN-06]",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([
    "completed active requirement RCN-04 has no requirements-completed summary activation",
  ]);
});

test("checked requirement with pending traceability fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "- [ ] **RCN-04**",
        "- [x] **RCN-04**",
      );
      replaceInFile(
        files,
        PHASE115_SUMMARY,
        "requirements-completed: [RCN-04, RCN-05, RCN-06]",
        "requirements-completed: [RCN-05, RCN-06]",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([
    "active requirement RCN-04 has inconsistent checklist and traceability completion state",
  ]);
});

test("unchecked requirement with complete traceability fails independently", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "| RCN-04 | Phase 125 | Pending |",
        "| RCN-04 | Phase 125 | Complete |",
      );
      replaceInFile(
        files,
        PHASE115_SUMMARY,
        "requirements-completed: [RCN-04, RCN-05, RCN-06]",
        "requirements-completed: [RCN-05, RCN-06]",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([
    "active requirement RCN-04 has inconsistent checklist and traceability completion state",
  ]);
});

test("deferred FUT summary collision remains excluded", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE115_SUMMARY,
        "requirements-completed: [RCN-04, RCN-05, RCN-06]",
        "requirements-completed: [RCN-04, RCN-05, RCN-06, FUT-01]",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

test("archived verification token collision remains excluded", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-04.",
        "No active first requirement token.",
      );
      files.set(
        ".planning/milestones/v2.0/phases/999-VERIFICATION.md",
        verificationText(["RCN-04"], "archived-lifecycle"),
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([orphanFailure("RCN-04")]);
});

test("duplicate checklist ownership fails", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      insertBefore(
        files,
        REQUIREMENTS_FILE,
        "## Deferred Requirements",
        "- [ ] **RCN-04**: Duplicate checklist ownership.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain("active requirement checklist duplicates RCN-04");
});

test("duplicate traceability ownership fails without activating an orphan", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        REQUIREMENTS_FILE,
        "| RCN-04 | Phase 125 | Pending |",
      );
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-04.",
        "No first requirement token.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain(
    "active requirement RCN-04 must have exactly one traceability row; found 2",
  );
  expect(failures).not.toContain(orphanFailure("RCN-04"));
});

test("missing traceability ownership fails without activating an orphan", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "| RCN-04 | Phase 125 | Pending |",
        "",
      );
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-04.",
        "No first requirement token.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain(
    "active requirement RCN-04 must have exactly one traceability row; found 0",
  );
  expect(failures).not.toContain(orphanFailure("RCN-04"));
});

test("owner phase missing from active roadmap fails", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        REQUIREMENTS_FILE,
        "| RCN-04 | Phase 125 | Pending |",
        "| RCN-04 | Phase 999 | Pending |",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain(
    "active requirement RCN-04 traceability owner Phase 999 is not in the active roadmap",
  );
});

test("duplicate active roadmap phase fails", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      insertBefore(
        files,
        ".planning/ROADMAP.md",
        "### Phase Details",
        "- [ ] **Phase 125: Duplicate phase** - Invalid ownership.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain(
    "active roadmap phase 125 appears more than once",
  );
});

test("malformed CONTEXT lifecycle metadata fails closed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_CONTEXT,
        "lifecycle_mode: yolo",
        "lifecycle_mode: yolo\nlifecycle_mode: yolo",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures.join("\n")).toContain(
    `${PHASE125_CONTEXT} requires exactly one lifecycle_mode field; found 2`,
  );
  expect(failures).toContain(orphanFailure("RCN-04"));
});

test("malformed VERIFICATION lifecycle metadata fails closed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
        [
          `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
          `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures.join("\n")).toContain(
    `${PHASE125_VERIFICATION} requires exactly one phase_lifecycle_id field; found 2`,
  );
  expect(failures).toContain(orphanFailure("RCN-05"));
});

test("invalid lifecycle verification cannot mask its only token", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "lifecycle_validated: true",
        "lifecycle_validated: false",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toContain(
    `${PHASE125_VERIFICATION} requires lifecycle_validated: true`,
  );
  expect(failures).toContain(orphanFailure("RCN-04"));
  expect(failures).toContain(orphanFailure("RCN-05"));
  expect(failures).toContain(orphanFailure("RCN-06"));
});

test("RCN-040 near-token collision does not cover RCN-04", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE125_VERIFICATION,
        "Verified RCN-04.",
        "Verified RCN-040.",
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([orphanFailure("RCN-04")]);
});

test("block-list requirements-completed has lifecycle-valid coverage", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        PHASE115_SUMMARY,
        "requirements-completed: [RCN-04, RCN-05, RCN-06]",
        [
          "requirements-completed:",
          "  - RCN-04",
          "  - RCN-05",
          "  - RCN-06",
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: root,
  });

  // Assert
  expect(failures).toEqual([]);
});

test("real repository reports only the documented staged orphans", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");
  const verificationExists = existsSync(
    path.join(repoRoot, PHASE125_VERIFICATION),
  );

  // Act
  const failures = checkActiveMilestoneVerificationTraceability({
    maybeRootDir: repoRoot,
  });

  // Assert
  if (verificationExists) {
    expect(failures).toEqual([]);
    return;
  }
  expect(failures).toEqual(REQUIREMENTS.map(orphanFailure));
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

function orphanFailure(requirementId: string): string {
  return `activated requirement ${requirementId} is missing lifecycle-valid active-phase verification coverage`;
}

function replaceInFile(
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

function appendToFile(
  files: Map<FixtureFile, string>,
  relativePath: string,
  value: string,
): void {
  files.set(relativePath, `${files.get(relativePath) ?? ""}\n${value}`);
}

function insertBefore(
  files: Map<FixtureFile, string>,
  relativePath: string,
  marker: string,
  value: string,
): void {
  replaceInFile(files, relativePath, marker, `${value}\n${marker}`);
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

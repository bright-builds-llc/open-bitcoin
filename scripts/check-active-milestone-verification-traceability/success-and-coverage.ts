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
import { REQUIREMENTS, REQUIREMENTS_FILE, PHASE115_SUMMARY, PHASE125_VERIFICATION, createFixture, orphanFailure, replaceInFile, appendToFile, insertBefore } from "./test-fixtures.ts";

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

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
import { REQUIREMENTS_FILE, PHASE115_SUMMARY, PHASE125_VERIFICATION, createFixture, orphanFailure, replaceInFile, appendToFile, insertBefore, verificationText } from "./test-fixtures.ts";

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

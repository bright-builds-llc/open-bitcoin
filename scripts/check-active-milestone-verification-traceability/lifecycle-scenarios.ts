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
import { PHASE125_LIFECYCLE_ID, REQUIREMENTS, PHASE115_SUMMARY, PHASE125_CONTEXT, PHASE125_VERIFICATION, createFixture, orphanFailure, replaceInFile } from "./test-fixtures.ts";

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
  const repoRoot = path.resolve(import.meta.dir, "../..");
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

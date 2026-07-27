import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase117ParityUatReleaseBoundary } from "../check-phase117-parity-uat-release-boundary";
import { createFixture, replace } from "./test-fixtures.ts";

test("fails_when_gap_closure_requirement_maps_to_stale_phase", () => {
  // Arrange
  const root = createFixture({
    maybeMutate(files) {
      const current = files.get(".planning/REQUIREMENTS.md") ?? "";
      files.set(
        ".planning/REQUIREMENTS.md",
        current.replace("| CMP-05 | Phase 118 |", "| CMP-05 | Phase 113 |"),
      );
    },
  });
  const gapRoot = createFixture({
    gapClosureStage: true,
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| CMP-05 | Phase 126 | Pending |",
        "| CMP-05 | Phase 118 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");
  const gapFailures = checkPhase117ParityUatReleaseBoundary(gapRoot).join("\n");

  // Assert
  expect(failures).toContain("CMP-05 must map to Phase 118 exactly once");
  expect(gapFailures).toContain("CMP-05 must map to Phase 126 exactly once");
});

test("fails_when_post_audit_gap_planning_retains_stale_requirement_ownership", () => {
  // Arrange
  const root = createFixture({
    postAuditGapPlanning: true,
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| BSRV-03 | Phase 127 | Pending |",
        "| BSRV-03 | Phase 110 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase117ParityUatReleaseBoundary(root).join("\n");

  // Assert
  expect(failures).toContain("BSRV-03 must map to Phase 127 exactly once");
});

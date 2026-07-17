import { afterEach, expect, test } from "bun:test";
import { rmSync } from "node:fs";

import { checkPhase124MilestoneCloseoutReconciliation } from "./check-phase124-milestone-closeout-reconciliation";
import {
  createFixture,
  replace,
} from "./check-phase124-milestone-closeout-reconciliation.fixtures";

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_the_strict_gap_closure_stage", () => {
  // Arrange
  const root = createFixture(tempRoots, {
    gapClosureStage: true,
    includeVerification: true,
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails_wrong_gap_closure_ownership", () => {
  // Arrange
  const root = createFixture(tempRoots, {
    gapClosureStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| HARD-05 | Phase 126 | Pending |",
        "| HARD-05 | Phase 124 | Pending |",
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("HARD-05 must be owned by Phase 126");
});

test("fails_gap_closure_count_drift", () => {
  // Arrange
  const root = createFixture(tempRoots, {
    gapClosureStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(files, ".planning/REQUIREMENTS.md", "Complete: 30", "Complete: 31");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("gap-closure coverage counts");
});

test("fails_missing_gap_closure_audit_orphan", () => {
  // Arrange
  const root = createFixture(tempRoots, {
    gapClosureStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(files, ".planning/v2.1-MILESTONE-AUDIT.md", "- id: RCN-06", "- id: missing");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("gap-closure audit orphan RCN-06");
});

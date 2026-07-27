import { expect, test } from "bun:test";
import { tempRoots, createFixture, PHASE129_SUMMARY_04_FILE, spawnSync, rmSync, path, checkPhase124MilestoneCloseoutReconciliation, append, ARCHIVE_ROUTE, CONTEXT_FILE, createPhase124Fixture, LIFECYCLE_ID, PHASE128_EXECUTION_ROUTE, PHASE129_ROUTE, PHASE129_VERIFICATION_FILE, PHASE117_CHECK, PHASE117_TEST, PHASE124_CHECK, PHASE124_TEST, replace, RESOLVED_DEBT_IDS, SUMMARY_FILE, VERIFICATION_FILE } from "./setup.ts";
test("phase124_real_repository_corpus_passes", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "../..");

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: repoRoot });

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_phase128_plan04_execution_stage", () => {
  // Arrange
  const root = createFixture({ maybePhase128Stage: "executing_plan_04" });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_completed_phase128_route_to_phase129", () => {
  // Arrange
  const root = createFixture({ maybePhase128Stage: "complete" });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("phase128_plan04_execution_rejects_count_and_route_drift", () => {
  // Arrange
  const countRoot = createFixture({
    maybePhase128Stage: "executing_plan_04",
    maybeMutate(files) {
      replace(files, ".planning/REQUIREMENTS.md", "Complete: 36", "Complete: 35");
    },
  });
  const routeRoot = createFixture({
    maybePhase128Stage: "executing_plan_04",
    maybeMutate(files) {
      replace(
        files,
        ".planning/ROADMAP.md",
        PHASE128_EXECUTION_ROUTE,
        `Run \`${PHASE129_ROUTE}\`.`,
      );
    },
  });

  // Act
  const countFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: countRoot,
  }).join("\n");
  const routeFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: routeRoot,
  }).join("\n");

  // Assert
  expect(countFailures).toContain("requirements coverage");
  expect(routeFailures).toContain("Phase 128 execution route");
});

test("completed_phase128_rejects_lifecycle_and_phase129_route_drift", () => {
  // Arrange
  const lifecycleRoot = createFixture({
    maybePhase128Stage: "complete",
    maybeMutate(files) {
      replace(
        files,
        ".planning/ROADMAP.md",
        "- [x] **Phase 128: Production Compact Announcement Transport**",
        "- [ ] **Phase 128: Production Compact Announcement Transport**",
      );
    },
  });
  const routeRoot = createFixture({
    maybePhase128Stage: "complete",
    maybeMutate(files) {
      replace(
        files,
        ".planning/STATE.md",
        PHASE129_ROUTE,
        PHASE128_EXECUTION_ROUTE,
      );
    },
  });

  // Act
  const lifecycleFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: lifecycleRoot,
  }).join("\n");
  const routeFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: routeRoot,
  }).join("\n");

  // Assert
  expect(lifecycleFailures).toContain("Phase 128 lifecycle state");
  expect(routeFailures).toContain("primary route");
});

test("passes_each_legal_phase129_reconciliation_stage", () => {
  // Arrange
  const stages = ["gaps_open", "verified_pre_promotion", "archive_ready"] as const;
  const roots = stages.map((stage) => createFixture({ maybePhase129Stage: stage }));

  // Act
  const failuresPerStage = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }),
  );

  // Assert
  for (const failures of failuresPerStage) expect(failures).toEqual([]);
});

test("rejects_audit_passed_while_a_phase129_checkbox_is_pending", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      replace(files, ".planning/REQUIREMENTS.md", "- [x] **OBS-01**", "- [ ] **OBS-01**");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 archive-ready requirements checklist is missing - [x] **OBS-01**",
  );
});

test("rejects_39_of_39_requirements_without_a_verification_artifact", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      files.delete(PHASE129_VERIFICATION_FILE);
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 archive-ready lifecycle requires exactly one verification artifact",
  );
});

test("rejects_promotion_with_a_lifecycle_invalid_verification", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      replace(
        files,
        PHASE129_VERIFICATION_FILE,
        "lifecycle_validated: true",
        "lifecycle_validated: false",
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("requires exactly one lifecycle_validated: true");
});

test("rejects_checked_phase129_roadmap_row_while_audit_reports_gaps", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "gaps_open",
    maybeMutate(files) {
      replace(
        files,
        ".planning/ROADMAP.md",
        "- [ ] **Phase 129: Integration Guardrails and Milestone Reconciliation**",
        "- [x] **Phase 129: Integration Guardrails and Milestone Reconciliation**",
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 archive-ready audit must not contain status: gaps_found",
  );
});

test("rejects_premature_archive_route_in_the_verified_pre_promotion_stage", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "verified_pre_promotion",
    maybeMutate(files) {
      replace(
        files,
        ".planning/ROADMAP.md",
        `Run \`${PHASE129_ROUTE}\`.`,
        `Run \`${ARCHIVE_ROUTE}\`.`,
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("P124 post-audit roadmap route is missing");
});

test("rejects_archive_ready_hard05_ownership_outside_phase129", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| HARD-05 | Phase 129 | Complete |",
        "| HARD-05 | Phase 124 | Complete |",
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 archive-ready requirement ownership is missing | HARD-05 | Phase 129 | Complete |",
  );
});

test("archive_ready_keeps_the_legacy_final_audit_path_unreachable", () => {
  // Arrange
  const root = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      replace(files, ".planning/ROADMAP.md", "#### Phase 125:", "#### Phase 125X:");
      replace(files, ".planning/ROADMAP.md", "#### Phase 126:", "#### Phase 126X:");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 archive-ready roadmap topology is missing #### Phase 125:",
  );
  expect(failures).toContain(
    "P124 archive-ready roadmap topology is missing #### Phase 126:",
  );
});

test("fails_each_single_field_archive_ready_end_state_mutation", () => {
  // Arrange
  const mutations = [
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      "status: passed",
      "status: gaps_found",
      "P124 archive-ready audit is missing status: passed",
    ],
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      'requirements: "39/39"',
      'requirements: "38/39"',
      'P124 archive-ready audit is missing requirements: "39/39"',
    ],
    [
      ".planning/REQUIREMENTS.md",
      "- [x] **BOUND-02**",
      "- [ ] **BOUND-02**",
      "P124 archive-ready requirements checklist is missing - [x] **BOUND-02**",
    ],
    [
      ".planning/ROADMAP.md",
      "**Requirements:** OBS-01, BOUND-02, HARD-05\n**Plans:** 4/4 plans complete",
      "**Requirements:** OBS-01, BOUND-02, HARD-05\n**Plans:** 3/4 plans executed",
      "P124 archive-ready Phase 129 plan state is missing **Plans:** 4/4 plans complete",
    ],
    [
      ".planning/ROADMAP.md",
      "- Satisfied: 39",
      "- Satisfied: 36",
      "P124 archive-ready roadmap topology is missing Satisfied: 39",
    ],
    [
      ".planning/ROADMAP.md",
      `Run \`${ARCHIVE_ROUTE}\`.`,
      `Run \`${PHASE129_ROUTE}\`.`,
      "P124 archive-ready roadmap route is missing",
    ],
    [
      ".planning/STATE.md",
      ARCHIVE_ROUTE,
      PHASE129_ROUTE,
      "P124 archive-ready routing .planning/STATE.md",
    ],
    [
      ".planning/MILESTONES.md",
      ARCHIVE_ROUTE,
      "/gsd-plan-phase 128",
      "P124 archive-ready routing .planning/MILESTONES.md",
    ],
    [
      ".planning/PROJECT.md",
      ARCHIVE_ROUTE,
      "/gsd-execute-phase 129",
      "P124 archive-ready routing .planning/PROJECT.md",
    ],
    [
      ".planning/v2.1-MILESTONE-AUDIT.md",
      "scripts/check-phase124-milestone-gap-closure.ts exceeds 1,500 lines and concentrates unrelated lifecycle assertions.",
      "no retained maintainability debt",
      "P124 archive-ready audit is missing scripts/check-phase124-milestone-gap-closure.ts",
    ],
    [
      PHASE129_SUMMARY_04_FILE,
      "requirements-completed: [OBS-01, BOUND-02, HARD-05]",
      "requirements-completed: []",
      "P124 archive-ready lifecycle requires a summary listing OBS-01, BOUND-02, HARD-05",
    ],
  ] as const;
  const roots = mutations.map(([file, needle, replacement]) =>
    createFixture({
      maybePhase129Stage: "archive_ready",
      maybeMutate(files) {
        replace(files, file, needle, replacement);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const [index, message] of messages.entries()) {
    expect(message).toContain(mutations[index]?.[3] ?? "");
  }
});

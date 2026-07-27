import { expect, test } from "bun:test";
import { tempRoots, createFixture, PHASE129_SUMMARY_04_FILE, spawnSync, rmSync, path, checkPhase124MilestoneCloseoutReconciliation, append, ARCHIVE_ROUTE, CONTEXT_FILE, createPhase124Fixture, LIFECYCLE_ID, PHASE128_EXECUTION_ROUTE, PHASE129_ROUTE, PHASE129_VERIFICATION_FILE, PHASE117_CHECK, PHASE117_TEST, PHASE124_CHECK, PHASE124_TEST, replace, RESOLVED_DEBT_IDS, SUMMARY_FILE, VERIFICATION_FILE } from "./setup.ts";
test("fails_reintroduced_audit_gap_inventory_and_resolved_tech_debt", () => {
  // Arrange
  const gapRoot = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      append(files, ".planning/v2.1-MILESTONE-AUDIT.md", "- id: GAP-01");
    },
  });
  const debtRoot = createFixture({
    maybePhase129Stage: "archive_ready",
    maybeMutate(files) {
      append(files, ".planning/v2.1-MILESTONE-AUDIT.md", "phase: cross-cutting-verification");
    },
  });

  // Act
  const gapFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: gapRoot,
  }).join("\n");
  const debtFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: debtRoot,
  }).join("\n");

  // Assert
  expect(gapFailures).toContain("P124 archive-ready audit must not contain - id: GAP-0");
  expect(debtFailures).toContain(
    "P124 archive-ready audit must not contain phase: cross-cutting-verification",
  );
});

test("rejects_verified_pre_promotion_generator_drift_and_early_final_summary", () => {
  // Arrange
  const generatorRoot = createFixture({
    maybePhase129Stage: "verified_pre_promotion",
    maybeMutate(files) {
      replace(
        files,
        PHASE129_VERIFICATION_FILE,
        "generated_by: gsd-verifier",
        "generated_by: gsd-execute-plan",
      );
    },
  });
  const earlySummaryRoot = createFixture({
    maybePhase129Stage: "verified_pre_promotion",
    maybeMutate(files) {
      files.set(PHASE129_SUMMARY_04_FILE, "premature summary");
    },
  });

  // Act
  const generatorFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: generatorRoot,
  }).join("\n");
  const earlySummaryFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: earlySummaryRoot,
  }).join("\n");

  // Assert
  expect(generatorFailures).toContain("requires exactly one generated_by: gsd-verifier");
  expect(earlySummaryFailures).toContain(
    "P124 verified pre-promotion requires 129-04-SUMMARY.md to be absent",
  );
});

test("passes_the_evidence_reconciled_stage", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_final_archive_ready_stage", () => {
  // Arrange
  const root = createFixture({ finalStage: true, includeVerification: true });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_promoted_pre_summary_stage", () => {
  // Arrange
  const root = createFixture({ promotedStage: true, includeVerification: true });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails_the_promoted_pre_summary_stage_without_verification", () => {
  // Arrange
  const root = createFixture({ promotedStage: true });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("verification provenance missing");
});

test("fails_the_promoted_pre_summary_stage_after_summary_exists", () => {
  // Arrange
  const root = createFixture({
    promotedStage: true,
    includeVerification: true,
    maybeMutate(files) {
      files.set(SUMMARY_FILE, "summary exists");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("current plan summary to be absent");
});

test("fails_each_intermediate_count_mutation", () => {
  // Arrange
  const mutations = [
    ["v2.1 requirements: 39 total", "v2.1 requirements: 38 total"],
    ["Mapped to phases: 39", "Mapped to phases: 38"],
    ["Complete: 38", "Complete: 37"],
    ["Pending hardening and closeout: 1", "Pending hardening and closeout: 2"],
    ["Unmapped: 0", "Unmapped: 1"],
  ] as const;
  const roots = mutations.map(([needle, replacement]) =>
    createFixture({
      maybeMutate(files) {
        replace(files, ".planning/REQUIREMENTS.md", needle, replacement);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("counts");
});

test("fails_each_final_count_mutation", () => {
  // Arrange
  const mutations = [
    ["Satisfied: 39", "Satisfied: 38"],
    ["Pending hardening and closeout: 0", "Pending hardening and closeout: 1"],
    ["Unmapped: 0", "Unmapped: 1"],
  ] as const;
  const roots = mutations.map(([needle, replacement]) =>
    createFixture({
      finalStage: true,
      maybeMutate(files) {
        replace(files, ".planning/ROADMAP.md", needle, replacement);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("counts");
});

test("fails_duplicate_or_wrong_HARD_05_ownership", () => {
  // Arrange
  const duplicateRoot = createFixture({
    maybeMutate(files) {
      append(files, ".planning/REQUIREMENTS.md", "| HARD-05 | Phase 124 | Pending |");
    },
  });
  const wrongPhaseRoot = createFixture({
    maybeMutate(files) {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        "| HARD-05 | Phase 124 | Pending |",
        "| HARD-05 | Phase 123 | Pending |",
      );
    },
  });
  // Act
  const duplicateFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: duplicateRoot,
  }).join("\n");
  const wrongPhaseFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: wrongPhaseRoot,
  }).join("\n");
  // Assert
  expect(duplicateFailures).toContain("exactly one traceability owner");
  expect(wrongPhaseFailures).toContain("owned exactly once by Phase 124");
});

test("fails_premature_or_incoherent_stage_state", () => {
  // Arrange
  const prematureRoot = createFixture({
    maybeMutate(files) {
      replace(files, ".planning/REQUIREMENTS.md", "- [ ] **HARD-05**", "- [x] **HARD-05**");
    },
  });
  const missingHardeningRoot = createFixture({
    maybeMutate(files) {
      replace(files, ".planning/REQUIREMENTS.md", "- [x] **HARD-04**", "- [ ] **HARD-04**");
    },
  });

  // Act
  const prematureFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: prematureRoot,
  }).join("\n");
  const hardeningFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: missingHardeningRoot,
  }).join("\n");

  // Assert
  expect(prematureFailures).toContain("final");
  expect(hardeningFailures).toContain("requires checked HARD-04");
});

test("fails_final_audit_status_and_each_score_mutation", () => {
  // Arrange
  const mutations = [
    ["status: passed", "status: tech_debt"],
    ['requirements: "39/39"', 'requirements: "38/39"'],
    ['phases: "15/15"', 'phases: "14/15"'],
    ["tech_debt: []", "tech_debt:\n  - unresolved"],
  ] as const;
  const roots = mutations.map(([needle, replacement]) =>
    createFixture({
      finalStage: true,
      maybeMutate(files) {
        replace(files, ".planning/v2.1-MILESTONE-AUDIT.md", needle, replacement);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("final canonical audit");
});

test("fails_each_resolved_debt_ledger_mutation", () => {
  // Arrange
  const roots = RESOLVED_DEBT_IDS.map((debtId) =>
    createFixture({
      finalStage: true,
      maybeMutate(files) {
        replace(files, ".planning/v2.1-MILESTONE-AUDIT.md", debtId, "MISSING-DEBT");
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("resolved debt ledger");
});

test("fails_archive_route_missing_from_each_authoritative_file", () => {
  // Arrange
  const files = [
    ".planning/ROADMAP.md",
    ".planning/STATE.md",
    ".planning/v2.1-MILESTONE-AUDIT.md",
  ] as const;
  const roots = files.map((file) =>
    createFixture({
      finalStage: true,
      maybeMutate(fixtureFiles) {
        replace(fixtureFiles, file, ARCHIVE_ROUTE, "missing archive route");
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("final archive route");
});

test("fails_stale_final_planning_and_execution_routes", () => {
  // Arrange
  const planRoot = createFixture({
    finalStage: true,
    maybeMutate(files) {
      append(files, ".planning/ROADMAP.md", "/gsd-plan-phase 125");
    },
  });
  const executeRoot = createFixture({
    finalStage: true,
    maybeMutate(files) {
      append(files, ".planning/STATE.md", "/gsd-execute-phase 124");
    },
  });

  // Act
  const planFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: planRoot,
  }).join("\n");
  const executeFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: executeRoot,
  }).join("\n");

  // Assert
  expect(planFailures).toContain("final stale route");
  expect(executeFailures).toContain("final stale route");
});

test("fails_each_optional_verification_provenance_mutation", () => {
  // Arrange
  const mutations = [
    ["status: passed", "status: gaps_found"],
    ["lifecycle_validated: true", "lifecycle_validated: false"],
    [`phase_lifecycle_id: ${LIFECYCLE_ID}`, "phase_lifecycle_id: stale-id"],
  ] as const;
  const roots = mutations.map(([needle, replacement]) =>
    createFixture({
      finalStage: true,
      includeVerification: true,
      maybeMutate(files) {
        replace(files, VERIFICATION_FILE, needle, replacement);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("verification provenance");
});

test("fails_archive_ready_stage_when_verification_is_stale", () => {
  // Arrange
  const root = createFixture({
    finalStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(
        files,
        VERIFICATION_FILE,
        'generated_at: "2026-07-16T22:21:10Z"',
        'generated_at: "2026-07-16T20:00:00Z"',
      );
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain("lifecycle verification is stale relative to");
});

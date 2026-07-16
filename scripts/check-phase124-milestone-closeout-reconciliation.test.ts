import { afterEach, expect, test } from "bun:test";
import { spawnSync } from "node:child_process";
import { rmSync } from "node:fs";
import path from "node:path";

import { checkPhase124MilestoneCloseoutReconciliation } from "./check-phase124-milestone-closeout-reconciliation";
import {
  append,
  ARCHIVE_ROUTE,
  CONTEXT_FILE,
  createFixture as createPhase124Fixture,
  LIFECYCLE_ID,
  PHASE117_CHECK,
  PHASE117_TEST,
  PHASE124_CHECK,
  PHASE124_TEST,
  replace,
  RESOLVED_DEBT_IDS,
  SUMMARY_FILE,
  VERIFICATION_FILE,
} from "./check-phase124-milestone-closeout-reconciliation.fixtures";

const tempRoots: string[] = [];

const createFixture = (
  options?: Parameters<typeof createPhase124Fixture>[1],
): string => createPhase124Fixture(tempRoots, options);

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("phase124_real_repository_corpus_passes", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: repoRoot });

  // Assert
  expect(failures).toEqual([]);
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

test("fails_archive_ready_stage_with_wrong_input_lifecycle_identity", () => {
  // Arrange
  const root = createFixture({
    finalStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(files, CONTEXT_FILE, "lifecycle_mode: yolo", "lifecycle_mode: manual");
      replace(files, CONTEXT_FILE, `phase_lifecycle_id: ${LIFECYCLE_ID}`, "phase_lifecycle_id: stale");
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(`${CONTEXT_FILE} requires exactly one lifecycle_mode: yolo`);
  expect(failures).toContain(`${CONTEXT_FILE} requires exactly one phase_lifecycle_id`);
});

test("real_archive_ready_path_does_not_depend_on_home_local_tools", () => {
  // Arrange
  const repoRoot = path.resolve(import.meta.dir, "..");

  // Act
  const result = spawnSync(
    "bun",
    ["run", "scripts/check-phase124-milestone-closeout-reconciliation.ts"],
    {
      cwd: repoRoot,
      encoding: "utf8",
      env: { ...process.env, HOME: "/tmp/open-bitcoin-phase124-empty-home" },
    },
  );

  // Assert
  expect(result.status).toBe(0);
  expect(`${result.stdout}${result.stderr}`).not.toContain(".codex/get-shit-done");
});

test("fails_archive_ready_stage_without_final_summary", () => {
  // Arrange
  const root = createFixture({
    finalStage: true,
    includeVerification: true,
    maybeMutate(files) {
      files.delete(SUMMARY_FILE);
    },
  });

  // Act
  const failures = checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n");

  // Assert
  expect(failures).toContain(`archive-ready lifecycle missing ${SUMMARY_FILE}`);
});

test("fails_duplicate_or_body_only_verification_frontmatter_values", () => {
  // Arrange
  const duplicateRoot = createFixture({
    finalStage: true,
    includeVerification: true,
    maybeMutate(files) {
      replace(files, VERIFICATION_FILE, "status: passed", "status: passed\nstatus: gaps_found");
    },
  });
  const bodyOnlyRoot = createFixture({
    finalStage: true,
    includeVerification: true,
    maybeMutate(files) {
      files.set(
        VERIFICATION_FILE,
        [
          "---",
          "phase: 124-milestone-closeout-reconciliation",
          "---",
          "status: passed",
          "lifecycle_validated: true",
          `phase_lifecycle_id: ${LIFECYCLE_ID}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const duplicateFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: duplicateRoot,
  }).join("\n");
  const bodyOnlyFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: bodyOnlyRoot,
  }).join("\n");

  // Assert
  expect(duplicateFailures).toContain("exactly one status: passed");
  expect(bodyOnlyFailures).toContain("exactly one status: passed");
  expect(bodyOnlyFailures).toContain("exactly one lifecycle_validated: true");
});

test("fails_each_positive_no_claim_boundary_mutation", () => {
  // Arrange
  const topics = [
    "public block serving by default",
    "public compact relay by default",
    "archive-node",
    "package relay",
    "filter serving",
    "public-network CI",
    "production full-node readiness",
    "production-funds wallet",
  ] as const;
  const roots = topics.map((topic) =>
    createFixture({
      maybeMutate(files) {
        append(files, "README.md", `Open Bitcoin supports ${topic}.`);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("no-claim boundary");
});

test("fails_mixed_deferred_and_positive_claims_across_planning_only_surfaces", () => {
  // Arrange
  const mutations = [
    [".planning/PROJECT.md", "Package relay remains deferred, but Open Bitcoin supports production full-node readiness."],
    [".planning/ROADMAP.md", "Package relay remains deferred, while Open Bitcoin supports production-funds wallet."],
    [".planning/v2.1-MILESTONE-AUDIT.md", "Package relay remains deferred whereas Open Bitcoin provides archive-node."],
    [".planning/PROJECT.md", "Package relay remains deferred; Open Bitcoin enables filter serving."],
    [".planning/ROADMAP.md", "| Package relay remains deferred | Open Bitcoin ships public-network CI |"],
  ] as const;
  const roots = mutations.map(([file, claim]) =>
    createFixture({
      maybeMutate(files) {
        append(files, file, claim);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase124MilestoneCloseoutReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) expect(message).toContain("no-claim boundary");
});

test("fails_visible_and_executable_verifier_order_mutations", () => {
  // Arrange
  const visibleRoot = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        `${PHASE124_CHECK}\n${PHASE117_TEST}`,
        `${PHASE117_TEST}\n${PHASE124_CHECK}`,
      );
    },
  });
  const executableRoot = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        `run_step "check Phase 124" ${PHASE124_CHECK}\nrun_step "test Phase 117" ${PHASE117_TEST}`,
        `run_step "test Phase 117" ${PHASE117_TEST}\nrun_step "check Phase 124" ${PHASE124_CHECK}`,
      );
    },
  });

  // Act
  const visibleFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: visibleRoot,
  }).join("\n");
  const executableFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: executableRoot,
  }).join("\n");

  // Assert
  expect(visibleFailures).toContain("visible verifier order");
  expect(executableFailures).toContain("executable verifier order");
});

test("fails_missing_checker_command_or_phase_checker_after_phase117", () => {
  // Arrange
  const missingRoot = createFixture({
    maybeMutate(files) {
      replace(files, "scripts/verify.sh", PHASE124_TEST, "missing Phase 124 test");
    },
  });
  const finalGateRoot = createFixture({
    maybeMutate(files) {
      replace(
        files,
        "scripts/verify.sh",
        `${PHASE117_CHECK}\nVERIFY_COMMAND_ORDER`,
        `${PHASE117_CHECK}\nbun test scripts/check-phase125-synthetic.test.ts\nbun run scripts/check-phase125-synthetic.ts\nVERIFY_COMMAND_ORDER`,
      );
      append(
        files,
        "scripts/verify.sh",
        'run_step "test Phase 125" bun test scripts/check-phase125-synthetic.test.ts',
      );
      append(
        files,
        "scripts/verify.sh",
        'run_step "check Phase 125" bun run scripts/check-phase125-synthetic.ts',
      );
    },
  });
  const multilineGateRoot = createFixture({
    maybeMutate(files) {
      append(
        files,
        "scripts/verify.sh",
        [
          "run_step \\",
          '  "test Phase 125" \\',
          "  bun \\",
          "  test \\",
          "  scripts/check-phase125-synthetic.test.ts",
          "run_step \\",
          '  "check Phase 125" \\',
          "  bun \\",
          "  run \\",
          "  scripts/check-phase125-synthetic.ts",
        ].join("\n"),
      );
    },
  });

  // Act
  const missingFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: missingRoot,
  }).join("\n");
  const finalGateFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: finalGateRoot,
  }).join("\n");
  const multilineGateFailures = checkPhase124MilestoneCloseoutReconciliation({
    rootDir: multilineGateRoot,
  }).join("\n");

  // Assert
  expect(missingFailures).toContain("verifier mutation command count");
  expect(finalGateFailures).toContain("visible verifier final gate");
  expect(finalGateFailures).toContain("executable verifier final gate");
  expect(multilineGateFailures).toContain("executable verifier final gate");
});

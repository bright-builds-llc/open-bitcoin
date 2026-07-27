import { expect, test } from "bun:test";
import { tempRoots, createFixture, PHASE129_SUMMARY_04_FILE, spawnSync, rmSync, path, checkPhase124MilestoneCloseoutReconciliation, append, ARCHIVE_ROUTE, CONTEXT_FILE, createPhase124Fixture, LIFECYCLE_ID, PHASE128_EXECUTION_ROUTE, PHASE129_ROUTE, PHASE129_VERIFICATION_FILE, PHASE117_CHECK, PHASE117_TEST, PHASE124_CHECK, PHASE124_TEST, replace, RESOLVED_DEBT_IDS, SUMMARY_FILE, VERIFICATION_FILE } from "./setup.ts";
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
  const repoRoot = path.resolve(import.meta.dir, "../..");

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

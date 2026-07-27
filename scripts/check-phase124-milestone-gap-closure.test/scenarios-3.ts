import { expect, test } from "bun:test";
import { PHASE125_DIRECTORY, PHASE125_SUMMARY_01, PHASE125_SUMMARY_03, PHASE125_SUMMARY_04, PHASE127_DIRECTORY, PHASE127_LIFECYCLE_ID, PHASE128_ROUTE, ROUTING_FILES, tempRoots, mkdirSync, readFileSync, rmSync, writeFileSync, path, checkPhase124MilestoneCloseoutReconciliation, ACTIVE_TRACEABILITY_CHECK, ACTIVE_TRACEABILITY_TEST, append, createFixture, PHASE124_CHECK, PHASE125_LIFECYCLE_ID, PHASE125_ROUTE, PHASE125_VERIFICATION_FILE, PHASE126_ROUTE, PHASE126_LIFECYCLE_ID, PHASE126_VERIFICATION_FILE, PHASE127_ROUTE, replace, stageFixture, phase126StageFixture, postAuditGapPlanningFixture, check, replaceRoutes, promoteRequirements, promotePhase126Requirements, phase125Summary, addPhase127Artifacts, phase127Artifact, promotePhase127Requirements, writeRootFile, replaceRootFile } from "./setup.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, FixtureFile } from "./setup.ts";
test("verification_stage_requires_exactly_one_visible_and_executable_traceability_pair", () => {
  // Arrange
  const mutations = [
    [ACTIVE_TRACEABILITY_TEST, "missing visible active traceability test"],
    [ACTIVE_TRACEABILITY_CHECK, `${ACTIVE_TRACEABILITY_CHECK}\n${ACTIVE_TRACEABILITY_CHECK}`],
    [
      `run_step "test active traceability" ${ACTIVE_TRACEABILITY_TEST}`,
      'run_step "test active traceability" missing-active-traceability-test',
    ],
    [
      `run_step "check active traceability" ${ACTIVE_TRACEABILITY_CHECK}`,
      [
        `run_step "check active traceability" ${ACTIVE_TRACEABILITY_CHECK}`,
        `run_step "check active traceability duplicate" ${ACTIVE_TRACEABILITY_CHECK}`,
      ].join("\n"),
    ],
  ] as const;
  const roots = mutations.map(([needle, replacement]) =>
    stageFixture("post_summary", (files) => {
      replace(files, "scripts/verify.sh", needle, replacement);
    }),
  );

  // Act
  const messages = roots.map((root) => check(root).join("\n"));

  // Assert
  for (const message of messages) {
    expect(message).toContain("active traceability");
    expect(message).toContain("command count");
  }
});

test("verification_stage_rejects_traceability_pair_outside_phase124_phase117_interval", () => {
  // Arrange
  const visibleRoot = stageFixture("post_summary", (files) => {
    replace(
      files,
      "scripts/verify.sh",
      `${PHASE124_CHECK}\n${ACTIVE_TRACEABILITY_TEST}`,
      `${ACTIVE_TRACEABILITY_TEST}\n${PHASE124_CHECK}`,
    );
  });
  const executableRoot = stageFixture("post_summary", (files) => {
    replace(
      files,
      "scripts/verify.sh",
      [
        `run_step "check Phase 124" ${PHASE124_CHECK}`,
        `run_step "test active traceability" ${ACTIVE_TRACEABILITY_TEST}`,
      ].join("\n"),
      [
        `run_step "test active traceability" ${ACTIVE_TRACEABILITY_TEST}`,
        `run_step "check Phase 124" ${PHASE124_CHECK}`,
      ].join("\n"),
    );
  });

  // Act
  const visibleFailures = check(visibleRoot).join("\n");
  const executableFailures = check(executableRoot).join("\n");

  // Assert
  expect(visibleFailures).toContain("visible verifier order");
  expect(executableFailures).toContain("executable verifier order");
});

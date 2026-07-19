import { afterEach, expect, test } from "bun:test";
import { rmSync } from "node:fs";

import type {
  Phase125LifecycleStage,
  Phase126CloseoutStage,
} from "./check-phase124-milestone-gap-closure";
import { checkPhase124MilestoneCloseoutReconciliation } from "./check-phase124-milestone-closeout-reconciliation";
import {
  ACTIVE_TRACEABILITY_CHECK,
  ACTIVE_TRACEABILITY_TEST,
  append,
  createFixture,
  type FixtureFile,
  PHASE124_CHECK,
  PHASE125_LIFECYCLE_ID,
  PHASE125_ROUTE,
  PHASE125_VERIFICATION_FILE,
  PHASE126_ROUTE,
  PHASE126_LIFECYCLE_ID,
  PHASE126_VERIFICATION_FILE,
  PHASE127_ROUTE,
  replace,
} from "./check-phase124-milestone-closeout-reconciliation.fixtures";

const PHASE125_DIRECTORY =
  ".planning/phases/125-compact-download-verification-traceability-closure";
const PHASE125_SUMMARY_01 =
  `${PHASE125_DIRECTORY}/125-01-SUMMARY.md` as const;
const PHASE125_SUMMARY_03 =
  `${PHASE125_DIRECTORY}/125-03-SUMMARY.md` as const;
const PHASE125_SUMMARY_04 =
  `${PHASE125_DIRECTORY}/125-04-SUMMARY.md` as const;
const ROUTING_FILES = [
  ".planning/ROADMAP.md",
  ".planning/PROJECT.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
] as const;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes_the_planned_stage", () => {
  // Arrange
  const root = stageFixture("planned");

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_pre_verification_stage", () => {
  // Arrange
  const root = stageFixture("pre_verification");

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_verification_written_pre_promotion_stage", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion");

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_post_verification_stage", () => {
  // Arrange
  const root = stageFixture("post_verification");

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_post_summary_stage", () => {
  // Arrange
  const root = stageFixture("post_summary");

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

for (const stage of [
  "candidate",
  "verified_pre_promotion",
  "promoted_pre_summary",
  "archive_ready",
] as const) {
  test(`passes_the_phase126_${stage}_stage`, () => {
    // Arrange
    const root = phase126StageFixture(stage);

    // Act
    const failures = check(root);

    // Assert
    expect(failures).toEqual([]);
  });
}

test("passes_the_post_audit_gap_planning_stage", () => {
  // Arrange
  const root = postAuditGapPlanningFixture();

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("post_audit_gap_planning_rejects_wrong_ownership_and_counts", () => {
  // Arrange
  const ownershipRoot = postAuditGapPlanningFixture((files) => {
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      "| BSRV-03 | Phase 127 | Pending |",
      "| BSRV-03 | Phase 128 | Pending |",
    );
  });
  const countRoot = postAuditGapPlanningFixture((files) => {
    replace(files, ".planning/REQUIREMENTS.md", "Complete: 29", "Complete: 28");
  });

  // Act
  const ownershipFailures = check(ownershipRoot).join("\n");
  const countFailures = check(countRoot).join("\n");

  // Assert
  expect(ownershipFailures).toContain("BSRV-03 must be owned by Phase 127");
  expect(countFailures).toContain("post-audit requirements coverage");
});

test("post_audit_gap_planning_rejects_topology_audit_and_route_drift", () => {
  // Arrange
  const topologyRoot = postAuditGapPlanningFixture((files) => {
    replace(
      files,
      ".planning/ROADMAP.md",
      "#### Phase 128: Production Compact Announcement Transport\n**Depends on:** Phase 127",
      "#### Phase 128: Production Compact Announcement Transport\n**Depends on:** Phase 126",
    );
  });
  const auditRoot = postAuditGapPlanningFixture((files) => {
    replace(
      files,
      ".planning/v2.1-MILESTONE-AUDIT.md",
      "status: gaps_found",
      "status: passed",
    );
  });
  const routeRoot = postAuditGapPlanningFixture((files) => {
    replace(files, ".planning/STATE.md", PHASE127_ROUTE, PHASE126_ROUTE);
  });

  // Act
  const topologyFailures = check(topologyRoot).join("\n");
  const auditFailures = check(auditRoot).join("\n");
  const routeFailures = check(routeRoot).join("\n");

  // Assert
  expect(topologyFailures).toContain("post-audit Phase 128 dependency");
  expect(auditFailures).toContain("post-audit audit score");
  expect(routeFailures).toContain("post-audit primary route .planning/STATE.md");
});

test("post_audit_gap_planning_keeps_the_no_claim_boundary", () => {
  // Arrange
  const root = postAuditGapPlanningFixture((files) => {
    append(files, ".planning/PROJECT.md", "Open Bitcoin supports package relay.");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("no-claim boundary");
});

test("phase126_rejects_mixed_requirement_counts", () => {
  // Arrange
  const root = phase126StageFixture("candidate", (files) => {
    replace(files, ".planning/REQUIREMENTS.md", "- [ ] **CMP-05**", "- [x] **CMP-05**");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 Phase 126 requirement projection must be uniformly pending or promoted",
  );
});

test("phase126_rejects_verification_lifecycle_mismatch", () => {
  // Arrange
  const root = phase126StageFixture("verified_pre_promotion", (files) => {
    replace(
      files,
      PHASE126_VERIFICATION_FILE,
      `phase_lifecycle_id: ${PHASE126_LIFECYCLE_ID}`,
      "phase_lifecycle_id: stale-lifecycle",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "phase_lifecycle_id must match Phase 126 CONTEXT",
  );
});

test("phase126_rejects_premature_promotion", () => {
  // Arrange
  const root = phase126StageFixture("candidate", promotePhase126Requirements);

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "P124 Phase 126 promoted projection requires lifecycle-valid verification",
  );
});

test("phase126_rejects_stale_plan_progress", () => {
  // Arrange
  const root = phase126StageFixture("promoted_pre_summary", (files) => {
    replace(files, ".planning/ROADMAP.md", "3/4 plans executed", "4/4 plans complete");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("P124 promoted_pre_summary Phase 126 plans");
});

test("phase126_rejects_stale_phase_progress", () => {
  // Arrange
  const root = phase126StageFixture("archive_ready", (files) => {
    replace(
      files,
      ".planning/ROADMAP.md",
      "- [x] **Phase 126:",
      "- [ ] **Phase 126:",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("P124 archive_ready Phase 126 state");
});

test("phase126_rejects_stale_routes", () => {
  // Arrange
  const candidateRoot = phase126StageFixture("candidate", (files) => {
    append(files, ".planning/STATE.md", PHASE125_ROUTE);
  });
  const archiveRoot = phase126StageFixture("archive_ready", (files) => {
    append(files, ".planning/STATE.md", PHASE126_ROUTE);
  });

  // Act
  const candidateFailures = check(candidateRoot).join("\n");
  const archiveFailures = check(archiveRoot).join("\n");

  // Assert
  expect(candidateFailures).toContain("P124 candidate stale Phase 125 route");
  expect(archiveFailures).toContain("P124 archive_ready stale Phase 126 route");
});

test("planned_rejects_a_summary", () => {
  // Arrange
  const root = stageFixture("planned", (files) => {
    files.set(PHASE125_SUMMARY_01, phase125Summary("01"));
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("pre_verification Phase 125 plans");
});

test("pre_verification_rejects_a_fourth_summary_without_verification", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    files.set(PHASE125_SUMMARY_03, phase125Summary("03"));
    files.set(PHASE125_SUMMARY_04, phase125Summary("04"));
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("artifact combination does not match a legal lifecycle stage");
});

test("verification_written_pre_promotion_rejects_promoted_counts", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion", promoteRequirements);

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("post_verification");
});

test("every_pre_promotion_stage_rejects_premature_phase126_routing", () => {
  // Arrange
  const stages = [
    "planned",
    "pre_verification",
    "verification_written_pre_promotion",
  ] as const;
  const roots = stages.map((stage) =>
    stageFixture(stage, (files) => replaceRoutes(files, PHASE125_ROUTE, PHASE126_ROUTE)),
  );

  // Act
  const messages = roots.map((root) => check(root).join("\n"));

  // Assert
  for (const message of messages) {
    expect(message).toContain("premature Phase 126 route");
  }
});

test("pre_promotion_rejects_missing_primary_route_in_each_canonical_file", () => {
  // Arrange
  const roots = ROUTING_FILES.map((file) => ({
    file,
    root: stageFixture("planned", (files) => {
      replace(files, file, PHASE125_ROUTE, "Primary route intentionally absent.");
    }),
  }));

  // Act
  const results = roots.map(({ file, root }) => ({
    failures: check(root).join("\n"),
    file,
  }));

  // Assert
  for (const { failures, file } of results) {
    expect(failures).toContain(`Phase 125 primary route ${file}`);
  }
});

test("post_verification_rejects_a_missing_verification", () => {
  // Arrange
  const root = stageFixture("post_verification", (files) => {
    files.delete(PHASE125_VERIFICATION_FILE);
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("promoted projection requires lifecycle-valid verification");
});

test("post_verification_rejects_a_premature_checked_phase", () => {
  // Arrange
  const root = stageFixture("post_verification", (files) => {
    replace(
      files,
      ".planning/ROADMAP.md",
      "- [ ] **Phase 125:",
      "- [x] **Phase 125:",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("post_verification Phase 125 state");
});

test("post_verification_rejects_stale_phase125_routing", () => {
  // Arrange
  const root = stageFixture("post_verification", (files) => {
    replaceRoutes(files, PHASE126_ROUTE, PHASE125_ROUTE);
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("stale Phase 125 route");
});

test("post_summary_rejects_three_of_four_progress", () => {
  // Arrange
  const root = stageFixture("post_summary", (files) => {
    replace(files, ".planning/ROADMAP.md", "4/4 plans complete", "3/4 plans executed");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("post_summary Phase 125 plans");
});

test("post_summary_rejects_mixed_complete_and_stale_progress_narratives", () => {
  // Arrange
  const root = stageFixture("post_summary", (files) => {
    append(
      files,
      ".planning/ROADMAP.md",
      "Phase 125 remains at 3/4 plans executed and awaits summary bookkeeping.",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "post_summary contradictory Phase 125 narrative .planning/ROADMAP.md",
  );
});

test("post_summary_rejects_stale_phase125_routing", () => {
  // Arrange
  const root = stageFixture("post_summary", (files) => {
    replaceRoutes(files, PHASE126_ROUTE, PHASE125_ROUTE);
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("stale Phase 125 route");
});

test("rejects_wrong_phase125_ownership", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      "| RCN-06 | Phase 125 | Pending |",
      "| RCN-06 | Phase 126 | Pending |",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("RCN-06 must be owned by Phase 125");
});

test("rejects_each_phase126_ownership_drift", () => {
  // Arrange
  const requirements = ["CMP-05", "RCN-02", "RCN-03", "GOV-04", "BOUND-01", "HARD-05"];
  const roots = requirements.map((requirement) =>
    stageFixture("pre_verification", (files) => {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        `| ${requirement} | Phase 126 | Pending |`,
        `| ${requirement} | Phase 125 | Pending |`,
      );
    }),
  );

  // Act
  const messages = roots.map((root) => check(root).join("\n"));

  // Assert
  for (const [index, requirement] of requirements.entries()) {
    expect(messages[index]).toContain(`${requirement} must be owned by Phase 126`);
  }
});

test("rejects_each_phase126_status_drift", () => {
  // Arrange
  const requirements = ["CMP-05", "RCN-02", "RCN-03", "GOV-04", "BOUND-01", "HARD-05"];
  const roots = requirements.map((requirement) =>
    stageFixture("post_summary", (files) => {
      replace(
        files,
        ".planning/REQUIREMENTS.md",
        `| ${requirement} | Phase 126 | Pending |`,
        `| ${requirement} | Phase 126 | Complete |`,
      );
    }),
  );

  // Act
  const messages = roots.map((root) => check(root).join("\n"));

  // Assert
  for (const [index, requirement] of requirements.entries()) {
    expect(messages[index]).toContain(`traceability status is invalid for ${requirement}`);
  }
});

test("rejects_wrong_requirement_coverage_counts", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    replace(files, ".planning/REQUIREMENTS.md", "Complete: 30", "Complete: 31");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("requirements coverage counts");
});

test("rejects_wrong_audit_requirement_count", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    replace(files, ".planning/v2.1-MILESTONE-AUDIT.md", '"30/39"', '"31/39"');
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("audit requirements");
});

test("rejects_wrong_audit_phase_count", () => {
  // Arrange
  const root = stageFixture("post_summary", (files) => {
    replace(files, ".planning/v2.1-MILESTONE-AUDIT.md", '"16/17"', '"15/17"');
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("audit phases");
});

test("rejects_wrong_verification_lifecycle_id", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion", (files) => {
    replace(
      files,
      PHASE125_VERIFICATION_FILE,
      `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
      "phase_lifecycle_id: stale-lifecycle",
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("phase_lifecycle_id must match Phase 125 CONTEXT");
});

test("rejects_wrong_verification_lifecycle_mode", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion", (files) => {
    replace(files, PHASE125_VERIFICATION_FILE, "lifecycle_mode: yolo", "lifecycle_mode: manual");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("lifecycle_mode must match Phase 125 CONTEXT");
});

test("rejects_wrong_verification_status", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion", (files) => {
    replace(files, PHASE125_VERIFICATION_FILE, "status: passed", "status: gaps_found");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("requires status: passed");
});

test("rejects_duplicate_verification_frontmatter", () => {
  // Arrange
  const root = stageFixture("verification_written_pre_promotion", (files) => {
    append(files, PHASE125_VERIFICATION_FILE, "---\nstatus: passed\n---");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("must contain exactly one YAML frontmatter block");
});

test("rejects_mismatched_summary_plan_number", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    replace(files, PHASE125_SUMMARY_01, 'plan: "01"', 'plan: "04"');
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("plan number must match its filename");
});

test("rejects_milestone_completion_routing", () => {
  // Arrange
  const root = stageFixture("post_summary", (files) => {
    append(files, ".planning/STATE.md", "/gsd-complete-milestone v2.1");
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("milestone completion route");
});

test("rejects_phase117_when_it_is_not_the_final_checker", () => {
  // Arrange
  const root = stageFixture("pre_verification", (files) => {
    append(
      files,
      "scripts/verify.sh",
      'run_step "check Phase 125 synthetic" bun run scripts/check-phase125-synthetic.ts',
    );
  });

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain("executable verifier final gate");
});

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

function stageFixture(
  stage: Phase125LifecycleStage["kind"],
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    maybePhase125Stage: stage,
    maybeMutate,
  });
}

function phase126StageFixture(
  stage: Phase126CloseoutStage["kind"],
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    maybePhase126Stage: stage,
    maybeMutate,
  });
}

function postAuditGapPlanningFixture(
  maybeMutate?: (files: Map<FixtureFile, string>) => void,
): string {
  return createFixture(tempRoots, {
    postAuditGapPlanning: true,
    maybeMutate,
  });
}

function check(root: string): string[] {
  return checkPhase124MilestoneCloseoutReconciliation({ rootDir: root });
}

function replaceRoutes(
  files: Map<FixtureFile, string>,
  route: string,
  replacement: string,
): void {
  for (const file of ROUTING_FILES) {
    replace(files, file, route, replacement);
  }
}

function promoteRequirements(files: Map<FixtureFile, string>): void {
  for (const requirement of ["RCN-04", "RCN-05", "RCN-06"]) {
    replace(files, ".planning/REQUIREMENTS.md", `- [ ] **${requirement}**`, `- [x] **${requirement}**`);
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `| ${requirement} | Phase 125 | Pending |`,
      `| ${requirement} | Phase 125 | Complete |`,
    );
  }
  replace(files, ".planning/REQUIREMENTS.md", "Complete: 30", "Complete: 33");
  replace(
    files,
    ".planning/REQUIREMENTS.md",
    "Pending hardening and closeout: 9",
    "Pending hardening and closeout: 6",
  );
}

function promotePhase126Requirements(files: Map<FixtureFile, string>): void {
  for (const requirement of [
    "CMP-05",
    "RCN-02",
    "RCN-03",
    "GOV-04",
    "BOUND-01",
    "HARD-05",
  ]) {
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `- [ ] **${requirement}**`,
      `- [x] **${requirement}**`,
    );
    replace(
      files,
      ".planning/REQUIREMENTS.md",
      `| ${requirement} | Phase 126 | Pending |`,
      `| ${requirement} | Phase 126 | Complete |`,
    );
  }
  replace(files, ".planning/REQUIREMENTS.md", "Complete: 33", "Complete: 39");
  replace(
    files,
    ".planning/REQUIREMENTS.md",
    "Pending hardening and closeout: 6",
    "Pending hardening and closeout: 0",
  );
}

function phase125Summary(planNumber: "01" | "03" | "04"): string {
  return [
    "---",
    "phase: 125-compact-download-verification-traceability-closure",
    `plan: "${planNumber}"`,
    "requirements-completed: []",
    "generated_by: gsd-execute-plan",
    "lifecycle_mode: yolo",
    `phase_lifecycle_id: ${PHASE125_LIFECYCLE_ID}`,
    'generated_at: "2026-07-17T15:00:00Z"',
    "---",
    "fixture summary",
  ].join("\n");
}

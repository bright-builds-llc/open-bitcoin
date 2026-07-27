import { expect, test } from "bun:test";
import { PHASE125_DIRECTORY, PHASE125_SUMMARY_01, PHASE125_SUMMARY_03, PHASE125_SUMMARY_04, PHASE127_DIRECTORY, PHASE127_LIFECYCLE_ID, PHASE128_ROUTE, ROUTING_FILES, tempRoots, mkdirSync, readFileSync, rmSync, writeFileSync, path, checkPhase124MilestoneCloseoutReconciliation, ACTIVE_TRACEABILITY_CHECK, ACTIVE_TRACEABILITY_TEST, append, createFixture, PHASE124_CHECK, PHASE125_LIFECYCLE_ID, PHASE125_ROUTE, PHASE125_VERIFICATION_FILE, PHASE126_ROUTE, PHASE126_LIFECYCLE_ID, PHASE126_VERIFICATION_FILE, PHASE127_ROUTE, replace, stageFixture, phase126StageFixture, postAuditGapPlanningFixture, check, replaceRoutes, promoteRequirements, promotePhase126Requirements, phase125Summary, addPhase127Artifacts, phase127Artifact, promotePhase127Requirements, writeRootFile, replaceRootFile } from "./setup.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, FixtureFile } from "./setup.ts";
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

test("passes_the_active_phase127_execution_stage", () => {
  // Arrange
  const root = postAuditGapPlanningFixture();
  addPhase127Artifacts(root, 3, false);
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "**Plans:** 0 plans",
    "**Plans:** 3/4 plans executed",
  );

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("passes_the_completed_phase127_stage_and_routes_to_phase128", () => {
  // Arrange
  const root = postAuditGapPlanningFixture();
  addPhase127Artifacts(root, 4, true);
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "**Plans:** 0 plans",
    "**Plans:** 4/4 plans complete",
  );
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "- [ ] **Phase 127: Authoritative Network State Unification**",
    "- [x] **Phase 127: Authoritative Network State Unification**",
  );
  promotePhase127Requirements(root);
  for (const file of [
    ".planning/ROADMAP.md",
    ".planning/PROJECT.md",
    ".planning/STATE.md",
    ".planning/MILESTONES.md",
  ]) {
    replaceRootFile(root, file, PHASE127_ROUTE, PHASE128_ROUTE);
  }

  // Act
  const failures = check(root);

  // Assert
  expect(failures).toEqual([]);
});

test("phase127_rejects_stale_summary_lifecycle_identity", () => {
  // Arrange
  const root = postAuditGapPlanningFixture();
  addPhase127Artifacts(root, 3, false);
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "**Plans:** 0 plans",
    "**Plans:** 3/4 plans executed",
  );
  replaceRootFile(
    root,
    `${PHASE127_DIRECTORY}/127-03-SUMMARY.md`,
    `phase_lifecycle_id: ${PHASE127_LIFECYCLE_ID}`,
    "phase_lifecycle_id: stale-lifecycle",
  );

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "phase_lifecycle_id must match Phase 127 CONTEXT",
  );
});

test("completed_phase127_rejects_a_stale_primary_route", () => {
  // Arrange
  const root = postAuditGapPlanningFixture();
  addPhase127Artifacts(root, 4, true);
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "**Plans:** 0 plans",
    "**Plans:** 4/4 plans complete",
  );
  replaceRootFile(
    root,
    ".planning/ROADMAP.md",
    "- [ ] **Phase 127: Authoritative Network State Unification**",
    "- [x] **Phase 127: Authoritative Network State Unification**",
  );
  promotePhase127Requirements(root);
  for (const file of [
    ".planning/ROADMAP.md",
    ".planning/PROJECT.md",
    ".planning/STATE.md",
    ".planning/MILESTONES.md",
  ]) {
    replaceRootFile(root, file, PHASE127_ROUTE, PHASE128_ROUTE);
  }
  writeRootFile(
    root,
    ".planning/STATE.md",
    `${readFileSync(path.join(root, ".planning/STATE.md"), "utf8")}\n${PHASE127_ROUTE}`,
  );

  // Act
  const failures = check(root).join("\n");

  // Assert
  expect(failures).toContain(
    "completed Phase 127 routing .planning/STATE.md must not retain",
  );
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
  // An audit flipped to passed now claims the Phase 129 archive-ready stage,
  // whose full condition set rejects the gaps-open mixture fail-closed.
  expect(auditFailures).toContain(
    "P124 archive-ready checked requirement count must be 39; found 29",
  );
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

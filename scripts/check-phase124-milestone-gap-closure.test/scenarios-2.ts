import { expect, test } from "bun:test";
import { PHASE125_DIRECTORY, PHASE125_SUMMARY_01, PHASE125_SUMMARY_03, PHASE125_SUMMARY_04, PHASE127_DIRECTORY, PHASE127_LIFECYCLE_ID, PHASE128_ROUTE, ROUTING_FILES, tempRoots, mkdirSync, readFileSync, rmSync, writeFileSync, path, checkPhase124MilestoneCloseoutReconciliation, ACTIVE_TRACEABILITY_CHECK, ACTIVE_TRACEABILITY_TEST, append, createFixture, PHASE124_CHECK, PHASE125_LIFECYCLE_ID, PHASE125_ROUTE, PHASE125_VERIFICATION_FILE, PHASE126_ROUTE, PHASE126_LIFECYCLE_ID, PHASE126_VERIFICATION_FILE, PHASE127_ROUTE, replace, stageFixture, phase126StageFixture, postAuditGapPlanningFixture, check, replaceRoutes, promoteRequirements, promotePhase126Requirements, phase125Summary, addPhase127Artifacts, phase127Artifact, promotePhase127Requirements, writeRootFile, replaceRootFile } from "./setup.ts";
import type { Phase125LifecycleStage, Phase126CloseoutStage, FixtureFile } from "./setup.ts";
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

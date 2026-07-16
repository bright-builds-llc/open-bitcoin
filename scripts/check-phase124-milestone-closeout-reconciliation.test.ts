import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase124MilestoneCloseoutReconciliation } from "./check-phase124-milestone-closeout-reconciliation";

const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
const PHASE124_TEST =
  "bun test scripts/check-phase124-milestone-closeout-reconciliation.test.ts";
const PHASE124_CHECK =
  "bun run scripts/check-phase124-milestone-closeout-reconciliation.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";
const PHASE117_CHECK =
  "bun run scripts/check-phase117-parity-uat-release-boundary.ts";
const LIFECYCLE_ID = "124-2026-07-16T20-19-53";
const ARCHIVE_ROUTE = "/gsd-complete-milestone v2.1";
const VERIFICATION_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-VERIFICATION.md";
const SUMMARY_FILE =
  ".planning/phases/124-milestone-closeout-reconciliation/124-02-SUMMARY.md";
const REQUIRED_FILES = [
  ".planning/REQUIREMENTS.md",
  ".planning/ROADMAP.md",
  ".planning/STATE.md",
  ".planning/v2.1-MILESTONE-AUDIT.md",
  ".planning/PROJECT.md",
  "README.md",
  "docs/parity/release-readiness.md",
  "docs/parity/production-claim-boundary.md",
  "scripts/verify.sh",
] as const;
const REQUIREMENT_IDS = [
  ...range("BSRV", 6),
  ...range("CMP", 6),
  ...range("RCN", 7),
  ...range("GOV", 5),
  ...range("OBS", 5),
  ...range("BOUND", 5),
  ...range("HARD", 5),
] as const;
const RESOLVED_DEBT_IDS = [
  "DEBT-01-INBOUND-GETBLOCKTXN",
  "DEBT-02-PHASE112-TEST-VOCABULARY",
  "DEBT-03-SUCCESSFUL-BLOCK-WRITE-EVIDENCE",
  "DEBT-04-RECEIVE-INDEPENDENT-TIMEOUT",
  "DEBT-05-AUTHORITATIVE-RUNTIME-PROJECTION",
  "DEBT-06-MILESTONE-METADATA-RECONCILIATION",
] as const;

type RequiredFile = (typeof REQUIRED_FILES)[number];
type FixtureFile = RequiredFile | typeof SUMMARY_FILE | typeof VERIFICATION_FILE;
type FixtureOptions = {
  finalStage?: boolean;
  includeVerification?: boolean;
  maybeMutate?: (files: Map<FixtureFile, string>) => void;
  promotedStage?: boolean;
};

const tempRoots: string[] = [];

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

test("fails_missing_checker_command_or_phase117_final_gate", () => {
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
        `${PHASE117_CHECK}\n${PHASE124_CHECK}\nVERIFY_COMMAND_ORDER`,
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

  // Assert
  expect(missingFailures).toContain("verifier mutation command count");
  expect(finalGateFailures).toContain("verifier live command count");
});

function createFixture(options: FixtureOptions = {}): string {
  const phaseComplete = options.finalStage ?? false;
  const finalStage = phaseComplete || (options.promotedStage ?? false);
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase124-"));
  tempRoots.push(root);
  const noClaim =
    "Package relay, filter serving, public-network CI, archive-node behavior, production full-node readiness, and production-funds wallet use remain deferred.";
  const files = new Map<FixtureFile, string>([
    [".planning/REQUIREMENTS.md", createRequirements(finalStage)],
    [".planning/ROADMAP.md", createRoadmap(phaseComplete)],
    [".planning/STATE.md", createState(phaseComplete)],
    [".planning/v2.1-MILESTONE-AUDIT.md", createAudit(finalStage)],
    [".planning/PROJECT.md", noClaim],
    ["README.md", noClaim],
    ["docs/parity/release-readiness.md", noClaim],
    ["docs/parity/production-claim-boundary.md", noClaim],
    ["scripts/verify.sh", createVerifyScript()],
  ]);
  if (options.includeVerification) {
    files.set(
      VERIFICATION_FILE,
      [
        "---",
        "phase: 124-milestone-closeout-reconciliation",
        "status: passed",
        "lifecycle_validated: true",
        `phase_lifecycle_id: ${LIFECYCLE_ID}`,
        "---",
      ].join("\n"),
    );
  }
  options.maybeMutate?.(files);
  for (const [file, text] of files) {
    const absolutePath = path.join(root, file);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, `${text}\n`);
  }
  return root;
}

function createRequirements(finalStage: boolean): string {
  const completeCount = finalStage ? 39 : 38;
  const pendingCount = finalStage ? 0 : 1;
  const checklist = REQUIREMENT_IDS.map((id) => {
    const checked = id !== "HARD-05" || finalStage;
    return `- [${checked ? "x" : " "}] **${id}**: fixture requirement`;
  });
  const traceability = REQUIREMENT_IDS.map((id) => {
    const status = id === "HARD-05" && !finalStage ? "Pending" : "Complete";
    return `| ${id} | Phase ${phaseFor(id)} | ${status} |`;
  });
  return [
    ...checklist,
    ...traceability,
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Complete: ${completeCount}`,
    `- Pending hardening and closeout: ${pendingCount}`,
    "- Unmapped: 0",
  ].join("\n");
}

function createRoadmap(phaseComplete: boolean): string {
  const completeCount = phaseComplete ? 39 : 38;
  const pendingCount = phaseComplete ? 0 : 1;
  const phase124State = phaseComplete ? "x" : " ";
  const phase124Plans = phaseComplete ? "2/2 plans complete" : "1/2 plans executed";
  const maybeRoute = phaseComplete ? `\n## Next Step\n${ARCHIVE_ROUTE}` : "";
  return [
    `- [${phase124State}] **Phase 124: Milestone Closeout Reconciliation**`,
    "#### Phase 122: Compact Relay Peer Completion",
    "**Plans:** 1/1 plans complete",
    "#### Phase 123: Runtime Timing and Evidence Integrity",
    "**Plans:** 7/7 plans complete",
    "#### Phase 124: Milestone Closeout Reconciliation",
    `**Plans:** ${phase124Plans}`,
    "- v2.1 requirements: 39 total",
    "- Mapped to phases: 39",
    `- Satisfied: ${completeCount}`,
    `- Pending hardening and closeout: ${pendingCount}`,
    "- Unmapped: 0",
    maybeRoute,
  ].join("\n");
}

function createState(finalStage: boolean): string {
  if (finalStage) return `Phase 124 verified. Next action: ${ARCHIVE_ROUTE}`;
  return "Phase 124 evidence reconciled; HARD-05 pending";
}

function createAudit(finalStage: boolean): string {
  if (!finalStage) {
    return [
      "---",
      "status: tech_debt",
      "scores:",
      '  requirements: "34/34"',
      '  phases: "12/12"',
      "---",
      "Phase 124 closeout verification pending; do not archive.",
    ].join("\n");
  }
  return [
    "---",
    "status: passed",
    "scores:",
    '  requirements: "39/39"',
    '  phases: "15/15"',
    "gaps:",
    "  requirements: []",
    "  integration: []",
    "  flows: []",
    "tech_debt: []",
    "---",
    "## Resolved Hardening Debt",
    ...RESOLVED_DEBT_IDS.map((id) => `- ${id}: resolved with current evidence.`),
    `## Next Step\n${ARCHIVE_ROUTE}`,
  ].join("\n");
}

function createVerifyScript(): string {
  const commands = [
    PHASE123_TEST,
    PHASE123_CHECK,
    PHASE124_TEST,
    PHASE124_CHECK,
    PHASE117_TEST,
    PHASE117_CHECK,
  ];
  return [
    ": <<'VERIFY_COMMAND_ORDER'",
    ...commands,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 123" ${PHASE123_TEST}`,
    `run_step "check Phase 123" ${PHASE123_CHECK}`,
    `run_step "test Phase 124" ${PHASE124_TEST}`,
    `run_step "check Phase 124" ${PHASE124_CHECK}`,
    `run_step "test Phase 117" ${PHASE117_TEST}`,
    `run_step "check Phase 117" ${PHASE117_CHECK}`,
  ].join("\n");
}

function phaseFor(id: string): number {
  if (id === "HARD-05") return 124;
  if (id === "HARD-01") return 122;
  if (id.startsWith("HARD-")) return 123;
  if (id.startsWith("BOUND-")) return 117;
  if (id.startsWith("OBS-")) return id === "OBS-03" ? 121 : 116;
  if (id.startsWith("GOV-")) return 111;
  if (id.startsWith("RCN-")) return 115;
  if (id.startsWith("CMP-")) return 113;
  return 110;
}

function range(prefix: string, count: number): string[] {
  return Array.from({ length: count }, (_, index) => `${prefix}-${String(index + 1).padStart(2, "0")}`);
}

function replace(
  files: Map<FixtureFile, string>,
  file: FixtureFile,
  needle: string,
  replacement: string,
): void {
  files.set(file, (files.get(file) ?? "").replace(needle, replacement));
}

function append(files: Map<FixtureFile, string>, file: FixtureFile, value: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${value}`);
}

import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase98TraceabilityReconciliation } from "./check-phase98-traceability-reconciliation";

const PHASE97_TEST_COMMAND =
  "bun test scripts/check-phase97-inbound-metrics.test.ts";
const PHASE97_CHECKER_COMMAND =
  "bun run scripts/check-phase97-inbound-metrics.ts";
const PHASE98_TEST_COMMAND =
  "bun test scripts/check-phase98-traceability-reconciliation.test.ts";
const PHASE98_CHECKER_COMMAND =
  "bun run scripts/check-phase98-traceability-reconciliation.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const CANONICAL_ASSIGNMENTS = {
  "INB-01": 98,
  "INB-02": 98,
  "INB-03": 98,
  "INB-04": 98,
  "INB-05": 97,
  "PERM-01": 91,
  "PERM-02": 91,
  "PERM-03": 91,
  "PERM-04": 91,
  "ADDR-01": 92,
  "ADDR-02": 92,
  "ADDR-03": 92,
  "ADDR-04": 92,
  "EVICT-01": 93,
  "EVICT-02": 93,
  "EVICT-03": 96,
  "EVICT-04": 96,
  "DOS-01": 94,
  "DOS-02": 94,
  "DOS-03": 96,
  "DOS-04": 97,
  "DOS-05": 94,
  "BOUND-01": 95,
  "BOUND-02": 95,
  "BOUND-03": 95,
  "BOUND-04": 95,
  "BOUND-05": 95,
  "BOUND-06": 98,
} as const;
const TARGET_FILES = [
  ".planning/milestones/v1.9-REQUIREMENTS.md",
  ".planning/milestones/v1.9-ROADMAP.md",
  ".planning/STATE.md",
  ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
  ".planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md",
  ".planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md",
  ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
  ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
  "docs/parity/release-readiness.md",
  "scripts/check-phase95-network-participation-release-boundary.test.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with complete Phase 98 traceability corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails stale Phase 90 canonical ownership", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/milestones/v1.9-REQUIREMENTS.md",
        "| INB-01 | Phase 98 | Complete |",
        "| INB-01 | Phase 90 | Complete |",
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 canonical ownership");
});

test("fails stale BOUND-06 canonical ownership", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/milestones/v1.9-REQUIREMENTS.md",
        "| BOUND-06 | Phase 98 | Complete |",
        "| BOUND-06 | Phase 95 | Complete |",
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("BOUND-06");
});

test("fails stale roadmap or state status text", () => {
  // Arrange
  const stalePhrases = [
    "21/28",
    "7 pending",
    "/gsd-plan-phase 96",
    "Phase 97 and 98 are still unplanned",
  ];
  const roots = stalePhrases.map((phrase) =>
    createFixture({
      maybeMutateFiles(files) {
        appendToFile(files, ".planning/milestones/v1.9-ROADMAP.md", phrase);
      },
    }),
  );

  // Act
  const messages = roots.map((root) =>
    checkPhase98TraceabilityReconciliation({ rootDir: root }).join("\n"),
  );

  // Assert
  for (const message of messages) {
    expect(message).toContain("P98 stale status");
  }
});

test("fails stale audit closure evidence", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
        'requirements: "23/28"',
        'requirements: "21/28"',
      );
      replaceInFile(
        files,
        ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
        "status: pending_final_verification",
        "status: gaps_found",
      );
      replaceInFile(
        files,
        ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
        "Phase 97 verification: passed",
        "",
      );
      replaceInFile(
        files,
        ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
        ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 audit closure");
});

test("fails missing selected verification notes", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        ".planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md",
        "Canonical ownership note: Phase 95 remains historical release-boundary evidence for BOUND-01 through BOUND-05; Phase 98 is the canonical closure phase for BOUND-06.",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 verification notes");
});

test("fails missing Phase 98 verifier wiring", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(files, "scripts/verify.sh", PHASE98_TEST_COMMAND, "");
      replaceInFile(files, "scripts/verify.sh", PHASE98_CHECKER_COMMAND, "");
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 verifier wiring");
});

test("fails Phase 98 verifier wiring before Phase 97", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "scripts/verify.sh",
        [
          "#!/usr/bin/env bash",
          "set -euo pipefail",
          ": <<'VERIFY_COMMAND_ORDER'",
          PHASE98_TEST_COMMAND,
          PHASE98_CHECKER_COMMAND,
          PHASE97_TEST_COMMAND,
          PHASE97_CHECKER_COMMAND,
          "VERIFY_COMMAND_ORDER",
          `run_step "test Phase 98 traceability reconciliation checker" ${PHASE98_TEST_COMMAND}`,
          `run_step "check Phase 98 traceability reconciliation" ${PHASE98_CHECKER_COMMAND}`,
          `run_step "test Phase 97 inbound metrics checker" ${PHASE97_TEST_COMMAND}`,
          `run_step "check Phase 97 inbound metrics" ${PHASE97_CHECKER_COMMAND}`,
          `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 verifier wiring");
});

test("fails when comments contain correct command order but run_step order is stale", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "scripts/verify.sh",
        [
          "#!/usr/bin/env bash",
          "set -euo pipefail",
          "# Phase 96 is followed by Phase 97. Phase 97 is followed by Phase 98.",
          ": <<'VERIFY_COMMAND_ORDER'",
          PHASE97_TEST_COMMAND,
          PHASE97_CHECKER_COMMAND,
          PHASE98_TEST_COMMAND,
          PHASE98_CHECKER_COMMAND,
          "VERIFY_COMMAND_ORDER",
          `# ${PHASE97_TEST_COMMAND}`,
          `# ${PHASE97_CHECKER_COMMAND}`,
          `# ${PHASE98_TEST_COMMAND}`,
          `# ${PHASE98_CHECKER_COMMAND}`,
          `run_step "test Phase 98 traceability reconciliation checker" ${PHASE98_TEST_COMMAND}`,
          `run_step "check Phase 98 traceability reconciliation" ${PHASE98_CHECKER_COMMAND}`,
          `run_step "test Phase 97 inbound metrics checker" ${PHASE97_TEST_COMMAND}`,
          `run_step "check Phase 97 inbound metrics" ${PHASE97_CHECKER_COMMAND}`,
          `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
        ].join("\n"),
      );
    },
  });

  // Act
  const failures = checkPhase98TraceabilityReconciliation({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P98 verifier wiring executable command order");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase98-"));
  tempRoots.push(root);

  const files = fixtureFiles();
  options.maybeMutateFiles?.(files);

  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }

  return root;
}

function fixtureFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    [".planning/milestones/v1.9-REQUIREMENTS.md", requirementsText()],
    [".planning/milestones/v1.9-ROADMAP.md", roadmapText()],
    [".planning/STATE.md", stateText()],
    [".planning/milestones/v1.9-MILESTONE-AUDIT.md", auditText()],
    [
      ".planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md",
      phase90VerificationText(),
    ],
    [
      ".planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md",
      phase95VerificationText(),
    ],
    [
      ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
      phase97VerificationText(),
    ],
    [
      ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
      phase98VerificationText(),
    ],
    ["docs/parity/release-readiness.md", releaseReadinessText()],
    [
      "scripts/check-phase95-network-participation-release-boundary.test.ts",
      phase95FixtureText(),
    ],
    ["scripts/verify.sh", verifyScriptText()],
  ]);
}

function requirementsText(): string {
  return [
    "# Requirements",
    "| Requirement | Phase | Status |",
    "| --- | --- | --- |",
    ...Object.entries(CANONICAL_ASSIGNMENTS).map(
      ([requirement, phase]) => `| ${requirement} | Phase ${phase} | Complete |`,
    ),
    "**Coverage:**",
    "- v1.9 requirements: 28 total",
    "- Mapped to phases: 28",
    "- Unmapped: 0",
    "- Complete after Phase 98 verification: 28",
    "- Pending Phase 98 verification: 0",
  ].join("\n");
}

function roadmapText(): string {
  return [
    "# Roadmap",
    "Phase 98 is complete and verified.",
    "**Coverage:** 28/28 v1.9 requirements mapped, 0 unmapped. 28 complete after Phase 98 verification.",
    "| Phase | Requirements | Count |",
    "| --- | --- | ---: |",
    "| Phase 90 | — | 0 |",
    "| Phase 91 | PERM-01, PERM-02, PERM-03, PERM-04 | 4 |",
    "| Phase 92 | ADDR-01, ADDR-02, ADDR-03, ADDR-04 | 4 |",
    "| Phase 93 | EVICT-01, EVICT-02 | 2 |",
    "| Phase 94 | DOS-01, DOS-02, DOS-05 | 3 |",
    "| Phase 95 | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05 | 5 |",
    "| Phase 96 | EVICT-03, EVICT-04, DOS-03 | 3 |",
    "| Phase 97 | INB-05, DOS-04 | 2 |",
    "| Phase 98 | INB-01, INB-02, INB-03, INB-04, BOUND-06 | 5 |",
  ].join("\n");
}

function stateText(): string {
  return [
    "# Project State",
    "Phase 98 Traceability Reconciliation complete.",
    "| 97 | Inbound Metrics Sample Production | INB-05, DOS-04 | Complete |",
    "| 98 | Traceability Reconciliation | INB-01, INB-02, INB-03, INB-04, BOUND-06 | Complete |",
    "Phase 90 remains historical implementation evidence while Phase 98 is canonical closure.",
  ].join("\n");
}

function auditText(): string {
  return [
    "---",
    "status: pending_final_verification",
    "scores:",
    '  requirements: "23/28"',
    '  phases: "8/9"',
    "---",
    "# v1.9 Milestone Audit",
    "Phase 97 verification: passed",
    ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
    "INB-05 and DOS-04 are complete.",
    "## Phase 98 Traceability Reconciliation",
    "INT-03-traceability-reconciliation",
    "FLOW-03-phase-completion-to-traceability",
    "Final status remains pending until .planning/phases/98-traceability-reconciliation/98-VERIFICATION.md is written and scripts/check-phase98-traceability-reconciliation.ts passes.",
  ].join("\n");
}

function phase90VerificationText(): string {
  return [
    "# Phase 90 Verification",
    "Canonical ownership note: Phase 90 remains historical implementation evidence for INB-01 through INB-04; Phase 98 is the canonical closure phase for INB-01 through INB-04. Phase 97 is the canonical closure phase for INB-05.",
  ].join("\n");
}

function phase95VerificationText(): string {
  return [
    "# Phase 95 Verification",
    "Canonical ownership note: Phase 95 remains historical release-boundary evidence for BOUND-01 through BOUND-05; Phase 98 is the canonical closure phase for BOUND-06.",
  ].join("\n");
}

function phase97VerificationText(): string {
  return [
    "# Phase 97 Verification",
    "Canonical ownership note: Phase 97 is the canonical closure phase for INB-05 and DOS-04.",
  ].join("\n");
}

function phase98VerificationText(): string {
  return [
    "# Phase 98 Verification",
    "Phase 98 verification passed for INB-01, INB-02, INB-03, INB-04, and BOUND-06.",
    "scripts/check-phase98-traceability-reconciliation.ts",
  ].join("\n");
}

function releaseReadinessText(): string {
  return [
    "# Release Readiness",
    "BOUND-06 traceability covers Phase 90 through Phase 98.",
    "scripts/check-phase98-traceability-reconciliation.ts",
    "v1.9 does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, production service operation, or production full-node readiness.",
  ].join("\n");
}

function phase95FixtureText(): string {
  return [
    "Requirement traceability stays exactly once across Phase 90 through Phase 98.",
    "No active v1.9 stale pending-count fixture remains here.",
  ].join("\n");
}

function verifyScriptText(): string {
  return [
    "#!/usr/bin/env bash",
    "set -euo pipefail",
    "# Phase 96 is followed by Phase 97. Phase 97 is followed by Phase 98.",
    ": <<'VERIFY_COMMAND_ORDER'",
    PHASE97_TEST_COMMAND,
    PHASE97_CHECKER_COMMAND,
    PHASE98_TEST_COMMAND,
    PHASE98_CHECKER_COMMAND,
    "VERIFY_COMMAND_ORDER",
    `run_step "test Phase 97 inbound metrics checker" ${PHASE97_TEST_COMMAND}`,
    `run_step "check Phase 97 inbound metrics" ${PHASE97_CHECKER_COMMAND}`,
    `run_step "test Phase 98 traceability reconciliation checker" ${PHASE98_TEST_COMMAND}`,
    `run_step "check Phase 98 traceability reconciliation" ${PHASE98_CHECKER_COMMAND}`,
    `run_step "check pure-core dependencies" ${PURE_CORE_COMMAND}`,
  ].join("\n");
}

function appendToFile(files: Map<TargetFile, string>, file: TargetFile, line: string): void {
  files.set(file, `${files.get(file) ?? ""}\n${line}\n`);
}

function replaceInFile(
  files: Map<TargetFile, string>,
  file: TargetFile,
  search: string,
  replacement: string,
): void {
  const current = files.get(file) ?? "";
  files.set(file, current.replaceAll(search, replacement));
}

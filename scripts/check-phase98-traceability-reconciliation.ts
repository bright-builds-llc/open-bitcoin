#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE97_TEST_COMMAND =
  "bun test scripts/check-phase97-inbound-metrics.test.ts";
const PHASE97_CHECKER_COMMAND =
  "bun run scripts/check-phase97-inbound-metrics.ts";
const PHASE98_TEST_COMMAND =
  "bun test scripts/check-phase98-traceability-reconciliation.test.ts";
const PHASE98_CHECKER_COMMAND =
  "bun run scripts/check-phase98-traceability-reconciliation.ts";
const PURE_CORE_COMMAND = "bash scripts/check-pure-core-deps.sh";
const REQUIREMENT_PHASE_ASSIGNMENTS = {
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
const ROADMAP_TRACEABILITY_ROWS = [
  "| Phase 90 | — | 0 |",
  "| Phase 91 | PERM-01, PERM-02, PERM-03, PERM-04 | 4 |",
  "| Phase 92 | ADDR-01, ADDR-02, ADDR-03, ADDR-04 | 4 |",
  "| Phase 93 | EVICT-01, EVICT-02 | 2 |",
  "| Phase 94 | DOS-01, DOS-02, DOS-05 | 3 |",
  "| Phase 95 | BOUND-01, BOUND-02, BOUND-03, BOUND-04, BOUND-05 | 5 |",
  "| Phase 96 | EVICT-03, EVICT-04, DOS-03 | 3 |",
  "| Phase 97 | INB-05, DOS-04 | 2 |",
  "| Phase 98 | INB-01, INB-02, INB-03, INB-04, BOUND-06 | 5 |",
] as const;
const STALE_STATUS_PHRASES = [
  "21/28",
  "7 pending",
  "/gsd-plan-phase 96",
  "Phase 97 and 98 are still unplanned",
  "Phase 97 and 98 are still unplanned and unverified",
] as const;
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
type CheckPhase98Options = { rootDir?: string };

export function checkPhase98TraceabilityReconciliation(
  options: CheckPhase98Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyCanonicalOwnership(texts, failures);
  verifyStaleStatus(texts, failures);
  verifyAuditClosure(
    texts.get(".planning/milestones/v1.9-MILESTONE-AUDIT.md") ?? "",
    failures,
  );
  verifyVerificationNotes(texts, failures);
  verifyReleaseReadiness(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P98 missing required corpus file: ${relativePath}`);
    return "";
  }

  return readFileSync(absolutePath, "utf8");
}

function verifyCanonicalOwnership(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const requirements = texts.get(".planning/milestones/v1.9-REQUIREMENTS.md") ?? "";
  const roadmap = texts.get(".planning/milestones/v1.9-ROADMAP.md") ?? "";
  const state = texts.get(".planning/STATE.md") ?? "";

  requireContains(
    requirements,
    "v1.9 requirements: 28 total",
    "P98 canonical ownership requirements coverage",
    failures,
  );
  requireContains(
    requirements,
    "Mapped to phases: 28",
    "P98 canonical ownership requirements coverage",
    failures,
  );
  requireContains(
    requirements,
    "Unmapped: 0",
    "P98 canonical ownership requirements coverage",
    failures,
  );
  for (const [requirement, phase] of Object.entries(REQUIREMENT_PHASE_ASSIGNMENTS)) {
    const rowPattern = new RegExp(`\\|\\s*${escapeRegExp(requirement)}\\s*\\|\\s*Phase ${phase}\\s*\\|`);
    if (!rowPattern.test(requirements)) {
      failures.push(
        `P98 canonical ownership missing ${requirement} -> Phase ${phase}`,
      );
    }
  }
  for (const row of ROADMAP_TRACEABILITY_ROWS) {
    requireContains(roadmap, row, "P98 canonical ownership roadmap traceability", failures);
  }
  requireContains(
    state,
    "Phase 98",
    "P98 canonical ownership state traceability",
    failures,
  );
  requireContains(
    state,
    "INB-01, INB-02, INB-03, INB-04, BOUND-06",
    "P98 canonical ownership state traceability",
    failures,
  );
}

function verifyStaleStatus(texts: Map<TargetFile, string>, failures: string[]): void {
  const filesToScan: TargetFile[] = [
    ".planning/milestones/v1.9-REQUIREMENTS.md",
    ".planning/milestones/v1.9-ROADMAP.md",
    ".planning/STATE.md",
    ".planning/milestones/v1.9-MILESTONE-AUDIT.md",
    "docs/parity/release-readiness.md",
    "scripts/check-phase95-network-participation-release-boundary.test.ts",
  ];
  for (const file of filesToScan) {
    const text = texts.get(file) ?? "";
    for (const phrase of STALE_STATUS_PHRASES) {
      if (text.includes(phrase)) {
        failures.push(`P98 stale status in ${file}: ${phrase}`);
      }
    }
  }
}

function verifyAuditClosure(audit: string, failures: string[]): void {
  for (const phrase of [
    'requirements: "21/28"',
    "status: gaps_found",
    "Phase 97 and 98 are still unplanned and unverified",
  ]) {
    if (audit.includes(phrase)) {
      failures.push(`P98 audit closure still contains stale audit text: ${phrase}`);
    }
  }
  for (const needle of [
    "Phase 97 verification: passed",
    ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
    "INB-05",
    "DOS-04",
    "Phase 98 Traceability Reconciliation",
    "INT-03-traceability-reconciliation",
    "FLOW-03-phase-completion-to-traceability",
    ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
    "scripts/check-phase98-traceability-reconciliation.ts",
  ]) {
    requireContains(audit, needle, "P98 audit closure", failures);
  }
}

function verifyVerificationNotes(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const phase90 = texts.get(
    ".planning/phases/90-inbound-listener-and-admission-policy/90-VERIFICATION.md",
  ) ?? "";
  const phase95 = texts.get(
    ".planning/phases/95-network-participation-evidence-and-release-boundary/95-VERIFICATION.md",
  ) ?? "";
  const phase97 = texts.get(
    ".planning/phases/97-inbound-metrics-sample-production/97-VERIFICATION.md",
  ) ?? "";
  const phase98 = texts.get(
    ".planning/phases/98-traceability-reconciliation/98-VERIFICATION.md",
  ) ?? "";

  requireContains(
    phase90,
    "Canonical ownership note: Phase 90 remains historical implementation evidence for INB-01 through INB-04; Phase 98 is the canonical closure phase for INB-01 through INB-04. Phase 97 is the canonical closure phase for INB-05.",
    "P98 verification notes",
    failures,
  );
  requireContains(
    phase95,
    "Canonical ownership note: Phase 95 remains historical release-boundary evidence for BOUND-01 through BOUND-05; Phase 98 is the canonical closure phase for BOUND-06.",
    "P98 verification notes",
    failures,
  );
  requireContains(
    phase97,
    "Canonical ownership note: Phase 97 is the canonical closure phase for INB-05 and DOS-04.",
    "P98 verification notes",
    failures,
  );
  requireContains(
    phase98,
    "scripts/check-phase98-traceability-reconciliation.ts",
    "P98 verification notes",
    failures,
  );
}

function verifyReleaseReadiness(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const releaseReadiness = texts.get("docs/parity/release-readiness.md") ?? "";
  const phase95Fixture =
    texts.get("scripts/check-phase95-network-participation-release-boundary.test.ts") ?? "";
  for (const text of [releaseReadiness, phase95Fixture]) {
    if (text.includes("Phase 90 through Phase 95")) {
      failures.push("P98 stale status still references Phase 90 through Phase 95 traceability");
    }
  }
  requireContains(
    releaseReadiness,
    "Phase 90 through Phase 98",
    "P98 stale status release-readiness traceability",
    failures,
  );
  requireContains(
    releaseReadiness,
    "scripts/check-phase98-traceability-reconciliation.ts",
    "P98 stale status release-readiness traceability",
    failures,
  );
  for (const boundary of [
    "transaction relay",
    "compact block relay",
    "mempool propagation",
    "public inbound defaults",
    "production service operation",
    "production full-node readiness",
  ]) {
    requireContains(
      releaseReadiness,
      boundary,
      "P98 stale status release-readiness no-claim boundary",
      failures,
    );
  }
}

function verifyVerifierWiring(verifyScript: string, failures: string[]): void {
  const maybeOrderBlock = verifyScript.match(
    /^: <<'VERIFY_COMMAND_ORDER'\n([\s\S]*?)\nVERIFY_COMMAND_ORDER\n/m,
  );
  if (maybeOrderBlock === null) {
    failures.push("P98 verifier wiring missing VERIFY_COMMAND_ORDER block");
  } else {
    requireOrdered(
      maybeOrderBlock[1],
      [
        PHASE97_TEST_COMMAND,
        PHASE97_CHECKER_COMMAND,
        PHASE98_TEST_COMMAND,
        PHASE98_CHECKER_COMMAND,
      ],
      "P98 verifier wiring visible command order",
      failures,
    );
  }

  const executableText = executableVerifyText(verifyScript);
  const runSteps = executableRunSteps(executableText);
  requireRunStep(
    runSteps,
    "test Phase 98 traceability reconciliation checker",
    PHASE98_TEST_COMMAND,
    failures,
  );
  requireRunStep(
    runSteps,
    "check Phase 98 traceability reconciliation",
    PHASE98_CHECKER_COMMAND,
    failures,
  );
  requireContains(
    executableText,
    "Phase 97 is followed by Phase 98",
    "P98 verifier wiring",
    failures,
  );
  requireOrderedRunStepCommands(
    runSteps.map((runStep) => runStep.command),
    [
      PHASE97_TEST_COMMAND,
      PHASE97_CHECKER_COMMAND,
      PHASE98_TEST_COMMAND,
      PHASE98_CHECKER_COMMAND,
      PURE_CORE_COMMAND,
    ],
    "P98 verifier wiring executable command order",
    failures,
  );
}

type RunStep = {
  label: string;
  command: string;
};

function executableVerifyText(text: string): string {
  return text.replace(
    /^: <<'VERIFY_COMMAND_ORDER'\n[\s\S]*?\nVERIFY_COMMAND_ORDER\n/m,
    "",
  );
}

function executableRunSteps(text: string): RunStep[] {
  return text
    .split("\n")
    .map((line) => line.match(/^run_step\s+"([^"]+)"\s+(.+)$/))
    .filter((maybeMatch): maybeMatch is RegExpMatchArray => maybeMatch !== null)
    .map((match) => ({
      label: match[1],
      command: match[2],
    }));
}

function requireRunStep(
  runSteps: RunStep[],
  label: string,
  command: string,
  failures: string[],
): void {
  const hasRunStep = runSteps.some(
    (runStep) => runStep.label === label && runStep.command === command,
  );
  if (!hasRunStep) {
    failures.push(`P98 verifier wiring missing executable run_step: ${label} ${command}`);
  }
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing required text: ${needle}`);
  }
}

function requireOrderedRunStepCommands(
  commands: readonly string[],
  expectedCommands: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const expectedCommand of expectedCommands) {
    const index = commands.indexOf(expectedCommand);
    if (index === -1) {
      failures.push(`${label} missing ${expectedCommand}`);
      continue;
    }
    if (index <= cursor) {
      failures.push(`${label} has ${expectedCommand} out of order`);
      continue;
    }
    cursor = index;
  }
}

function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle);
    if (index === -1) {
      failures.push(`${label} missing ${needle}`);
      continue;
    }
    if (index <= cursor) {
      failures.push(`${label} has ${needle} out of order`);
      continue;
    }
    cursor = index;
  }
}

function escapeRegExp(text: string): string {
  return text.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
}

if (import.meta.main) {
  const failures = checkPhase98TraceabilityReconciliation();
  if (failures.length > 0) {
    console.error("Phase 98 traceability reconciliation checker failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log("Phase 98 traceability reconciliation checker passed.");
}

#!/usr/bin/env bun

import path from "node:path";
import { readSourceCorpus } from "./source-corpus";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE78_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/78-progress-guarantees-and-stall-diagnosis";
const PHASE78_REQUIREMENTS = ["PROG-01", "PROG-02", "PROG-03", "PROG-04"] as const;
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const PHASE78_TEST_COMMAND = "bun test scripts/check-phase78-progress-guarantees.test.ts";
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const SURFACE_ID = "phase78-progress-guarantees-stall-diagnosis";
const FORBIDDEN_RUNTIME_CREDIT_EXPRESSION =
  "summary.headers_received > 0 || summary.blocks_received > 0";
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
  "sleep 86400",
  "multi-day wall-clock",
  "lsof",
  "/proc/",
] as const;
const PLAN_FILES = [
  `${PHASE_DIR}/78-01-PLAN.md`,
  `${PHASE_DIR}/78-02-PLAN.md`,
  `${PHASE_DIR}/78-03-PLAN.md`,
  `${PHASE_DIR}/78-04-PLAN.md`,
  `${PHASE_DIR}/78-05-PLAN.md`,
  `${PHASE_DIR}/78-06-PLAN.md`,
  `${PHASE_DIR}/78-07-PLAN.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

const SOURCE_ANCHORS = {
  "packages/open-bitcoin-node/src/status/progress_guarantee.rs": [
    "ProgressCreditEvidence",
    "ProgressCreditKind",
    "RejectedProgressActivityKind",
    "ProgressWindowEvidence",
    "NoProgressThresholdEvidence",
    "PeerContributionEvidence",
    "StalledSubsystem",
    "StallDiagnosisEvidence",
  ],
  "packages/open-bitcoin-node/src/status.rs": [
    "progress_credit",
    "expected_progress_window",
    "no_progress_threshold",
    "last_useful_work",
    "last_peer_contribution",
    "stall_diagnosis",
  ],
  "packages/open-bitcoin-node/src/sync/progress.rs": [
    "made_validated_durable_progress",
    "classify_progress_credit",
    "classify_stall_diagnosis",
  ],
  "packages/open-bitcoin-node/src/sync/runtime_state.rs": [
    "made_validated_durable_progress",
    "classify_progress_credit",
    "classify_stall_diagnosis",
    "write_progress_guarantee_log",
  ],
} as const satisfies AnchorMap;

const TEST_ANCHORS = {
  "packages/open-bitcoin-node/src/sync/tests/soak.rs": [
    "phase78_header_and_download_activity_do_not_credit_soak_progress",
  ],
  "packages/open-bitcoin-node/src/sync/tests.rs": [
    "phase78_branch_competition_does_not_credit_replacement_tip_before_connect",
    "phase78_current_at_tip_credits_stay_current_useful_work",
    "phase78_stale_inflight_cleanup_preserves_prior_credit_and_rotates_peer",
    "phase78_no_credit_peer_rotation_keeps_last_peer_contribution_without_credit",
    "phase78_validation_stall_classifies_validation_subsystem",
    "phase78_storage_resource_pressure_outranks_peer_retry_advice",
    "phase78_operator_stop_and_shutdown_classify_local_subsystems",
  ],
} as const satisfies AnchorMap;

const DOC_ANCHORS = {
  "docs/operator/runtime-guide.md": [
    "progress_credit",
    "last_useful_work",
    "last_peer_contribution",
    "expected_progress_window",
    "no_progress_threshold",
    "stall_diagnosis",
    "validated_durable_active_chain",
    "current_at_best_known_tip",
    "storage_or_resource_pressure",
    "at_tip_waiting",
    "operator_stop",
    "local_shutdown",
    "Headers, downloaded block bodies, peer messages, in-flight requests, retries, and report generation are evidence only and do not advance the credited progress",
  ],
  "docs/architecture/status-snapshot.md": [
    "progress_credit",
    "last_useful_work",
    "last_peer_contribution",
    "expected_progress_window",
    "no_progress_threshold",
    "stall_diagnosis",
    "validated_durable_active_chain",
    "current_at_best_known_tip",
    "storage_or_resource_pressure",
    "at_tip_waiting",
    "operator_stop",
    "local_shutdown",
  ],
  "docs/architecture/operator-observability.md": [
    "progress_credit",
    "last_useful_work",
    "last_peer_contribution",
    "expected_progress_window",
    "no_progress_threshold",
    "stall_diagnosis",
  ],
} as const satisfies AnchorMap;

const PARITY_FILES = [
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/README.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;

function readText(relativePath: string, failures: string[]): string {
  try {
    return readSourceCorpus(REPO_ROOT, relativePath);
  } catch {
    failures.push(`missing required file: ${relativePath}`);
    return "";
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

function requireNotContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain Phase 78 forbidden text: ${needle}`);
  }
}

function requireAnchors(anchors: AnchorMap, failures: string[]): void {
  for (const [file, needles] of Object.entries(anchors)) {
    const text = readText(file, failures);
    for (const needle of needles) {
      requireContains(text, needle, file, failures);
    }
  }
}

function frontmatterFor(text: string): string {
  if (!text.startsWith("---")) {
    return text;
  }

  const endIndex = text.indexOf("\n---", 3);
  if (endIndex === -1) {
    return text;
  }

  return text.slice(0, endIndex);
}

function verifyPlanRequirements(failures: string[]): void {
  const frontmatters = PLAN_FILES.map((planFile) =>
    frontmatterFor(readText(planFile, failures)),
  ).join("\n");

  for (const requirement of PHASE78_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 78 plan frontmatter", failures);
  }
}

function verifyRuntimeCreditSource(failures: string[]): void {
  const runtimeState = readText("packages/open-bitcoin-node/src/sync/runtime_state.rs", failures);
  requireNotContains(
    runtimeState,
    FORBIDDEN_RUNTIME_CREDIT_EXPRESSION,
    "packages/open-bitcoin-node/src/sync/runtime_state.rs",
    failures,
  );
}

function verifyParityCoverage(failures: string[]): void {
  for (const file of PARITY_FILES) {
    const text = readText(file, failures);
    requireContains(text, SURFACE_ID, file, failures);
    for (const requirement of PHASE78_REQUIREMENTS) {
      requireContains(text, requirement, file, failures);
    }
  }
}

function verifyVerifyScript(failures: string[]): void {
  const verifyScript = readText("scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE77_CHECKER_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE78_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE78_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const lines = verifyScript
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const phase77CheckerIndex = lines.indexOf(PHASE77_CHECKER_COMMAND);
  const phase78TestIndex = lines.indexOf(PHASE78_TEST_COMMAND);
  const phase78CheckerIndex = lines.indexOf(PHASE78_CHECKER_COMMAND);
  if (
    phase77CheckerIndex === -1 ||
    phase78TestIndex !== phase77CheckerIndex + 1 ||
    phase78CheckerIndex !== phase78TestIndex + 1
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 78 checker test and checker immediately after the Phase 77 checker",
    );
  }

  for (const forbidden of FORBIDDEN_VERIFY_STRINGS) {
    requireNotContains(verifyScript, forbidden, "scripts/verify.sh", failures);
  }
}

function main(): void {
  const failures: string[] = [];

  verifyPlanRequirements(failures);
  requireAnchors(SOURCE_ANCHORS, failures);
  requireAnchors(TEST_ANCHORS, failures);
  requireAnchors(DOC_ANCHORS, failures);
  verifyRuntimeCreditSource(failures);
  verifyParityCoverage(failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 78 progress guarantees and stall diagnosis boundaries");
}

main();

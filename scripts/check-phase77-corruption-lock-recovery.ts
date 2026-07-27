#!/usr/bin/env bun

import path from "node:path";
import { readSourceCorpus } from "./source-corpus";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE77_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/77-corruption-and-lock-recovery-hardening";
const PHASE77_REQUIREMENTS = ["REC-05", "REC-06", "REC-07", "REC-08"] as const;
const PHASE76_CHECKER_COMMAND = "bun run scripts/check-phase76-resource-bounds.ts";
const PHASE77_TEST_COMMAND = "bun test scripts/check-phase77-corruption-lock-recovery.test.ts";
const PHASE77_CHECKER_COMMAND = "bun run scripts/check-phase77-corruption-lock-recovery.ts";
const SURFACE_ID = "phase77-corruption-and-lock-recovery-hardening";
const NO_MUTATION_BOUNDARY =
  "Phase 77 does not delete lock files, clear recovery markers, repair stores, compact stores, reindex stores, relocate datadirs, mutate source datadirs, scan OS process tables, or upload support bundles automatically.";
const FORBIDDEN_VERIFY_STRINGS = [
  "run-live-mainnet-smoke",
  "--manual-peer",
  "--restart-after-progress",
  "systemctl",
  "launchctl",
  "openbitcoinsync=mainnet-ibd",
  "sleep 86400",
  "lsof",
  "/proc/",
] as const;
const PLAN_FILES = [
  `${PHASE_DIR}/77-01-PLAN.md`,
  `${PHASE_DIR}/77-02-PLAN.md`,
  `${PHASE_DIR}/77-03-PLAN.md`,
  `${PHASE_DIR}/77-04-PLAN.md`,
  `${PHASE_DIR}/77-05-PLAN.md`,
  `${PHASE_DIR}/77-06-PLAN.md`,
  `${PHASE_DIR}/77-07-PLAN.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

const SOURCE_ANCHORS = {
  "packages/open-bitcoin-node/src/recovery.rs": [
    "RecoveryEvidenceSnapshot",
    "RecoveryActionClass",
    "RecoveryCause",
    "RecoveryEvidenceBasis",
    "LockEvidenceKind",
  ],
  "packages/open-bitcoin-node/src/storage/lock_probe.rs": [
    "probe_fjall_lock",
    "FJALL_LOCK_FILE_NAME",
  ],
  "packages/open-bitcoin-node/src/storage/fjall_store.rs": [
    "fjall::Error::Locked",
    "database locked by another process",
  ],
} as const satisfies AnchorMap;

const STATUS_AND_SUPPORT_ANCHORS = {
  "packages/open-bitcoin-node/src/status.rs": [
    "recovery_evidence: FieldAvailability<RecoveryEvidenceSnapshot>",
  ],
  "packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs": [
    "collect_status_recovery_evidence",
    "probe_fjall_lock",
  ],
  "packages/open-bitcoin-cli/src/operator/status/render.rs": ["Recovery evidence:"],
  "packages/open-bitcoin-cli/src/operator/status/service_status.rs": [
    "probe-only status does not open Fjall stores",
  ],
  "packages/open-bitcoin-cli/src/operator/status/tests.rs": [
    "status_recovery_evidence_",
    "stale_lock_evidence",
    "concurrent_datadir_use",
    "probe-only status does not open Fjall stores",
  ],
  "packages/open-bitcoin-cli/src/operator/support.rs": ["recovery_evidence"],
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs": ["status.recovery_evidence"],
  "packages/open-bitcoin-cli/src/operator/support/render.rs": ["## Recovery Evidence"],
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "support_recovery_evidence_",
    "recovery_evidence",
  ],
} as const satisfies AnchorMap;

const LIVE_SMOKE_ANCHORS = {
  "scripts/run-live-mainnet-smoke.ts": [
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ],
  "scripts/test-run-live-mainnet-smoke.sh": [
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ],
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs": [
    "recoveryEvidence",
    "recoveryActionClass",
    "recoveryCause",
    "recoveryNextAction",
    "maybeRecoveryEvidenceUnavailableReason",
  ],
  "packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs": [
    "phase77_live_smoke_summary_preserves_recovery_evidence",
    "live_smoke_recovery_evidence_",
  ],
  "docs/operator/runtime-guide.md": ["bash scripts/test-run-live-mainnet-smoke.sh"],
} as const satisfies AnchorMap;

const SOAK_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/soak/ledger.rs": [
    "maybe_recovery_action_class_label",
    "maybe_recovery_cause_label",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/runtime/helpers.rs": [
    "maybe_recovery_action_class_label",
    "maybe_recovery_cause_label",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/report.rs": [
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/tests.rs": [
    "soak_recovery_evidence_",
    "Recovery action class",
    "Recovery cause",
    "Recovery next action",
  ],
} as const satisfies AnchorMap;

const DOC_AND_PARITY_ANCHORS = {
  "docs/operator/runtime-guide.md": [NO_MUTATION_BOUNDARY],
  "docs/architecture/storage-decision.md": [NO_MUTATION_BOUNDARY],
  "docs/parity/index.json": [SURFACE_ID, ...PHASE77_REQUIREMENTS],
  "docs/parity/README.md": [SURFACE_ID, ...PHASE77_REQUIREMENTS],
  "docs/parity/checklist.md": [SURFACE_ID, ...PHASE77_REQUIREMENTS],
} as const satisfies AnchorMap;
const PARITY_FILES = [
  "docs/parity/index.json",
  "docs/parity/README.md",
  "docs/parity/checklist.md",
  "docs/parity/release-readiness.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;

const DETERMINISTIC_TEST_ANCHORS = {
  "packages/open-bitcoin-node/src/storage/fjall_store/tests.rs": [
    "lock_probe_",
    "fjall_recovery_evidence_",
  ],
  "packages/open-bitcoin-cli/src/operator/status/tests.rs": ["status_recovery_evidence_"],
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": ["support_recovery_evidence_"],
  "packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs": [
    "live_smoke_recovery_evidence_",
  ],
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs": [
    "dashboard_recovery_evidence_",
  ],
  "packages/open-bitcoin-cli/src/operator/soak/tests/runtime.rs": ["soak_recovery_evidence_"],
} as const satisfies AnchorMap;

const PROBE_ONLY_STATUS_SUPPORT_FILES = [
  "packages/open-bitcoin-cli/src/operator/status/recovery_evidence.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/sync_state.rs",
  "packages/open-bitcoin-cli/src/operator/status/service_status.rs",
  "packages/open-bitcoin-cli/src/operator/support.rs",
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
    failures.push(`${label} must not contain default-verification or probe mutation text: ${needle}`);
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

  for (const requirement of PHASE77_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 77 plan frontmatter", failures);
  }
}

function verifyParityCoverage(failures: string[]): void {
  const parityText = PARITY_FILES.map((file) => readText(file, failures)).join("\n");
  requireContains(parityText, SURFACE_ID, "Phase 77 parity docs", failures);
  for (const requirement of PHASE77_REQUIREMENTS) {
    requireContains(parityText, requirement, "Phase 77 parity docs", failures);
  }
}

function verifyProbeOnlyStatusAndSupport(failures: string[]): void {
  for (const file of PROBE_ONLY_STATUS_SUPPORT_FILES) {
    const text = readText(file, failures);
    requireNotContains(text, "FjallNodeStore::open", file, failures);
    requireNotContains(text, "Database::builder", file, failures);
    requireNotContains(text, "WalletRegistry::load", file, failures);
  }
}

function verifyVerifyScript(failures: string[]): void {
  const verifyScript = readText("scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE76_CHECKER_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE77_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE77_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const lines = verifyScript
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const phase76CheckerIndex = lines.indexOf(PHASE76_CHECKER_COMMAND);
  const phase77TestIndex = lines.indexOf(PHASE77_TEST_COMMAND);
  const phase77CheckerIndex = lines.indexOf(PHASE77_CHECKER_COMMAND);
  if (
    phase76CheckerIndex === -1 ||
    phase77TestIndex !== phase76CheckerIndex + 1 ||
    phase77CheckerIndex !== phase77TestIndex + 1
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 77 checker test and checker immediately after the Phase 76 checker",
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
  requireAnchors(STATUS_AND_SUPPORT_ANCHORS, failures);
  requireAnchors(LIVE_SMOKE_ANCHORS, failures);
  requireAnchors(SOAK_ANCHORS, failures);
  requireAnchors(DOC_AND_PARITY_ANCHORS, failures);
  requireAnchors(DETERMINISTIC_TEST_ANCHORS, failures);
  verifyParityCoverage(failures);
  verifyProbeOnlyStatusAndSupport(failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 77 corruption and lock recovery hardening boundaries");
}

main();

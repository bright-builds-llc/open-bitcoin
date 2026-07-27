#!/usr/bin/env bun

import path from "node:path";
import { readSourceCorpus, readSourceRoot } from "./source-corpus";

const REPO_ROOT_OVERRIDE_ENV = "OPEN_BITCOIN_PHASE79_REPO_ROOT";
const maybeRepoRoot = process.env[REPO_ROOT_OVERRIDE_ENV];
const REPO_ROOT =
  maybeRepoRoot === undefined ? path.resolve(import.meta.dir, "..") : path.resolve(maybeRepoRoot);
const PHASE_DIR = ".planning/phases/79-diagnostics-and-support-bundle-forensics";
const PHASE79_REQUIREMENTS = ["DIAG-01", "DIAG-02", "DIAG-03", "DIAG-04"] as const;
const PHASE78_CHECKER_COMMAND = "bun run scripts/check-phase78-progress-guarantees.ts";
const PHASE79_TEST_COMMAND = "bun test scripts/check-phase79-diagnostics-support-bundle.test.ts";
const PHASE79_CHECKER_COMMAND = "bun run scripts/check-phase79-diagnostics-support-bundle.ts";
const SURFACE_ID = "phase79-diagnostics-support-bundle-forensics";
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
const FORBIDDEN_SUPPORT_OUTPUT_STRINGS = [
  "rpcpassword=phase79-secret",
  "rpcauth=phase79-secret",
  "phase79 wallet seed phrase",
  "raw daemon stdout phase79-secret",
  "raw daemon stderr phase79-secret",
  "raw live-smoke input phase79-secret",
  "raw options phase79-secret",
  "endpoint table phase79-secret",
] as const;

const PLAN_FILES = [
  `${PHASE_DIR}/79-01-PLAN.md`,
  `${PHASE_DIR}/79-02-PLAN.md`,
  `${PHASE_DIR}/79-03-PLAN.md`,
  `${PHASE_DIR}/79-04-PLAN.md`,
] as const;

type AnchorMap = Record<string, readonly string[]>;

const SOURCE_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/support/forensics.rs": [
    "SupportForensicsEvidence",
    "ForensicTimelineEntry",
    "CheckpointChainEvidence",
    "ForensicNarrative",
    "ForensicVerdict",
    "ForensicConfidence",
    "ForensicSourceEvidence",
    "ForensicRedactionEvidence",
    "sha256-json-v1",
    "open-bitcoin-support-forensics-v1",
    "likely_cause",
    "evidence_basis",
    "next_action",
    "confidence",
  ],
  "packages/open-bitcoin-cli/src/operator/support.rs": [
    "support_forensics",
    "resource_bound_evidence",
    "collect_soak_support_evidence",
    "redaction_summary",
  ],
  "packages/open-bitcoin-cli/src/operator/support/render.rs": [
    "push_support_forensics",
    "## Forensic Timeline",
    "## Checkpoint Chain",
    "## Failure Narrative",
    "csv_or_unavailable",
  ],
  "packages/open-bitcoin-cli/src/operator/support/resource_bounds.rs": [
    "ResourceBoundSupportEvidence",
    "maybe_projected_bundle_size_bytes",
    "ResourceBoundKind::SupportBundle",
  ],
  "packages/open-bitcoin-cli/src/operator/support/progress_guarantee.rs": [
    "progress_guarantee_summary",
    "stall_diagnosis_summary",
    "progress_credit_summary_text",
  ],
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs": [
    "validated_active_chain",
    "peer_contribution",
    "no_progress_or_reorg_events",
    "stall_diagnosis",
    "live_smoke_final_status",
  ],
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs": [
    "summarize_final_status",
    "maybe_no_progress_cause",
    "maybe_recovery_evidence_unavailable_reason",
  ],
  "packages/open-bitcoin-cli/src/operator/dashboard/model/sync_section.rs": [
    "progress_credit_text",
    "stall_diagnosis_text",
    "recovery::recovery_evidence",
  ],
  "packages/open-bitcoin-rpc/src/method/node.rs": [
    "OpenBitcoinSyncStatusRequest",
    "OpenBitcoinSyncControlResponse",
    "RuntimeMetadata",
  ],
  "packages/open-bitcoin-node/src/sync/types/summary.rs": [
    "validated_active_chain_height",
    "progress_credit",
    "last_peer_contribution",
    "stall_diagnosis",
    "latest_stop_reason",
  ],
  "packages/open-bitcoin-node/src/status.rs": [
    "OpenBitcoinStatusSnapshot",
    "progress_credit",
    "resource_bounds",
    "recovery_evidence",
    "no_progress_diagnosis",
    "stall_diagnosis",
  ],
} as const satisfies AnchorMap;

const TEST_ANCHORS = {
  "packages/open-bitcoin-cli/src/operator/support/tests.rs": [
    "phase79_support_forensics_projection_builds_timeline_chain_and_narrative",
    "phase79_support_forensics_projection_detects_sequence_gaps_and_truncation",
    "phase79_support_forensics_projection_keeps_unavailable_evidence_conservative",
    "phase79_support_forensics_json_includes_sidecar_contract",
    "phase79_support_forensics_json_excludes_sensitive_seed_material",
    "phase79_support_forensics_markdown_renders_timeline_chain_and_failure_narrative",
    "phase79_support_forensics_cross_surface_agreement_uses_shared_status_contract",
    "phase79_support_forensics_markdown_and_json_exclude_forbidden_sensitive_material",
  ],
} as const satisfies AnchorMap;

const DOC_ANCHORS = {
  "docs/operator/runtime-guide.md": [
    "### Phase 79 support bundle forensics",
    "support_forensics",
    "forensic timeline",
    "checkpoint chain",
    "failure narrative",
    "likely_cause",
    "evidence_basis",
    "next_action",
    "confidence",
    "soak_stable",
    "blocker_diagnosed",
    "inconclusive",
    "collection_failed",
    "not authenticity",
    "support bundle existence, elapsed time, peer reachability, daemon startup, raw logs, or stale reports do not prove soak stability",
    "public-network-free",
    "service-manager-free",
  ],
  "docs/architecture/status-snapshot.md": [
    "## Phase 79 shared diagnostic contract and support-forensics sidecar",
    "OpenBitcoinStatusSnapshot",
    "support_forensics",
    "resource_bound_evidence.maybe_projected_bundle_size_bytes",
    "checkpoint-chain validation",
  ],
  "docs/architecture/operator-observability.md": [
    "## Phase 79 support forensics projection",
    "CLI status",
    "dashboard status",
    "RPC status",
    "metrics",
    "structured logs",
    "live-smoke summaries",
    "soak reports",
    "bounded labels and counts, not high-cardinality forensic objects",
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
const PARITY_ANCHORS = [
  "support_forensics",
  "forensic timeline",
  "checkpoint chain",
  "failure narrative",
  "likely cause",
  "evidence basis",
  "next action",
  "confidence",
  "redaction",
  "size bounds",
  "timeline ordering",
  "cross-surface consistency",
] as const;
const PARITY_NON_CLAIMS = [
  "inbound serving",
  "relay",
  "production-funds wallet use",
  "migration apply mode",
  "packaging",
  "GUI",
  "hosted dashboards",
  "public-network default checks",
  "multi-day default gates",
  "automatic support-bundle upload",
  "production-node readiness",
] as const;
const PRODUCTION_SUPPORT_OUTPUT_FILES = [
  "packages/open-bitcoin-cli/src/operator/support.rs",
  "packages/open-bitcoin-cli/src/operator/support/forensics.rs",
  "packages/open-bitcoin-cli/src/operator/support/render.rs",
  "packages/open-bitcoin-cli/src/operator/support/soak_evidence.rs",
] as const;

function readText(relativePath: string, failures: string[]): string {
  try {
    return readSourceCorpus(REPO_ROOT, relativePath);
  } catch {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }
}

function readRootText(relativePath: string, failures: string[]): string {
  try {
    return readSourceRoot(REPO_ROOT, relativePath);
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
    failures.push(`${label} must not contain Phase 79 forbidden text: ${needle}`);
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

  for (const requirement of PHASE79_REQUIREMENTS) {
    requireContains(frontmatters, requirement, "Phase 79 plan frontmatter", failures);
  }
}

function verifyParityCoverage(failures: string[]): void {
  for (const file of PARITY_FILES) {
    const text = readText(file, failures);
    requireContains(text, SURFACE_ID, file, failures);
    for (const requirement of PHASE79_REQUIREMENTS) {
      requireContains(text, requirement, file, failures);
    }
    for (const anchor of PARITY_ANCHORS) {
      requireContains(text, anchor, file, failures);
    }
    for (const nonClaim of PARITY_NON_CLAIMS) {
      requireContains(text, nonClaim, file, failures);
    }
  }
}

function verifySupportRedactionBoundaries(failures: string[]): void {
  for (const file of PRODUCTION_SUPPORT_OUTPUT_FILES) {
    const text = readRootText(file, failures);
    for (const forbidden of FORBIDDEN_SUPPORT_OUTPUT_STRINGS) {
      requireNotContains(text, forbidden, file, failures);
    }
  }
}

function verifyVerifyScript(failures: string[]): void {
  const verifyScript = readText("scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE78_CHECKER_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE79_TEST_COMMAND, "scripts/verify.sh", failures);
  requireContains(verifyScript, PHASE79_CHECKER_COMMAND, "scripts/verify.sh", failures);

  const lines = verifyScript
    .split("\n")
    .map((line) => line.trim())
    .filter((line) => line.length > 0);
  const phase78CheckerIndex = lines.indexOf(PHASE78_CHECKER_COMMAND);
  const phase79TestIndex = lines.indexOf(PHASE79_TEST_COMMAND);
  const phase79CheckerIndex = lines.indexOf(PHASE79_CHECKER_COMMAND);
  if (
    phase78CheckerIndex === -1 ||
    phase79TestIndex !== phase78CheckerIndex + 1 ||
    phase79CheckerIndex !== phase79TestIndex + 1
  ) {
    failures.push(
      "scripts/verify.sh must run the Phase 79 checker test and checker immediately after the Phase 78 checker",
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
  verifyParityCoverage(failures);
  verifySupportRedactionBoundaries(failures);
  verifyVerifyScript(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 79 diagnostics and support bundle forensics boundaries");
}

main();

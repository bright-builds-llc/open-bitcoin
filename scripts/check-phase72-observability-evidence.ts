#!/usr/bin/env bun

import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE_DIR = ".planning/phases/72-operator-observability-and-support-evidence";
const PLAN_FILES = [
  `${PHASE_DIR}/72-01-PLAN.md`,
  `${PHASE_DIR}/72-02-PLAN.md`,
  `${PHASE_DIR}/72-03-PLAN.md`,
  `${PHASE_DIR}/72-04-PLAN.md`,
] as const;
const REQUIREMENT_IDS = ["OBS-01", "OBS-02", "OBS-03", "OBS-04"] as const;
const STATUS_SURFACE_FILES = [
  "packages/open-bitcoin-cli/src/operator/status/render.rs",
  "packages/open-bitcoin-cli/src/operator/status/render/tests.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
] as const;
const SUPPORT_FILES = [
  "packages/open-bitcoin-cli/src/operator/support/evidence.rs",
  "packages/open-bitcoin-cli/src/operator/support.rs",
  "packages/open-bitcoin-cli/src/operator/support/live_smoke.rs",
  "packages/open-bitcoin-cli/src/operator/support/live_smoke/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/render.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "packages/open-bitcoin-cli/tests/operator_binary.rs",
] as const;
const TELEMETRY_FILES = [
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/sync/types/summary.rs",
  "packages/open-bitcoin-node/src/sync/types/summary/tests.rs",
  "scripts/run-live-mainnet-smoke.ts",
  "scripts/test-run-live-mainnet-smoke.sh",
] as const;
const DOC_FILES = [
  "docs/operator/runtime-guide.md",
  "docs/architecture/status-snapshot.md",
  "docs/architecture/operator-observability.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/catalog/chainstate.md",
  "docs/parity/catalog/operator-runtime-release-hardening.md",
] as const;
const RAW_SUPPORT_FIELDS = [
  "stdoutTail",
  "stderrTail",
  "rawPeerTable",
  "rawLogTail",
  "walletMaterial",
  "rpcpassword",
  "rpcauth",
  "__cookie__",
] as const;

type Phase72Fixture = {
  connectedHeight: string;
  connectedHash: string;
  connectedWork: string;
  validatedHeight: string;
  validatedHash: string;
  validatedWork: string;
  tipFreshness: string;
  recoveryCategory: string;
  peerContributionConnected: string;
  peerContributionFailed: string;
  nextAction: string;
};

type SurfaceComparison = {
  surface: string;
  file: string;
  available: Record<string, readonly string[]>;
  unavailable: Record<string, readonly string[]>;
};

function repoPath(relativePath: string): string {
  return path.join(REPO_ROOT, relativePath);
}

async function readText(relativePath: string, failures: string[]): Promise<string> {
  const file = Bun.file(repoPath(relativePath));
  if (!(await file.exists())) {
    failures.push(`missing required file: ${relativePath}`);
    return "";
  }
  return file.text();
}

async function readJoined(files: readonly string[], failures: string[]): Promise<string> {
  const parts = [];
  for (const file of files) {
    parts.push(await readText(file, failures));
  }
  return parts.join("\n");
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
    failures.push(`${label} must not contain default verification command: ${needle}`);
  }
}

function sharedStatusSnapshotFixture(): Phase72Fixture {
  const hash = "1111111111111111111111111111111111111111111111111111111111111111";
  return {
    connectedHeight: "840004",
    connectedHash: hash,
    connectedWork: "840005",
    validatedHeight: "840004",
    validatedHash: hash,
    validatedWork: "840005",
    tipFreshness: "fresh",
    recoveryCategory: "resource_exhaustion",
    peerContributionConnected: "3",
    peerContributionFailed: "1",
    nextAction: "free storage and retry validation",
  };
}

function phase72AvailableFixture(): Record<string, string> {
  const fixture = sharedStatusSnapshotFixture();
  return {
    connected_height: fixture.connectedHeight,
    connected_hash: fixture.connectedHash,
    connected_work: fixture.connectedWork,
    validated_active_chain_height: fixture.validatedHeight,
    maybe_validated_active_chain_hash: fixture.validatedHash,
    maybe_validated_active_chain_work: fixture.validatedWork,
    best_known_tip_freshness: fixture.tipFreshness,
    recovery_category: fixture.recoveryCategory,
    peer_contribution_connected: fixture.peerContributionConnected,
    peer_contribution_failed: fixture.peerContributionFailed,
    next_action: fixture.nextAction,
  };
}

function phase72UnavailableReasonFixture(): Record<string, string> {
  return {
    best_known_tip: "Unavailable: best-known tip evidence unavailable",
    stay_current: "Unavailable: stay-current state unavailable",
    connected_active_chain_hash: "Unavailable: connected active-chain hash unavailable",
    connected_active_chain_work: "Unavailable: connected active-chain work unavailable",
    validated_active_chain_hash: "Unavailable: validated active-chain hash unavailable",
    validated_active_chain_work: "Unavailable: validated active-chain work unavailable",
    latest_reorg: "Unavailable: no reorg evidence recorded",
    reconcile_progress: "Unavailable: reconcile progress unavailable",
    next_action: "Unavailable: guidance withheld",
    matrix_placeholder: "Unavailable: {reason}",
  };
}

function phase72SurfaceComparisonMatrix(): readonly SurfaceComparison[] {
  const available = phase72AvailableFixture();
  const unavailable = phase72UnavailableReasonFixture();

  return [
    {
      surface: "CLI human status",
      file: "packages/open-bitcoin-cli/src/operator/status/render/tests.rs",
      available: {
        connected_height: [`connected_blocks=${available.connected_height}`],
        validated_active_chain_height: [
          `validated_active_chain_height=${available.validated_active_chain_height}`,
        ],
        maybe_validated_active_chain_hash: [
          `validated_active_chain_hash=${available.maybe_validated_active_chain_hash}`,
        ],
        maybe_validated_active_chain_work: [
          `validated_active_chain_work=${available.maybe_validated_active_chain_work}`,
        ],
        best_known_tip_freshness: [available.best_known_tip_freshness],
      },
      unavailable: {
        best_known_tip: [`Sync best-known tip: ${unavailable.best_known_tip}`],
        stay_current: [`Sync stay-current: ${unavailable.stay_current}`],
        latest_reorg: [`Sync latest reorg: ${unavailable.latest_reorg}`],
        reconcile_progress: [`Sync reconcile: ${unavailable.reconcile_progress}`],
        next_action: [unavailable.next_action],
      },
    },
    {
      surface: "Dashboard projection",
      file: "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
      available: {
        validated_active_chain_height: [
          `validated_active_chain_height=${available.validated_active_chain_height}`,
        ],
        maybe_validated_active_chain_hash: [
          `validated_active_chain_hash=${available.maybe_validated_active_chain_hash}`,
        ],
        maybe_validated_active_chain_work: [
          `validated_active_chain_work=${available.maybe_validated_active_chain_work}`,
        ],
      },
      unavailable: {
        best_known_tip: [unavailable.best_known_tip],
        stay_current: [unavailable.stay_current],
        latest_reorg: [unavailable.latest_reorg],
        reconcile_progress: [unavailable.reconcile_progress],
      },
    },
    {
      surface: "RPC durable sync status",
      file: "packages/open-bitcoin-rpc/src/dispatch/tests.rs",
      available: {
        validated_active_chain_height: ["validated_active_chain_height", "json!(840_004)"],
        maybe_validated_active_chain_hash: [
          "maybe_validated_active_chain_hash",
          `json!("11".repeat(32))`,
        ],
        maybe_validated_active_chain_work: [
          "maybe_validated_active_chain_work",
          `json!("${available.maybe_validated_active_chain_work}")`,
        ],
        best_known_tip_freshness: ["best_known_tip", `json!("${available.best_known_tip_freshness}")`],
      },
      unavailable: {
        best_known_tip: ["best-known tip evidence unavailable"],
        stay_current: ["stay-current state unavailable"],
        latest_reorg: ["no reorg evidence recorded"],
        reconcile_progress: ["reconcile progress unavailable"],
      },
    },
    {
      surface: "Support evidence",
      file: "packages/open-bitcoin-cli/tests/operator_binary.rs",
      available: {
        connected_height: [`json!(840_004)`],
        connected_hash: [`json!("${available.connected_hash}")`],
        connected_work: [`json!("${available.connected_work}")`],
        validated_active_chain_height: [`json!(840_004)`],
        maybe_validated_active_chain_hash: [`json!("${available.maybe_validated_active_chain_hash}")`],
        maybe_validated_active_chain_work: [`json!("${available.maybe_validated_active_chain_work}")`],
        peer_contribution_connected: [`"connected": ${available.peer_contribution_connected}`],
        peer_contribution_failed: [`"failed": ${available.peer_contribution_failed}`],
      },
      unavailable: {
        connected_active_chain_work: [unavailable.connected_active_chain_work],
        validated_active_chain_work: [unavailable.validated_active_chain_work],
      },
    },
    {
      surface: "Metrics and structured logs",
      file: "packages/open-bitcoin-node/src/sync/types/summary/tests.rs",
      available: {
        validated_active_chain_height: [
          `validated_active_chain_height=${available.validated_active_chain_height}`,
        ],
        maybe_validated_active_chain_work: [
          `validated_active_chain_work=${available.maybe_validated_active_chain_work}`,
        ],
        peer_contribution_connected: [
          `peer_contribution_connected=${available.peer_contribution_connected}`,
        ],
        peer_contribution_failed: [
          `peer_contribution_failed=${available.peer_contribution_failed}`,
        ],
      },
      unavailable: {},
    },
    {
      surface: "Live-smoke summary",
      file: "scripts/test-run-live-mainnet-smoke.sh",
      available: {
        validated_active_chain_height: ["validatedActiveChainHeight", `${available.validated_active_chain_height}`],
        maybe_validated_active_chain_hash: [available.maybe_validated_active_chain_hash],
        maybe_validated_active_chain_work: [available.maybe_validated_active_chain_work],
        best_known_tip_freshness: [`freshness !== "${available.best_known_tip_freshness}"`],
        peer_contribution_connected: [`connected !== ${available.peer_contribution_connected}`],
        peer_contribution_failed: [`failed !== ${available.peer_contribution_failed}`],
      },
      unavailable: {
        validated_active_chain_height: ["validated active-chain height unavailable"],
      },
    },
    {
      surface: "Live-smoke Markdown fallbacks",
      file: "scripts/run-live-mainnet-smoke.ts",
      available: {},
      unavailable: {
        validated_active_chain_height: ["validated active-chain height unavailable"],
        validated_active_chain_hash: [unavailable.validated_active_chain_hash],
        validated_active_chain_work: [unavailable.validated_active_chain_work],
      },
    },
    {
      surface: "Operator docs",
      file: "docs/operator/runtime-guide.md",
      available: {
        recovery_category: [available.recovery_category],
        next_action: [available.next_action],
      },
      unavailable: {
        generic_unavailable: [unavailable.matrix_placeholder],
      },
    },
  ];
}

async function collectSurfaceComparisonValues(
  matrix: readonly SurfaceComparison[],
  failures: string[],
): Promise<Map<string, string>> {
  const values = new Map<string, string>();
  for (const comparison of matrix) {
    const text = await readText(comparison.file, failures);
    for (const [field, needles] of Object.entries(comparison.available)) {
      for (const needle of needles) {
        if (text.includes(needle)) {
          values.set(`${comparison.surface}:${field}:available:${needle}`, needle);
        }
      }
    }
    for (const [field, needles] of Object.entries(comparison.unavailable)) {
      for (const needle of needles) {
        if (text.includes(needle)) {
          values.set(`${comparison.surface}:${field}:unavailable:${needle}`, needle);
        }
      }
    }
  }
  return values;
}

async function requireSurfaceAgreement(
  matrix: readonly SurfaceComparison[],
  failures: string[],
): Promise<void> {
  const collected = await collectSurfaceComparisonValues(matrix, failures);
  for (const comparison of matrix) {
    for (const [field, needles] of Object.entries(comparison.available)) {
      for (const needle of needles) {
        if (!collected.has(`${comparison.surface}:${field}:available:${needle}`)) {
          failures.push(`${comparison.surface} missing Phase 72 available value for ${field}: ${needle}`);
        }
      }
    }
    for (const [field, needles] of Object.entries(comparison.unavailable)) {
      for (const needle of needles) {
        if (!collected.has(`${comparison.surface}:${field}:unavailable:${needle}`)) {
          failures.push(`${comparison.surface} missing Phase 72 unavailable evidence for ${field}: ${needle}`);
        }
      }
    }
  }
}

async function verifyRequirements(failures: string[]): Promise<void> {
  const planText = await readJoined(PLAN_FILES, failures);
  for (const requirementId of REQUIREMENT_IDS) {
    requireContains(planText, requirementId, `${PHASE_DIR}/72-*-PLAN.md`, failures);
  }
}

async function verifyStatusSurfaces(failures: string[]): Promise<void> {
  const statusText = await readJoined(STATUS_SURFACE_FILES, failures);
  for (const needle of [
    "phase72_cli_status_renders_full_sync_truth_contract",
    "phase72_dashboard_projects_full_sync_truth_contract",
    "open_bitcoin_sync_status_returns_phase72_durable_truth_contract",
    "get_blockchain_info_does_not_expose_phase72_support_fields",
    "validated_active_chain_height",
    "maybe_validated_active_chain_hash",
    "maybe_validated_active_chain_work",
    "best_known_tip",
    "stay_current",
    "no_progress_diagnosis",
    "latest_reorg",
    "reconcile_progress",
    "resource_pressure",
  ]) {
    requireContains(statusText, needle, STATUS_SURFACE_FILES.join(", "), failures);
  }
  const rpcTests = await readText("packages/open-bitcoin-rpc/src/dispatch/tests.rs", failures);
  for (const needle of [
    "get_blockchain_info_does_not_expose_phase72_support_fields",
    '"evidence_verdict"',
    "!serialized.contains(forbidden)",
  ]) {
    requireContains(rpcTests, needle, "RPC Phase 72 baseline exclusion guard", failures);
  }
  await requireSurfaceAgreement(phase72SurfaceComparisonMatrix(), failures);
}

async function verifySupportEvidence(failures: string[]): Promise<void> {
  const supportText = await readJoined(SUPPORT_FILES, failures);
  for (const needle of [
    "SupportEvidenceVerdict",
    "full_sync_evidence",
    "derive_full_sync_evidence",
    '["full_sync_evidence"]["verdict"]["label"]',
    "Evidence verdict:",
    "phase72_support_verdict_",
    "open_bitcoin_support_bundle_includes_phase72_full_sync_evidence_and_typed_verdict",
    "phase72_live_smoke_summary_preserves_full_sync_evidence_without_raw_report",
    "sync_to_tip_proven",
    "stay_current_proven",
    "diagnosed_blocker",
    "inconclusive",
  ]) {
    requireContains(supportText, needle, SUPPORT_FILES.join(", "), failures);
  }

  for (const file of [
    "packages/open-bitcoin-cli/src/operator/support.rs",
    "packages/open-bitcoin-cli/src/operator/support/render.rs",
  ]) {
    const productionText = stripRustTests(await readText(file, failures));
    for (const field of RAW_SUPPORT_FIELDS) {
      requireNotContains(productionText, field, file, failures);
    }
  }
}

async function verifyTelemetryAndLiveSmoke(failures: string[]): Promise<void> {
  const telemetryText = await readJoined(TELEMETRY_FILES, failures);
  for (const needle of [
    "ValidatedActiveChainHeight",
    "validated_active_chain_height",
    "phase72_summary_metrics_and_logs_carry_full_sync_truth_dimensions",
    "validated_active_chain_work=",
    "resource_pressure_target_outbound_peers=",
    "peer_contribution_connected=",
    "phase72 live-smoke final status evidence missing",
    "validatedActiveChainHeight",
    "maybeValidatedActiveChainHeightUnavailableReason",
    "maybeValidatedActiveChainHash",
    "maybeValidatedActiveChainWork",
    "bestKnownTip",
    "stayCurrentNextAction",
    "noProgressDiagnosis",
    "latestReorg",
    "reconcileProgress",
    "peerContribution",
    "schema_version: 2",
  ]) {
    requireContains(telemetryText, needle, TELEMETRY_FILES.join(", "), failures);
  }
}

async function verifyDocs(failures: string[]): Promise<void> {
  const docs = await readJoined(DOC_FILES, failures);
  for (const needle of [
    "Phase 72 full-sync evidence and support verdicts",
    "sync_to_tip_proven",
    "stay_current_proven",
    "diagnosed_blocker",
    "inconclusive",
    "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
    "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet support bundle --output-dir=/tmp/open-bitcoin-support",
    "README impact reviewed:",
    "validated_active_chain_height",
    "maybe_validated_active_chain_hash",
    "maybe_validated_active_chain_work",
    "best_known_tip",
    "stay_current",
    "no_progress_diagnosis",
    "no_progress_next_action",
    "latest_reorg",
    "reconcile_progress",
    "resource_pressure",
    "peer_contribution",
    "latest_stop_reason",
    "evidence_verdict",
    "inbound serving",
    "address relay",
    "block serving",
    "transaction relay",
    "compact block relay",
    "production-funds wallet",
    "migration apply mode",
    "signed packaging",
    "Windows service support",
    "GUI",
    "hosted dashboards",
    "broad production-node readiness",
  ]) {
    requireContains(docs, needle, DOC_FILES.join(", "), failures);
  }
}

async function verifyVerifyScript(failures: string[]): Promise<void> {
  const verifyScript = await readText("scripts/verify.sh", failures);
  const phase71 = "bun run scripts/check-phase71-resource-restart.ts";
  const phase72 = "bun run scripts/check-phase72-observability-evidence.ts";
  requireContains(verifyScript, phase71, "scripts/verify.sh", failures);
  requireContains(verifyScript, phase72, "scripts/verify.sh", failures);

  const phase71Index = verifyScript.indexOf(phase71);
  const phase72Index = verifyScript.indexOf(phase72);
  if (phase71Index === -1 || phase72Index === -1 || phase72Index < phase71Index) {
    failures.push("scripts/verify.sh must run the Phase 72 checker after the Phase 71 checker");
  }

  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "--manual-peer", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "systemctl", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "launchctl", "scripts/verify.sh", failures);
  requireNotContains(verifyScript, "openbitcoinsync=mainnet-ibd", "scripts/verify.sh", failures);
}

async function verifyParityBreadcrumbCoverage(failures: string[]): Promise<void> {
  const breadcrumbText = await readText("docs/parity/source-breadcrumbs.json", failures);
  const breadcrumbChecker = await readText("scripts/check-parity-breadcrumbs.ts", failures);
  requireContains(
    breadcrumbText,
    "packages/open-bitcoin-cli/src/operator/sync_truth_render.rs",
    "docs/parity/source-breadcrumbs.json",
    failures,
  );
  requireContains(
    breadcrumbChecker,
    "source-breadcrumbs.json",
    "scripts/check-parity-breadcrumbs.ts",
    failures,
  );
}

function stripRustTests(text: string): string {
  const testIndex = text.indexOf("#[cfg(test)]");
  if (testIndex === -1) {
    return text;
  }
  return text.slice(0, testIndex);
}

async function main(): Promise<void> {
  const failures: string[] = [];
  await verifyRequirements(failures);
  await verifyStatusSurfaces(failures);
  await verifySupportEvidence(failures);
  await verifyTelemetryAndLiveSmoke(failures);
  await verifyDocs(failures);
  await verifyVerifyScript(failures);
  await verifyParityBreadcrumbCoverage(failures);

  if (failures.length > 0) {
    for (const failure of failures) {
      console.error(failure);
    }
    process.exit(1);
  }

  console.log("validated Phase 72 observability/support evidence");
}

await main();

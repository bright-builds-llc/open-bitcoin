#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const LIVE_SMOKE_FIXTURE_PATH = "scripts/test-run-live-mainnet-smoke.sh";
const PHASE_62_FIELDS = [
  "lifecycle",
  "phase",
  "configured_targets",
  "attempt_counters",
  "progress_signal",
  "last_successful_progress_unix_seconds",
  "latest_stop_reason",
  "last_error",
  "recovery_category",
  "recovery_action",
  "resource_pressure",
  "peer health",
  "header_height",
  "downloaded_block_height",
  "maybe_downloaded_block_hash",
  "connected_block_height",
  "maybe_connected_block_hash",
  "messages_processed",
  "headers_received",
  "blocks_received",
] as const;
const RUST_STATUS_FIELDS = [
  "SyncConfiguredTargets",
  "SyncAttemptCounters",
  "SyncStopReasonStatus",
  "configured_targets",
  "attempt_counters",
  "latest_stop_reason",
] as const;
const TYPESCRIPT_REPORT_FIELDS = [
  "configuredTargets",
  "attemptCounters",
  "latestStopReason",
] as const;
const STATUS_RENDER_LABELS = [
  "Sync configured targets",
  "Sync attempts",
  "Sync latest stop reason",
] as const;
const DASHBOARD_LABELS = [
  "Configured targets",
  "Attempt counters",
  "Latest stop reason",
] as const;
const STRUCTURED_LOG_LABELS = [
  "progress_signal=",
  "latest_stop_reason=",
  "recovery_category=",
  "target_outbound_peers=",
  "target_header_height=",
  "messages_processed",
  "headers_received",
  "blocks_received",
] as const;

function readText(relativePath: string): string {
  return readFileSync(path.join(REPO_ROOT, relativePath), "utf8");
}

function requireContains(text: string, needle: string, label: string): void {
  if (!text.includes(needle)) {
    throw new Error(`${label} missing required text: ${needle}`);
  }
}

function requireNotContains(text: string, needle: string, label: string): void {
  if (text.includes(needle)) {
    throw new Error(`${label} must not contain: ${needle}`);
  }
}

function requireAllContains(
  text: string,
  needles: readonly string[],
  label: string,
): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function verifyStatusTypes(statusRs: string, summaryRs: string): void {
  requireAllContains(statusRs, RUST_STATUS_FIELDS, "packages/open-bitcoin-node/src/status.rs");
  requireAllContains(
    summaryRs,
    [
      "SyncConfiguredTargets",
      "SyncAttemptCounters",
      "SyncStopReasonStatus",
      "configured_targets",
      "attempt_counters",
      "latest_stop_reason",
      "target_outbound_peers",
      "maybe_target_header_height",
      "progress_signal_name",
    ],
    "packages/open-bitcoin-node/src/sync/types/summary.rs",
  );
}

function verifyRustAndCliSurfaces(
  statusRender: string,
  dashboardModel: string,
  runtimeSupport: string,
  rpcNode: string,
): void {
  requireAllContains(
    statusRender,
    [...RUST_STATUS_FIELDS, ...STATUS_RENDER_LABELS],
    "packages/open-bitcoin-cli/src/operator/status/render.rs",
  );
  requireAllContains(
    dashboardModel,
    [...RUST_STATUS_FIELDS, ...DASHBOARD_LABELS],
    "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  );
  requireAllContains(
    runtimeSupport,
    [
      "Configured targets",
      "Attempt counters",
      "Progress signal",
      "Latest stop reason",
      "Resource pressure",
      "Peer health",
      "Bounded counters",
      "messages_processed",
      "headers_received",
      "blocks_received",
    ],
    "packages/open-bitcoin-cli/src/operator/runtime/support.rs",
  );
  requireAllContains(
    rpcNode,
    ["progress_signal=", "latest_stop_reason=", "recovery_category="],
    "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  );
}

function verifyStructuredEvidence(summaryRs: string, liveSmoke: string): void {
  requireAllContains(
    summaryRs,
    STRUCTURED_LOG_LABELS,
    "packages/open-bitcoin-node/src/sync/types/summary.rs",
  );
  requireAllContains(
    liveSmoke,
    [
      ...TYPESCRIPT_REPORT_FIELDS,
      "ConfiguredTargetsSummary",
      "AttemptCountersSummary",
      "StopReasonSummary",
      "configuredTargetsFromValue",
      "attemptCountersFromValue",
      "stopReasonFromValue",
      "Signal | Configured Targets | Attempts",
      "Latest Stop Reason",
      "Final Durable Status",
      "Daemon Output Summary",
    ],
    "scripts/run-live-mainnet-smoke.ts",
  );
}

function verifyLiveSmokeFixture(liveSmokeFixture: string): void {
  requireAllContains(
    liveSmokeFixture,
    [
      "configuredTargets",
      "attemptCounters",
      "latestStopReason",
      "Signal | Configured Targets | Attempts",
      "Latest Stop Reason",
      "stdoutTail",
      "stderrTail",
    ],
    LIVE_SMOKE_FIXTURE_PATH,
  );
}

function verifyDocs(
  runtimeGuide: string,
  statusSnapshot: string,
  operatorObservability: string,
): void {
  const docs = [runtimeGuide, statusSnapshot, operatorObservability].join("\n");
  requireAllContains(docs, PHASE_62_FIELDS, "Phase 62 docs");
  requireAllContains(
    runtimeGuide,
    [
      "Phase 62 sync truth fields",
      "open-bitcoin status",
      "open-bitcoin dashboard",
      "open-bitcoin sync status",
      "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
      "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
      "public-network live-smoke evidence remains opt-in UAT",
    ],
    "docs/operator/runtime-guide.md",
  );
  requireContains(
    statusSnapshot,
    "Unavailable: {reason}",
    "docs/architecture/status-snapshot.md",
  );
  requireAllContains(
    operatorObservability,
    [
      "metrics remain bounded numeric samples",
      "does not persist raw daemon tails",
      "latest_stop_reason",
    ],
    "docs/architecture/operator-observability.md",
  );
}

function verifyVerifyScript(verifyScript: string): void {
  requireContains(
    verifyScript,
    "bun run scripts/check-phase62-sync-truth-surfaces.ts",
    "scripts/verify.sh",
  );
  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
  requireNotContains(verifyScript, "--manual-peer", "scripts/verify.sh");
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh");
}

function main(): void {
  const statusRs = readText("packages/open-bitcoin-node/src/status.rs");
  const summaryRs = readText("packages/open-bitcoin-node/src/sync/types/summary.rs");
  const statusRender = readText("packages/open-bitcoin-cli/src/operator/status/render.rs");
  const dashboardModel = readText("packages/open-bitcoin-cli/src/operator/dashboard/model.rs");
  const runtimeSupport = readText("packages/open-bitcoin-cli/src/operator/runtime/support.rs");
  const rpcNode = readText("packages/open-bitcoin-rpc/src/dispatch/node.rs");
  const liveSmoke = readText("scripts/run-live-mainnet-smoke.ts");
  const liveSmokeFixture = readText(LIVE_SMOKE_FIXTURE_PATH);
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const statusSnapshot = readText("docs/architecture/status-snapshot.md");
  const operatorObservability = readText("docs/architecture/operator-observability.md");
  const verifyScript = readText("scripts/verify.sh");

  verifyStatusTypes(statusRs, summaryRs);
  verifyRustAndCliSurfaces(statusRender, dashboardModel, runtimeSupport, rpcNode);
  verifyStructuredEvidence(summaryRs, liveSmoke);
  verifyLiveSmokeFixture(liveSmokeFixture);
  verifyDocs(runtimeGuide, statusSnapshot, operatorObservability);
  verifyVerifyScript(verifyScript);

  console.log("validated Phase 62 sync truth surfaces");
}

main();

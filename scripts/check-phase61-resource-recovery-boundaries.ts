#!/usr/bin/env bun

import { readFileSync } from "node:fs";
import path from "node:path";

const REPO_ROOT = path.resolve(import.meta.dir, "..");
const RECOVERY_CATEGORY_LABELS = [
  "clean_shutdown",
  "unclean_shutdown",
  "incompatible_schema",
  "store_corruption",
  "storage_lock_contention",
  "storage_backend_failure",
  "resource_exhaustion",
  "invalid_peer_data",
  "public_network_unreachable",
  "operator_cancellation",
] as const;
const RESOURCE_PRESSURE_FIELDS = [
  "blocks_in_flight",
  "max_header_requests_in_flight_per_peer",
  "max_headers_per_message",
  "max_blocks_in_flight_per_peer",
  "max_blocks_in_flight_total",
  "max_messages_per_peer",
  "max_sync_rounds",
  "outbound_peers",
  "target_outbound_peers",
] as const;
const RR_01_BOUND_STATEMENTS = [
  "peer retry state is keyed by resolved endpoint and bounded by candidate peers/outbound target per cycle",
  "durable storage writes are synchronous adapter calls with no queued write backlog",
] as const;
const STATUS_COMMANDS = [
  "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
  "bazel run //packages/open-bitcoin-cli:open_bitcoin -- --datadir=/tmp/open-bitcoin-mainnet sync status --format json",
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

function requireAllContains(text: string, needles: readonly string[], label: string): void {
  for (const needle of needles) {
    requireContains(text, needle, label);
  }
}

function verifyRuntimeGuide(runtimeGuide: string): void {
  requireContains(runtimeGuide, "sync.recovery_category", "docs/operator/runtime-guide.md");
  requireAllContains(
    runtimeGuide,
    RECOVERY_CATEGORY_LABELS,
    "docs/operator/runtime-guide.md",
  );
  requireAllContains(
    runtimeGuide,
    RESOURCE_PRESSURE_FIELDS,
    "docs/operator/runtime-guide.md",
  );
  requireAllContains(
    runtimeGuide,
    RR_01_BOUND_STATEMENTS,
    "docs/operator/runtime-guide.md",
  );
  requireAllContains(runtimeGuide, STATUS_COMMANDS, "docs/operator/runtime-guide.md");
  requireContains(
    runtimeGuide,
    "support bundle --output-dir=/tmp/open-bitcoin-support",
    "docs/operator/runtime-guide.md",
  );
  requireContains(runtimeGuide, "bash scripts/verify.sh", "docs/operator/runtime-guide.md");
  requireContains(
    runtimeGuide,
    "bash scripts/test-run-live-mainnet-smoke.sh",
    "docs/operator/runtime-guide.md",
  );
}

function verifyStatusSnapshot(statusSnapshot: string): void {
  requireContains(
    statusSnapshot,
    "sync.recovery_category",
    "docs/architecture/status-snapshot.md",
  );
  requireAllContains(
    statusSnapshot,
    RECOVERY_CATEGORY_LABELS,
    "docs/architecture/status-snapshot.md",
  );
  requireAllContains(
    statusSnapshot,
    RESOURCE_PRESSURE_FIELDS,
    "docs/architecture/status-snapshot.md",
  );
}

function verifyOperatorObservability(operatorObservability: string): void {
  requireContains(
    operatorObservability,
    "recovery_category",
    "docs/architecture/operator-observability.md",
  );
  requireContains(
    operatorObservability,
    "bounded numeric samples",
    "docs/architecture/operator-observability.md",
  );
  requireAllContains(
    operatorObservability,
    RR_01_BOUND_STATEMENTS,
    "docs/architecture/operator-observability.md",
  );
}

function verifyVerifyScript(verifyScript: string): void {
  requireContains(
    verifyScript,
    "bun run scripts/check-phase61-resource-recovery-boundaries.ts",
    "scripts/verify.sh",
  );
  requireNotContains(verifyScript, "run-live-mainnet-smoke", "scripts/verify.sh");
  requireNotContains(verifyScript, "--manual-peer", "scripts/verify.sh");
  requireNotContains(verifyScript, "--restart-after-progress", "scripts/verify.sh");
}

function main(): void {
  const runtimeGuide = readText("docs/operator/runtime-guide.md");
  const statusSnapshot = readText("docs/architecture/status-snapshot.md");
  const operatorObservability = readText("docs/architecture/operator-observability.md");
  const verifyScript = readText("scripts/verify.sh");

  verifyRuntimeGuide(runtimeGuide);
  verifyStatusSnapshot(statusSnapshot);
  verifyOperatorObservability(operatorObservability);
  verifyVerifyScript(verifyScript);

  console.log("validated Phase 61 resource/recovery boundaries");
}

main();

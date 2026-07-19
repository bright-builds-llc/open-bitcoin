#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE116_TEST_COMMAND =
  "bun test scripts/check-phase116-operator-block-relay-evidence.test.ts";
const PHASE116_CHECKER_COMMAND =
  "bun run scripts/check-phase116-operator-block-relay-evidence.ts";
const PHASE121_TEST_COMMAND =
  "bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts";
const PHASE121_CHECKER_COMMAND =
  "bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts";
const CLOSED_FLOW =
  "DurableSyncRuntime::network -> one availability-gated BlockRelayRuntimeEvidenceSnapshot -> block_relay_metric_samples / block_relay_log_record -> retained metrics / structured logs";

const TARGET_FILES = [
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/metrics.rs",
  "packages/open-bitcoin-node/src/sync/runtime_state.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  "packages/open-bitcoin-node/src/metrics/block_relay.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "docs/architecture/operator-observability.md",
  "scripts/check-phase121-block-relay-metrics-log-runtime.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckPhase121Options = { rootDir?: string };

export function checkPhase121BlockRelayMetricsLogRuntime(
  options: CheckPhase121Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyAuthoritativeSnapshotAndPersist(texts, failures);
  verifyRuntimeTests(texts, failures);
  verifyHelperReuse(texts, failures);
  verifyObsoleteProviderAbsent(texts, failures);
  verifyDocs(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimCreep(texts, failures);
  verifyNoTwinWorker(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P121 missing required corpus file: ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyAuthoritativeSnapshotAndPersist(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  const metrics = texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? "";
  const runtime = texts.get("packages/open-bitcoin-node/src/sync/runtime_state.rs") ?? "";

  requireExactCount(
    sync,
    "self.network.block_relay_runtime_evidence_snapshot()?",
    1,
    "P121 authoritative typed snapshot",
    failures,
  );
  for (const needle of [
    "match snapshot.status.block_serving.activation",
    "FieldAvailability::Available(_) => Ok(Some(snapshot))",
    "FieldAvailability::Unavailable { .. } => Ok(None)",
  ]) {
    requireContains(sync, needle, "P121 activation omission", failures);
  }
  requireContains(
    sync,
    "let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?;",
    "P121 authoritative snapshot local",
    failures,
  );
  requireContains(
    sync,
    "self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp)",
    "P121 same snapshot metrics argument",
    failures,
  );
  requireContains(
    sync,
    "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
    "P121 same snapshot log argument",
    failures,
  );
  requireExactCount(
    sync,
    "maybe_block_relay_snapshot.as_ref()",
    2,
    "P121 same snapshot reuse",
    failures,
  );
  requireOrdered(
    sync,
    [
      "let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?;",
      "self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp)",
      "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
    ],
    "P121 authoritative projection order",
    failures,
  );

  for (const needle of [
    "if let Some(snapshot) = maybe_block_relay_snapshot",
    "samples.extend(block_relay_metric_samples(",
    "snapshot.served_count",
  ]) {
    requireContains(metrics, needle, "P121 runtime metrics projection", failures);
  }
  requireContains(metrics, "append_metric_samples", "P121 retained metrics append", failures);

  for (const needle of [
    "let Some(snapshot) = maybe_block_relay_snapshot else",
    "block_relay_log_record(&snapshot.status, snapshot.served_count",
  ]) {
    requireContains(runtime, needle, "P121 structured log projection", failures);
  }
  requireContains(
    runtime,
    "append_structured_record",
    "P121 structured log append",
    failures,
  );
}

function verifyRuntimeTests(texts: Map<TargetFile, string>, failures: string[]): void {
  const legacyTests = texts.get("packages/open-bitcoin-node/src/sync/tests.rs") ?? "";
  for (const needle of [
    "persist_metrics_appends_block_relay_status_samples_with_sync_samples",
    "persist_metrics_omits_block_relay_samples_without_snapshot",
    "write_block_relay_log_emits_when_status_available",
    "write_block_relay_log_omits_when_status_unavailable",
    "write_block_relay_log_omits_sensitive_markers",
  ]) {
    requireContains(legacyTests, needle, "P121 runtime tests", failures);
  }

  const projectionTests =
    texts.get("packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs") ?? "";
  for (const needle of [
    "phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log",
    "phase123_sync_network_compact_activity_projects_same_snapshot_to_metrics_and_log",
    "eligibility.eligible_peer_count, 2",
    "block_served_write_count()",
    '.expect("authoritative block write count")',
  ]) {
    requireContains(projectionTests, needle, "P121 authoritative projection tests", failures);
  }
}

function verifyHelperReuse(texts: Map<TargetFile, string>, failures: string[]): void {
  const metricHelper = texts.get("packages/open-bitcoin-node/src/metrics/block_relay.rs") ?? "";
  const logHelper = texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "";
  requireContains(
    metricHelper,
    "pub fn block_relay_metric_samples",
    "P121 metric helper reuse",
    failures,
  );
  requireContains(
    logHelper,
    "pub fn block_relay_log_record",
    "P121 log helper reuse",
    failures,
  );
}

function verifyObsoleteProviderAbsent(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const syncRuntime = [
    texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? "",
    texts.get("packages/open-bitcoin-node/src/sync/runtime_state.rs") ?? "",
  ].join("\n");
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  for (const token of [
    "set_block_relay_metric_status_provider",
    "maybe_block_relay_metric_status_provider",
  ]) {
    requireAbsent(syncRuntime, token, "P121 obsolete provider wiring", failures);
    requireAbsent(daemon, token, "P121 obsolete daemon provider wiring", failures);
  }
  requireAbsent(daemon, "block_relay_context", "P121 obsolete daemon provider wiring", failures);
}

function verifyDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const docs = normalizeWhitespace(
    texts.get("docs/architecture/operator-observability.md") ?? "",
  );
  for (const needle of [
    "Phase 121",
    CLOSED_FLOW,
    "runtime-only",
    "non-serialized",
    "ManagedRpcContext",
    "separate network",
    "not the sync projection source",
    "aggregate-only",
  ]) {
    requireContains(docs, needle, "P121 operator-observability docs", failures);
  }
}

function normalizeWhitespace(text: string): string {
  return text.replaceAll(/\s+/g, " ").trim();
}

function verifyVerifierWiring(verifyScript: string, failures: string[]): void {
  for (const needle of [
    PHASE121_TEST_COMMAND,
    PHASE121_CHECKER_COMMAND,
    'run_step "test Phase 121 block-relay metrics and log runtime checker"',
    'run_step "check Phase 121 block-relay metrics and log runtime"',
  ]) {
    requireContains(verifyScript, needle, "P121 verifier wiring", failures);
  }
  requireOrdered(
    verifyScript,
    [
      PHASE116_TEST_COMMAND,
      PHASE116_CHECKER_COMMAND,
      PHASE121_TEST_COMMAND,
      PHASE121_CHECKER_COMMAND,
    ],
    "P121 verifier command order",
    failures,
  );
}

function verifyNoClaimCreep(texts: Map<TargetFile, string>, failures: string[]): void {
  const docs = texts.get("docs/architecture/operator-observability.md") ?? "";
  for (const phrase of [
    "enables package relay",
    "enables public inbound default",
    "proves production full-node readiness",
    "enables compact block relay",
    "adds a new RPC",
    "adds a new CLI",
    "adds a new dashboard",
    "adds a new support field",
    "unified mutable network",
  ]) {
    requireAbsent(docs, phrase, "P121 no-claim boundary", failures);
  }
}

function verifyNoTwinWorker(texts: Map<TargetFile, string>, failures: string[]): void {
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  for (const phrase of ["start_block_relay_metrics_worker", "persist_block_relay_metrics_once"]) {
    requireAbsent(daemon, phrase, "P121 no twin metrics worker", failures);
  }
}

function requireContains(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (!text.includes(needle)) {
    failures.push(`${label} missing ${needle}`);
  }
}

function requireAbsent(
  text: string,
  needle: string,
  label: string,
  failures: string[],
): void {
  if (text.includes(needle)) {
    failures.push(`${label} must not contain ${needle}`);
  }
}

function requireExactCount(
  text: string,
  needle: string,
  expected: number,
  label: string,
  failures: string[],
): void {
  const actual = text.split(needle).length - 1;
  if (actual !== expected) {
    failures.push(`${label} expected ${expected} occurrence(s) of ${needle}, found ${actual}`);
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

if (import.meta.main) {
  const failures = checkPhase121BlockRelayMetricsLogRuntime();
  if (failures.length > 0) {
    console.error("Phase 121 block-relay metrics and log runtime checker failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log("Phase 121 block-relay metrics and log runtime checker passed.");
}

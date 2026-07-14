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
  "BlockRelayEvidenceStatus -> block_relay_metric_samples / block_relay_log_record -> DurableSyncRuntime persist_metrics / structured logs";

const TARGET_FILES = [
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/metrics.rs",
  "packages/open-bitcoin-node/src/sync/runtime_state.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
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

  verifyProviderAndPersist(texts, failures);
  verifyLogEmission(texts, failures);
  verifyRuntimeTests(texts, failures);
  verifyHelperReuse(texts, failures);
  verifyDaemonWiring(texts, failures);
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

function verifyProviderAndPersist(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = `${texts.get("packages/open-bitcoin-node/src/sync.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? ""
  }`;
  const metrics = texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? "";
  for (const needle of [
    "set_block_relay_metric_status_provider",
    "FieldAvailability<BlockRelayEvidenceStatus>",
  ]) {
    requireContains(sync, needle, "P121 runtime provider hook", failures);
  }
  for (const needle of [
    "block_relay_metric_samples",
    "samples.extend",
    "append_metric_samples",
    "FieldAvailability::Available(status)",
  ]) {
    requireContains(metrics, needle, "P121 runtime metrics append", failures);
  }
}

function verifyLogEmission(texts: Map<TargetFile, string>, failures: string[]): void {
  const runtime = texts.get("packages/open-bitcoin-node/src/sync/runtime_state.rs") ?? "";
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  for (const needle of [
    "write_block_relay_log",
    "block_relay_log_record",
    "append_structured_record",
  ]) {
    requireContains(runtime, needle, "P121 structured log emission", failures);
  }
  requireContains(sync, "write_block_relay_log", "P121 sync tick log wiring", failures);
}

function verifyRuntimeTests(texts: Map<TargetFile, string>, failures: string[]): void {
  const tests = texts.get("packages/open-bitcoin-node/src/sync/tests.rs") ?? "";
  for (const needle of [
    "persist_metrics_appends_block_relay_status_samples_with_sync_samples",
    "persist_metrics_omits_block_relay_samples_when_status_unavailable",
    "write_block_relay_log_emits_when_status_available",
    "write_block_relay_log_omits_when_status_unavailable",
    "write_block_relay_log_omits_sensitive_markers",
  ]) {
    requireContains(tests, needle, "P121 runtime tests", failures);
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

function verifyDaemonWiring(texts: Map<TargetFile, string>, failures: string[]): void {
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  for (const needle of [
    "set_block_relay_metric_status_provider",
    "block_relay_evidence_status",
    "BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON",
  ]) {
    requireContains(daemon, needle, "P121 open-bitcoind provider", failures);
  }
}

function verifyDocs(texts: Map<TargetFile, string>, failures: string[]): void {
  const docs = texts.get("docs/architecture/operator-observability.md") ?? "";
  for (const needle of [
    "Phase 121",
    "DurableSyncRuntime",
    "persist_metrics",
    "block_relay_metric_samples",
    CLOSED_FLOW,
  ]) {
    requireContains(docs, needle, "P121 operator-observability docs", failures);
  }
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

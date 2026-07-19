#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const PHASE96_TEST_COMMAND =
  "bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts";
const PHASE96_CHECKER_COMMAND =
  "bun run scripts/check-phase96-peer-policy-runtime-bridge.ts";
const PHASE97_TEST_COMMAND =
  "bun test scripts/check-phase97-inbound-metrics.test.ts";
const PHASE97_CHECKER_COMMAND =
  "bun run scripts/check-phase97-inbound-metrics.ts";
const CLOSED_FLOW =
  "InboundPeerServingStatus aggregate counters -> fixed MetricSample values -> FjallNodeStore::append_metric_samples -> dashboard/status/support retained history";

const INBOUND_METRIC_VARIANTS = [
  "InboundAdmittedPeerCount",
  "InboundRejectedPeerCount",
  "InboundCapRejectCount",
  "InboundReservedSlotRejectCount",
  "InboundDuplicateRejectCount",
  "InboundSelfConnectionRejectCount",
  "InboundPermissionedAdmitCount",
  "InboundProtectedAdmitCount",
  "InboundInactivePermissionEffectCount",
  "InboundPermissionValidationFailureCount",
  "InboundEvictionCandidateCount",
  "InboundDisconnectCount",
  "InboundActiveBanCount",
  "InboundMisbehaviorObservationCount",
  "InboundProtectedNoActionCount",
  "InboundResourcePressureActiveCount",
  "InboundReadQueuePressureCount",
  "InboundWriteQueuePressureCount",
  "InboundRequestCapReachedCount",
  "InboundPayloadRejectedCount",
  "InboundTimeoutDisconnectCount",
  "InboundChurnRejectedCount",
  "InboundReconnectSuppressedCount",
] as const;

const TARGET_FILES = [
  "packages/open-bitcoin-node/src/metrics.rs",
  "packages/open-bitcoin-node/src/metrics/tests.rs",
  "packages/open-bitcoin-node/src/status/inbound.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/metrics.rs",
  "packages/open-bitcoin-node/src/sync/runtime_state.rs",
  "packages/open-bitcoin-node/src/sync/tests.rs",
  "packages/open-bitcoin-rpc/src/config.rs",
  "packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs",
  "packages/open-bitcoin-rpc/src/context/inbound_status.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs",
  "packages/open-bitcoin-rpc/src/dispatch/node.rs",
  "packages/open-bitcoin-rpc/src/method/node.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs",
  "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
  "packages/open-bitcoin-cli/src/operator/status.rs",
  "packages/open-bitcoin-cli/src/operator/status/tests.rs",
  "packages/open-bitcoin-cli/src/operator/support/tests.rs",
  "packages/open-bitcoin-node/src/status/tests.rs",
  "docs/architecture/operator-observability.md",
  "docs/operator/runtime-guide.md",
  "scripts/check-phase97-inbound-metrics.ts",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckPhase97Options = { rootDir?: string };

export function checkPhase97InboundMetrics(
  options: CheckPhase97Options = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();

  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyInboundMetricVariants(texts, failures);
  verifyInboundStatusAndConfig(texts, failures);
  verifyMapper(texts, failures);
  verifyRuntimeAppendAndProvider(texts, failures);
  verifyRpcAndCliStatusSurface(texts, failures);
  verifyDashboard(texts, failures);
  verifyStatusSupportAndDocs(texts, failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  verifyNoClaimCreep(texts, failures);

  return failures;
}

function readText(repoRoot: string, relativePath: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(`P97 missing required corpus file: ${relativePath}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyInboundMetricVariants(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const metrics = texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "";
  const dashboard = `${texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs") ?? ""
  }`;
  const checker = texts.get("scripts/check-phase97-inbound-metrics.ts") ?? "";
  for (const variant of INBOUND_METRIC_VARIANTS) {
    requireContains(metrics, `MetricKind::${variant}`, "P97 metrics mapper", failures);
    requireContains(dashboard, `MetricKind::${variant}`, "P97 dashboard candidates", failures);
    requireContains(checker, variant, "P97 checker constants", failures);
  }
}

function verifyInboundStatusAndConfig(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const inbound = texts.get("packages/open-bitcoin-node/src/status/inbound.rs") ?? "";
  const config = texts.get("packages/open-bitcoin-rpc/src/config.rs") ?? "";
  const loader =
    texts.get("packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs") ?? "";
  const context = `${texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-rpc/src/context/inbound_status.rs") ?? ""
  }`;
  for (const needle of [
    "inactive_permission_effect_observations: u32",
    "permission_validation_failures: u32",
    "#[serde(default)]",
  ]) {
    requireContains(inbound, needle, "P97 inbound status aggregates", failures);
  }
  for (const needle of [
    "inbound_permission_validation_failures: u32",
    "count_inbound_permission_validation_failures",
    "ParsedPeerPermissionClass::parse",
    "duplicate_literal_ip_address",
    "inbound_permission_validation_failure_count_is_config_validation_aggregate",
  ]) {
    requireContains(`${config}\n${loader}`, needle, "P97 config validation aggregate", failures);
  }
  for (const needle of [
    "admission.inactive_permission_effect_observations",
    "self.inbound_permission_validation_failures",
    "permission_validation_failures:",
  ]) {
    requireContains(context, needle, "P97 status projection aggregate", failures);
  }
}

function verifyMapper(texts: Map<TargetFile, string>, failures: string[]): void {
  const metrics = texts.get("packages/open-bitcoin-node/src/metrics.rs") ?? "";
  const tests = texts.get("packages/open-bitcoin-node/src/metrics/tests.rs") ?? "";
  for (const needle of [
    "pub fn inbound_metric_samples",
    "FieldAvailability<InboundPeerServingStatus>",
    "let FieldAvailability::Available(status) = inbound else",
    "return Vec::new();",
    "MetricSample::new",
    "status.inactive_permission_effect_observations",
    "status.permission_validation_failures",
  ]) {
    requireContains(metrics, needle, "P97 inbound metric mapper", failures);
  }
  requireAbsent(
    metrics,
    "inactive_permission_effects.len()",
    "P97 inbound metric mapper",
    failures,
  );
  for (const needle of [
    "unavailable_inbound_status_emits_no_metric_samples",
    "inbound_status_maps_to_each_fixed_inbound_metric_kind",
    "inactive_permission_metric_uses_observation_count_not_label_count",
  ]) {
    requireContains(tests, needle, "P97 mapper tests", failures);
  }
}

function verifyRuntimeAppendAndProvider(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = `${texts.get("packages/open-bitcoin-node/src/sync.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? ""
  }`;
  const runtime = `${texts.get("packages/open-bitcoin-node/src/sync/runtime_state.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-node/src/sync/metrics.rs") ?? ""
  }`;
  const runtimeTests = texts.get("packages/open-bitcoin-node/src/sync/tests.rs") ?? "";
  const daemon = `${texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs") ?? ""
  }`;
  const daemonTests = texts.get("packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs") ?? "";
  for (const needle of [
    "set_inbound_metric_status_provider",
    "FieldAvailability<InboundPeerServingStatus>",
  ]) {
    requireContains(sync, needle, "P97 runtime provider hook", failures);
  }
  for (const needle of [
    "let mut samples = summary.metric_samples(timestamp);",
    "samples.extend(inbound_metric_samples(&provider(), timestamp));",
    "append_metric_samples(\n            &samples,",
    "MetricRetentionPolicy::default()",
  ]) {
    requireContains(runtime, needle, "P97 runtime metrics append", failures);
  }
  for (const needle of [
    "persist_metrics_appends_inbound_status_samples_with_sync_samples",
    "persist_metrics_omits_inbound_samples_when_status_unavailable",
  ]) {
    requireContains(runtimeTests, needle, "P97 runtime metrics tests", failures);
  }
  for (const needle of [
    "set_inbound_metric_status_provider",
    "try_lock()",
    "current_inbound_status()",
    "ManagedRpcContext::from_runtime_config_with_network_handle",
    "fn start_inbound_metrics_worker",
    "fn persist_inbound_metrics_once",
    "inbound_metric_samples(&inbound, timestamp)",
    "append_metric_samples(&samples, retention, timestamp, persist_mode)",
  ]) {
    requireContains(daemon, needle, "P97 open-bitcoind inbound provider", failures);
  }
  for (const needle of [
    "open_bitcoind_inbound_metrics_worker_persists_sync_disabled_inbound_samples",
    "ManagedRpcContext::from_runtime_config_with_store",
    "wait_for_inbound_metric_sample",
    "MetricKind::InboundAdmittedPeerCount",
  ]) {
    requireContains(daemonTests, needle, "P97 open-bitcoind inbound metrics worker test", failures);
  }
}

function verifyRpcAndCliStatusSurface(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const context = `${texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-rpc/src/context/inbound_status.rs") ?? ""
  }`;
  const method = texts.get("packages/open-bitcoin-rpc/src/method/node.rs") ?? "";
  const dispatch = texts.get("packages/open-bitcoin-rpc/src/dispatch/node.rs") ?? "";
  const cliStatus = texts.get("packages/open-bitcoin-cli/src/operator/status.rs") ?? "";
  const cliStatusTests = texts.get("packages/open-bitcoin-cli/src/operator/status/tests.rs") ?? "";
  for (const needle of [
    "pub fn from_runtime_config_with_store",
    "maybe_metrics_store: maybe_store.clone()",
    "pub fn set_metrics_store",
    "pub fn metrics_status",
    "load_metrics_status(MetricRetentionPolicy::default())",
  ]) {
    requireContains(context, needle, "P97 RPC context retained metrics status", failures);
  }
  for (const needle of ["pub metrics: MetricsStatus", "#[serde(default)]"]) {
    requireContains(method, needle, "P97 RPC network status metrics response", failures);
  }
  requireContains(
    dispatch,
    "metrics: context.metrics_status()",
    "P97 RPC network status metrics dispatch",
    failures,
  );
  for (const needle of [
    "let network_status = collect_open_bitcoin_network_status(rpc_client);",
    "let metrics = network_status.metrics;",
    "metrics,",
  ]) {
    requireContains(cliStatus, needle, "P97 CLI live status metrics projection", failures);
  }
  for (const needle of [
    "fake_live_rpc_maps_metrics_from_open_bitcoin_network_status",
    "MetricKind::InboundAdmittedPeerCount",
    "snapshot.metrics.samples",
  ]) {
    requireContains(cliStatusTests, needle, "P97 CLI live status metrics test", failures);
  }
}

function verifyDashboard(texts: Map<TargetFile, string>, failures: string[]): void {
  const dashboard = `${texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model.rs") ?? ""}\n${
    texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs") ?? ""
  }`;
  const tests = texts.get("packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs") ?? "";
  for (const needle of [
    "pub const MAX_DASHBOARD_CHARTS: usize = 8",
    "pub const DASHBOARD_METRIC_KINDS: [MetricKind; 8]",
    "INBOUND_DASHBOARD_METRIC_CANDIDATES: [MetricKind; 23]",
    "fn dashboard_metric_kinds",
    "retained_inbound_metric_kinds",
    "dashboard_metric_kinds(snapshot)",
  ]) {
    requireContains(dashboard, needle, "P97 dashboard bounded selector", failures);
  }
  requireAbsent(dashboard, "[MetricKind; 31]", "P97 dashboard bounded selector", failures);
  requireContains(
    tests,
    "dashboard_charts_render_retained_inbound_metric_samples_without_expanding_row",
    "P97 dashboard tests",
    failures,
  );
}

function verifyStatusSupportAndDocs(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const statusTests = texts.get("packages/open-bitcoin-node/src/status/tests.rs") ?? "";
  const supportTests = texts.get("packages/open-bitcoin-cli/src/operator/support/tests.rs") ?? "";
  const architecture = texts.get("docs/architecture/operator-observability.md") ?? "";
  const runtimeGuide = texts.get("docs/operator/runtime-guide.md") ?? "";
  requireContains(
    statusTests,
    "status_metrics_json_preserves_retained_inbound_samples_without_dynamic_labels",
    "P97 status sample tests",
    failures,
  );
  requireContains(
    supportTests,
    "support_bundle_preserves_retained_inbound_metric_samples",
    "P97 support sample tests",
    failures,
  );
  for (const text of [architecture, runtimeGuide]) {
    requireContains(text, CLOSED_FLOW, "P97 docs closed metrics flow", failures);
  }
  for (const needle of [
    "status --format json",
    "dashboard --tick-ms 1000",
    "support bundle --output-dir=/tmp/open-bitcoin-inbound-support",
    "Retained local inbound metric evidence does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, packaged service operation, or production full-node readiness.",
  ]) {
    requireContains(runtimeGuide, needle, "P97 runtime guide commands and boundary", failures);
  }
}

function verifyVerifierWiring(verifyScript: string, failures: string[]): void {
  for (const needle of [
    PHASE97_TEST_COMMAND,
    PHASE97_CHECKER_COMMAND,
    'run_step "test Phase 97 inbound metrics checker"',
    'run_step "check Phase 97 inbound metrics"',
  ]) {
    requireContains(verifyScript, needle, "P97 verifier wiring", failures);
  }
  requireOrdered(
    verifyScript,
    [PHASE96_TEST_COMMAND, PHASE96_CHECKER_COMMAND, PHASE97_TEST_COMMAND, PHASE97_CHECKER_COMMAND],
    "P97 verifier command order",
    failures,
  );
}

function verifyNoClaimCreep(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const docs = `${texts.get("docs/architecture/operator-observability.md") ?? ""}\n${
    texts.get("docs/operator/runtime-guide.md") ?? ""
  }`;
  for (const phrase of [
    "enables transaction relay",
    "enables compact block relay",
    "enables mempool propagation",
    "enables public inbound default",
    "enables production service operation",
    "proves production full-node readiness",
  ]) {
    requireAbsent(docs, phrase, "P97 no-claim boundary", failures);
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
  const failures = checkPhase97InboundMetrics();
  if (failures.length > 0) {
    console.error("Phase 97 inbound metrics checker failed:");
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }
  console.log("Phase 97 inbound metrics checker passed.");
}

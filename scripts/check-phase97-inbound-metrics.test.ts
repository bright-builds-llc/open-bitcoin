import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase97InboundMetrics } from "./check-phase97-inbound-metrics";

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
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const CLOSED_FLOW =
  "InboundPeerServingStatus aggregate counters -> fixed MetricSample values -> FjallNodeStore::append_metric_samples -> dashboard/status/support retained history";
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with complete Phase 97 inbound metrics corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when mapper omits an inbound metric kind", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/metrics.rs",
        "MetricKind::InboundReconnectSuppressedCount",
        "MetricKind::SyncHeight",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("InboundReconnectSuppressedCount");
});

test("fails when inactive permission metric uses label count", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/metrics.rs",
        "status.inactive_permission_effect_observations",
        "status.inactive_permission_effects.len()",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("inactive_permission_effects.len()");
});

test("fails missing runtime append extension", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/metrics.rs",
        "samples.extend(inbound_metric_samples(&provider(), timestamp));",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("samples.extend");
});

test("fails dashboard row expansion", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs",
        "pub const DASHBOARD_METRIC_KINDS: [MetricKind; 8]",
        "pub const DASHBOARD_METRIC_KINDS: [MetricKind; 31]",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("[MetricKind; 31]");
});

test("fails claim creep and missing closed-flow docs", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "docs/operator/runtime-guide.md",
        "Phase 97 enables transaction relay and proves production full-node readiness.",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain(CLOSED_FLOW);
  expect(message).toContain("enables transaction relay");
});

test("fails missing verifier order", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "scripts/verify.sh",
        "bun test scripts/check-phase97-inbound-metrics.test.ts",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase97InboundMetrics({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("out of order");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase97-"));
  tempRoots.push(root);
  const files = completeFiles();
  options.maybeMutateFiles?.(files);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function completeFiles(): Map<TargetFile, string> {
  return new Map<TargetFile, string>([
    [
      "packages/open-bitcoin-node/src/metrics.rs",
      `
pub fn inbound_metric_samples(inbound: &FieldAvailability<InboundPeerServingStatus>, timestamp_unix_seconds: u64) -> Vec<MetricSample> {
  let FieldAvailability::Available(status) = inbound else { return Vec::new(); };
  vec![
    ${INBOUND_METRIC_VARIANTS.map(
      (variant) =>
        `MetricSample::new(MetricKind::${variant}, f64::from(status.placeholder_count), timestamp_unix_seconds),`,
    ).join("\n    ")}
    MetricSample::new(MetricKind::InboundInactivePermissionEffectCount, f64::from(status.inactive_permission_effect_observations), timestamp_unix_seconds),
    MetricSample::new(MetricKind::InboundPermissionValidationFailureCount, f64::from(status.permission_validation_failures), timestamp_unix_seconds),
  ]
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/metrics/tests.rs",
      `
fn unavailable_inbound_status_emits_no_metric_samples() {}
fn inbound_status_maps_to_each_fixed_inbound_metric_kind() {}
fn inactive_permission_metric_uses_observation_count_not_label_count() {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/status/inbound.rs",
      `
pub struct InboundPeerServingStatus {
  #[serde(default)]
  pub inactive_permission_effect_observations: u32,
  #[serde(default)]
  pub permission_validation_failures: u32,
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync.rs",
      `
pub fn set_inbound_metric_status_provider<F>(&mut self, provider: F)
where F: Fn() -> FieldAvailability<InboundPeerServingStatus> + Send + Sync + 'static {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/metrics.rs",
      `
pub fn set_inbound_metric_status_provider<F>(&mut self, provider: F)
where F: Fn() -> FieldAvailability<InboundPeerServingStatus> + Send + Sync + 'static {}
let mut samples = summary.metric_samples(timestamp);
samples.extend(inbound_metric_samples(&provider(), timestamp));
self.store.append_metric_samples(
            &samples,
            MetricRetentionPolicy::default(),
            timestamp,
            self.config.persist_mode,
)?;
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/runtime_state.rs",
      "",
    ],
    [
      "packages/open-bitcoin-node/src/sync/tests.rs",
      `
fn persist_metrics_appends_inbound_status_samples_with_sync_samples() {}
fn persist_metrics_omits_inbound_samples_when_status_unavailable() {}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/config.rs",
      "pub struct RuntimeConfig { pub inbound_permission_validation_failures: u32 }",
    ],
    [
      "packages/open-bitcoin-rpc/src/config/loader/open_bitcoin_runtime.rs",
      `
fn count_inbound_permission_validation_failures() { ParsedPeerPermissionClass::parse(); duplicate_literal_ip_address; }
fn inbound_permission_validation_failure_count_is_config_validation_aggregate() {}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/context/network.rs",
      `
pub fn from_runtime_config_with_store() { maybe_metrics_store: maybe_store.clone(); }
pub fn set_metrics_store() {}
pub fn metrics_status() { load_metrics_status(MetricRetentionPolicy::default()); }
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/context/inbound_status.rs",
      `
admission.inactive_permission_effect_observations;
self.inbound_permission_validation_failures;
permission_validation_failures: self.inbound_permission_validation_failures,
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      `
ManagedRpcContext::from_runtime_config_with_store();
sync_runtime.set_inbound_metric_status_provider(|| { shared_context.try_lock(); context.current_inbound_status() });
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/bin/open_bitcoind/inbound_metrics.rs",
      `
fn start_inbound_metrics_worker() {}
fn persist_inbound_metrics_once() {
  inbound_metric_samples(&inbound, timestamp);
  store.append_metric_samples(&samples, retention, timestamp, persist_mode);
}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs",
      `
fn open_bitcoind_inbound_metrics_worker_persists_sync_disabled_inbound_samples() {
  ManagedRpcContext::from_runtime_config_with_store();
  wait_for_inbound_metric_sample();
  MetricKind::InboundAdmittedPeerCount;
}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/dispatch/node.rs",
      "OpenBitcoinNetworkStatusResponse { metrics: context.metrics_status() }",
    ],
    [
      "packages/open-bitcoin-rpc/src/method/node.rs",
      `
pub struct OpenBitcoinNetworkStatusResponse {
  #[serde(default)]
  pub metrics: MetricsStatus,
}
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/status.rs",
      `
let network_status = collect_open_bitcoin_network_status(rpc_client);
let metrics = network_status.metrics;
OpenBitcoinStatusSnapshot { metrics, }
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/status/tests.rs",
      `
fn fake_live_rpc_maps_metrics_from_open_bitcoin_network_status() {
  MetricKind::InboundAdmittedPeerCount;
  snapshot.metrics.samples;
}
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/dashboard/model.rs",
      `
fn dashboard_charts(snapshot: &OpenBitcoinStatusSnapshot) { dashboard_metric_kinds(snapshot); }
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/dashboard/model/metrics.rs",
      `
pub const MAX_DASHBOARD_CHARTS: usize = 8;
pub const DASHBOARD_METRIC_KINDS: [MetricKind; 8] = [];
pub const INBOUND_DASHBOARD_METRIC_CANDIDATES: [MetricKind; 23] = [
  ${INBOUND_METRIC_VARIANTS.map((variant) => `MetricKind::${variant}`).join(",\n  ")}
];
fn dashboard_metric_kinds(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<MetricKind> { retained_inbound_metric_kinds(snapshot); vec![] }
fn retained_inbound_metric_kinds(snapshot: &OpenBitcoinStatusSnapshot) -> Vec<MetricKind> { vec![] }
`,
    ],
    [
      "packages/open-bitcoin-cli/src/operator/dashboard/model/tests.rs",
      "fn dashboard_charts_render_retained_inbound_metric_samples_without_expanding_row() {}",
    ],
    [
      "packages/open-bitcoin-cli/src/operator/support/tests.rs",
      "fn support_bundle_preserves_retained_inbound_metric_samples() {}",
    ],
    [
      "packages/open-bitcoin-node/src/status/tests.rs",
      "fn status_metrics_json_preserves_retained_inbound_samples_without_dynamic_labels() {}",
    ],
    [
      "docs/architecture/operator-observability.md",
      CLOSED_FLOW,
    ],
    [
      "docs/operator/runtime-guide.md",
      `
${CLOSED_FLOW}
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- status --format json
bazel run //packages/open-bitcoin-cli:open_bitcoin -- status --format json
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- dashboard --tick-ms 1000
bazel run //packages/open-bitcoin-cli:open_bitcoin -- dashboard --tick-ms 1000
cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-inbound-support
bazel run //packages/open-bitcoin-cli:open_bitcoin -- support bundle --output-dir=/tmp/open-bitcoin-inbound-support
Retained local inbound metric evidence does not claim transaction relay, compact block relay, mempool propagation, public inbound defaults, packaged service operation, or production full-node readiness.
`,
    ],
    [
      "scripts/check-phase97-inbound-metrics.ts",
      INBOUND_METRIC_VARIANTS.join("\n"),
    ],
    [
      "scripts/verify.sh",
      `
bun test scripts/check-phase96-peer-policy-runtime-bridge.test.ts
bun run scripts/check-phase96-peer-policy-runtime-bridge.ts
bun test scripts/check-phase97-inbound-metrics.test.ts
bun run scripts/check-phase97-inbound-metrics.ts
run_step "test Phase 97 inbound metrics checker" bun test scripts/check-phase97-inbound-metrics.test.ts
run_step "check Phase 97 inbound metrics" bun run scripts/check-phase97-inbound-metrics.ts
`,
    ],
  ]);
}

function replaceInFile(
  files: Map<TargetFile, string>,
  file: TargetFile,
  search: string,
  replacement: string,
): void {
  const current = files.get(file) ?? "";
  files.set(file, current.replace(search, replacement));
}

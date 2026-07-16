import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase121BlockRelayMetricsLogRuntime } from "./check-phase121-block-relay-metrics-log-runtime";

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
type FixtureOptions = {
  maybeMutateFiles?: (files: Map<TargetFile, string>) => void;
};

const CLOSED_FLOW =
  "DurableSyncRuntime::network -> one availability-gated BlockRelayRuntimeEvidenceSnapshot -> block_relay_metric_samples / block_relay_log_record -> retained metrics / structured logs";
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with corrected authoritative Phase 121 runtime corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("passes against the real repository corpus", () => {
  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime();

  // Assert
  expect(failures).toEqual([]);
});

test("fails when obsolete block-relay provider wiring returns", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "packages/open-bitcoin-node/src/sync.rs",
        "set_block_relay_metric_status_provider maybe_block_relay_metric_status_provider",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 obsolete provider wiring");
});

test("fails when direct sync-network sampling is removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync.rs",
        "self.network.block_relay_runtime_evidence_snapshot()",
        "rpc_context.block_relay_runtime_evidence_snapshot()",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain(
    "P121 authoritative snapshot missing self.network.block_relay_runtime_evidence_snapshot()",
  );
});

test("fails when structured logging uses a second sample", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync.rs",
        "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
        "let second_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot();\nself.write_block_relay_log(&mut summary, second_block_relay_snapshot.as_ref(), timestamp);",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain(
    "P121 same snapshot log argument missing maybe_block_relay_snapshot.as_ref()",
  );
});

test("fails when unavailable evidence is projected", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync.rs",
        "FieldAvailability::Unavailable { .. } => None",
        "FieldAvailability::Unavailable { .. } => Some(snapshot)",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 activation omission");
});

test("fails when the metric helper is no longer reused", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/metrics/block_relay.rs",
        "pub fn block_relay_metric_samples",
        "pub fn replacement_metric_samples",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 metric helper reuse");
});

test("fails when retained metric persistence is removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/metrics.rs",
        "append_metric_samples",
        "discard_metric_samples",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 retained metrics append");
});

test("fails when structured log append is removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/runtime_state.rs",
        "append_structured_record",
        "discard_structured_record",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 structured log append");
});

test("fails when the sensitive-marker runtime guard is removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/tests.rs",
        "write_block_relay_log_omits_sensitive_markers",
        "write_block_relay_log_allows_sensitive_markers",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("write_block_relay_log_omits_sensitive_markers");
});

test("fails when Phase 121 verifier inclusion is removed", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "scripts/verify.sh",
        "bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts",
        "",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain(
    "bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts",
  );
});

test("fails when bounded no-claim documentation is broadened", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "docs/architecture/operator-observability.md",
        "Phase 121 enables package relay and proves production full-node readiness.",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain("P121 no-claim boundary");
  expect(message).toContain("enables package relay");
});

test("fails when a twin block-relay metrics worker appears", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      appendToFile(
        files,
        "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
        "start_block_relay_metrics_worker();",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("P121 no twin metrics worker");
});

function createFixture(options: FixtureOptions = {}): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase121-"));
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
      "packages/open-bitcoin-node/src/sync.rs",
      `
fn maybe_authoritative_block_relay_snapshot(&self) -> Option<BlockRelayRuntimeEvidenceSnapshot> {
  let snapshot = self.network.block_relay_runtime_evidence_snapshot();
  match snapshot.status.block_serving.activation {
    FieldAvailability::Available(_) => Some(snapshot),
    FieldAvailability::Unavailable { .. } => None,
  }
}
let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot();
self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp);
self.write_summary_logs(&mut summary, timestamp);
self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/metrics.rs",
      `
if let Some(snapshot) = maybe_block_relay_snapshot {
  samples.extend(block_relay_metric_samples(
    &snapshot.status,
    snapshot.served_count,
    timestamp,
  ));
}
append_metric_samples(&samples, MetricRetentionPolicy::default());
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/runtime_state.rs",
      `
pub(super) fn write_block_relay_log(
  &self,
  maybe_block_relay_snapshot: Option<&BlockRelayRuntimeEvidenceSnapshot>,
) {
  let Some(snapshot) = maybe_block_relay_snapshot else { return; };
  let record = block_relay_log_record(&snapshot.status, snapshot.served_count, timestamp);
  self.append_structured_record(&record);
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/tests.rs",
      `
fn persist_metrics_appends_block_relay_status_samples_with_sync_samples() {}
fn persist_metrics_omits_block_relay_samples_without_snapshot() {}
fn write_block_relay_log_emits_when_status_available() {}
fn write_block_relay_log_omits_when_status_unavailable() {}
fn write_block_relay_log_omits_sensitive_markers() {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
      `
fn phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log() {}
fn phase123_sync_network_compact_activity_projects_same_snapshot_to_metrics_and_log() {
  assert_eq!(eligibility.eligible_peer_count, 2);
  assert_eq!(runtime.network.block_served_write_count(), 9);
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/metrics/block_relay.rs",
      `
pub fn block_relay_metric_samples(
  status: &BlockRelayEvidenceStatus,
  served_count: u64,
  timestamp: u64,
) -> Vec<MetricSample> {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/logging.rs",
      `
pub fn block_relay_log_record(
  status: &BlockRelayEvidenceStatus,
  served_count: u64,
  timestamp: u64,
) -> StructuredLogRecord {}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      `
let context = ManagedRpcContext::from_runtime_config(runtime);
sync_runtime.set_inbound_metric_status_provider(move || context.current_inbound_status());
`,
    ],
    [
      "docs/architecture/operator-observability.md",
      `
## Phase 121 block-relay runtime projection

${CLOSED_FLOW}.

The snapshot combines the unchanged sanitized BlockRelayEvidenceStatus with a
runtime-only, non-serialized served_count. ManagedRpcContext owns a separate network
instance and is not the sync projection source. Unavailable evidence is omitted.
Aggregate-only redaction remains in force. This does not claim public block serving
by default, package relay, public inbound defaults, or production full-node readiness.
`,
    ],
    [
      "scripts/check-phase121-block-relay-metrics-log-runtime.ts",
      "export function checkPhase121BlockRelayMetricsLogRuntime() {}\n",
    ],
    [
      "scripts/verify.sh",
      `
bun test scripts/check-phase116-operator-block-relay-evidence.test.ts
bun run scripts/check-phase116-operator-block-relay-evidence.ts
bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts
run_step "test Phase 116 operator block-relay evidence checker" bun test scripts/check-phase116-operator-block-relay-evidence.test.ts
run_step "check Phase 116 operator block-relay evidence" bun run scripts/check-phase116-operator-block-relay-evidence.ts
run_step "test Phase 121 block-relay metrics and log runtime checker" bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts
run_step "check Phase 121 block-relay metrics and log runtime" bun run scripts/check-phase121-block-relay-metrics-log-runtime.ts
`,
    ],
  ]);
}

function replaceInFile(
  files: Map<TargetFile, string>,
  relativePath: TargetFile,
  from: string,
  to: string,
): void {
  const current = files.get(relativePath) ?? "";
  files.set(relativePath, current.replaceAll(from, to));
}

function appendToFile(
  files: Map<TargetFile, string>,
  relativePath: TargetFile,
  addition: string,
): void {
  const current = files.get(relativePath) ?? "";
  files.set(relativePath, `${current}\n${addition}\n`);
}

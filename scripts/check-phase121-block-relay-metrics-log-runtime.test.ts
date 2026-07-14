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
  "BlockRelayEvidenceStatus -> block_relay_metric_samples / block_relay_log_record -> DurableSyncRuntime persist_metrics / structured logs";
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) {
    rmSync(root, { force: true, recursive: true });
  }
});

test("passes with complete Phase 121 block-relay metrics and log runtime corpus", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("fails when provider setter is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync.rs",
        "set_block_relay_metric_status_provider",
        "set_inbound_metric_status_provider",
      );
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/metrics.rs",
        "set_block_relay_metric_status_provider",
        "set_inbound_metric_status_provider",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("set_block_relay_metric_status_provider");
});

test("fails when persist append extension is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/metrics.rs",
        "samples.extend",
        "samples.push_all_placeholder",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("samples.extend");
});

test("fails when write_block_relay_log is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/runtime_state.rs",
        "write_block_relay_log",
        "write_summary_logs",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("write_block_relay_log");
});

test("fails when open-bitcoind daemon wiring is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
        "set_block_relay_metric_status_provider",
        "set_inbound_metric_status_provider",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("set_block_relay_metric_status_provider");
});

test("fails when sensitive-marker runtime test is missing", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      replaceInFile(
        files,
        "packages/open-bitcoin-node/src/sync/tests.rs",
        "write_block_relay_log_omits_sensitive_markers",
        "write_block_relay_log_omits_when_status_unavailable",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  expect(failures.join("\n")).toContain("write_block_relay_log_omits_sensitive_markers");
});

test("fails claim creep and missing closed-flow docs", () => {
  // Arrange
  const root = createFixture({
    maybeMutateFiles(files) {
      files.set(
        "docs/architecture/operator-observability.md",
        "Phase 121 enables package relay and proves production full-node readiness.",
      );
    },
  });

  // Act
  const failures = checkPhase121BlockRelayMetricsLogRuntime({ rootDir: root });

  // Assert
  const message = failures.join("\n");
  expect(message).toContain(CLOSED_FLOW);
  expect(message).toContain("enables package relay");
});

test("fails missing verifier wiring", () => {
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
self.write_block_relay_log(&mut summary, timestamp);
set_block_relay_metric_status_provider
FieldAvailability<BlockRelayEvidenceStatus>
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/metrics.rs",
      `
pub fn set_block_relay_metric_status_provider<F>(&mut self, provider: F)
where F: Fn() -> FieldAvailability<BlockRelayEvidenceStatus> + Send + Sync + 'static {}
if let FieldAvailability::Available(status) = provider() {
  samples.extend(block_relay_metric_samples(&status, timestamp));
}
append_metric_samples(
            &samples,
            MetricRetentionPolicy::default(),
);
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/runtime_state.rs",
      `
pub(super) fn write_block_relay_log(&self, summary: &mut SyncRunSummary, timestamp: i64) {
  let record = block_relay_log_record(&status, timestamp);
  self.append_structured_record(&record);
}
`,
    ],
    [
      "packages/open-bitcoin-node/src/sync/tests.rs",
      `
fn persist_metrics_appends_block_relay_status_samples_with_sync_samples() {}
fn persist_metrics_omits_block_relay_samples_when_status_unavailable() {}
fn write_block_relay_log_emits_when_status_available() {}
fn write_block_relay_log_omits_when_status_unavailable() {}
fn write_block_relay_log_omits_sensitive_markers() {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/metrics/block_relay.rs",
      `
pub fn block_relay_metric_samples(status: &BlockRelayEvidenceStatus, timestamp: u64) -> Vec<MetricSample> {}
`,
    ],
    [
      "packages/open-bitcoin-node/src/logging.rs",
      `
pub fn block_relay_log_record(status: &BlockRelayEvidenceStatus, timestamp: u64) -> StructuredLogRecord {}
`,
    ],
    [
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      `
sync_runtime.set_block_relay_metric_status_provider(move || {
  let status = context.block_relay_evidence_status();
  FieldAvailability::unavailable(BLOCK_SERVING_EVIDENCE_UNAVAILABLE_REASON)
});
`,
    ],
    [
      "docs/architecture/operator-observability.md",
      `
## Phase 121 block-relay runtime projection

${CLOSED_FLOW}

DurableSyncRuntime persist_metrics and structured-log paths project
block_relay_metric_samples when Available via the provider.
`,
    ],
    [
      "scripts/check-phase121-block-relay-metrics-log-runtime.ts",
      `
export function checkPhase121BlockRelayMetricsLogRuntime() {}
`,
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

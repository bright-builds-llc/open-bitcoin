import { expect, test } from "bun:test";
import { checkPhase161OperatorIndexEvidence, loadSources } from "./check-phase161-operator-index-evidence.ts";

const clean = loadSources();
const N = "packages/open-bitcoin-node/src/";
const C = "packages/open-bitcoin-cli/src/operator/";

test("current fixed source/document view passes", () => {
  // Arrange / Act
  const failures = checkPhase161OperatorIndexEvidence(clean);
  // Assert
  expect(failures).toEqual([]);
});

const mutations: [string, string, string, string][] = [
  ["authority edge", N + "network/operator_snapshot.rs", "basic_index: self.project_basic_index()", "basic_index: BasicIndexOperatorStatus::default()"],
  ["test-only authority", N + "network/operator_snapshot.rs", "fn project_basic_index(", "#[cfg(test)] fn project_basic_index("],
  ["status-triggered catchup", N + "network/operator_snapshot.rs", "let mut status =", "self.drive_basic_filter_index_turn();\n        let mut status ="],
  ["RPC edge", "packages/open-bitcoin-rpc/src/dispatch/node.rs", "basic_index: snapshot.basic_index().clone()", "basic_index: Default::default()"],
  ["collector edge", C + "status.rs", "let basic_index = network_status.basic_index;", "let basic_index = Default::default();"],
  ["human edge", C + "status/render.rs", "lines.extend(basic_index_lines(&snapshot.basic_index));", ""],
  ["dashboard edge", C + "dashboard/model.rs", "basic_index_section(&snapshot.basic_index)", "basic_index_section(&Default::default())"],
  ["support edge", C + "support/render/basic_index.rs", "basic_index_entries(status)", "basic_index_entries(&Default::default())"],
  ["false zero default", N + "status/basic_index.rs", "retention: FieldAvailability::unavailable(reason)", "retention: FieldAvailability::available(Default::default())"],
  ["legacy default", N + "status/basic_index.rs", "Self::unavailable(BasicIndexUnavailableReason::NotReported)", "Self::unavailable(BasicIndexUnavailableReason::LiveUnavailable)"],
  ["stopped false disabled", C + "status.rs", "BasicIndexUnavailableReason::LiveUnavailable", "BasicIndexUnavailableReason::NotReported"],
  ["raw serde reason", N + "status/basic_index.rs", "BasicIndexUnavailableReason::normalize(reason)", "reason"],
  ["raw human reason", C + "status/render/basic_index.rs", "BasicIndexUnavailableReason::normalize(reason)", "reason"],
  ["raw support reason", C + "support/redaction/basic_index.rs", "BasicIndexUnavailableReason::normalize(reason)", "reason"],
  ["support sanitizer removed", C + "support/redaction.rs", "basic_index::redact_basic_index(&mut status.basic_index);", ""],
  ["prospective accounting", N + "storage/fjall_store/filters/append.rs", "self.finish_basic_filter_batch(batch, &mut control)?;\n        control.install_retention(retained);", "control.install_retention(retained);\n        self.finish_basic_filter_batch(batch, &mut control)?;"],
  ["unpropagated publication error", N + "storage/fjall_store/filters/append.rs", "self.finish_basic_filter_batch(batch, &mut control)?;", "let _ = self.finish_basic_filter_batch(batch, &mut control);"],
  ["missing integrity invalidation", N + "storage/fjall_store/filters/publication/epoch.rs", "self.retention = super::super::retention::RetainedBasicFilterObservation::Incomplete;", ""],
  ["missing read budget", N + "storage/fjall_store/filters/retention.rs", "work.metadata_reads >= reads", "false"],
  ["missing byte budget", N + "storage/fjall_store/filters/retention.rs", "next_bytes > bytes", "false"],
  ["missing observation bound", N + "storage/fjall_store/filters/retention.rs", "basic_filter_operator_observation_with_limits(8, 4096)", "basic_filter_operator_observation_with_limits(u64::MAX, u64::MAX)"],
  ["scan on poll", N + "storage/fjall_store/filters/retention.rs", "let mut work = BasicOperatorObservationWork::default();", "let rows = self.block_index.iter();\n        let mut work = BasicOperatorObservationWork::default();"],
  ["stale owner branch", N + "network/operator_snapshot.rs", "&& branch == progress.branch_identity()", ""],
  ["invented durable height", N + "network/operator_snapshot.rs", "progress.maybe_safe_durable_endpoint().map(|id| id.height())", "progress.maybe_processed_endpoint().map(|id| id.height())"],
  ["missing formatter module", C + "status/render.rs", "mod basic_index;", ""],
  ["narrow critical order", C + "dashboard/app.rs", ".take(7)", ".take(0)"],
  ["stale producer capture", N + "sync/tests/filter_index/operator_status/captures.rs", "captures[name], *value,", "value, value,"],
  ["unsupported production claim", "README.md", "v2.5 remains pending.", "v2.5 is production ready."],
  ["removed integrated proof limit", "docs/parity/catalog/basic-compact-filters.md", "Phase 162 owns the complete integrated prune/reopen client proof.", "Phase 162 integrated proof is complete."],
  ["missing native execution", "scripts/verify.sh", 'run_step "check Phase 161 operator index evidence" bun run scripts/check-phase161-operator-index-evidence.ts', ""],
];

for (const [name, file, before, after] of mutations) {
  test(`reject ${name}`, () => {
    // Arrange
    const sources = new Map(clean);
    const source = sources.get(file)!;
    expect(source).toContain(before);
    sources.set(file, source.replace(before, after));
    // Act
    const failures = checkPhase161OperatorIndexEvidence(sources);
    // Assert
    expect(failures.length).toBeGreaterThan(0);
  });
}

test("comments cannot replace the removed authority call", () => {
  // Arrange
  const sources = new Map(clean);
  const file = N + "network/operator_snapshot.rs";
  sources.set(file, sources.get(file)!.replace("basic_index: self.project_basic_index()", "basic_index: Default::default() /* basic_index: self.project_basic_index() */"));
  // Act
  const failures = checkPhase161OperatorIndexEvidence(sources);
  // Assert
  expect(failures.some(failure => failure.includes("operator_snapshot"))).toBe(true);
});

test("missing fixed evidence fails closed", () => {
  // Arrange
  const sources = new Map(clean);
  sources.delete(N + "storage/fjall_store/filters/retention.rs");
  // Act
  const failures = checkPhase161OperatorIndexEvidence(sources);
  // Assert
  expect(failures.some(failure => failure.includes("required source"))).toBe(true);
});

test("unsupported readiness claim fails even when all fixed limits remain", () => {
  // Arrange
  const sources = new Map(clean);
  sources.set("README.md", sources.get("README.md") + "\nProduction readiness is proven.\n");
  // Act
  const failures = checkPhase161OperatorIndexEvidence(sources);
  // Assert
  expect(failures.some(failure => failure.includes("unsupported"))).toBe(true);
});

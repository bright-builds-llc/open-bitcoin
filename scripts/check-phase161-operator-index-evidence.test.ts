import { expect, test } from "bun:test";
import { checkPhase161OperatorIndexEvidence, loadSources } from "./check-phase161-operator-index-evidence.ts";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { createHash } from "node:crypto";
import { PHASE162_DOC, PHASE162_IMPLEMENTATION, PHASE162_COMPLETE, PHASE162_EVIDENCE } from "./check-phase158-validated-reorg.ts";

const clean = loadSources();
const N = "packages/open-bitcoin-node/src/";
const C = "packages/open-bitcoin-cli/src/operator/";
const CURRENT_DOCS = ["README.md", "packages/README.md", "docs/architecture/status-snapshot.md", "docs/architecture/operator-observability.md", "docs/operator/runtime-guide.md", "docs/parity/catalog/basic-compact-filters.md"];
function transitionSources(): Map<string, string> {
  const sources = new Map(clean);
  for (const file of PHASE162_EVIDENCE) sources.set(file, readFileSync(resolve(import.meta.dir, "..", file), "utf8"));
  for (const [file, source] of sources) sources.set(file, source.replaceAll(PHASE162_COMPLETE, PHASE162_IMPLEMENTATION));
  let requirements = sources.get(".planning/REQUIREMENTS.md")!;
  for (const id of ["CFPR-02", "CFGR-01", "CFGR-02"]) requirements = requirements.replace(`[x] **${id}**`, `[ ] **${id}**`).replace(`| ${id} | Phase 162 | Complete |`, `| ${id} | Phase 162 | Pending |`);
  sources.set(".planning/REQUIREMENTS.md", requirements);
  const index = JSON.parse(sources.get("docs/parity/index.json")!);
  const owner = index.checklist.surfaces.find((row: { id: string }) => row.id === "v2-5-integrated-retained-filter-proof");
  owner.status = "in_progress";
  if (!owner.evidence.includes(PHASE162_DOC)) owner.evidence.push(PHASE162_DOC);
  index.checklist.surfaces.find((row: { requirements: string[] }) => row.requirements.includes("CFOP-01")).known_gaps = [PHASE162_IMPLEMENTATION];
  sources.set("docs/parity/index.json", JSON.stringify(index));
  sources.set(PHASE162_DOC, `---\nphase_lifecycle_id: 162-2026-10-11T16-25-37\nlifecycle_mode: yolo\nimplementation_status: proven\ngenerated_at: "2026-10-11T23:00:00Z"\n---\n${PHASE162_IMPLEMENTATION}\n` + PHASE162_EVIDENCE.map(file => `| \`${file}\` | \`${createHash("sha256").update(sources.get(file)!).digest("hex")}\` |`).join("\n"));
  for (const file of CURRENT_DOCS) sources.set(file, sources.get(file)!.replace("Phase 162 owns the complete integrated prune/reopen client proof.", PHASE162_IMPLEMENTATION).replace("v2.5 remains pending.", "v2.5 remains unaudited and unshipped."));
  return sources;
}

test("all six current operator docs admit source-linked implementation while requirements remain pending", () => {
  // Arrange
  const sources = transitionSources();
  // Act / Assert
  expect(checkPhase161OperatorIndexEvidence(sources)).toEqual([]);
  sources.delete(PHASE162_DOC);
  expect(checkPhase161OperatorIndexEvidence(sources).some(failure => failure.includes("T-162-10"))).toBe(true);
});

for (const claim of ["BIP37 serving is supported.", "V0 filter is enabled.", "Assumeutxo is supported.", "Dual-chainstate is supported.", "Implicit repair is supported.", "Import is enabled.", "Download is supported.", "Archive-scale is proven.", "Archive-scale serving is proven.", "Public defaults are enabled.", "Public defaults/network CI are enabled.", "Public-network CI is enabled.", "Production service is ready.", "Production-service is ready.", "Production readiness is proven.", "Production-funds safety is proven.", "Funds-safety is established.", "v2.5 is shipped.", "Phase 162 is complete."]) {
  for (const file of CURRENT_DOCS) test(`implementation evidence cannot authorize ${claim} in ${file}`, () => {
    // Arrange
    const sources = transitionSources();
    sources.set(file, sources.get(file)! + "\n" + claim);
    // Act / Assert
    expect(checkPhase161OperatorIndexEvidence(sources).some(failure => failure.includes(file) && failure.includes("unsupported"))).toBe(true);
  });
}

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

for (const [name, file, originalBefore, after] of mutations) {
  test(`reject ${name}`, () => {
    // Arrange
    const sources = new Map(clean);
    const source = sources.get(file)!;
    const before = name === "unsupported production claim" && !source.includes(originalBefore) ? "v2.5 remains unaudited and unshipped."
      : name === "removed integrated proof limit" && !source.includes(originalBefore) ? (source.includes(PHASE162_COMPLETE) ? PHASE162_COMPLETE : PHASE162_IMPLEMENTATION) : originalBefore;
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

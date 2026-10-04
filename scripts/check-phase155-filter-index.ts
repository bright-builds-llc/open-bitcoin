#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

const NODE = "packages/open-bitcoin-node/src/";
const CORE = "packages/open-bitcoin-chainstate/src/";
const DOC = "docs/parity/catalog/basic-compact-filters.md";
const SURFACE = "v2-5-recoverable-basic-index-and-startup-protection";
const REQUIREMENTS = ["CFIX-02", "CFIX-04", "CFPR-03"];
const SOURCES = ["packages/bitcoin-knots/src/index/base.cpp",
  "packages/bitcoin-knots/src/index/blockfilterindex.cpp", "packages/bitcoin-knots/src/node/blockstorage.cpp"];
const RUST_FILES = [
  "packages/open-bitcoin-codec/src/block_filter/validation.rs",
  "packages/open-bitcoin-codec/src/block_filter/validation/tests.rs",
  CORE + "filter_index.rs", CORE + "filter_index/recovery.rs", CORE + "filter_index/tests.rs",
  CORE + "filter_index/tests/commitments.rs", NODE + "storage/filter_index.rs",
  NODE + "storage/filter_index/tests.rs", NODE + "storage/fjall_store/filters.rs",
  NODE + "storage/fjall_store/filters/publication.rs", NODE + "storage/fjall_store/filters/startup.rs",
  NODE + "storage/fjall_store/filters/tests.rs", NODE + "storage/fjall_store/filters/tests/faults.rs",
  NODE + "sync/tests/filter_index.rs", NODE + "sync/tests/filter_index/startup.rs",
  NODE + "sync/tests/filter_index/startup/recovery.rs", NODE + "sync/tests/filter_index/startup/faults.rs",
  NODE + "sync/tests/filter_index/recovery.rs", NODE + "sync/tests/filter_index/faults.rs",
];
const MANIFESTS = ["packages/open-bitcoin-node/Cargo.toml", "packages/open-bitcoin-chainstate/Cargo.toml",
  "packages/open-bitcoin-codec/Cargo.toml"];
export const CHECK_FILES = [...RUST_FILES, ...MANIFESTS, NODE + "storage.rs",
  NODE + "storage/fjall_store.rs", NODE + "chainstate/flush_lifecycle.rs", NODE + "sync/open_runtime.rs",
  NODE + "sync/tests.rs", CORE + "lib.rs", "docs/parity/index.json", "docs/parity/source-breadcrumbs.json",
  DOC, "README.md", "docs/parity/catalog/README.md", "scripts/verify.sh",
  "scripts/check-phase155-filter-index.ts", "scripts/check-phase155-filter-index.test.ts"];
const DENIED = /\b(?:filter index activation|scheduled catch-up|(?:compact[- ]filter )?(?:rpc|peer|operator) serving|runtime reorg|prune ownership|production readiness|production[- ]funds use|hardware power-loss proof)\s+(?:is|are)\s+(?:enabled|supported|available|active|ready|shipped|safe)\b/i;
type Surface = { id: string; requirements: string[]; evidence: string[];
  upstream: { sources: string[]; tests: string[] }; known_gaps: string[] };

/** Source/claim guard only: real durable Rust tests and the full native gate remain required. */
export function checkPhase155FilterIndex(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch (error) { failures.push(`${file}: cannot read required evidence: ${String(error)}`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  const requireText = (file: string, anchors: string[], category: string) => {
    for (const anchor of anchors) {
      if (!text(file).includes(anchor)) failures.push(`${file}: missing ${category}: ${anchor}`);
    }
  };
  const ordered = (source: string, anchors: string[], category: string) => {
    let cursor = -1;
    for (const anchor of anchors) {
      const next = source.indexOf(anchor, cursor + 1);
      if (next < 0) { failures.push(`${category}: missing or out-of-order ${anchor}`); return; }
      cursor = next;
    }
  };
  const initialize = text(NODE + "chainstate/flush_lifecycle.rs").split("pub fn initialize(")[1]?.split("impl FlushLifecycle")[0] ?? "";
  ordered(initialize, ["let recovered = apply_recovery_decision", "recovered.best_block()",
    "coins_recovery_outcome_after_success(", "store.recover_basic_filter_index_before_prune(maybe_best_block)?;",
    "resume_prune_intent(store, &locks)?;", "ManagerReadiness::ReadyToFlush", "CoinsCache::from_parent(recovered)"], "startup ordering");
  if (!/^    store\.recover_basic_filter_index_before_prune\(maybe_best_block\)\?;$/m.test(initialize)) {
    failures.push("startup ordering: mandatory guard must be an unconditional initialize statement");
  }
  ordered(text(NODE + "sync/open_runtime.rs"), ["pub fn open_with_runtime_activation(",
    "initialize(&store", "Chainstate::from_coins_cache", "ManagedChainstate::from_chainstate"], "runtime constructor");
  const startup = NODE + "storage/fjall_store/filters/startup.rs";
  requireText(startup, ["self.validate_basic_filter_records()?;", "VerifiedChainstateFence::new",
    "FilterRecoveryPlan::Reconcile", "FilterRecoveryPlan::Refuse", ".check_prune_intent(intent.height)",
    "fail_closed BASIC index startup", "self.publish_basic_filter_checkpoint"], "guard mechanism");
  ordered(text(startup), ["self.maybe_prune_intent()?", ".check_prune_intent(intent.height)",
    "self.publish_basic_filter_checkpoint"], "guard ordering");
  requireText(NODE + "storage/fjall_store/filters/publication.rs", ["SyncAll", "control.poisoned = true",
    "self.verify_basic_filter_fence(fence)?", "PRUNE_LOCKS_KEY", "codec::STATE_KEY", "128"], "atomic publication");
  requireText(NODE + "storage/filter_index.rs", ["basic_filter:v1:record:", "basic_filter:v1:active:",
    "basic_filter:v1:state", "validate_basic_filter_encoding", "double_sha256"], "bounded additive envelopes");
  requireText(NODE + "storage.rs", ["pub const CURRENT: Self = Self(2);"], "schema 2 compatibility");
  requireText(NODE + "sync/tests.rs", ["mod filter_index;"], "test registration");
  requireText(NODE + "sync/tests/filter_index.rs", ["mod recovery;", "mod faults;", "mod startup;"], "test registration");
  requireText(CORE + "lib.rs", ["pub mod filter_index;"], "core registration");
  requireText(NODE + "sync/tests/filter_index/recovery.rs", [".stage_connect_block_with_current_time(",
    "BasicFilterInputs::from_historical", "staged.undo.transactions", "DurableSyncRuntime::open(",
    "FjallNodeStore::open(&self.path)", "filter_index_production_validated_ahead_records_do_not_mint_cursor",
    "filter_index_production_validated_equal_height_fork_retains_common_prefix", "history.reopen()",
    "original.reopen()", "maybe_active_basic_filter_record(2)", "assert_rows_and_payloads",
    "filter_index_production_validated_missing_prefix_preserves_history_and_intent"], "validated history");
  requireText(NODE + "sync/tests/filter_index/faults.rs", [".batch_write_with_limit(", "partial_buffered_commits()",
    "compatible_metadata", "history.reopen()", "lifecycle.execute_flush(", "FilterPublicationFault::BeforeChainMeta",
    "FilterPublicationFault::BeforeRecords", "FilterPublicationFault::BeforeCheckpoint",
    "FilterPublicationFault::BeforeProtection", "FilterPublicationFault::AfterCommit", "snapshot_index(&history.path)",
    "maybe_prune_intent()", "collect_unspent_hint()",
    "filter_index_production_after_record_commit_error_retains_ahead_rows_without_cursor"], "fault evidence");
  requireText(NODE + "storage/fjall_store/filters/tests/faults.rs", ["fixtures(256)", "4 * records.len() - 1",
    "validate_basic_filter_records()"], "linear scan evidence");
  requireText(NODE + "sync/tests/filter_index/startup.rs", [
    "filter_index_production_reopen_refuses_required_history_before_prune",
    "filter_index_production_reopen_directly_protects_genesis_and_height_one",
    "filter_index_production_reopen_refuses_corrupt_immutable_filter",
    "filter_index_production_reopen_refuses_corrupt_projection",
    "filter_index_production_reopen_refuses_missing_recovered_coins_authority",
    "filter_index_production_reopen_preserves_legacy_safe_resume",
    "filter_index_production_reopen_checks_rewound_required_input_before_mutation"], "runtime matrix");
  requireText(NODE + "sync/tests/filter_index/startup/recovery.rs", [
    "filter_index_production_reopen_refuses_same_height_saved_fence_conflict",
    "filter_index_production_reopen_reconciles_only_common_recovered_ancestry",
    "filter_index_production_reopen_refuses_missing_durable_metadata"], "runtime matrix");
  for (const file of [CORE + "filter_index.rs", CORE + "filter_index/recovery.rs",
    "packages/open-bitcoin-codec/src/block_filter/validation.rs"]) {
    if (/std::(?:fs|io|net|time|process)|(?:fjall|tokio)::|FjallNodeStore|SystemTime/.test(text(file))) {
      failures.push(`${file}: pure policy must remain free of I/O/runtime effects`);
    }
  }
  checkDependencies(text, failures);
  checkParity(text("docs/parity/index.json"), failures);
  checkBreadcrumbs(text("docs/parity/source-breadcrumbs.json"), text, failures);
  requireText(DOC, ["CFIX-02", "CFIX-04", "CFPR-03", "schema-2", "flat files/LevelDB",
    "checkpoint-visible projection", "heights 0/1", "historical publication provenance", "not current authority",
    "linear retained-history work", "bounded additional record memory", "not a total runtime/memory cap",
    "hardware power-loss simulation", "159–162 own RPC/peer/operator serving", "remain deferred",
    "Structural checks supplement", "Category-only", "No implicit repair or download"], "scoped documentation");
  for (const file of [DOC, "README.md", "docs/parity/catalog/README.md"]) {
    const maybeClaim = text(file).match(DENIED);
    if (maybeClaim) failures.push(`${file}: unsupported claim: ${maybeClaim[0]}`);
  }
  const steps = text("scripts/verify.sh").split("\n").filter(line => line.startsWith("run_step "));
  for (const command of ["bun test ./scripts/check-phase155-filter-index.test.ts",
    "bun run scripts/check-phase155-filter-index.ts", "bun test ./scripts/check-phase154-basic-filters.test.ts",
    "bun run scripts/check-phase154-basic-filters.ts", "bun run scripts/generate-basic-filter-vectors.ts --check"]) {
    if (!steps.some(line => line.endsWith(command))) failures.push(`default verifier missing executable step: ${command}`);
  }
  return failures;
}

function checkDependencies(text: (file: string) => string, failures: string[]): void {
  const allowed = [
    ["fjall", "fs4", "getrandom", "open-bitcoin-codec", "open-bitcoin-core", "open-bitcoin-mempool",
      "open-bitcoin-network", "open-bitcoin-wallet", "serde", "serde_json"],
    ["open-bitcoin-consensus", "open-bitcoin-primitives"], ["open-bitcoin-primitives"],
  ];
  for (const [index, file] of MANIFESTS.entries()) {
    const actual = [...text(file).split("[dependencies]")[1]?.split(/\n\[/)[0]?.matchAll(/^([\w-]+)\s*=/gm) ?? []]
      .map(match => match[1]).sort();
    if (JSON.stringify(actual) !== JSON.stringify(allowed[index].toSorted())) failures.push(`${file}: production dependency expansion/drift`);
  }
}
function checkParity(source: string, failures: string[]): void {
  try {
    const index = JSON.parse(source) as { baseline: string; checklist: { surfaces: Surface[] } };
    if (index.baseline !== "29.3.knots20260210") failures.push("parity index: wrong baseline pin");
    const matching = index.checklist.surfaces.filter(row => row.id === SURFACE);
    if (matching.length !== 1) { failures.push("parity index: expected one phase 155 surface owner"); return; }
    const surface = matching[0];
    if (JSON.stringify(surface.requirements.toSorted()) !== JSON.stringify(REQUIREMENTS.toSorted())) failures.push("parity index: phase 155 requirement scope drift");
    for (const requirement of REQUIREMENTS) {
      const owners = index.checklist.surfaces.filter(row => row.requirements.includes(requirement));
      if (owners.length !== 1 || owners[0].id !== SURFACE) failures.push(`${requirement}: wrong surface owner`);
    }
    for (const file of [CORE + "filter_index/recovery.rs", NODE + "storage/fjall_store/filters/startup.rs",
      NODE + "sync/tests/filter_index/recovery.rs", NODE + "sync/tests/filter_index/faults.rs",
      "scripts/check-phase155-filter-index.ts", "scripts/check-phase155-filter-index.test.ts", DOC]) {
      if (!surface.evidence.includes(file)) failures.push(`parity surface evidence missing: ${file}`);
    }
    for (const source of SOURCES) {
      if (!surface.upstream.sources.includes(source)) failures.push(`parity source anchor missing: ${source}`);
    }
    if (!surface.known_gaps.some(gap => gap.includes("remain deferred"))) failures.push("parity surface missing deferred gaps");
    const maybeClaim = JSON.stringify(surface).match(DENIED);
    if (maybeClaim) failures.push(`parity surface: unsupported claim: ${maybeClaim[0]}`);
  } catch (error) { failures.push(`parity index invalid contract: ${String(error)}`); }
}
function checkBreadcrumbs(source: string, text: (file: string) => string, failures: string[]): void {
  try {
    const manifest = JSON.parse(source) as { groups: { files: string[]; breadcrumbs: string[] }[] };
    for (const file of RUST_FILES) {
      const matching = manifest.groups.filter(group => group.files.includes(file));
      if (matching.length !== 1 || !matching[0].breadcrumbs.length) {
        failures.push(`breadcrumb manifest must register exactly one mapped group: ${file}`); continue;
      }
      for (const anchor of matching[0].breadcrumbs) {
        if (!text(file).includes(`// - ${anchor}`)) failures.push(`${file}: missing source breadcrumb: ${anchor}`);
      }
    }
  } catch (error) { failures.push(`breadcrumb manifest invalid: ${String(error)}`); }
}
if (import.meta.main) {
  const failures = checkPhase155FilterIndex();
  if (failures.length) {
    console.error("Phase 155 recoverable filter index evidence check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 155 recovery/startup source, parity and scoped claim links validated; durable tests remain required.");
}

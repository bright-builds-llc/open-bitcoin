#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { CONTRACTS, CORE, NODE, RPC, REGISTRATIONS } from "./check-phase156-prune-coordination/contracts.ts";
import { compact, hasOrderedCode, maybeRustFunction, rustCode } from "./check-phase156-prune-coordination/rust-evidence.ts";
export { CONTRACTS };
export const DOC = "docs/parity/catalog/basic-compact-filters.md";
export const SURFACE = "v2-5-index-owned-manual-and-automatic-prune-coordination";
const HEADING = "## Index-owned manual and automatic prune coordination";
const SOURCES = ["packages/bitcoin-knots/src/index/base.cpp",
  "packages/bitcoin-knots/src/index/blockfilterindex.cpp", "packages/bitcoin-knots/src/node/blockstorage.cpp"];
const BASELINE_TESTS = ["packages/bitcoin-knots/test/functional/feature_index_prune.py",
  "packages/bitcoin-knots/test/functional/feature_pruning.py"];
const EXTRA: [string, string, string[]][] = [
  [CORE + "filter_index/lifecycle.rs", "next", ["checked_add(1)", "IndexLifecycleError::GenerationExhausted"]],
  [NODE + "storage/fjall_store/filters/ownership.rs", "maybe_basic_filter_work", [
    "self.filter_publication_guard()?", "IndexLifecycle::Active", "self.verify_basic_filter_fence(fence)?",
    "self.materialize_basic_filter_owner_guarded(", "IndexWorkIdentity::new(", "Arc::clone(&self.filter_publication)"]],
  [NODE + "storage/fjall_store/filters/publication.rs", "publish_basic_filter_checkpoint", [
    "self.filter_publication_guard()?", "self.check_basic_filter_work_guarded(work, fence, &control)?",
    "self.publish_basic_filter_checkpoint_guarded("]],
  [NODE + "storage/fjall_store/filters/publication.rs", "verify_basic_filter_fence", [
    "view.head_blocks()", "view.best_block()", "self.load_chain_meta_for_open()?", "VerifiedChainstateFence::new(", "durable != *fence"]],
  [NODE + "network/runtime_authority/prune_flush.rs", "replace_prune_lock", ["refuse_reserved_name(&record.name)?", "self.mutate("]],
  [NODE + "network/runtime_authority/prune_flush.rs", "refuse_reserved_name", ["name == BASIC_INDEX_PRUNE_LOCK", "Err("]],
  [RPC + "context/prune.rs", "replace_prune_lock", ["refuse_reserved_name(&name)?", "height_last < height_first", ".replace_prune_lock(record.clone())"]],
  [RPC + "context/prune.rs", "refuse_reserved_name", ["name == BASIC_INDEX_PRUNE_LOCK", "RpcFailure::invalid_parameter("]],
  [NODE + "storage/fjall_store/prune.rs", "sync_prune_intent", ["self.filter_publication_guard()?", "self.check_prune_candidate_guarded(", "self.sync_prune_intent_guarded(intent)"]],
  [NODE + "storage/fjall_store/prune.rs", "check_prune_candidate_guarded", [
    "self.maybe_basic_filter_owner_guarded(control)?", "self.maybe_owned_prune_ancestry(maybe_owner)?", "check_owned_prune_candidate(", "self.load_prune_locks()?"]],
  [NODE + "storage/fjall_store/prune.rs", "check_owned_prune_candidate", [
    "EffectiveIndexOwnership::maybe_effective_protection", "position.height == intent.height", "position.block_hash == intent.block_hash", ".check_prune_intent(intent.height)"]],
  [NODE + "storage/fjall_store/prune.rs", "commit_paired_delete", ["self.commit_paired_delete_observing("]],
  [NODE + "chainstate/flush_lifecycle.rs", "protects_height", ["EffectiveIndexOwnership::maybe_effective_protection", "protection.check_prune_intent(height).is_err()"]],
  [NODE + "chainstate/flush_lifecycle.rs", "load_prune_protection", ["Err(StorageError::UnavailableNamespace"]],
  [NODE + "chainstate/flush_lifecycle/fjall_sink.rs", "load_prune_protection", ["FjallNodeStore::load_prune_protection(self)"]],
  [NODE + "chainstate/fjall_store.rs", "load_prune_protection", ["self.store.load_prune_protection()"]],
  [NODE + "chainstate/flush_lifecycle/prune_apply.rs", "classify_protected_prune_height", [
    "protection.check_prune_intent(height).is_err()", "ClassifiedHeight::SkippedLock", "classify_prune_height("]],
  [NODE + "network/runtime_authority/automatic_prune.rs", "invalidate", [
    "self.maybe_completed_key = None", "self.maybe_last_measurement_seconds = None", "self.maybe_last_measured_protection_identity = None"]],
];
const RUST_FILES = [...new Set([...CONTRACTS.flatMap(row => [row.file, row.testFile]),
  ...REGISTRATIONS.map(([file]) => file), ...EXTRA.map(([file]) => file)])];
export const CHECK_FILES = [...RUST_FILES, ...SOURCES, ...BASELINE_TESTS, "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json", DOC, "README.md", "docs/parity/checklist.md", "scripts/verify.sh",
  "scripts/check-phase156-prune-coordination.ts", "scripts/check-phase156-prune-coordination.test.ts",
  "scripts/check-phase156-prune-coordination/contracts.ts", "scripts/check-phase156-prune-coordination/rust-evidence.ts"];
type Surface = { id: string; status: string; requirements: string[]; evidence: string[];
  upstream: { sources: string[]; tests: string[] }; known_gaps: string[] };

/** Source/claim/provenance guard only; recorded runtime tests and root native gate remain required. */
export function checkPhase156PruneCoordination(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: cannot read required evidence`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  for (const row of CONTRACTS) {
    checkBody(text(row.file), row.file, row.symbol, row.anchors, row.threat, failures);
    const maybeTest = maybeRustFunction(text(row.testFile), row.testName);
    if (!maybeTest || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeTest.attributes)
      || !hasAssertions(text(row.testFile), maybeTest.body)) {
      failures.push(`${row.threat}: missing named executable behavioral test: ${row.testFile}:${row.testName}`);
    }
  }
  for (const [file, symbol, anchors] of EXTRA) checkBody(text(file), file, symbol, anchors, "production boundary", failures);
  for (const [file, modules] of REGISTRATIONS) {
    for (const module of modules) {
      if (!compact(rustCode(text(file))).includes(compact(module))) failures.push(`${file}: missing test registration: ${module}`);
    }
  }
  checkConcreteProof(text, failures);
  checkParity(text("docs/parity/index.json"), failures);
  checkBreadcrumbs(text("docs/parity/source-breadcrumbs.json"), text, failures);
  checkDocumentation(text, failures);
  checkVerifier(text("scripts/verify.sh"), failures);
  return [...new Set(failures)];
}

function checkBody(source: string, file: string, symbol: string, anchors: readonly string[],
  category: string, failures: string[]): void {
  const maybeFunction = maybeRustFunction(source, symbol, true);
  if (!maybeFunction || /#\[cfg\(test\)\]/.test(maybeFunction.attributes)
    || !hasOrderedCode(maybeFunction.body, anchors)) {
    failures.push(`${category}: missing ordinary production body contract: ${file}:${symbol}`);
  }
}

function hasAssertions(source: string, body: string): boolean {
  if (/\bassert(?:_eq|_ne)?\s*!/.test(body)) return true;
  for (const match of body.matchAll(/\b(assert_\w+)\s*\(/g)) {
    const maybeHelper = maybeRustFunction(source, match[1]);
    if (maybeHelper && /\bassert(?:_eq|_ne)?\s*!/.test(maybeHelper.body)) return true;
  }
  return false;
}

function checkConcreteProof(text: (file: string) => string, failures: string[]): void {
  const required: [string, string, string[]][] = [
    [NODE + "sync/tests/filter_index/prune_faults.rs", "filter_index_runtime_constructor_materializes_legacy_nonempty_prefix_durably", ["DurableSyncRuntime::open("]],
    [NODE + "sync/tests/filter_index/faults.rs", "filter_index_production_reopens_every_record_and_checkpoint_fault_boundary", [
      "FilterPublicationFault::BeforeRecords", "FilterPublicationFault::BeforeCheckpoint", "FilterPublicationFault::BeforeProtection", "FilterPublicationFault::AfterCommit", "drop(store)", ".reopen()"]],
    [NODE + "sync/tests/filter_index/lifecycle.rs", "filter_index_production_host_disable_faults_recover_valid_retained_lifecycle", [
      "FilterPublicationFault::BeforeDisable", "FilterPublicationFault::AfterDisable", "FilterPublicationFault::BeforeRelease", "FilterPublicationFault::AfterRelease"]],
    [NODE + "sync/tests/filter_index/prune_coordination.rs", "filter_index_runtime_direct_and_resume_required_genesis_and_height_one_refuse", ["[0_usize, 1]", "commit_paired_delete(", "resume_prune_intent("]],
    [RPC + "bin/open_bitcoind/tests/automatic_prune/index_protection.rs", "index_protection_ordinary_legal_target_retains_then_deletes_only_after_disable", [
      "populate_threshold(", "FlushMode::Periodic", "FlushMode::Always", "disable_basic_filter_index()"]],
  ];
  for (const [file, name, anchors] of required) {
    const maybeTest = maybeRustFunction(text(file), name);
    if (!maybeTest || !hasOrderedCode(maybeTest.body, anchors)) failures.push(`${file}: missing concrete fault/reopen/ordinary evidence: ${name}`);
  }
  const owner = text(NODE + "storage/fjall_store/filters/ownership.rs");
  const maybeToken = rustCode(owner).match(/pub\(crate\) struct BasicFilterWorkToken\s*\{([^}]+)\}/)?.[1];
  if (!maybeToken || /\bpub\b/.test(maybeToken)) failures.push("production boundary: work token fields must remain private");
  for (const file of [CORE + "filter_index/lifecycle.rs", NODE + "storage/filter_index/ownership.rs"]) {
    if (/std::(?:fs|io|net|time|process)|(?:fjall|tokio)::|SystemTime/.test(rustCode(text(file)))) failures.push(`${file}: pure ownership policy/codec gained runtime effects`);
  }
}

function checkParity(source: string, failures: string[]): void {
  try {
    const index = JSON.parse(source) as { baseline: string; checklist: { surfaces: Surface[] } };
    if (index.baseline !== "29.3.knots20260210") failures.push("parity index: wrong baseline pin");
    const matching = index.checklist.surfaces.filter(row => row.id === SURFACE);
    if (matching.length !== 1) { failures.push("CFPR-01: expected unique Phase 156 parity surface"); return; }
    const row = matching[0];
    const owners = index.checklist.surfaces.filter(row => row.requirements.includes("CFPR-01"));
    if (owners.length !== 1 || owners[0].id !== SURFACE || JSON.stringify(row.requirements) !== '["CFPR-01"]') failures.push("CFPR-01: wrong surface owner");
    if (!["in_progress", "done"].includes(row.status)) failures.push("parity index: unsupported Phase 156 status");
    for (const file of [...new Set(CONTRACTS.flatMap(row => [row.file, row.testFile])), DOC,
      "scripts/check-phase156-prune-coordination.ts", "scripts/check-phase156-prune-coordination.test.ts", "scripts/verify.sh"]) {
      if (!row.evidence.includes(file)) failures.push(`parity surface evidence missing: ${file}`);
    }
    for (const file of SOURCES) if (!row.upstream.sources.includes(file)) failures.push(`parity source anchor missing: ${file}`);
    for (const file of BASELINE_TESTS) if (!row.upstream.tests.includes(file)) failures.push(`parity test anchor missing: ${file}`);
    if (!row.known_gaps.some(gap => gap.includes("remain deferred"))) failures.push("parity surface missing deferred scope");
    checkStringClaims(row, "parity surface", failures);
  } catch { failures.push("parity index: invalid Phase 156 contract"); }
}

function checkBreadcrumbs(source: string, text: (file: string) => string, failures: string[]): void {
  try {
    const manifest = JSON.parse(source) as { groups: { files: string[]; breadcrumbs: string[] }[] };
    // Central pre-existing registries can have an intentional none mapping;
    // every selected implementation and behavioral-test file needs actual roots.
    for (const file of new Set([...CONTRACTS.flatMap(row => [row.file, row.testFile]), ...EXTRA.map(([file]) => file)])) {
      const matching = manifest.groups.filter(group => group.files.includes(file));
      if (matching.length !== 1 || !matching[0].breadcrumbs.length) {
        failures.push(`breadcrumb manifest must register exactly one mapped group: ${file}`); continue;
      }
      for (const anchor of matching[0].breadcrumbs) if (!text(file).includes(`// - ${anchor}`)) failures.push(`${file}: missing source breadcrumb: ${anchor}`);
    }
  } catch { failures.push("breadcrumb manifest: invalid JSON contract"); }
}

function checkDocumentation(text: (file: string) => string, failures: string[]): void {
  const current = text(DOC).split(HEADING)[1] ?? "";
  const required = ["CFPR-01", "Commit", "ChainStateFlushed", "Stop", "SetBestBlockIndex", "heights 0/1",
    "same-generation", "record-only", "Disabled", "SyncAll", "logical", "550 MiB", "codec-valid",
    "coinbase maturity one", "software", "hardware", "full native gate", "remain deferred",
    "stale durable metadata", "generic paired-unlink no-op", "support-summary crash undercount",
    "Structural checks supplement", "schema-2", "11-byte", "legacy Active generation zero"];
  for (const anchor of required) if (!current.toLowerCase().includes(anchor.toLowerCase())) failures.push(`T-156-26: current scoped documentation missing: ${anchor}`);
  checkClaims(current, DOC, failures);
  const readme = text("README.md").split("### Internal index-owned prune coordination (Phase 156)")[1]?.split("\n## ")[0] ?? "";
  if (!readme.includes("CFPR-01") || !readme.includes("full native gate")) failures.push("T-156-26: README missing current Phase 156 scope/evidence limit");
  checkClaims(readme, "README.md", failures);
  if (!text("docs/parity/checklist.md").includes(SURFACE)) failures.push("T-156-26: checklist missing Phase 156 parity owner");
  for (const [file, symbols] of [[SOURCES[0], ["BaseIndex::Commit", "BaseIndex::ChainStateFlushed", "BaseIndex::Stop", "BaseIndex::SetBestBlockIndex"]],
    [SOURCES[1], ["BlockFilterIndex::CustomCommit", "BlockFilterIndex::CustomRewind"]],
    [SOURCES[2], ["BlockManager::FindFilesToPrune", "BlockManager::FindFilesToPruneManual"]]] as const) {
    for (const symbol of symbols) if (!rustCode(text(file)).includes(symbol)) failures.push(`pinned upstream source missing: ${file}:${symbol}`);
  }
  for (const file of BASELINE_TESTS) {
    if (!text(file).includes("def run_test(self)") || !text(file).includes("pruneblockchain(")) failures.push(`pinned upstream prune test missing concrete anchors: ${file}`);
  }
}

function checkClaims(source: string, file: string, failures: string[]): void {
  const subjects = /(?:public (?:BASIC |filter index |filter )?activation|filter index activation|scheduled catch-up|(?:filter )?RPC serving|peer serving|runtime reorg|operator projections|complete client-after-prune proof|production readiness|production-funds use|hardware power-loss proof|full native (?:verifier|verification|gate))/i;
  const positive = /\b(?:is|are)\s+(?:enabled|supported|available|active|ready|shipped|safe|established|complete)|\b(?:passed|succeeded)\b/i;
  for (const sentence of source.split(/(?<=[.!?])\s+|\n|;|\bbut\b|\bhowever\b|\band\b|\bwhile\b/)) {
    const maybeSubject = subjects.exec(sentence);
    if (!maybeSubject || !positive.test(sentence.slice(maybeSubject.index))) continue;
    if (/\bPhase 15[45](?:'s)?\b/.test(sentence) && !/\bPhase 156\b/.test(sentence)) continue;
    const prefix = sentence.slice(0, maybeSubject.index);
    if (/\b(?:does not|do not|no claim|not a|future phase)\b/i.test(prefix)) continue;
    failures.push(`T-156-26: unsupported current claim in ${file}`);
  }
}

function checkStringClaims(value: unknown, label: string, failures: string[]): void {
  if (typeof value === "string") { checkClaims(value, label, failures); return; }
  if (value === null || typeof value !== "object") return;
  for (const [key, child] of Object.entries(value)) checkStringClaims(child, `${label}.${key}`, failures);
}

function checkVerifier(source: string, failures: string[]): void {
  const steps = source.split("\n").filter(line => line.startsWith("run_step "));
  const commands = ["bun test ./scripts/check-phase155-filter-index.test.ts", "bun run scripts/check-phase155-filter-index.ts",
    "bun test ./scripts/check-phase156-prune-coordination.test.ts", "bun run scripts/check-phase156-prune-coordination.ts"];
  let cursor = -1;
  for (const command of commands) {
    const next = steps.findIndex(line => line.endsWith(command));
    if (next <= cursor) failures.push(`T-156-27: default verifier missing/out-of-order executable step: ${command}`);
    cursor = Math.max(cursor, next);
  }
}

if (import.meta.main) {
  const failures = checkPhase156PruneCoordination();
  if (failures.length) {
    console.error("Phase 156 prune coordination evidence check failed:");
    for (const failure of failures.slice(0, 40)) console.error(`- ${failure}`);
    if (failures.length > 40) console.error(`- ${failures.length - 40} additional source contract failures`);
    process.exit(1);
  }
  console.log("Phase 156 production boundaries, named behavioral evidence, parity and scoped claims validated; runtime/native proof remains required.");
}

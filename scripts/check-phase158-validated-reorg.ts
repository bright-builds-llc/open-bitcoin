#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { compact, hasAssertions, hasOrderedCode, maybeFunction, ordinaryRust, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";

const NODE = "packages/open-bitcoin-node/src/";
const CORE = "packages/open-bitcoin-chainstate/src/";
export const PHASE = ".planning/phases/158-validated-reorg-and-retained-branch-identity/";
export const DOC = "docs/parity/v2-5-validated-reorg.md";
export const SURFACE = "v2-5-validated-reorg-and-retained-branch-identity";
const DRIVER = NODE + "network/runtime_authority/filter_index/catch_up.rs";
const BRIDGE = NODE + "chainstate/fjall_store/reorg.rs";
const APPEND = NODE + "storage/fjall_store/filters/append.rs";
const REORG = NODE + "storage/fjall_store/filters/reorg.rs";
const PREFLIGHT = NODE + "chainstate/filter_reorg/preflight.rs";
const TEST = NODE + "sync/tests/filter_index/reorg/";
type Contract = { file: string; symbol: string; anchors: string[]; maybeTest?: boolean; maybeImpl?: string };
const row = (file: string, symbol: string, anchors: string[], maybeTest?: boolean, maybeImpl?: string): Contract => ({ file, symbol, anchors, maybeTest, maybeImpl });

/** Fixed actual callers and guarded callees; lexical checks supplement executed Rust behavior. */
export const CONTRACTS: Contract[] = [
  row(CORE + "engine/stage.rs", "stage_reorg", ["disconnect_blocks", "apply_connect_on_overlay(", "Ok(StagedChainstateReorg"]),
  row(CORE + "engine/stage.rs", "absorb_staged_reorg_with_receipt", ["self.absorb_overlay(overlay, maybe_best_block)", "self.active_chain = next_active_chain", "self.undo_by_block = next_undo_by_block", "let receipt = AcceptedChainstateReorg", "(transition, receipt)"]),
  row(NODE + "network/runtime_authority.rs", "reorg_to_branch", ["self.try_mutate", "network.reorg_to_branch("]),
  row(NODE + "network/mempool_lifecycle.rs", "reorg_to_branch", ["filter_index::reorg::prepare_reorg(", "install_prepared_reorg_preview(&prepared_chainstate)?", "apply_reorg_mempool_lifecycle(", "commit_prepared_reorg(prepared_chainstate)"]),
  row(NODE + "network/runtime_authority/filter_index/reorg.rs", "prepare_reorg", ["manager.prepare_reorg(disconnect, replacements, flags, params)"]),
  row(NODE + "chainstate.rs", "prepare_reorg", ["self.chainstate.stage_reorg(", "prepare_basic_index_reorg(&staged, replacement_branch)?", "Ok(PreparedChainstateReorg"]),
  row(NODE + "chainstate.rs", "install_prepared_reorg_preview", ["self.freeze_basic_index_reorg(prepared)", "install_staged_reorg_preview(&prepared.staged)"]),
  row(NODE + "chainstate/validation_history.rs", "commit_prepared_reorg", ["check_basic_index_reorg(&prepared)", "absorb_staged_reorg_with_receipt(prepared.staged)", "accept_basic_index_reorg(accepted, prepared.maybe_index)", "note_basic_index_failure(", "self.persist()"]),
  row(NODE + "chainstate/filter_reorg.rs", "prepare_basic_index_reorg", ["lineage.prepare_reorg_storage(staged)", "preflight_required_sources(", "replace_validated_branch(BasicIndexReplacementFacts", "AcceptedBasicFilterFacts::capture_reorg(", "checked_add(preflight_work)"]),
  row(NODE + "chainstate/filter_reorg.rs", "freeze_basic_index_reorg", ["index.validate_work_bound()?", "lineage.freeze_index_reorg(&index.storage)?", "self.invalidate_basic_index_owner()", "BasicIndexReorgState::PreviewFrozen"]),
  row(NODE + "chainstate/filter_reorg.rs", "accept_basic_index_reorg", ["self.invalidate_basic_index_owner()", "BasicIndexReorgState::AcceptedReplacement", "accepted.maybe_new_endpoint()", "lineage.complete_index_reorg(&accepted, index.storage)?", "owner.progress = trial"]),
  row(BRIDGE, "freeze_index_reorg", ["self.check_index_reorg(prepared)?", "self.store.suspend_basic_filter_reorg(prepared)?", "self.preview_frozen = true"]),
  row(BRIDGE, "complete_index_reorg", ["self.authorize_reorg(accepted, &prepared)?", "complete_basic_filter_reorg(prepared, authorization)?", "self.confirm_reorg(&achieved)"]),
  row(BRIDGE, "authorize_reorg", ["!prepared.belongs_to(&self.store)", "accepted.maybe_old_endpoint() != self.maybe_accepted", "accepted.maybe_new_endpoint() != prepared.maybe_new_endpoint()", "accepted.maybe_common_ancestor_endpoint() != prepared.maybe_ancestor_endpoint()", "self.maybe_accepted = accepted.maybe_new_endpoint()", "self.maybe_pending_reorg = Some("]),
  row(BRIDGE, "authorize_append_positions", ["self.preview_frozen", "positions.last().map(|p| (p.height, p.block_hash)) != self.maybe_accepted", "proof.generation(), proof.branch_identity()", "proof.processed().prefix()", ".get(first..end)", "Ok(ValidatedBasicFilterAppendPositions"]),
  row(BRIDGE, "validate_for", ["!self.store.shares_basic_filter_store(store)", "proof.generation()", "proof.branch_identity()", "proof.revision()", "self.processed != proof.processed()", "self.positions.len() != records.len()", "self.positions.iter().zip(records)", "position.height != id.height()", "position.block_hash != id.block_hash()", "position.previous_block_hash() != id.parent_hash()"], false, "ValidatedBasicFilterAppendPositions<'_>"),
  row(BRIDGE, "validate_for", ["!self.store.shares_basic_filter_store(store)", "!prepared.belongs_to(store)", "!self.facts.matches(prepared)"], false, "ValidatedBasicFilterReorg"),
  row(NODE + "chainstate/filter_index.rs", "authorize_basic_filter_append_positions", ["lineage.authorize_append_positions(", "proof.admit_work(positions.work())?", "positions.validate_for(self.store.inner(), proof, records)?"]),
  row(DRIVER, "drive_basic_filter_index_turn", ["self.drive_basic_filter_index_turn_with_budget(", "production_budget().map_err(lifecycle_error)?"]),
  row(DRIVER, "drive_turn", ["proof.generation() != progress.generation()", "proof.branch_identity() != progress.branch_identity()", "generate_records(", "if records.is_empty()", "store.prepare_basic_filter_append(proof, &records)?", "manager.authorize_basic_filter_append_positions(&proof, &records)?", "store.prepare_basic_filter_replacement_append(proof, positions, &records)?", "store.complete_basic_filter_append(prepared)?", "confirm_basic_index_checkpoint("]),
  row(APPEND, "prepare_basic_filter_append_guarded_positions", ["check_envelope_bounds(records)?", "let replacement = maybe_positions.is_some()", "BasicFilterProjectionAuthority::ValidatedReorg", "&& !replacement", "positions.validate_for(self, &proof, records)?", "verify_filter_record_predecessor(", "if existing != bytes", "!replacement", "proof_identity_is_recovered(&prepared.proof)", "&& !(replacement && maybe_next == Some(id.height()))"]),
  row(APPEND, "proof_identity_is_recovered", ["proof.identity.projection_authority", "BasicFilterProjectionAuthority::RecoveredPrefix"]),
  row(APPEND, "earned_basic_filter_safe_checkpoint", ["prepared.proof.durable_tip()", "prepared.proof.durable_displaced()", "return Ok(prepared.safe_checkpoint)", "processed.height() < height", "endpoint.height() != height || endpoint.block_hash() != hash"]),
  row(APPEND, "durable_displaced", ["self.identity.maybe_replacement.is_some_and(", "replacement.maybe_displaced_fence == Some(self.durable_tip())"]),
  row(REORG, "prepare_basic_filter_reorg", ["self.shared_basic_filter_prefix(previous.processed, staged, &mut work)?", "self.shared_basic_filter_prefix(previous.safe_checkpoint, staged, &mut work)?", "stronger_protection(", ".checked_add(1)", "BasicFilterProjectionAuthority::ValidatedReorg", "maybe_displaced_fence: staged"]),
  row(REORG, "complete_basic_filter_reorg", ["authorization.validate_for(self, &prepared)?", "self.filter_publication_guard()?", "control.revision != prepared.previous.revision", "CoinsView::best_block(&view)", "SyncAll", "codec::STATE_KEY", "codec::ownership::OWNER_KEY", "PRUNE_LOCKS_KEY", "self.finish_basic_filter_batch("]),
  row(PREFLIGHT, "required_sources", ["production_budget()?", "staged.transition().disconnected", "historical_source(", "maybe_basic_filter_reusable_record(", "historical_source(", "staged.maybe_replacement_undo", "BasicFilterInputs::from_historical("]),
  row(PREFLIGHT, "historical_source", ["maybe_basic_filter_turn_body(", "old_undo", "basic_filter_required_undo(", "BasicFilterInputs::from_historical("]),
  row(PREFLIGHT, "charge", ["work.checked_add(cost)", "!total.fits(maximum)", "return Err(", "*work = total"]),
  row(NODE + "storage/fjall_store/filters/turn_inputs.rs", "maybe_basic_filter_turn_body", ["self.block_index", ".get(", "if bytes.len() > 4_000_000", "remaining_body_bytes", "if !admit(work)?", "body_work(bytes.as_ref())?", "parse_block(bytes.as_ref())"]),
  row(NODE + "storage/fjall_store/filters/turn_inputs/undo.rs", "basic_filter_required_undo", ["self.chainstate", ".get(", "undo_key(hash)", "let len = bytes.len() as u64", "charge_append_work(", "undo_bytes: len", "decode_block_undo(bytes.as_ref())", "if &undo != expected", "return Err("]),
  row(NODE + "storage/fjall_store/filters/ownership/proofs.rs", "maybe_basic_filter_append_proof_with_limits", ["self.filter_publication_guard()?", "control.maybe_reorg_suspension.is_some()", "self.maybe_basic_filter_reorg_proof_guarded("]),
  row(NODE + "storage/fjall_store/filters/ownership.rs", "install_recovered_basic_filter_append_guarded", ["owner.checkpoint().checkpoint()", "control.maybe_append_identity = Some(", "processed: checkpoint", "safe_checkpoint: checkpoint", "BasicFilterProjectionAuthority::RecoveredPrefix"]),
  row(NODE + "storage/fjall_store/filters.rs", "maybe_active_basic_filter_record", ["self.maybe_basic_filter_checkpoint()?", "if height > endpoint.height()", "return Ok(None)", "self.basic_filter_projection(height)?", "self.load_basic_filter_record(hash)?"]),
  row(NODE + "storage/fjall_store/filters.rs", "load_basic_filter_record", ["codec::record_key(hash)", "codec::record_parent(", "while height != 0", "ancestors.into_iter().rev()", "codec::decode_record("]),
  row(NODE + "chainstate.rs", "flush_applying_plan", ["lineage.maybe_prepare(self.chainstate.tip())", "execute_flush_applying_plan(", "execution.wrote_coins", "pending", ".complete(self.maybe_validated_lineage.as_ref(), self.chainstate.tip())?", "self.store.confirm_validated_flush(completed)?", "self.note_basic_index_reorg_durable()"]),
  row(NODE + "chainstate/flush_lifecycle.rs", "complete_coins_write", ["coins_write_kind(decision)", "wrote_coins: false", "cache.flush()", "cache.sync()", "sink.persist_chain_meta(active_chain)?", "wrote_coins: true"]),
  row(NODE + "chainstate/fjall_store.rs", "complete", ["!self.store.shares_basic_filter_store(&lineage.store)", "Some(self.accepted) != lineage.maybe_accepted", "lineage.preview_frozen", "self.proof.generation() != lineage.generation", "self.proof.branch_identity() != lineage.branch", "Ok(CompletedValidatedFlush"]),
  row(NODE + "storage/fjall_store/filters/ownership.rs", "confirm_basic_filter_flush", ["control.maybe_reorg_suspension.is_some()", "!completed.belongs_to(self)", "pending.coins.previous != work.identity", "!pending.coins.completed", "work.identity.revision.checked_add(2) != Some(control.revision)", "completed.metadata_revision() != control.revision", "completed.accepted_endpoint()", "view.best_block()", "replacement.maybe_displaced_fence = None", "control.maybe_append_identity = Some(identity)"]),
  row(NODE + "storage/fjall_store/prune.rs", "commit_paired_delete_observing", ["self.filter_publication_guard()?", "self.check_prune_candidate_guarded(", "self.with_payload_mutation(|| self.commit_paired_delete_guarded("]),
  row(NODE + "storage/fjall_store/prune.rs", "check_prune_candidate_guarded", ["self.maybe_basic_filter_owner_guarded(control)?", "self.maybe_owned_prune_ancestry(maybe_owner)?", "check_owned_prune_candidate(", "height_forbidden_by_any_lock(intent.height, &self.load_prune_locks()?)"]),
  row(NODE + "storage/fjall_store/prune.rs", "resume_prune_intent", ["store.filter_publication_guard()", "maybe_basic_filter_owner_guarded(&control)", "check_owned_prune_candidate(", "store.load_prune_locks()", "ensure_intent_may_finish(", "finish_intent(store, intent)"]),
  row(NODE + "storage/fjall_store/prune/records.rs", "sync_prune_locks", ["self.filter_publication_guard()?", "self.load_prune_locks()?", "lock.name == BASIC_INDEX_PRUNE_LOCK", "if maybe_current != maybe_proposed", "return Err(", "self.sync_block_index_value("]),
  row(CORE + "filter_index/catch_up/reorg.rs", "replace_validated_branch", ["self.generation != facts.expected_generation", "self.branch_identity != facts.expected_branch_identity", "self.accepted_target != facts.old_target", ".checked_add(1)", "facts.protection", "Ok(Self {"]),
  row(TEST + "fixtures.rs", "reopen_inspecting", ["drop(runtime)", "inspect(&history.path)", "configured(FjallNodeStore::open(&history.path)", "runtime.consensus_params.coinbase_maturity = 1"], true),
  row(TEST + "failures.rs", "finish_recovered", ["fixture.reopen_inspecting(", "snapshot_index(path)", "codec::encode_record(record)", "drive_basic_filter_index_turn()"], true),
];

export const TESTS: [string, string][] = [
  ...["equal_height_index_before_ancestor", "equal_height_index_within_displaced_suffix", "equal_height_index_at_old_tip", "longer_replacement", "shorter_replacement", "shorter_unflushed_return_preserves_coins_fence_and_reopens", "disconnect_only_to_common_ancestor", "full_disconnect_has_empty_checkpoint_and_no_fabricated_target", "stale_owned_turn_cannot_publish", "return_to_original_then_subsequent_stored_connect", "actual_default_maturity_spends_flush_and_reopen", "sync_reconciliation_and_ordinary_live_resume"].map(name => [TEST + "branches.rs", "phase158_validated_reorg_" + name] as [string, string]),
  ...["displaced_exact_records_survive_real_reopen", "pruned_shared_source_survives_repeated_indexed_branches", "missing_consensus_body_refuses_despite_indexed_record", "missing_consensus_undo_refuses_despite_indexed_record", "continuous_startup_protection_precedes_interrupted_prune"].map(name => [TEST + "retention.rs", "phase158_validated_reorg_" + name] as [string, string]),
  ...["configured_reopen_index_ahead_of_old_coins_rebinds_conflicting_rows", "configured_reopen_rewinds_checkpoint_with_conflicting_suffix", "configured_reopen_new_coins_partial_replacement_projection", "deep_required_body_and_undo_loss_precedes_preview_and_mempool", "preview_mempool_refusal_has_no_accepted_receipt", "accepted_body_undo_coins_metadata_boundaries_reopen_truthfully", "rewind_publication_faults_preserve_actual_accepted_tip_and_reopen", "first_and_subsequent_append_faults_reopen_old_canonical_branch"].map(name => [TEST + "failures.rs", "phase158_reorg_failure_" + name] as [string, string]),
  ...["interrupted_intent_refuses_after_reorg_before_deletion", "reserved_crud_and_ifneeded_preserve_unflushed_coins", "preplanned_manual_required_source_skips_after_generation_change", "automatic_prune_remeasures_new_generation_and_earns_own_flush"].map(name => [TEST + "protection.rs", "phase158_reorg_protection_" + name] as [string, string]),
  ...["fixed_depth_complete_preparation_and_turns_do_not_scan_prefix", "exact_and_one_under_record_acquisition_preserve_effects", "retained_over_budget_gap_refuses_before_preview", "absolute_acquisition_exhaustion_refuses_before_effects"].map(name => [TEST + "measurements.rs", "phase158_reorg_measurement_" + name] as [string, string]),
];
const GUARDS = ["scripts/check-phase158-validated-reorg.ts", "scripts/check-phase158-validated-reorg.test.ts",
  "scripts/check-phase157-index-catch-up/rust-evidence.ts", "scripts/check-phase156-prune-coordination/rust-evidence.ts"];
const UPSTREAM = ["index/base.cpp", "index/blockfilterindex.cpp", "validation.cpp", "undo.h"].map(file => "packages/bitcoin-knots/src/" + file);
export const REGISTRATIONS: [string, string][] = [[NODE + "sync/tests/filter_index.rs", "mod reorg;"],
  ...["branches", "retention", "failures", "protection", "measurements"].map(name => [NODE + "sync/tests/filter_index/reorg.rs", `mod ${name};`] as [string, string])];
const SEALED: [string, string][] = [[CORE + "engine.rs", "StagedChainstateReorg"], [CORE + "engine.rs", "AcceptedChainstateReorg"],
  [BRIDGE, "ValidatedBasicFilterReorg"], [BRIDGE, "ValidatedBasicFilterAppendPositions"], [NODE + "storage/fjall_store/filters/ownership.rs", "BasicFilterAppendProof"]];
export const CHECK_FILES = [...new Set([...CONTRACTS.map(row => row.file), ...TESTS.map(([file]) => file), ...REGISTRATIONS.map(([file]) => file),
  ...SEALED.map(([file]) => file), ...GUARDS, ...UPSTREAM, DOC, "README.md", "packages/README.md", "docs/parity/index.json",
  "docs/parity/source-breadcrumbs.json", "scripts/verify.sh", PHASE + "158-UAT.md", PHASE + "158-REORG-MEASUREMENTS.md",
  ...["01", "02", "03", "04", "05", "06"].map(plan => PHASE + `158-${plan}-SUMMARY.md`)])];

/** Select one actual impl when the same method name occurs on distinct sealed types. */
export function maybeBoundary(source: string, contract: Contract): ReturnType<typeof maybeFunction> {
  if (!contract.maybeImpl) return maybeFunction(source, contract.symbol, !contract.maybeTest);
  const code = ordinaryRust(source);
  const marker = `impl ${contract.maybeImpl} {`;
  const start = code.indexOf(marker);
  if (start < 0 || code.indexOf(marker, start + marker.length) >= 0) return undefined;
  const next = code.indexOf("\nimpl ", start + marker.length);
  const maybeBody = maybeFunction(source.slice(start, next < 0 ? undefined : next), contract.symbol, true);
  return maybeBody ? { ...maybeBody, start: maybeBody.start + start, end: maybeBody.end + start } : undefined;
}

/** Inspect bounded current roots without archive discovery or future completion prerequisites. */
export function checkPhase158ValidatedReorg(maybeRoot?: string): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = new Map<string, string>();
  for (const file of CHECK_FILES) {
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: cannot read required evidence`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  for (const contract of CONTRACTS) {
    const maybeBody = maybeBoundary(text(contract.file), contract);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, contract.anchors)) failures.push(`T-158-07-01: actual boundary ${contract.file}:${contract.symbol}`);
  }
  for (const [file, name] of TESTS) {
    const maybeTest = maybeFunction(text(file), name);
    if (!maybeTest || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeTest.attributes)
      || /#\[\s*ignore\b/.test(maybeTest.attributes) || !hasAssertions(text(file), maybeTest.body)) failures.push(`T-158-07-01: executable behavior ${file}:${name}`);
  }
  for (const [file, module] of REGISTRATIONS) if (!compact(rustCode(text(file))).includes(compact(module))) failures.push(`T-158-07-01: module registration ${file}:${module}`);
  for (const [file, symbol] of SEALED) {
    const maybeFields = ordinaryRust(text(file)).match(new RegExp(`struct ${symbol}(?:<'a>)?\\s*\\{([^}]+)\\}`))?.[1];
    const maybePrivateFields = symbol === "BasicFilterAppendProof"
      ? maybeFields?.replace(/\bpub\s*\(\s*super\s*\)\s+identity\s*:/g, "identity:")
      : maybeFields;
    if (!maybePrivateFields || /\bpub\b/.test(maybePrivateFields)) failures.push(`T-158-07-01: private ${symbol} fields required`);
  }
  for (const file of [CORE + "engine/stage.rs", CORE + "filter_index/catch_up/reorg.rs"]) {
    if (/std::(?:fs|io|net|time|process)|(?:fjall|tokio)::|SystemTime/.test(ordinaryRust(text(file)))) failures.push(`T-158-07-01: pure core effects ${file}`);
  }
  checkBreadcrumbs(text, failures);
  checkDocuments(text, failures);
  checkParity(text, failures);
  checkVerifier(text("scripts/verify.sh"), failures);
  return [...new Set(failures)];
}

function checkBreadcrumbs(text: (file: string) => string, failures: string[]): void {
  try {
    const manifest = JSON.parse(text("docs/parity/source-breadcrumbs.json")) as { groups: { files: string[]; breadcrumbs: string[] }[] };
    for (const file of CHECK_FILES.filter(file => file.endsWith(".rs") && !file.includes("bitcoin-knots"))) {
      const matches = manifest.groups.filter(group => group.files.includes(file));
      if (matches.length !== 1 || matches[0].breadcrumbs.length === 0) { failures.push(`T-158-07-01: breadcrumb mapping ${file}`); continue; }
      for (const anchor of matches[0].breadcrumbs) if (!text(file).includes(`// - ${anchor}`)) failures.push(`T-158-07-01: breadcrumb comment ${file}`);
    }
  } catch { failures.push("T-158-07-01: breadcrumb manifest invalid"); }
}
function checkDocuments(text: (file: string) => string, failures: string[]): void {
  for (const anchor of ["Intentional local resource-policy difference", "54 preparation configurations", "149 complete ordinary turns", "maturity 100", "210,000", "Knots regtest 150", "NoTip", "independent source/security", "Required retained body", "1,025", "not shipped"]) {
    if (!text(DOC).includes(anchor)) failures.push(`T-158-07-01: scoped documentation missing ${anchor}`);
  }
  for (const anchor of ["54 concrete Fjall configurations and 149 ordinary bounded turns", "not isolated new-index timings", "401-block history", "98 record operations and 56 point reads"]) {
    if (!text(PHASE + "158-REORG-MEASUREMENTS.md").includes(anchor)) failures.push(`T-158-07-01: current measurement missing ${anchor}`);
  }
  for (const plan of ["01", "02", "03", "04", "05", "06"]) {
    const summary = text(PHASE + `158-${plan}-SUMMARY.md`);
    if (!summary.includes("phase_lifecycle_id: 158-2026-10-08T02-17-15") || !summary.includes("## Self-Check: PASSED")) failures.push(`T-158-07-01: current lifecycle summary ${plan}`);
  }
  for (const file of [DOC, "README.md", "packages/README.md", PHASE + "158-UAT.md"]) checkClaims(text(file), file, failures);
  for (const file of ["README.md", "packages/README.md"]) if (!text(file).includes("v2-5-validated-reorg.md")) failures.push(`T-158-07-01: contributor evidence link ${file}`);
  const uat = text(PHASE + "158-UAT.md");
  for (const anchor of ["scripts/command-timings.ts", "cargo test --manifest-path packages/Cargo.toml -p open-bitcoin-node --lib phase158_", "cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --", "bazel run //packages/open-bitcoin-cli:open_bitcoin --", "--help", "158-REORG-MEASUREMENTS.md"]) {
    if (!uat.includes(anchor)) failures.push(`T-158-07-01: repo-local UAT missing ${anchor}`);
  }
}
function checkClaims(source: string, file: string, failures: string[]): void {
  const subjects = /(?:peer (?:filter )?serving|filter(?:\/index)? RPC|RPC (?:filter )?serving|public serving defaults|production readiness|production[- ]funds (?:use|safety)|hardware power-loss proof|v2\.5|arbitrary retained forks|whole-runtime constant (?:memory|work)|isolated publication latency|deep reorg support)/gi;
  const positive = /\b(?:is|are|has|have)\s+(?:(?:now|fully|already|successfully|generally|unconditionally)\s+)*(?:enabled|supported|available|active|ready|shipped|safe|established|complete|guaranteed)|\b(?:passed|succeeded)\b/i;
  for (const clause of source.split(/(?<=[.!?])\s+|\n|;|\bbut\b|\bhowever\b|\band\b|\bwhile\b/)) {
    const matches = [...clause.matchAll(subjects)];
    for (const [index, subject] of matches.entries()) {
      const end = matches[index + 1]?.index ?? clause.length;
      const previous = index === 0 ? 0 : matches[index - 1].index + matches[index - 1][0].length;
      if (!positive.test(clause.slice(subject.index, end))) continue;
      if (/\b(?:does not|do not|no claim|not a|future phase)\b/i.test(clause.slice(previous, subject.index))) continue;
      failures.push(`T-158-07-01: unsupported claim in ${file}`);
    }
  }
}
function checkParity(text: (file: string) => string, failures: string[]): void {
  try {
    const index = JSON.parse(text("docs/parity/index.json")) as { baseline: string; checklist: { surfaces: { id: string; status: string; requirements: string[]; evidence: string[]; upstream: { sources: string[] }; intentional_differences: string[]; known_gaps: string[] }[] } };
    const owners = index.checklist.surfaces.filter(row => row.requirements.includes("CFIX-03"));
    if (index.baseline !== "29.3.knots20260210" || owners.length !== 1 || owners[0].id !== SURFACE) { failures.push("T-158-07-01: unique pinned CFIX-03 parity owner"); return; }
    const surface = owners[0];
    if (!["in_progress", "done"].includes(surface.status) || surface.requirements.length !== 1) failures.push("T-158-07-01: parity scope/status");
    for (const file of [DOC, ...new Set(CONTRACTS.map(row => row.file)), ...new Set(TESTS.map(([file]) => file)), PHASE + "158-UAT.md", PHASE + "158-REORG-MEASUREMENTS.md", ...GUARDS.slice(0, 2)]) {
      if (!surface.evidence.includes(file)) failures.push(`T-158-07-01: parity evidence ${file}`);
    }
    for (const file of UPSTREAM) if (!surface.upstream.sources.includes(file)) failures.push(`T-158-07-01: pinned upstream ${file}`);
    if (!surface.intentional_differences.some(item => item.includes("resource-policy") && item.includes("fully retained"))) failures.push("T-158-07-01: intentional finite resource-policy difference");
    if (!surface.known_gaps.some(item => item.includes("159") && item.includes("162") && item.includes("pending"))) failures.push("T-158-07-01: deferred serving boundary");
    const inspect = (value: unknown): void => {
      if (typeof value === "string") { checkClaims(value, "parity surface", failures); return; }
      if (value !== null && typeof value === "object") for (const child of Object.values(value)) inspect(child);
    };
    inspect(surface);
  } catch { failures.push("T-158-07-01: parity manifest invalid"); }
}
function checkVerifier(source: string, failures: string[]): void {
  const documented = source.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "";
  const executed = source.split("\n").filter(line => line.startsWith("run_step ")).join("\n");
  const commands = ["bun run scripts/check-phase157-index-catch-up.ts", "bun test ./scripts/check-phase158-validated-reorg.test.ts", "bun run scripts/check-phase158-validated-reorg.ts", "bun test scripts/check-phase121-block-relay-metrics-log-runtime.test.ts"];
  for (const [label, section] of [["documented", documented], ["executed", executed]]) {
    const lines = section.split("\n").filter(line => commands.some(command => line.endsWith(command)));
    if (lines.length !== commands.length || lines.some((line, index) => !line.endsWith(commands[index]))) failures.push(`T-158-07-03: ordered ${label} verifier routes`);
  }
}
if (import.meta.main) {
  const failures = checkPhase158ValidatedReorg();
  if (failures.length) {
    console.error("Phase 158 current source and claim check failed:");
    for (const failure of failures.slice(0, 40)) console.error(`- ${failure}`);
    if (failures.length > 40) console.error(`- ${failures.length - 40} additional bounded findings`);
    process.exit(1);
  }
  console.log("Phase 158 actual acceptance, source admission, recovered projection, immutable history, own-flush and scoped claim routes checked; executed native and independent final gates remain required.");
}

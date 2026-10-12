#!/usr/bin/env bun
import { existsSync, readFileSync } from "node:fs";
import { createHash } from "node:crypto";
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

export const PHASE162 = ".planning/phases/162-real-prune-retention-and-integrated-parity-proof/";
export const PHASE162_DOC = "docs/parity/v2-5-integrated-filter-proof.md";
export const PHASE162_IMPLEMENTATION = `Phase 162 scoped implementation evidence is available in ${PHASE162_DOC}; CFPR-02/CFGR-01/CFGR-02 remain Pending; v2.5 remains unaudited and unshipped.`;
export const PHASE162_COMPLETE = `Phase 162 scoped integrated proof is complete in ${PHASE162_DOC}; CFPR-02/CFGR-01/CFGR-02 are Complete; v2.5 remains unaudited and unshipped.`;
const P162_RPC = "packages/open-bitcoin-rpc/src/bin/open_bitcoind/";
export const PHASE162_EVIDENCE = [P162_RPC + "tests/filter_index/phase162.rs", P162_RPC + "tests/filter_index/phase162/retention.rs",
  P162_RPC + "tests/filter_index/phase162/fixtures.rs", P162_RPC + "coins_flush/tests/phase162.rs",
  "packages/testdata/phase162/recipe.rs", "scripts/phase162-filter-oracle.ts"];
export const PHASE162_FILES = [".planning/REQUIREMENTS.md", PHASE162 + "162-CONTEXT.md"];
const FORMAL162 = [["VERIFICATION", "passed", "gsd-verifier"], ["NATIVE-EVIDENCE", "passed", "gsd-execute-phase"],
  ["REVIEW", "clean", "gsd-code-reviewer"], ["SECURITY", "verified", "gsd-security-auditor"]] as const;
type ClaimState = "pending" | "implementation" | "complete" | "invalid";
type Text = (file: string) => string;

/** Future output is optional at bootstrap and read only from these fixed paths. */
export function loadPhase162OptionalSources(root: string): Map<string, string> {
  const sources = new Map<string, string>();
  if (!existsSync(resolve(root, PHASE162_DOC)) || !existsSync(resolve(root, ".planning/REQUIREMENTS.md"))) return sources;
  const requirements = readFileSync(resolve(root, ".planning/REQUIREMENTS.md"), "utf8");
  const complete = ["CFPR-02", "CFGR-01", "CFGR-02"].every(id => new RegExp(`^- \\[x\\] \\*\\*${id}\\*\\*:`, "mi").test(requirements));
  const files = [PHASE162_DOC, ...PHASE162_EVIDENCE, ...(complete ? FORMAL162.map(([name]) => PHASE162 + `162-${name}.md`) : [])];
  for (const file of files) {
    if (existsSync(resolve(root, file))) sources.set(file, readFileSync(resolve(root, file), "utf8"));
  }
  return sources;
}

function metadata(source: string, key: string): string {
  const maybeHeader = source.match(/^---\r?\n([\s\S]*?)\r?\n---(?:\r?\n|$)/)?.[1];
  const values = [...(maybeHeader ?? "").matchAll(new RegExp(`^${key}:\\s*["']?([^\\n"']+?)["']?\\s*$`, "gm"))];
  return values.length === 1 ? values[0][1] : "";
}

/** Claim publication is source-bound; this supplements runtime and independent review evidence. */
export function checkPhase162ClaimState(text: Text, failures: string[]): ClaimState {
  const fail = (reason: string): ClaimState => { failures.push(`T-162-10: ${reason}`); return "invalid"; };
  let owner;
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const owners = index.checklist.surfaces.filter((row: { requirements: string[] }) => row.requirements.some(id => ["CFPR-02", "CFGR-01", "CFGR-02"].includes(id)));
    owner = owners[0];
    if (index.baseline !== "29.3.knots20260210" || owners.length !== 1 || owner.id !== "v2-5-integrated-retained-filter-proof"
      || JSON.stringify(owner.requirements) !== '["CFPR-02","CFGR-01","CFGR-02"]' || !["in_progress", "done"].includes(owner.status)) return fail("unique scoped Phase 162 parity owner");
    for (const file of PHASE162_EVIDENCE) if (!owner.evidence.includes(file)) return fail(`registered Phase 162 evidence ${file}`);
  } catch { return fail("invalid Phase 162 parity ownership"); }
  const requirements = ["CFPR-02", "CFGR-01", "CFGR-02"].map(id => {
    const source = text(".planning/REQUIREMENTS.md");
    const boxes = [...source.matchAll(new RegExp(`^- \\[([ xX])\\] \\*\\*${id}\\*\\*:`, "gm"))];
    const rows = [...source.matchAll(new RegExp(`^\\|\\s*${id}\\s*\\|\\s*Phase 162\\s*\\|\\s*(Pending|Complete)\\s*\\|`, "gm"))];
    if (boxes.length !== 1 || rows.length !== 1 || (boxes[0][1] === " ") !== (rows[0][1] === "Pending")) return "invalid";
    return rows[0][1];
  });
  if (requirements.includes("invalid") || new Set(requirements).size !== 1) return fail("Phase 162 requirement checkbox/traceability consistency");
  const complete = requirements[0] === "Complete";
  const proof = text(PHASE162_DOC);
  const checked = (state: ClaimState): ClaimState => {
    const before = failures.length;
    checkPhase162ExcludedClaims(JSON.stringify(owner), "docs/parity/index.json", failures, state);
    if (proof) checkPhase162ExcludedClaims(proof, PHASE162_DOC, failures, state);
    return failures.length === before ? state : "invalid";
  };
  if (!proof) return complete || owner.status !== "in_progress" ? fail("premature Phase 162 activation") : checked("pending");
  const lifecycle = metadata(text(PHASE162 + "162-CONTEXT.md"), "phase_lifecycle_id");
  const generated = metadata(proof, "generated_at");
  if (!lifecycle || metadata(proof, "phase_lifecycle_id") !== lifecycle || metadata(proof, "lifecycle_mode") !== "yolo"
    || metadata(proof, "implementation_status") !== "proven" || !/^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/.test(generated)
    || !Number.isFinite(Date.parse(generated)) || !(Date.parse(generated) >= Date.parse(metadata(text(PHASE162 + "162-CONTEXT.md"), "generated_at")))
    || !owner.evidence.includes(PHASE162_DOC)) return fail("linked current-lifecycle Phase 162 implementation evidence");
  for (const file of PHASE162_EVIDENCE) {
    const hash = createHash("sha256").update(text(file)).digest("hex");
    const rows = proof.split(/\r?\n/).map(line => line.split("|").map(cell => cell.trim())).filter(cells => cells[1] === `\`${file}\``);
    if (!text(file) || rows.length !== 1 || JSON.stringify(rows[0]) !== JSON.stringify(["", `\`${file}\``, `\`${hash}\``, ""])) return fail(`Phase 162 source hash ${file}`);
  }
  const contracts: [string, string, string[]][] = [
    [PHASE162_EVIDENCE[0], "independent_oracle", ["recipe.fork(998, 4, 0x62)", "core_types::codec::encode_block(block)", "recipe.blocks", "process::run(", "output.reaped", "assert_eq!(result[", "recipe.spent_scripts"]],
    [PHASE162_EVIDENCE[1], "assert_pair", ["assert_eq!(", ".load_block(hash)", ".is_some()", "present", "assert_eq!(", ".load_undo(hash)", ".is_some()", "present"]],
    [PHASE162_EVIDENCE[1], "manual", ["invoke(fixture,", "json!([1000])", "assert_eq!", "json!([20])", "assert_pair(fixture, position.block_hash, false)", "settled.assert_unchanged(fixture)"]],
    [PHASE162_EVIDENCE[1], "automatic", ["Phase162Fixture::open_with_prune(fixture.close(), 550)", "add_pressure(&fixture)", ".flush_coins(", "assert!(!deletion.deleted_block_hashes.is_empty())", "assert_pair(&fixture, position.block_hash, false)", "Phase162Fixture::open_with_prune(fixture.close(), 550)", "assert_pair(&fixture, position.block_hash, false)"]],
    [PHASE162_EVIDENCE[2], "close", ["let Self", "drop(state)", "drop(shared)", "drop(opened)", "drop(store)", "Phase162Handoff"]],
    [PHASE162_EVIDENCE[3], "phase162_integrated_lifecycle", ["Phase162Fixture::seed()", "MaintenanceHarness::start(&fixture)", "worker.tick()", "assert_eq!(previous, 1000,", "extend_and_reorg(&fixture", "worker.shutdown()", "retention::manual(&fixture", "retention::automatic(fixture", "rpc::assert_phase162_rpc(&fixture)", "inbound::assert_phase162_inbound(fixture", "restrictions::assert_restrictions(fixture", "outbound::assert_outbound(handoff)", "retention::assert_pair(&fixture", "std::fs::remove_dir_all(&handoff.path)"]],
  ];
  for (const [file, symbol, anchors] of contracts) {
    const maybeBody = maybeFunction(text(file), symbol);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, anchors) || /#\[\s*(?:ignore|cfg\s*\(\s*any\s*\(\s*\))/.test(maybeBody.attributes)) return fail(`Phase 162 actual evidence ${file}:${symbol}`);
  }
  if (!complete) return owner.status === "in_progress" && proof.includes(PHASE162_IMPLEMENTATION) && !proof.includes(PHASE162_COMPLETE)
    ? checked("implementation") : fail("Phase 162 implementation/requirement consistency");
  if (owner.status !== "done" || !proof.includes(PHASE162_COMPLETE) || proof.includes(PHASE162_IMPLEMENTATION)) return fail("Phase 162 formal claim/requirement consistency");
  const digest = createHash("sha256").update(PHASE162_EVIDENCE.map(file => `${file}:${createHash("sha256").update(text(file)).digest("hex")}`).join("\n")).digest("hex");
  for (const [name, status, generator] of FORMAL162) {
    const report = text(PHASE162 + `162-${name}.md`);
    if (metadata(report, "status") !== status || metadata(report, "generated_by") !== generator || metadata(report, "phase_lifecycle_id") !== lifecycle
      || metadata(report, "lifecycle_mode") !== "yolo" || metadata(report, "source_digest") !== digest || !owner.evidence.includes(PHASE162 + `162-${name}.md`)
      || !(Date.parse(metadata(report, "generated_at")) >= Date.parse(generated))) return fail(`fresh scoped Phase 162 ${name} gate`);
  }
  if (metadata(text(PHASE162 + "162-VERIFICATION.md"), "lifecycle_validated") !== "true"
    || metadata(text(PHASE162 + "162-SECURITY.md"), "threats_open") !== "0"
    || !/^bash scripts\/verify\.sh[ \t]*$/m.test(text(PHASE162 + "162-NATIVE-EVIDENCE.md"))) return fail("Phase 162 full native/security/lifecycle gate");
  return checked("complete");
}

/** Only the exact evidence-authorized sentence earns a current-document exemption. */
export function phase162ClaimView(source: string, state: ClaimState): string {
  return state === "implementation" ? source.replaceAll(PHASE162_IMPLEMENTATION, "")
    : state === "complete" ? source.replaceAll(PHASE162_COMPLETE, "") : source;
}

/** Safety exclusions apply independently of implementation/formal state and historical limits. */
export function checkPhase162ExcludedClaims(source: string, file: string, failures: string[], state: ClaimState = "pending"): void {
  const view = phase162ClaimView(source, state);
  const subjects = /Phase\s+162(?:\s+(?:scoped|integrated|proof|implementation|evidence|client|retention))*|CF(?:PR-02|GR-0[12])|BIP37(?:\s+serving)?|V0(?:\s+(?:filter|serving))?|assumeutxo|dual[- ]chainstate|(?:implicit\s+)?(?:repair|reindex|import|download)|archive(?:[- ]scale)?(?:\s+serving)?|public(?:[- ]network\s+CI|[- ]defaults(?:\/network\s+CI)?|\s+(?:peer\s+)?serving(?:\s+defaults)?)|production(?:[- ]funds)?(?:[- ](?:service|readiness|use|safety))?|funds[- ]safety|v2\.5(?:\s+milestone)?/gi;
  const positive = /^\s+(?:(?:is|are|has|have|now|fully|already|successfully)\s+)*(?:available|active|enabled|supported|ready|shipped|safe|established|complete|proven|passed|succeeded|guaranteed)\b/i;
  for (const clause of view.split(/\n|;|(?<=[.!?])\s+|\bbut\b|\bhowever\b|\band\b|\bwhile\b/)) {
    const matches = [...clause.matchAll(subjects)];
    for (const [index, match] of matches.entries()) {
      const before = clause.slice(index === 0 ? 0 : matches[index - 1].index + matches[index - 1][0].length, match.index);
      if (positive.test(clause.slice(match.index + match[0].length, matches[index + 1]?.index)) && !/\b(?:does not|do not|no claim|not a|future phase)\b/i.test(before)) failures.push(`${file}: unsupported claim T-162-10`);
    }
  }
}

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
  row(NODE + "chainstate.rs", "flush_applying_plan", ["self.flush_applying_plan_with_header_checkpoint(", "mode, now, disk_free_bytes, plan, locks,", "&mut |_| Ok(())"]),
  row(NODE + "chainstate.rs", "flush_applying_plan_with_header_checkpoint", ["lineage.maybe_prepare(self.chainstate.tip())", "self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint(", "checkpoint,", "execution.wrote_coins", "pending", ".complete(self.maybe_validated_lineage.as_ref(), self.chainstate.tip())?", "self.store.confirm_validated_flush(completed)?", "self.note_basic_index_reorg_durable()"]),
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

/** Selected callback edges extend the historical contracts without changing claim ownership. */
export const CHECKPOINT_CONTRACTS: Contract[] = [
  row(NODE + "chainstate.rs", "persist", ["self.flush_with_mode(", "FlushMode::IfNeeded,", "self.store.disk_free_bytes()", ".map(|_| ())"]),
  row(NODE + "chainstate.rs", "flush_with_mode", ["self.flush_applying_plan(mode, now, disk_free_bytes, &PrunePlan::default(), &[])", ".map_err(|failure| failure.error)"]),
  row(NODE + "chainstate.rs", "flush_applying_plan_with_header_checkpoint", ["lineage.maybe_prepare(self.chainstate.tip())", "self.flush_lifecycle.execute_flush_applying_plan_with_header_checkpoint(", "checkpoint,", "execution.wrote_coins", "pending", ".complete(self.maybe_validated_lineage.as_ref(), self.chainstate.tip())?", "self.store.confirm_validated_flush(completed)?", "self.note_basic_index_reorg_durable()", "result.map_err(|error| FlushApplyError", "error,", "deleted_block_hashes: deleted"]),
  row(NODE + "chainstate/flush_lifecycle.rs", "execute_flush_applying_plan_with_header_checkpoint", ["if self.readiness != ManagerReadiness::ReadyToFlush", "return Err(coins_corruption(", "let decision = decide_flush(", "if plan.heights.is_empty() && coins_write_kind(decision).is_none()", "return Err(refuse_disk_space())", "return Ok(FlushExecution", "persist_ordered_prefix(sink, block_payloads, undo_window, header_entries)?", "checkpoint(sink)?;", "apply_prune_plan(sink, plan, locks, active_chain, on_deleted)?", "complete_coins_write(sink, cache, decision, active_chain, deleted_block_hashes)"]),
  row(NODE + "network/runtime_authority/prune_flush.rs", "flush_and_evict_pruned_blocks", ["let ManagedPeerNetwork", "chainstate.flush_applying_plan_with_header_checkpoint(", "&mut |sink: &S|", "sink.maybe_validation_history_store()", "TrustedHeaderSnapshot::maybe_capture(", "store, peer_manager.header_store()", "snapshot.persist()", "let deleted = match &outcome", "Err(failure) => failure.deleted_block_hashes.clone()", "network.blocks_by_hash.remove(hash)", "outcome.map_err(|failure| failure.error)"]),
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
export const CHECK_FILES = [...new Set([...CONTRACTS.map(row => row.file), ...CHECKPOINT_CONTRACTS.map(row => row.file), ...TESTS.map(([file]) => file), ...REGISTRATIONS.map(([file]) => file),
  ...SEALED.map(([file]) => file), ...PHASE162_FILES, ...GUARDS, ...UPSTREAM, DOC, "README.md", "packages/README.md", "docs/parity/index.json",
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
export function checkPhase158ValidatedReorg(maybeRoot?: string, maybeSources?: ReadonlyMap<string, string>): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = maybeSources ? new Map(maybeSources) : loadPhase162OptionalSources(root);
  for (const file of CHECK_FILES) {
    if (maybeSources) { if (!texts.has(file)) failures.push(`${file}: cannot read required evidence`); continue; }
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: cannot read required evidence`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  const claimState = checkPhase162ClaimState(text, failures);
  for (const contract of CONTRACTS) {
    const maybeBody = maybeBoundary(text(contract.file), contract);
    if (!maybeBody || !hasOrderedCode(maybeBody.body, contract.anchors)) failures.push(`T-158-07-01: actual boundary ${contract.file}:${contract.symbol}`);
  }
  checkCheckpointEdges(text, failures);
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
  checkDocuments(text, failures, claimState);
  checkParity(text, failures, claimState);
  checkVerifier(text("scripts/verify.sh"), failures);
  return [...new Set(failures)];
}

function checkCheckpointEdges(text: (file: string) => string, failures: string[]): void {
  for (const contract of CHECKPOINT_CONTRACTS) {
    const maybeBody = maybeBoundary(text(contract.file), contract);
    const body = compact(maybeBody?.body ?? "");
    let valid = maybeBody !== undefined && hasOrderedCode(maybeBody.body, contract.anchors);
    if (contract.symbol === "persist") {
      valid &&= !compact(ordinaryRust(text(contract.file))).includes("save_snapshot(self.chainstate.snapshot())");
    }
    if (contract.symbol === "execute_flush_applying_plan_with_header_checkpoint") {
      valid &&= (body.match(/checkpoint\(sink\)/g) ?? []).length === 1;
      const declaration = compact(ordinaryRust(text(contract.file)).slice(maybeBody?.start, maybeBody?.end));
      valid &&= declaration.includes("checkpoint:&mutimplFnMut(&S)->Result<(),StorageError>")
        && declaration.includes("->Result<FlushExecution,StorageError>");
    }
    if (contract.symbol === "flush_and_evict_pruned_blocks") {
      valid &&= (body.match(/TrustedHeaderSnapshot::maybe_capture\(/g) ?? []).length === 1
        && body.includes("snapshot.persist()},)");
    }
    if (!valid) failures.push(`T-162-15: ordered checkpoint boundary ${contract.file}:${contract.symbol}`);
  }
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
function checkDocuments(text: (file: string) => string, failures: string[], claimState: ClaimState): void {
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
  for (const file of [DOC, "README.md", "packages/README.md", PHASE + "158-UAT.md", PHASE + "158-REORG-MEASUREMENTS.md",
    ...["01", "02", "03", "04", "05", "06"].map(plan => PHASE + `158-${plan}-SUMMARY.md`)]) {
    const historical = file.startsWith(PHASE);
    checkClaims(historical ? text(file) : phase162ClaimView(text(file), claimState), file, failures);
    checkPhase162ExcludedClaims(text(file), file, failures, historical ? "pending" : claimState);
  }
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
function checkParity(text: (file: string) => string, failures: string[], claimState: ClaimState): void {
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
    if (!surface.known_gaps.some(item => (claimState !== "complete" && item.includes("159") && item.includes("162") && item.includes("pending"))
      || (claimState === "implementation" && item === PHASE162_IMPLEMENTATION) || (claimState === "complete" && item === PHASE162_COMPLETE))) failures.push("T-158-07-01: deferred serving boundary");
    const inspect = (value: unknown): void => {
      if (typeof value === "string") { checkClaims(phase162ClaimView(value, claimState), "parity surface", failures); checkPhase162ExcludedClaims(value, "parity surface", failures, claimState); return; }
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

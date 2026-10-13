#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { hasOrderedCode, maybeFunction, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { meaningfulAssertions, supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { maybeSelector } from "./check-phase160-peer-filter-serving.ts";
import { PHASE162_FILES, PHASE162_IMPLEMENTATION, PHASE162_COMPLETE, checkPhase162ClaimState, checkPhase162ExcludedClaims, loadPhase162OptionalSources, phase162ClaimView, sourceText } from "./check-phase158-validated-reorg.ts";

const N = "packages/open-bitcoin-node/src/";
const C = "packages/open-bitcoin-cli/src/operator/";
const R = "packages/open-bitcoin-rpc/src/";
const DOC = "docs/parity/catalog/basic-compact-filters.md";
type Contract = readonly [string, string, readonly string[]];

// These lexical seams supplement executed Rust tests; they cannot certify behavior.
export const CONTRACTS: readonly Contract[] = [
  [N + "network/runtime_authority.rs", "operator_snapshot", ["self.read(ManagedPeerNetwork::operator_snapshot)"]],
  [N + "network/operator_snapshot.rs", "operator_snapshot", ["basic_index: self.project_basic_index()"]],
  [N + "network/operator_snapshot.rs", "project_basic_index", ["maybe_configured_basic_indexing()", "maybe_basic_peer_scope()", "progress.initially_synchronized()", "maybe_basic_index_accepted_target()", "maybe_basic_index_failure()", "store.basic_filter_operator_observation()", "observation.maybe_read_fault", "project_read_fault(&mut status, fault)", "observation.retained_availability", "observation.retention", "observation.authority", "project_basic_owner("]],
  [N + "network/operator_snapshot.rs", "project_basic_owner", ["progress.initially_synchronized()", "authority.integrity", "!authority.suspended", "maybe_target == Some(progress.accepted_target())", "maybe_target == maybe_active_target", "generation == progress.generation()", "branch == progress.branch_identity()", "safe == endpoint_checkpoint(progress.maybe_safe_durable_endpoint())", "if !coherent", "FieldAvailability::unavailable(", "return;", "progress.maybe_processed_endpoint().map(|id| id.height())", "progress.maybe_safe_durable_endpoint().map(|id| id.height())", "progress.current_lag()"]],
  [R + "context/inbound_status.rs", "authoritative_operator_snapshot", ["self.network.operator_snapshot()?", "AuthoritativeOperatorSnapshot { network, inbound }"]],
  [R + "dispatch/node.rs", "open_bitcoin_network_status", ["context.authoritative_operator_snapshot()", "basic_index: snapshot.basic_index().clone()"]],
  [C + "status.rs", "collect_live_status_snapshot", ["let basic_index = network_status.basic_index;", "basic_index,"]],
  [C + "status.rs", "stopped_status_snapshot", ["basic_index:", "BasicIndexOperatorStatus::unavailable(", "BasicIndexUnavailableReason::LiveUnavailable"]],
  [N + "status/basic_index.rs", "BasicIndexOperatorStatus::default", ["Self::unavailable(BasicIndexUnavailableReason::NotReported)"]],
  [N + "status/basic_index.rs", "unavailable", ["configured_indexing: FieldAvailability::unavailable(reason)", "initial_sync_complete: FieldAvailability::unavailable(reason)", "maybe_safe_durable_height: FieldAvailability::unavailable(reason)", "retention: FieldAvailability::unavailable(reason)"]],
  [N + "status/basic_index.rs", "serialize", ["FieldAvailability::Unavailable { reason }", "BasicIndexUnavailableReason::normalize(reason)", ".serialize(serializer)"]],
  [N + "status/basic_index.rs", "deserialize", ["FieldAvailability::Unavailable { reason }", "BasicIndexUnavailableReason::normalize(&reason)"]],
  [C + "status/render.rs", "render_human_status", ["basic_index_lines(&snapshot.basic_index)", "prune_status_lines(&snapshot.prune)"]],
  [C + "status/render/basic_index.rs", "available", ["FieldAvailability::Unavailable { reason }", "BasicIndexUnavailableReason::normalize(reason)"]],
  [C + "status/render/basic_index.rs", "basic_index_entries", ["&status.operational_state", "&status.initial_sync_complete", "&status.maybe_lag_blocks", "&status.maybe_processed_height", "&status.maybe_safe_durable_height", "&status.maybe_failure", "&status.retained_availability", "&status.retention"]],
  [C + "dashboard/model.rs", "dashboard_sections", ["basic_index_section(&snapshot.basic_index)", "prune_section(&snapshot.prune)"]],
  [C + "dashboard/model/basic_index_section.rs", "basic_index_section", ["basic_index_entries(status)"]],
  [C + "dashboard/app.rs", "render_sections", [".take(7)", "compact_basic_value(row)", "return List::new("]],
  [C + "support/redaction.rs", "support_status_for_bundle", ["basic_index::redact_basic_index(&mut status.basic_index)"]],
  [C + "support/redaction/basic_index.rs", "sanitize", ["FieldAvailability::Unavailable { reason }", "BasicIndexUnavailableReason::normalize(reason)"]],
  [C + "support/render.rs", "render_support_markdown", ["basic_index::push_basic_index(&mut output, &bundle.status.basic_index)"]],
  [C + "support/render/basic_index.rs", "push_basic_index", ["basic_index_entries(status)"]],
  [N + "storage/fjall_store/filters/append.rs", "complete_basic_filter_append", ["control.retention_after_append(starting, &prepared.records)?", "self.finish_basic_filter_batch(batch, &mut control)?", "control.install_retention(retained)"]],
  [N + "storage/fjall_store/filters/publication.rs", "persist_basic_filter_records", ["control.retention_after_append(starting, &encoded)?", "self.finish_basic_filter_batch(batch, &mut control)?", "control.install_retention(retained)"]],
  [N + "storage/fjall_store/filters/publication/epoch.rs", "invalidate_read_integrity", ["self.read_integrity = false", "self.retention = super::super::retention::RetainedBasicFilterObservation::Incomplete", "self.advance_integrity_epoch()"]],
  [N + "storage/fjall_store/filters/retention.rs", "basic_filter_operator_observation", ["basic_filter_operator_observation_with_limits(8, 4096)"]],
  [N + "storage/fjall_store/filters/retention.rs", "basic_filter_operator_observation_with_limits", ["filter_publication_guard()", "control.retention.measurement()", "work.metadata_reads >= reads", "work.metadata_reads += 1", ".block_index", ".get(key)", "checked_add(value.len() as u64)", "next_bytes > bytes", "read(codec::STATE_KEY)?", "read(codec::ownership::OWNER_KEY)?", "id.revision == control.revision"]],
  [N + "storage/fjall_store/filters/retention.rs", "measurement", ["Self::Complete(value) => Ok(value)", "Self::Incomplete => Err(", "Self::Failed => Err(", "Self::BudgetExceeded => Err("]],
];

const TESTS: readonly Contract[] = [
  [R + "bin/open_bitcoind/tests/filter_index/operator_evidence.rs", "phase161_operator_rpc_frozen_progress_auth_and_real_reopen", ["DaemonFixture::", "captured(&fixture).await", "assert_eq!(", "StatusCode::UNAUTHORIZED", "fixture.close()", "DaemonFixture::open(", "assert_eq!(captured(&fixture).await, settled)"]],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "phase161_operator_consumers_genuine_catchup_current_lag_ahead_reopen", ["durable_basic_runtime(", "DurableSyncRuntime::open_configured(", "assert_consumers(", "drive_basic_filter_index_turn()", "assert_consumers("]],
  [C + "dashboard/app/tests/basic_index.rs", "phase161_basic_dashboard_cells_normal_shows_all_nineteen_adjacent_facts", ["draw(", "160, 55", "assert_eq!(right, expected)"]],
  [C + "dashboard/app/tests/basic_index.rs", "phase161_basic_dashboard_cells_narrow_preserves_seven_critical_adjacent_values", ["draw(", "80", "assert_eq!(region(", "expected)"]],
  [N + "sync/tests/filter_index/operator_status/captures.rs", "phase161_operator_authority_detailed_private_owner_failures_coexist_with_genuine_retention", ["ValidatedHistory::new(", "note_basic_index_failure(", "capture(&runtime)", "assert_eq!(", "verify_caller_capture("]],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index/captured.rs", "phase161_operator_consumers_hostile_captured_reasons_are_private", ["captured_response_server(", "assert_exports(", "assert_eq!(", "assert!(!text.contains("]],
];

const MODULES: readonly [string, string][] = [
  [N + "status.rs", "basic_index"], [N + "storage/fjall_store/filters.rs", "retention"],
  [N + "network.rs", "operator_snapshot"], [C + "status.rs", "render"],
  [C + "status/render.rs", "basic_index"], [C + "dashboard/model.rs", "basic_index_section"],
  [C + "support/redaction.rs", "basic_index"], [C + "support/render.rs", "basic_index"],
  ["packages/open-bitcoin-cli/tests/operator_flows.rs", "basic_index"],
  [R + "bin/open_bitcoind/tests/filter_index.rs", "operator_evidence"],
  [C + "dashboard/app/tests.rs", "basic_index"],
  ["packages/open-bitcoin-cli/tests/operator_flows/basic_index.rs", "captured"],
  [N + "sync/tests/filter_index/operator_status.rs", "captures"],
];
const DOCS = ["README.md", "packages/README.md", "docs/architecture/status-snapshot.md", "docs/architecture/operator-observability.md", "docs/operator/runtime-guide.md", DOC];
const LIMITS = [
  "Phase 162 owns the complete integrated prune/reopen client proof.",
  "v2.5 remains pending.", "No stronger v2.4 crash-window guarantee is claimed.",
];
export const CHECK_FILES = [...new Set([...CONTRACTS, ...TESTS].map(row => row[0]).concat(MODULES.map(row => row[0]), DOCS, PHASE162_FILES, ["docs/parity/index.json", "scripts/verify.sh"]))];

/** Load fixed current paths only; no archive traversal or generated evidence. */
export function loadSources(): Map<string, string> {
  return new Map([...loadPhase162OptionalSources(resolve(import.meta.dir, "..")), ...CHECK_FILES.map(file => [file, readFileSync(resolve(import.meta.dir, "..", file), "utf8")] as const)]);
}

/** In-memory source views let mutation tests remain isolated from the checkout. */
export function checkPhase161OperatorIndexEvidence(maybeSources?: ReadonlyMap<string, string>): string[] {
  const sources = maybeSources ?? loadSources();
  const failures: string[] = [];
  const text = sourceText(sources);
  const claimState = checkPhase162ClaimState(text, failures);
  for (const file of CHECK_FILES) if (!sources.has(file)) failures.push(`${file}: required source missing`);
  for (const [file, symbol, anchors] of [...CONTRACTS, ...TESTS]) {
    const source = text(file);
    const production = CONTRACTS.some(row => row[0] === file && row[1] === symbol);
    const maybeBody = maybeSelector(source, symbol, production);
    // Existing dead-code lint allowances do not change executable ownership.
    const ownership = source.replaceAll("#[cfg_attr(not(test), allow(dead_code))]", " ".repeat("#[cfg_attr(not(test), allow(dead_code))]".length));
    if (!maybeBody || !supportedOwnership(ownership, maybeBody.start, !production) || !hasOrderedCode(maybeBody.body, anchors)) failures.push(`${file}:${symbol}: actual function edge required`);
    if (!production && (!maybeBody || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeBody.attributes) || !meaningfulAssertions(source, maybeBody.body))) failures.push(`${file}:${symbol}: assertive executable caller required`);
  }
  for (const [file, symbol] of MODULES) {
    const source = text(file);
    const maybeDeclaration = rustCode(source).match(new RegExp(`\\bmod\\s+${symbol}\\s*;`));
    if (!maybeDeclaration || !supportedOwnership(source, maybeDeclaration.index!, /tests/.test(file))) failures.push(`${file}: module ${symbol} required`);
  }
  const retention = N + "storage/fjall_store/filters/retention.rs";
  const maybePoll = maybeFunction(text(retention), "basic_filter_operator_observation_with_limits", true);
  if (maybePoll && /\b(?:for|while|loop)\b|\.iter\s*\(|\.scan\s*\(|load_block\s*\(|load_undo\s*\(|\.to_vec\s*\(/.test(maybePoll.body)) failures.push(`${retention}: polling must not scan/copy history`);
  for (const [file, symbol] of [[N + "network/operator_snapshot.rs", "project_basic_index"], [retention, "basic_filter_operator_observation_with_limits"]]) {
    const maybeBody = maybeFunction(text(file), symbol, true);
    if (maybeBody && /\b(?:drive_basic_filter_index_turn|configure_basic_filter_index|recover_basic_filter_index|enable_basic_filter_index|disable_basic_filter_index|flush_coins|remove_prune_lock)\s*\(|\.mutate\s*\(/.test(maybeBody.body)) failures.push(`${file}:${symbol}: observation cannot activate/recover or change protection`);
  }
  const captures = N + "sync/tests/filter_index/operator_status/captures.rs";
  const maybeFreshness = maybeFunction(text(captures), "compare_fixture");
  if (!maybeFreshness || !hasOrderedCode(maybeFreshness.body, ["validate_fixture(captures)", "assert_eq!(", "captures[name], *value"])) failures.push(`${captures}: mandatory exact fresh producer comparison`);
  checkClaims(text, failures, claimState);
  checkNative(text("scripts/verify.sh"), failures);
  return [...new Set(failures)];
}

function checkClaims(text: (file: string) => string, failures: string[], claimState: ReturnType<typeof checkPhase162ClaimState>): void {
  for (const file of DOCS) {
    const scoped = claimState === "implementation" && text(file).includes(PHASE162_IMPLEMENTATION)
      || claimState === "complete" && text(file).includes(PHASE162_COMPLETE);
    for (const [index, limit] of LIMITS.entries()) if (!text(file).includes(limit)
      && !(index === 0 && scoped) && !(index === 1 && text(file).includes("v2.5 remains unaudited and unshipped."))) failures.push(`${file}: fixed scope limit ${limit}`);
    const view = phase162ClaimView(text(file), claimState, text);
    if (/v2\.5\s+(?:is\s+)?(?:production ready|complete|shipped)|Phase 162 integrated proof is complete|production readiness\s+is\s+(?:proven|complete|established)/i.test(view)) failures.push(`${file}: unsupported production/integrated claim`);
    checkPhase162ExcludedClaims(text(file), file, failures, claimState, text);
  }
  for (const phrase of ["Excluded: BIP37/V0, filter import, wallet acceleration, reindex/repair/download,", "archive-scale, assumeutxo, public defaults/network CI, production service/readiness", "and funds-safety."]) if (!text(DOC).includes(phrase)) failures.push(`${DOC}: excluded surface disclosure`);
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const rows = index.checklist.surfaces.filter((row: { requirements: string[] }) => row.requirements.includes("CFOP-01"));
    if (rows.length !== 1 || JSON.stringify(rows[0].requirements) !== '["CFOP-01"]' || !["in_progress", "done"].includes(rows[0].status)) { failures.push("docs/parity/index.json: unique CFOP-01 owner"); return; }
    for (const file of [DOC, "scripts/check-phase161-operator-index-evidence.ts", "scripts/check-phase161-operator-index-evidence.test.ts", N + "storage/fjall_store/filters/retention.rs"]) if (!rows[0].evidence.includes(file)) failures.push(`docs/parity/index.json: evidence ${file}`);
    if (!rows[0].known_gaps.some((gap: string) => (claimState !== "complete" && gap.includes("Phase 162") && gap.includes("pending"))
      || (claimState === "implementation" && gap === PHASE162_IMPLEMENTATION) || (claimState === "complete" && gap === PHASE162_COMPLETE))) failures.push("docs/parity/index.json: Phase 162 ownership");
    checkPhase162ExcludedClaims(JSON.stringify(rows[0]), "docs/parity/index.json", failures, claimState, text);
  } catch { failures.push("docs/parity/index.json: invalid claim inventory"); }
}

function checkNative(source: string, failures: string[]): void {
  // The first section is the fast contract/order list; the second executes default/full.
  const sections = [source.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "", source.split("\n").filter(line => line.startsWith("run_step ")).join("\n")];
  const commands = ["bun run scripts/check-phase160-peer-filter-serving.ts", "bun test ./scripts/check-phase161-operator-index-evidence.test.ts", "bun run scripts/check-phase161-operator-index-evidence.ts"];
  for (const section of sections) {
    const lines = section.split("\n").filter(line => commands.some(command => line.endsWith(command)));
    if (lines.length !== commands.length || lines.some((line, index) => !line.endsWith(commands[index]))) failures.push("scripts/verify.sh: ordered Phase 160/161 native commands required");
  }
}

if (import.meta.main) {
  const failures = checkPhase161OperatorIndexEvidence();
  if (failures.length) {
    console.error("Phase 161 scoped operator evidence check failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log(`Phase 161: ${CONTRACTS.length} function seams, ${TESTS.length} caller selectors and scoped claims checked; executed tests and full lifecycle gates remain independent.`);
}

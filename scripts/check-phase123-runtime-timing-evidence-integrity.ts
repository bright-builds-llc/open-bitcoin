#!/usr/bin/env bun

import { existsSync, readFileSync } from "node:fs";
import path from "node:path";

const DEFAULT_REPO_ROOT = path.resolve(import.meta.dir, "..");
const SURFACE_ID = "v2-1-runtime-timing-evidence-integrity";
const REQUIREMENTS = ["HARD-02", "HARD-03", "HARD-04"] as const;
const PHASE122_CHECK =
  "bun run scripts/check-phase122-compact-relay-peer-completion.ts";
const PHASE123_TEST =
  "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts";
const PHASE123_CHECK =
  "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts";
const PHASE117_TEST =
  "bun test scripts/check-phase117-parity-uat-release-boundary.test.ts";

const TARGET_FILES = [
  "packages/open-bitcoin-node/src/sync/types.rs",
  "packages/open-bitcoin-node/src/sync/tcp.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/session.rs",
  "packages/open-bitcoin-node/src/lib.rs",
  "packages/open-bitcoin-bench/src/runtime_fixtures.rs",
  "packages/open-bitcoin-node/src/network/block_relay_evidence.rs",
  "packages/open-bitcoin-node/src/metrics/block_relay.rs",
  "packages/open-bitcoin-node/src/logging.rs",
  "packages/open-bitcoin-node/src/status/block_relay_evidence.rs",
  "packages/open-bitcoin-rpc/src/context.rs",
  "packages/open-bitcoin-rpc/src/context/network.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener.rs",
  "packages/open-bitcoin-rpc/src/inbound_listener/tests.rs",
  "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
  "packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
  "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  "scripts/check-phase121-block-relay-metrics-log-runtime.ts",
  "docs/architecture/operator-observability.md",
  "docs/parity/index.json",
  "docs/parity/checklist.md",
  "docs/parity/catalog/p2p.md",
  "docs/parity/source-breadcrumbs.json",
  "scripts/verify.sh",
] as const;

type TargetFile = (typeof TARGET_FILES)[number];
type CheckOptions = { rootDir?: string };
type ParityIndex = {
  surfaces?: unknown[];
  checklist?: { surfaces?: unknown[] };
};
type NamedSurface = { name?: string; status?: string };
type ChecklistSurface = {
  id?: string;
  title?: string;
  status?: string;
  requirements?: string[];
  evidence?: string[];
  rationale?: string;
  upstream?: { sources?: string[]; tests?: string[] };
  known_gaps?: string[];
  suspected_unknowns?: string[];
};
type BreadcrumbManifest = {
  groups?: Array<{ files?: string[]; breadcrumbs?: string[] }>;
};

export function checkPhase123RuntimeTimingEvidenceIntegrity(
  options: CheckOptions = {},
): string[] {
  const repoRoot = path.resolve(options.rootDir ?? DEFAULT_REPO_ROOT);
  const failures: string[] = [];
  const texts = new Map<TargetFile, string>();
  for (const file of TARGET_FILES) {
    texts.set(file, readText(repoRoot, file, failures));
  }

  verifyTypedReceiveAndTcp(texts, failures);
  verifyProductionActivation(texts, failures);
  verifyIdleMaintenance(texts, failures);
  verifySuccessfulWriteEvidence(texts, failures);
  verifyPrivateProjection(texts, failures);
  verifyAuthoritativeRuntime(texts, failures);
  verifyFocusedTests(texts, failures);
  verifyPhase121Compatibility(texts, failures);
  verifyParity(texts, failures);
  verifyBreadcrumbs(texts.get("docs/parity/source-breadcrumbs.json") ?? "", failures);
  verifyVerifierWiring(texts.get("scripts/verify.sh") ?? "", failures);
  return failures;
}

function verifyProductionActivation(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  for (const needle of [
    "pub fn open_with_block_relay_activation",
    "ManagedPeerNetwork::with_sync_limits_and_block_relay_activation",
    "block_relay_activation",
  ]) {
    requireContains(sync, needle, "P123 sync production activation", failures);
  }
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  requireOrdered(
    daemon,
    ["DurableSyncRuntime::open_with_block_relay_activation(", "runtime.block_serving"],
    "P123 daemon sync activation wiring",
    failures,
  );
  for (const file of [
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  ] as const) {
    requireContains(
      texts.get(file) ?? "",
      "DurableSyncRuntime::open_with_block_relay_activation(",
      `P123 production activation test path ${file}`,
      failures,
    );
  }

  const rpcNetwork = texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  requireOrdered(
    rpcNetwork,
    ["ManagedPeerNetwork::new_with_block_relay_activation(", "config.block_serving"],
    "P123 inbound production activation wiring",
    failures,
  );
  const inboundTests = texts.get("packages/open-bitcoin-rpc/src/inbound_listener/tests.rs") ?? "";
  for (const needle of [
    "phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block",
    "phase123_disabled_runtime_config_does_not_serve_inbound_block",
  ]) {
    requireContains(inboundTests, needle, "P123 inbound production activation tests", failures);
  }
}

function readText(repoRoot: string, file: TargetFile, failures: string[]): string {
  const absolutePath = path.join(repoRoot, file);
  if (!existsSync(absolutePath)) {
    failures.push(`P123 missing required corpus file: ${file}`);
    return "";
  }
  return readFileSync(absolutePath, "utf8");
}

function verifyTypedReceiveAndTcp(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const types = texts.get("packages/open-bitcoin-node/src/sync/types.rs") ?? "";
  for (const needle of [
    "pub enum SyncPeerReceiveOutcome",
    "Message(WireNetworkMessage)",
    "Idle",
    "Closed",
    "Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
  ]) {
    requireContains(types, needle, "P123 typed receive outcomes", failures);
  }
  requireContains(
    texts.get("packages/open-bitcoin-node/src/lib.rs") ?? "",
    "SyncPeerReceiveOutcome",
    "P123 public receive re-export",
    failures,
  );

  for (const file of [
    "packages/open-bitcoin-node/src/sync/tcp.rs",
    "packages/open-bitcoin-bench/src/runtime_fixtures.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
  ] as const) {
    const text = texts.get(file) ?? "";
    requireContains(
      text,
      "Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
      `P123 first-party receive migration ${file}`,
      failures,
    );
    requireAbsent(
      text,
      "Result<Option<WireNetworkMessage>, SyncRuntimeError>",
      `P123 first-party receive migration ${file}`,
      failures,
    );
  }

  const tcp = texts.get("packages/open-bitcoin-node/src/sync/tcp.rs") ?? "";
  for (const needle of [
    "Ok(0) if allow_clean_idle && filled == 0",
    "return Ok(ReadStageOutcome::Closed)",
    "allow_clean_idle",
    "filled == 0",
    "io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut",
    "return Ok(ReadStageOutcome::Idle)",
    "unexpected EOF after {filled} of {} frame bytes",
    "payload read ended without a complete frame",
  ]) {
    requireContains(tcp, needle, "P123 byte-progress TCP framing", failures);
  }
}

function verifyIdleMaintenance(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  const session = texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  const syncSession = `${sync}\n${session}`;
  requireOrdered(
    syncSession,
    [
      "SyncPeerReceiveOutcome::Idle =>",
      "current_timestamp = (controls.0)();",
      ".expire_compact_download_timeouts(current_timestamp)?",
      ".any(|(target_peer_id, _message)| *target_peer_id != peer_id)",
      ".map(|(_target_peer_id, message)| message)",
      "self.send_all(&mut session, &outbound)?;",
      "if !self.peer_has_compact_download_in_flight(peer_id)",
      "self.complete_peer_session_progress(&mut progress, peer_id);",
      "return Ok(());",
      "continue;",
      "SyncPeerReceiveOutcome::Closed =>",
    ],
    "P123 idle clock target send order",
    failures,
  );
  requireOrdered(
    syncSession,
    [
      "SyncPeerReceiveOutcome::Message(message) =>",
      "current_timestamp = (controls.0)();",
      "messages_received = messages_received.saturating_add(1);",
      "SyncPeerReceiveOutcome::Idle =>",
    ],
    "P123 message receive clock",
    failures,
  );
  const normalizedSync = normalizeWhitespace(syncSession);
  for (const needle of [
    "progress.record_activity(current_timestamp);",
    "self.network.receive_sync_message( peer_id, message, current_timestamp, self.verify_flags",
    "block_reconcile::reconcile_best_chain(self, current_timestamp)?",
  ]) {
    requireContains(normalizedSync, needle, "P123 session timestamp propagation", failures);
  }
  requireAbsent(
    session,
    "MAX_CONSECUTIVE_IDLE_WAKES_PER_SESSION",
    "P123 no fixed idle-wake cutoff",
    failures,
  );
  for (const needle of [
    "fn peer_has_compact_download_in_flight",
    ".compact_download_peer_state(peer_id)",
    ".is_some_and(|state| !state.in_flight.is_empty())",
  ]) {
    requireContains(session, needle, "P123 compact-work-aware idle yield", failures);
  }
  requireContains(
    sync,
    "pub fn sync_until_idle_with_clock_and_cancel",
    "P123 cancellation-aware sync API",
    failures,
  );
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  requireOrdered(
    daemon,
    [
      "let mut shutdown_latched = false;",
      "let mut should_cancel = ||",
      "daemon_sync_shutdown_requested(&shutdown_receiver)",
      "sync_until_idle_with_clock_and_cancel(",
      "if shutdown_latched",
    ],
    "P123 daemon live-session cancellation",
    failures,
  );
}

function verifySuccessfulWriteEvidence(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const session = texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  requireOrdered(
    session,
    [
      "session.send(message, self.config.network.magic())?;",
      "self.network.acknowledge_wire_message_written(message);",
    ],
    "P123 sync send-before-ack",
    failures,
  );

  const context = texts.get("packages/open-bitcoin-rpc/src/context.rs") ?? "";
  for (const needle of [
    "pub(crate) struct EncodedWireResponse",
    "pub(crate) message: WireNetworkMessage",
    "pub(crate) bytes: Vec<u8>",
  ]) {
    requireContains(context, needle, "P123 typed inbound carrier", failures);
  }
  const network = texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  requireOrdered(
    network,
    [
      "let encoded = self.network.encode_messages(&responses)?;",
      ".into_iter()",
      ".zip(encoded)",
      "EncodedWireResponse { message, bytes }",
    ],
    "P123 pre-write complete-batch encoding",
    failures,
  );

  const listener = texts.get("packages/open-bitcoin-rpc/src/inbound_listener.rs") ?? "";
  requireOrdered(
    listener,
    [
      "let Ok(WriteWireMessageOutcome::Written) = write_result else",
      ".acknowledge_wire_message_written(&response.message)",
    ],
    "P123 inbound Written-only acknowledgement",
    failures,
  );
  requireAbsent(
    `${network}\n${listener}`,
    "decode_wire(&response.bytes)",
    "P123 inbound byte decoding",
    failures,
  );
  requireContains(
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener/tests.rs") ?? "",
    "phase123_inbound_encoding_failure_does_not_increment_served",
    "P123 inbound encoding failure guard test",
    failures,
  );
}

function verifyPrivateProjection(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const evidence =
    texts.get("packages/open-bitcoin-node/src/network/block_relay_evidence.rs") ?? "";
  for (const needle of [
    "pub(crate) struct BlockRelayRuntimeEvidenceSnapshot",
    "pub(crate) served_count: u64",
    "pub(super) struct ManagedBlockRelayEvidenceState",
    "served_count: u64",
    "record_wire_message_written",
    "WireNetworkMessage::Block(_)",
  ]) {
    requireContains(evidence, needle, "P123 private served evidence", failures);
  }
  requireAbsent(
    texts.get("packages/open-bitcoin-node/src/status/block_relay_evidence.rs") ?? "",
    "served_count",
    "P123 unchanged public block-relay status",
    failures,
  );

  const metrics = texts.get("packages/open-bitcoin-node/src/metrics/block_relay.rs") ?? "";
  for (const needle of ["served_count: u64", "MetricKind::BlockServedCount", "served_count as f64"]) {
    requireContains(metrics, needle, "P123 explicit served metric projection", failures);
  }
  requireAbsent(
    metrics,
    "eligible_peer_count as f64",
    "P123 served metric eligibility proxy",
    failures,
  );
  const logging = texts.get("packages/open-bitcoin-node/src/logging.rs") ?? "";
  for (const needle of ["pub fn block_relay_log_record", "served_count: u64", "served_count,"]) {
    requireContains(logging, needle, "P123 explicit served log projection", failures);
  }
}

function verifyAuthoritativeRuntime(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  requireExactCount(
    sync,
    "self.network.block_relay_runtime_evidence_snapshot()",
    1,
    "P123 one authoritative runtime snapshot",
    failures,
  );
  requireOrdered(
    sync,
    [
      "let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot();",
      "self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp)",
      "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
    ],
    "P123 shared metric/log snapshot",
    failures,
  );
  requireExactCount(
    sync,
    "maybe_block_relay_snapshot.as_ref()",
    2,
    "P123 shared metric/log snapshot reference",
    failures,
  );
  const daemon = texts.get("packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs") ?? "";
  for (const token of [
    "set_block_relay_metric_status_provider",
    "maybe_block_relay_metric_status_provider",
    "block_relay_context",
  ]) {
    requireAbsent(`${sync}\n${daemon}`, token, "P123 obsolete block-relay provider", failures);
  }
  requireContains(
    sync,
    "maybe_inbound_metric_status_provider",
    "P123 inbound provider preserved",
    failures,
  );
}

function verifyFocusedTests(texts: Map<TargetFile, string>, failures: string[]): void {
  const timing = texts.get("packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs") ?? "";
  for (const needle of [
    "phase123_tcp_zero_progress_timeout_is_idle",
    "phase123_tcp_clean_eof_is_closed",
    "phase123_partial_frame_timeout_is_not_clean_idle",
    "phase123_partial_frame_eof_is_not_clean_closed",
    "phase123_idle_before_timeout_retains_session_without_fallback_or_progress",
    "phase123_idle_after_fake_clock_emits_same_peer_full_block_fallback",
    "phase123_idle_wake_does_not_consume_message_budget",
    "phase123_message_after_idle_uses_session_clock_for_compact_timeout",
    "phase123_idle_session_without_compact_work_yields_after_first_wake",
    "phase123_compact_download_survives_five_second_idle_cadence_until_timeout",
    "phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
    "phase123_closed_receive_ends_session",
    "phase123_target_mismatch_is_not_written_to_current_session",
  ]) requireContains(timing, needle, "P123 runtime timing tests", failures);

  const daemonTests =
    texts.get("packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs") ?? "";
  requireContains(
    daemonTests,
    "phase123_daemon_shutdown_cancels_live_silent_peer_session",
    "P123 daemon cancellation test",
    failures,
  );

  const write = texts.get("packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs") ?? "";
  for (const needle of [
    "phase123_sync_block_write_success_increments_served_once",
    "phase123_sync_block_write_failure_does_not_increment_served",
    "phase123_sync_non_block_write_does_not_increment_served",
    "phase123_sync_partial_batch_counts_each_successful_block_before_failure",
    "phase123_sync_two_successful_blocks_before_later_failure_count_two",
  ]) requireContains(write, needle, "P123 sync write evidence tests", failures);

  const projection = texts.get("packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs") ?? "";
  for (const needle of [
    "phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log",
    "phase123_sync_network_compact_activity_projects_same_snapshot_to_metrics_and_log",
    "phase123_inbound_metric_provider_remains_unchanged",
  ]) requireContains(projection, needle, "P123 runtime projection tests", failures);
}

function verifyPhase121Compatibility(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const checker =
    texts.get("scripts/check-phase121-block-relay-metrics-log-runtime.ts") ?? "";
  for (const needle of [
    "P121 authoritative snapshot",
    "P121 activation omission",
    "P121 same snapshot reuse",
    "P121 obsolete provider wiring",
    "P121 no-claim boundary",
  ]) requireContains(checker, needle, "P123 migrated Phase 121 guarantees", failures);
  const docs = normalizeWhitespace(
    texts.get("docs/architecture/operator-observability.md") ?? "",
  );
  for (const needle of ["runtime-only", "non-serialized", "not the sync projection source"]) {
    requireContains(docs, needle, "P123 operator evidence provenance", failures);
  }
}

function normalizeWhitespace(text: string): string {
  return text.replaceAll(/\s+/g, " ").trim();
}

function verifyParity(texts: Map<TargetFile, string>, failures: string[]): void {
  const indexText = texts.get("docs/parity/index.json") ?? "";
  let index: ParityIndex;
  try {
    index = JSON.parse(indexText) as ParityIndex;
  } catch (error) {
    failures.push(`P123 parity index JSON parse failed: ${String(error)}`);
    return;
  }
  const named = (index.surfaces ?? []).filter(
    (entry) => (entry as NamedSurface).name === SURFACE_ID,
  ) as NamedSurface[];
  if (named.length !== 1 || named[0]?.status !== "done") {
    failures.push(`P123 parity index must contain one done surface: ${SURFACE_ID}`);
  }
  const matches = (index.checklist?.surfaces ?? []).filter(
    (entry) => (entry as ChecklistSurface).id === SURFACE_ID,
  ) as ChecklistSurface[];
  if (matches.length !== 1 || matches[0]?.status !== "done") {
    failures.push(`P123 parity checklist must contain one done surface: ${SURFACE_ID}`);
    return;
  }
  const surface = matches[0];
  if (JSON.stringify(surface?.requirements) !== JSON.stringify(REQUIREMENTS)) {
    failures.push("P123 parity requirements must be exactly HARD-02,HARD-03,HARD-04");
  }
  const parityCorpus = [
    JSON.stringify(surface),
    texts.get("docs/parity/checklist.md") ?? "",
    texts.get("docs/parity/catalog/p2p.md") ?? "",
  ].join("\n");
  for (const needle of [
    "receive-independent idle expiration",
    "successful typed Block writes",
    "runtime-only served evidence",
    "unchanged public status",
    "one authoritative sync-network snapshot",
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
  ]) requireContains(parityCorpus, needle, "P123 exact parity evidence", failures);
  for (const needle of [
    "default-off",
    "package relay",
    "filter serving",
    "public-network CI",
    "production service",
    "production full-node readiness",
    "production-funds",
    "blocking runtime",
    "existing timeout constant",
  ]) requireContains(parityCorpus, needle, "P123 parity no-claim boundary", failures);
}

function verifyBreadcrumbs(text: string, failures: string[]): void {
  let manifest: BreadcrumbManifest;
  try {
    manifest = JSON.parse(text) as BreadcrumbManifest;
  } catch (error) {
    failures.push(`P123 breadcrumb JSON parse failed: ${String(error)}`);
    return;
  }
  for (const file of [
    "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs",
    "packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs",
  ]) {
    const matches = (manifest.groups ?? []).filter((group) => group.files?.includes(file));
    if (matches.length !== 1) {
      failures.push(`P123 breadcrumb must contain exactly one group for ${file}`);
      continue;
    }
    const breadcrumbs = matches[0]?.breadcrumbs ?? [];
    if (!breadcrumbs.includes("packages/bitcoin-knots/src/net.cpp") ||
        !breadcrumbs.includes("packages/bitcoin-knots/src/net_processing.cpp")) {
      failures.push(`P123 breadcrumb missing Knots runtime anchors for ${file}`);
    }
  }
}

function verifyVerifierWiring(text: string, failures: string[]): void {
  requireExactCount(text, PHASE123_TEST, 2, "P123 verifier mutation command", failures);
  requireExactCount(text, PHASE123_CHECK, 2, "P123 verifier live checker command", failures);
  requireExactCount(
    text,
    'run_step "test Phase 123 runtime timing and evidence integrity checker"',
    1,
    "P123 verifier test label",
    failures,
  );
  requireExactCount(
    text,
    'run_step "check Phase 123 runtime timing and evidence integrity"',
    1,
    "P123 verifier checker label",
    failures,
  );
  requireRepeatedOrder(
    text,
    [PHASE122_CHECK, PHASE123_TEST, PHASE123_CHECK, PHASE117_TEST],
    2,
    "P123 verifier Phase 122/123/117 order",
    failures,
  );
}

function requireContains(text: string, needle: string, label: string, failures: string[]): void {
  if (!text.includes(needle)) failures.push(`${label} missing ${needle}`);
}

function requireAbsent(text: string, needle: string, label: string, failures: string[]): void {
  if (text.includes(needle)) failures.push(`${label} must not contain ${needle}`);
}

function requireExactCount(
  text: string,
  needle: string,
  expected: number,
  label: string,
  failures: string[],
): void {
  const actual = text.split(needle).length - 1;
  if (actual !== expected) {
    failures.push(`${label} expected ${expected} occurrence(s) of ${needle}, found ${actual}`);
  }
}

function requireOrdered(
  text: string,
  needles: readonly string[],
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (const needle of needles) {
    const index = text.indexOf(needle, cursor + 1);
    if (index === -1) failures.push(`${label} missing or out of order ${needle}`);
    else cursor = index;
  }
}

function requireRepeatedOrder(
  text: string,
  needles: readonly string[],
  repetitions: number,
  label: string,
  failures: string[],
): void {
  let cursor = -1;
  for (let repetition = 0; repetition < repetitions; repetition += 1) {
    for (const needle of needles) {
      const index = text.indexOf(needle, cursor + 1);
      if (index === -1) {
        failures.push(`${label} missing repetition ${repetition + 1}: ${needle}`);
        return;
      }
      cursor = index;
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity();
  if (failures.length > 0) {
    console.error("Phase 123 runtime timing and evidence integrity checker failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log("Phase 123 runtime timing and evidence integrity checker passed.");
}

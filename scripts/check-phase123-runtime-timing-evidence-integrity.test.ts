import { afterEach, expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

import { checkPhase123RuntimeTimingEvidenceIntegrity } from "./check-phase123-runtime-timing-evidence-integrity";

const TARGET_FILES = [
  "packages/open-bitcoin-node/src/sync/types.rs",
  "packages/open-bitcoin-node/src/sync/tcp.rs",
  "packages/open-bitcoin-node/src/sync.rs",
  "packages/open-bitcoin-node/src/sync/session.rs",
  "packages/open-bitcoin-node/src/sync/block_reconcile.rs",
  "packages/open-bitcoin-node/src/sync/block_response.rs",
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
type Mutator = (files: Map<TargetFile, string>) => void;
const tempRoots: string[] = [];

afterEach(() => {
  for (const root of tempRoots.splice(0)) rmSync(root, { force: true, recursive: true });
});

test("complete synthetic Phase 123 corpus passes", () => {
  // Arrange
  const root = createFixture();

  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: root });

  // Assert
  expect(failures).toEqual([]);
});

test("phase123_real_repository_corpus_passes", () => {
  // Arrange
  const repositoryRoot = path.resolve(import.meta.dir, "..");

  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({
    rootDir: repositoryRoot,
  });

  // Assert
  expect(failures).toEqual([]);
});

test.each([
  [
    "Idle collapsed into Closed",
    "P123 typed receive outcomes missing Idle",
    mutate("packages/open-bitcoin-node/src/sync/types.rs", "Idle", "Closed"),
  ],
  [
    "idle clock removed",
    "P123 idle clock target send order missing or out of order current_timestamp = (controls.0)();",
    mutate("packages/open-bitcoin-node/src/sync/session.rs", "current_timestamp = (controls.0)();", "current_timestamp = timestamp;"),
  ],
  [
    "message receive clock removed",
    "P123 message receive clock missing or out of order messages_received = messages_received.saturating_add(1);",
    mutateFirst(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "SyncPeerReceiveOutcome::Message(message) => current_timestamp = (controls.0)(); messages_received = messages_received.saturating_add(1);",
      "SyncPeerReceiveOutcome::Message(message) => messages_received = messages_received.saturating_add(1);",
    ),
  ],
  [
    "idle target guard removed",
    "P123 idle clock target send order missing or out of order .any(|(target_peer_id, _message)| *target_peer_id != peer_id)",
    mutate("packages/open-bitcoin-node/src/sync/session.rs", ".any(|(target_peer_id, _message)| *target_peer_id != peer_id)", ".is_empty()"),
  ],
  [
    "session timestamp propagation removed",
    "P123 session timestamp propagation missing progress.record_activity(current_timestamp);",
    mutate(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "progress.record_activity(current_timestamp);",
      "progress.record_activity(timestamp);",
    ),
  ],
  [
    "response-work session retention removed",
    "P123 idle clock target send order missing or out of order if !self.peer_has_pending_download_work(peer_id)",
    mutate(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "if !self.peer_has_pending_download_work(peer_id)",
      "if true",
    ),
  ],
  [
    "compact fallback ordinary tracking removed",
    "P123 idle clock target send order missing or out of order block_reconcile::request_tracked_blocks(",
    mutate(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "block_reconcile::request_tracked_blocks(",
      "block_reconcile::request_missing_blocks(",
    ),
  ],
  [
    "ordinary response retention removed",
    "P123 response-work-aware idle yield missing .peer_requested_blocks(peer_id)",
    mutate(
      "packages/open-bitcoin-node/src/sync/session.rs",
      ".peer_requested_blocks(peer_id)",
      ".peer_requested_blocks(PeerId::MAX)",
    ),
  ],
  [
    "compact fallback active-tip classification removed",
    "P123 compact fallback active-tip response classification missing || self.block_extends_active_tip(block)",
    mutate(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "|| self.block_extends_active_tip(block)",
      "|| false",
    ),
  ],
  [
    "five-second compact cadence regression removed",
    "P123 runtime timing tests missing phase123_compact_download_survives_five_second_idle_cadence_until_timeout",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_compact_download_survives_five_second_idle_cadence_until_timeout",
      "phase123_compact_download_cadence_test_removed",
    ),
  ],
  [
    "slow-message timestamp regression removed",
    "P123 runtime timing tests missing phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
      "phase123_slow_message_timestamp_test_removed",
    ),
  ],
  [
    "compact fallback response regression removed",
    "P123 runtime timing tests missing phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      "phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
      "phase123_compact_fallback_response_test_removed",
    ),
  ],
  [
    "compact fallback persistence assertion removed",
    "P123 compact fallback response regression missing .load_block(expected_hash)",
    mutate(
      "packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs",
      ".load_block(expected_hash)",
      ".load_block(BlockHash::from_byte_array([0_u8; 32]))",
    ),
  ],
  [
    "daemon live-session cancellation removed",
    "P123 daemon live-session cancellation missing or out of order sync_until_idle_with_clock_and_cancel(",
    mutate(
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      "sync_until_idle_with_clock_and_cancel(",
      "sync_until_idle_with_clock(",
    ),
  ],
  [
    "bench restores old Option receive",
    "P123 first-party receive migration packages/open-bitcoin-bench/src/runtime_fixtures.rs missing Result<SyncPeerReceiveOutcome, SyncRuntimeError>",
    mutate("packages/open-bitcoin-bench/src/runtime_fixtures.rs", "Result<SyncPeerReceiveOutcome, SyncRuntimeError>", "Result<Option<WireNetworkMessage>, SyncRuntimeError>"),
  ],
  [
    "public receive re-export removed",
    "P123 public receive re-export missing SyncPeerReceiveOutcome",
    mutate("packages/open-bitcoin-node/src/lib.rs", "SyncPeerReceiveOutcome", "SyncPeerSession"),
  ],
  [
    "sync acknowledgement moved before send",
    "P123 sync send-before-ack missing or out of order self.network.acknowledge_wire_message_written(message)?;",
    (files: Map<TargetFile, string>) => files.set(
      "packages/open-bitcoin-node/src/sync/session.rs",
      "self.network.acknowledge_wire_message_written(message)?; session.send(message, self.config.network.magic())?;",
    ),
  ],
  [
    "inbound Written acknowledgement removed",
    "P123 inbound Written-only acknowledgement missing or out of order .acknowledge_wire_message_written(&response.message)",
    mutate("packages/open-bitcoin-rpc/src/inbound_listener.rs", ".acknowledge_wire_message_written(&response.message)", ".record_response_intent(&response.message)"),
  ],
  [
    "encoding bypassed with fabricated bytes before write",
    "P123 pre-write complete-batch encoding missing or out of order let encoded = self.network.encode_messages(&responses)?;",
    mutate("packages/open-bitcoin-rpc/src/context/network.rs", "let encoded = self.network.encode_messages(&responses)?;", "let encoded = fabricated_bytes(&responses); // bypass encoding failure"),
  ],
  [
    "acknowledgement moved before encoding",
    "P123 pre-write complete-batch encoding missing or out of order .into_iter()",
    (files: Map<TargetFile, string>) => files.set(
      "packages/open-bitcoin-rpc/src/context/network.rs",
      ".into_iter() .zip(encoded) EncodedWireResponse { message, bytes } let encoded = self.network.encode_messages(&responses)?;",
    ),
  ],
  [
    "exact encoding failure test removed",
    "P123 inbound encoding failure guard test missing phase123_inbound_encoding_failure_does_not_increment_served",
    mutate("packages/open-bitcoin-rpc/src/inbound_listener/tests.rs", "phase123_inbound_encoding_failure_does_not_increment_served", "phase123_inbound_encoding_failure_is_ignored"),
  ],
  [
    "encoded bytes are decoded at acknowledgement",
    "P123 inbound byte decoding must not contain decode_wire(&response.bytes)",
    append("packages/open-bitcoin-rpc/src/inbound_listener.rs", "decode_wire(&response.bytes);"),
  ],
  [
    "eligible peer proxy restored",
    "P123 served metric eligibility proxy must not contain eligible_peer_count as f64",
    mutate("packages/open-bitcoin-node/src/metrics/block_relay.rs", "served_count as f64", "eligible_peer_count as f64"),
  ],
  [
    "served_count added to public status",
    "P123 unchanged public block-relay status must not contain served_count",
    append("packages/open-bitcoin-node/src/status/block_relay_evidence.rs", "pub served_count: u64,"),
  ],
  [
    "obsolete provider restored",
    "P123 obsolete block-relay provider must not contain set_block_relay_metric_status_provider",
    append("packages/open-bitcoin-node/src/sync.rs", "set_block_relay_metric_status_provider(provider);"),
  ],
  [
    "daemon sync activation removed",
    "P123 daemon sync activation wiring missing or out of order runtime.block_serving",
    mutate(
      "packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs",
      "runtime.block_serving",
      "BlockRelayActivationPolicy::default()",
    ),
  ],
  [
    "inbound block-serving activation removed",
    "P123 inbound production activation wiring missing or out of order config.block_serving",
    mutate(
      "packages/open-bitcoin-rpc/src/context/network.rs",
      "config.block_serving",
      "BlockRelayActivationPolicy::default()",
    ),
  ],
  [
    "structured log resamples",
    "P123 shared metric/log snapshot missing or out of order self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);",
    mutate("packages/open-bitcoin-node/src/sync.rs", "self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp);", "let second = self.maybe_authoritative_block_relay_snapshot(); self.write_block_relay_log(&mut summary, second.as_ref(), timestamp);"),
  ],
  [
    "unavailable omission test removed",
    "P123 runtime projection tests missing phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log",
    mutate("packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs", "phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log", "phase123_unobserved_authoritative_network_emits_zero_block_relay_metrics_and_log"),
  ],
  [
    "one HARD requirement mutated",
    "P123 parity requirements must be exactly HARD-02,HARD-03,HARD-04",
    mutate("docs/parity/index.json", "HARD-03", "HARD-05"),
  ],
  [
    "mutation command removed from verifier",
    "P123 verifier mutation command expected 2 occurrence(s) of bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts, found 1",
    mutateFirst("scripts/verify.sh", "bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts", ""),
  ],
  [
    "live checker command removed from verifier",
    "P123 verifier live checker command expected 2 occurrence(s) of bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts, found 1",
    mutateFirst("scripts/verify.sh", "bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts", ""),
  ],
] as const)("fails the %s mutation", (_label, expectedFailure, maybeMutate) => {
  // Arrange
  const root = createFixture(maybeMutate as Mutator);

  // Act
  const failures = checkPhase123RuntimeTimingEvidenceIntegrity({ rootDir: root });

  // Assert
  expect(failures).toContain(expectedFailure);
});

function createFixture(maybeMutate?: Mutator): string {
  const root = mkdtempSync(path.join(tmpdir(), "open-bitcoin-phase123-"));
  tempRoots.push(root);
  const files = completeFiles();
  maybeMutate?.(files);
  for (const [relativePath, contents] of files) {
    const absolutePath = path.join(root, relativePath);
    mkdirSync(path.dirname(absolutePath), { recursive: true });
    writeFileSync(absolutePath, contents);
  }
  return root;
}

function completeFiles(): Map<TargetFile, string> {
  const timingTests = [
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
    "phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
    "phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
    "phase123_closed_receive_ends_session",
    "phase123_target_mismatch_is_not_written_to_current_session",
  ].join(" ");
  const writeTests = [
    "phase123_sync_block_write_success_increments_served_once",
    "phase123_sync_block_write_failure_does_not_increment_served",
    "phase123_sync_non_block_write_does_not_increment_served",
    "phase123_sync_partial_batch_counts_each_successful_block_before_failure",
    "phase123_sync_two_successful_blocks_before_later_failure_count_two",
  ].join(" ");
  const projectionTests = [
    "phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log",
    "phase123_sync_network_compact_activity_projects_same_snapshot_to_metrics_and_log",
    "phase123_inbound_metric_provider_remains_unchanged",
  ].join(" ");
  const parityNarrative = [
    "receive-independent idle expiration",
    "successful typed Block writes",
    "runtime-only served evidence",
    "unchanged public status",
    "one authoritative sync-network snapshot",
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
    "packages/bitcoin-knots/test/functional/p2p_compactblocks.py",
    "default-off package relay filter serving public-network CI production service",
    "production full-node readiness production-funds blocking runtime existing timeout constant",
  ].join(" ");
  const parityIndex = JSON.stringify({
    surfaces: [{ name: "v2-1-runtime-timing-evidence-integrity", status: "done" }],
    checklist: {
      surfaces: [{
        id: "v2-1-runtime-timing-evidence-integrity",
        title: "v2.1 Runtime Timing and Evidence Integrity",
        status: "done",
        requirements: ["HARD-02", "HARD-03", "HARD-04"],
        evidence: ["scripts/check-phase123-runtime-timing-evidence-integrity.ts"],
        rationale: parityNarrative,
        upstream: {
          sources: ["packages/bitcoin-knots/src/net.cpp", "packages/bitcoin-knots/src/net_processing.cpp"],
          tests: ["packages/bitcoin-knots/test/functional/p2p_compactblocks.py"],
        },
        known_gaps: ["blocking runtime existing timeout constant"],
        suspected_unknowns: ["default-off package relay filter serving public-network CI production service production full-node readiness production-funds"],
      }],
    },
  });
  const breadcrumbs = JSON.stringify({ groups: [
    breadcrumb("packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs", true),
    breadcrumb("packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs", false),
    breadcrumb("packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs", true),
  ] });
  const verifyRegion = `
bun run scripts/check-phase122-compact-relay-peer-completion.ts
bun test scripts/check-phase123-runtime-timing-evidence-integrity.test.ts
bun run scripts/check-phase123-runtime-timing-evidence-integrity.ts
bun test scripts/check-phase117-parity-uat-release-boundary.test.ts`;
  return new Map<TargetFile, string>([
    ["packages/open-bitcoin-node/src/sync/types.rs", "pub enum SyncPeerReceiveOutcome { Message(WireNetworkMessage), Idle, Closed } fn receive() -> Result<SyncPeerReceiveOutcome, SyncRuntimeError>"],
    ["packages/open-bitcoin-node/src/sync/tcp.rs", "fn receive() -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> { Ok(0) if allow_clean_idle && filled == 0 => return Ok(ReadStageOutcome::Closed); allow_clean_idle filled == 0 io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut return Ok(ReadStageOutcome::Idle); unexpected EOF after {filled} of {} frame bytes payload read ended without a complete frame }"],
    ["packages/open-bitcoin-node/src/sync.rs", `pub fn open_with_block_relay_activation ManagedPeerNetwork::with_sync_limits_and_block_relay_activation block_relay_activation maybe_inbound_metric_status_provider self.network.block_relay_runtime_evidence_snapshot()? let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?; self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp); self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp); pub fn sync_until_idle_with_clock_and_cancel`],
    ["packages/open-bitcoin-node/src/sync/session.rs", `session.send(message, self.config.network.magic())?; self.network.acknowledge_wire_message_written(message)?; SyncPeerReceiveOutcome::Message(message) => current_timestamp = (controls.0)(); messages_received = messages_received.saturating_add(1); SyncPeerReceiveOutcome::Idle => current_timestamp = (controls.0)(); .expire_compact_download_timeouts(current_timestamp)? .any(|(target_peer_id, _message)| *target_peer_id != peer_id) let fallback_block_hashes = targeted block_reconcile::request_tracked_blocks( self.send_all(&mut session, &outbound)?; if !self.peer_has_pending_download_work(peer_id) self.complete_peer_session_progress(&mut progress, peer_id); return Ok(()); continue; SyncPeerReceiveOutcome::Closed => progress.record_activity(current_timestamp); let block_response_was_requested = || self.block_extends_active_tip(block) block_reconcile::release_inflight_for_message(self, &message); self.network.receive_sync_message( peer_id, message, current_timestamp, self.verify_flags self.record_block_disposition( self.persist_progress()?; block_reconcile::reconcile_best_chain(self, current_timestamp)? fn peer_has_pending_download_work .compact_download_peer_state(peer_id) .is_some_and(|state| !state.in_flight.is_empty()) .peer_requested_blocks(peer_id) .any(|block_hash| self.inflight_blocks.contains(block_hash))`],
    ["packages/open-bitcoin-node/src/sync/block_reconcile.rs", `pub(super) fn request_tracked_blocks .request_missing_blocks(peer_id, &requested)? .inflight_blocks .insert(BlockHash::from(item.object_hash))`],
    ["packages/open-bitcoin-node/src/sync/block_response.rs", `fn block_extends_active_tip`],
    ["packages/open-bitcoin-node/src/lib.rs", "SyncPeerReceiveOutcome"],
    ["packages/open-bitcoin-bench/src/runtime_fixtures.rs", "Result<SyncPeerReceiveOutcome, SyncRuntimeError>"],
    ["packages/open-bitcoin-node/src/network/block_relay_evidence.rs", "pub(crate) struct BlockRelayRuntimeEvidenceSnapshot { pub(crate) served_count: u64 } pub(super) struct ManagedBlockRelayEvidenceState { served_count: u64 } record_wire_message_written WireNetworkMessage::Block(_)"],
    ["packages/open-bitcoin-node/src/metrics/block_relay.rs", "served_count: u64 MetricKind::BlockServedCount served_count as f64"],
    ["packages/open-bitcoin-node/src/logging.rs", "pub fn block_relay_log_record served_count: u64 served_count,"],
    ["packages/open-bitcoin-node/src/status/block_relay_evidence.rs", "pub struct BlockRelayEvidenceStatus {}"],
    ["packages/open-bitcoin-rpc/src/context.rs", "pub(crate) struct EncodedWireResponse { pub(crate) message: WireNetworkMessage, pub(crate) bytes: Vec<u8> }"],
    ["packages/open-bitcoin-rpc/src/context/network.rs", "ManagedPeerNetwork::new_with_block_relay_activation( config.block_serving let encoded = self.network.encode_messages(&responses)?; .into_iter() .zip(encoded) EncodedWireResponse { message, bytes }"],
    ["packages/open-bitcoin-rpc/src/inbound_listener.rs", "let Ok(WriteWireMessageOutcome::Written) = write_result else { return; }; .acknowledge_wire_message_written(&response.message)"],
    ["packages/open-bitcoin-rpc/src/inbound_listener/tests.rs", "phase123_inbound_encoding_failure_does_not_increment_served phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block phase123_disabled_runtime_config_does_not_serve_inbound_block"],
    ["packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs", "DurableSyncRuntime::open_with_block_relay_activation( runtime.block_serving set_inbound_metric_status_provider let mut shutdown_latched = false; let mut should_cancel = || daemon_sync_shutdown_requested(&shutdown_receiver) sync_until_idle_with_clock_and_cancel( if shutdown_latched"],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests.rs", "phase123_daemon_shutdown_cancels_live_silent_peer_session"],
    ["packages/open-bitcoin-node/src/sync/tests/runtime_timing_cases.rs", `DurableSyncRuntime::open_with_block_relay_activation( Result<SyncPeerReceiveOutcome, SyncRuntimeError> ${timingTests} outcomes.extend((0..13) WireNetworkMessage::Block( compact_block.clone(), ) summary.peer_outcomes[0].contribution.blocks_received, 1 .load_block(expected_hash)`],
    ["packages/open-bitcoin-node/src/sync/tests/runtime_write_evidence_cases.rs", `Result<SyncPeerReceiveOutcome, SyncRuntimeError> ${writeTests}`],
    ["packages/open-bitcoin-node/src/sync/tests/runtime_projection_cases.rs", `DurableSyncRuntime::open_with_block_relay_activation( ${projectionTests}`],
    ["scripts/check-phase121-block-relay-metrics-log-runtime.ts", "P121 authoritative snapshot P121 activation omission P121 same snapshot reuse P121 obsolete provider wiring P121 no-claim boundary"],
    ["docs/architecture/operator-observability.md", "runtime-only non-serialized not the sync projection source"],
    ["docs/parity/index.json", parityIndex],
    ["docs/parity/checklist.md", parityNarrative],
    ["docs/parity/catalog/p2p.md", parityNarrative],
    ["docs/parity/source-breadcrumbs.json", breadcrumbs],
    ["scripts/verify.sh", `${verifyRegion}\n${verifyRegion}\nrun_step "test Phase 123 runtime timing and evidence integrity checker"\nrun_step "check Phase 123 runtime timing and evidence integrity"`],
  ]);
}

function breadcrumb(file: string, includeFunctional: boolean) {
  const anchors = [
    "packages/bitcoin-knots/src/net.cpp",
    "packages/bitcoin-knots/src/net_processing.cpp",
  ];
  if (includeFunctional) anchors.push("packages/bitcoin-knots/test/functional/p2p_compactblocks.py");
  return { files: [file], breadcrumbs: anchors };
}

function mutate(file: TargetFile, from: string, to: string): Mutator {
  return (files) => files.set(file, (files.get(file) ?? "").replaceAll(from, to));
}

function mutateFirst(file: TargetFile, from: string, to: string): Mutator {
  return (files) => files.set(file, (files.get(file) ?? "").replace(from, to));
}

function append(file: TargetFile, addition: string): Mutator {
  return (files) => files.set(file, `${files.get(file) ?? ""}\n${addition}`);
}

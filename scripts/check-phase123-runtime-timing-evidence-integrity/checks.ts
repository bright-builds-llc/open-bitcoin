import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { DEFAULT_REPO_ROOT, TARGET_FILES, TargetFile, CheckOptions } from "./constants.ts";
import { readText } from "./filesystem.ts";
import { verifySuccessfulWriteEvidence, verifyPrivateProjection, verifyAuthoritativeRuntime, verifyFocusedTests } from "./evidence.ts";
import { verifyPhase121Compatibility, verifyParity, verifyBreadcrumbs, verifyVerifierWiring } from "./parity.ts";
import { normalizeWhitespace, requireContains, requireAbsent, requireOrdered } from "./helpers.ts";

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

export function verifyProductionActivation(
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
    ["DurableSyncRuntime::open_with_runtime_activation(", "runtime.block_serving"],
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

export function verifyTypedReceiveAndTcp(
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

export function verifyIdleMaintenance(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  const session = texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  const emissionTerminal =
    texts.get("packages/open-bitcoin-node/src/sync/session/emission_terminal.rs") ?? "";
  const blockReconcile = texts.get("packages/open-bitcoin-node/src/sync/block_reconcile.rs") ?? "";
  const blockResponse = texts.get("packages/open-bitcoin-node/src/sync/block_response.rs") ?? "";
  const syncSession = `${sync}\n${session}`;
  requireOrdered(
    syncSession,
    [
      "SyncPeerReceiveOutcome::Idle =>",
      "current_timestamp = (controls.0)();",
      ".expire_compact_download_timeouts(current_timestamp)?",
      ".any(|(target_peer_id, _message)| *target_peer_id != peer_id)",
      "let fallback_block_hashes = targeted",
      "block_reconcile::request_tracked_blocks(",
      "self.send_all_for_peer(&mut session, peer_id, &outbound)?;",
      "if !self.peer_has_pending_download_work(peer_id)",
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
    "block_reconcile::reconcile_best_chain_for_live_session( self, current_timestamp, )?",
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
    "fn peer_has_pending_download_work",
    ".compact_download_peer_state(peer_id)",
    ".is_some_and(|state| !state.in_flight.is_empty())",
    ".peer_requested_blocks(peer_id)",
    ".any(|block_hash| self.inflight_blocks.contains(block_hash))",
  ]) {
    requireContains(session, needle, "P123 response-work-aware idle yield", failures);
  }
  for (const needle of [
    "pub(super) fn request_tracked_blocks",
    ".request_missing_blocks(peer_id, &requested)?",
    ".inflight_blocks",
    ".insert(BlockHash::from(item.object_hash))",
  ]) {
    requireContains(blockReconcile, needle, "P123 tracked compact fallback", failures);
  }
  requireOrdered(
    syncSession,
    [
      "let block_response_was_requested =",
      "block_reconcile::release_inflight_for_message(self, &message);",
      "self.network.receive_sync_message(",
      "self.record_block_disposition(",
      "let reconcile_progress = block_reconcile::reconcile_best_chain_for_live_session(",
      "self.record_reconcile_progress(reconcile_progress);",
      "self.persist_progress_and_dispatch_tip()?;",
    ],
    "P123 requested fallback response consumption",
    failures,
  );
  requireOrdered(
    emissionTerminal,
    [
      "pub(super) fn send_peer_emissions",
      "session.send(&message, network_magic)",
      "capability.acknowledge_write()",
    ],
    "P123 sync peer emission post-write completion",
    failures,
  );
  requireContains(
    blockResponse,
    "fn block_extends_active_tip",
    "P123 compact fallback active-tip classification",
    failures,
  );
  requireContains(
    session,
    "|| self.block_extends_active_tip(block)",
    "P123 compact fallback active-tip response classification",
    failures,
  );
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

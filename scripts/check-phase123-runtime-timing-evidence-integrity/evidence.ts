import { existsSync, readFileSync } from "node:fs";
import path from "node:path";
import { TargetFile } from "./constants.ts";
import { normalizeWhitespace, requireContains, requireAbsent, requireExactCount, requireOrdered } from "./helpers.ts";

export function verifySuccessfulWriteEvidence(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const session = texts.get("packages/open-bitcoin-node/src/sync/session.rs") ?? "";
  requireOrdered(
    session,
    [
      "session.send(message, self.config.network.magic())?;",
      "self.network.acknowledge_wire_message_written(message)?;",
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

  const listener = [
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener.rs") ?? "",
    texts.get(
      "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
    ) ?? "",
  ].join("\n");
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

export function verifyPrivateProjection(
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

export function verifyAuthoritativeRuntime(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const sync = texts.get("packages/open-bitcoin-node/src/sync.rs") ?? "";
  requireExactCount(
    sync,
    "self.network.block_relay_runtime_evidence_snapshot()?",
    1,
    "P123 one typed authoritative runtime snapshot",
    failures,
  );
  requireOrdered(
    sync,
    [
      "let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?;",
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

export function verifyFocusedTests(texts: Map<TargetFile, string>, failures: string[]): void {
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
    "phase123_compact_timeout_fallback_consumes_matching_block_before_yield",
    "phase123_slow_messages_without_idle_timestamp_compact_at_receipt",
    "phase123_closed_receive_ends_session",
    "phase123_target_mismatch_is_not_written_to_current_session",
  ]) requireContains(timing, needle, "P123 runtime timing tests", failures);
  const normalizedTiming = normalizeWhitespace(timing);
  for (const needle of [
    "outcomes.extend((0..13)",
    "WireNetworkMessage::Block( compact_block.clone(), )",
    "summary.peer_outcomes[0].contribution.blocks_received, 1",
    ".load_block(expected_hash)",
  ]) {
    requireContains(
      normalizedTiming,
      needle,
      "P123 compact fallback response regression",
      failures,
    );
  }

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

import { TargetFile } from "./constants.ts";
import { normalizeWhitespace, requireContains, requireAbsent, requireExactCount, requireOrdered } from "./helpers.ts";
import { maybeFunction, ordinaryRust } from "../check-phase157-index-catch-up/rust-evidence.ts";
import { normalizeRust, rustCallArguments, rustFunction } from "../rust-source-invariants.ts";

function ordinaryOwner(source: string, owner: string): string {
  const code = ordinaryRust(source);
  const escaped = owner.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const maybeOwner = new RegExp(`(?:^|[};])\\s*((?:#\\[[^\\]]*\\]\\s*)*)${escaped}\\s*\\{`).exec(code);
  if (maybeOwner === null || normalizeRust(maybeOwner[1] ?? "").includes("#[cfg(test)]")) return "";
  return rustFunction(code, owner);
}

export function verifySuccessfulWriteEvidence(
  texts: Map<TargetFile, string>,
  failures: string[],
): void {
  const terminal = texts.get("packages/open-bitcoin-node/src/sync/session/emission_terminal.rs") ?? "";
  const body = (source: string, name: string): string =>
    normalizeWhitespace(maybeFunction(source, name, true)?.body ?? "");
  const runtimeOwner = ordinaryOwner(terminal, "impl super::DurableSyncRuntime");
  requireContains(runtimeOwner, "impl super::DurableSyncRuntime", "P123 ordinary transport runtime owner", failures);
  const session = body(runtimeOwner, "send_all_with_cancel");
  requireOrdered(
    session,
    [
      "let buffers = messages .iter() .map(|message|",
      "self.network .encode_pending_network_message(peer_id, message)",
      ".collect::<Result<Vec<_>, _>>() .map_err(super::filter_serving::transport_error)?;",
      "for (message, buffer) in messages.iter().zip(buffers)",
      "if should_cancel()",
      "session.send_encoded(buffer.bytes(), self.config.network.magic())?;",
      "self.network.acknowledge_wire_message_written(message)?;",
    ],
    "P123 sync send-before-ack",
    failures,
  );
  requireExactCount(session, "acknowledge_wire_message_written(", 1, "P123 ordinary single post-write acknowledgement", failures);
  requireOrdered(body(runtimeOwner, "send_all_for_peer_with_cancel"), [
    "self.send_all_with_cancel(session, peer_id, messages, should_cancel)?;",
    "self.announcement_outboxes.take_peer_emissions(peer_id)?;",
    "send_peer_emissions( &self.network, session, peer_id, self.config.network.magic(), emissions, should_cancel, )",
  ], "P123 governed outbound production delegation", failures);
  const tcp = texts.get("packages/open-bitcoin-node/src/sync/tcp.rs") ?? "";
  const tcpOwner = ordinaryOwner(tcp, "impl SyncPeerSession for TcpPeerSession");
  requireContains(tcpOwner, "impl SyncPeerSession for TcpPeerSession", "P123 concrete exact transport owner", failures);
  requireOrdered(body(tcpOwner, "send_encoded"), [
    "self.stream .write_all(bytes) .map_err(",
  ], "P123 concrete exact transport write", failures);

  const inboundCarrier = [
    texts.get("packages/open-bitcoin-rpc/src/context/inbound_wire.rs") ?? "",
  ].join("\n");
  const carrierOwner = ordinaryOwner(inboundCarrier, "pub(crate) struct EncodedWireResponse");
  for (const needle of [
    "pub(crate) struct EncodedWireResponse",
    "pub(crate) message: WireNetworkMessage",
    "pub(crate) bytes: GenericPendingBuffer",
  ]) {
    requireContains(carrierOwner, needle, "P123 typed inbound carrier", failures);
  }
  const network = texts.get("packages/open-bitcoin-rpc/src/context/network.rs") ?? "";
  const resolvedOwner = ordinaryOwner(inboundCarrier, "impl ResolvedInboundWireResponses");
  const planOwner = ordinaryOwner(inboundCarrier, "impl<S: ChainstateStore, V: CoinsView> InboundWireResponsePlan<S, V>");
  requireOrdered(
    body(resolvedOwner, "push_encoded"),
    [
      "match network.encode_pending_network_message(peer_id, &message)",
      "Ok(bytes) =>",
      ".push(InboundWireStep::Encoded(Box::new(EncodedWireResponse { message, bytes, maybe_block_serve_intent, maybe_tx_write_capability, })))",
      "Err(_) =>",
      "self.failed = true;",
    ],
    "P123 pre-write complete-batch encoding",
    failures,
  );
  requireOrdered(body(planOwner, "resolve"), [
    "for response in core::mem::take(&mut self.responses)",
    "resolved.push_encoded(message, None, None, &self.network, self.peer_id);",
    "self.resolve_block_intent(intent, &mut resolved);",
    "resolved",
  ], "P123 complete inbound plan resolution", failures);
  requireContains(body(planOwner, "resolve_block_intent"),
    "resolved.push_encoded(response, Some(intent), None, &self.network, self.peer_id);",
    "P123 durable inbound governed encoding", failures);
  requireOrdered(body(inboundCarrier, "resolve_inbound_wire_responses"), [
    ".prepare_inbound_wire_message(peer_id, message, timestamp)",
    "let mut resolved = plan.resolve();",
    "if resolved.failed",
    "return None;",
    "Some(resolved.responses)",
  ], "P123 inbound complete-batch resolution", failures);

  const listener = [
    texts.get("packages/open-bitcoin-rpc/src/inbound_listener.rs") ?? "",
    texts.get(
      "packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs",
    ) ?? "",
  ].join("\n");
  requireOrdered(
    body(listener, "handle_inbound_stream"),
    [
      "resolve_inbound_wire_responses(",
      "steps: VecDeque::from(encoded_responses)",
      "while let Some(step) = steps.steps.pop_front()",
      "let write_result = tokio::select! { result = write_all_for_state( &stream, response.bytes.bytes(),",
      "if !acknowledge_inbound_response_write(&write_result, &mut response, &context).await",
    ],
    "P123 inbound Written-only acknowledgement",
    failures,
  );
  const gate = maybeFunction(listener, "acknowledge_inbound_response_write", true)?.body ?? "";
  const calls = rustCallArguments(gate, "acknowledge_encoded_wire_response");
  if (calls.length !== 1 || calls[0]?.map(normalizeRust).join("|") !==
    "matches!(write_result,Ok(WriteWireMessageOutcome::Written))|response|context" ||
    normalizeRust(gate) !== "acknowledge_encoded_wire_response(matches!(write_result,Ok(WriteWireMessageOutcome::Written)),response,context,).await") {
    failures.push("P123 inbound Written-only delegation must forward the actual write result");
  }
  const completion = body(inboundCarrier, "acknowledge_encoded_wire_response");
  for (const callee of ["acknowledge_wire_message_written(", "complete_peer_emission(", "complete_block_serves("]) {
    requireExactCount(completion, callee, 1, "P123 inbound single gated completion", failures);
  }
  requireOrdered(completion, [
    "if let Some(capability) = response.maybe_tx_write_capability.take()",
    "return if was_written",
    ".complete_peer_emission(capability.acknowledge_write())",
    "else { context.abort_peer_emission(capability).is_ok()",
    "let Some(intent) = response.maybe_block_serve_intent.take() else",
    "return !was_written || context .acknowledge_wire_message_written(&response.message) .is_ok();",
    "let outcome = if was_written { ManagedBlockServeCompletionOutcome::Written } else { ManagedBlockServeCompletionOutcome::TransportFailed };",
    ".complete_block_serves(&[intent.completion(outcome)])",
  ], "P123 inbound Written-only acknowledgement", failures);
  const expectedCompletion = `
    let context = context.lock().await;
    if let Some(capability) = response.maybe_tx_write_capability.take() {
      return if was_written {
        context.complete_peer_emission(capability.acknowledge_write()).is_ok()
      } else { context.abort_peer_emission(capability).is_ok() };
    }
    let Some(intent) = response.maybe_block_serve_intent.take() else {
      return !was_written || context.acknowledge_wire_message_written(&response.message).is_ok();
    };
    let outcome = if was_written { ManagedBlockServeCompletionOutcome::Written }
      else { ManagedBlockServeCompletionOutcome::TransportFailed };
    context.complete_block_serves(&[intent.completion(outcome)]).is_ok()
  `;
  if (normalizeRust(completion) !== normalizeRust(expectedCompletion)) {
    failures.push("P123 inbound Written-only acknowledgement must keep all completion branches gated");
  }
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

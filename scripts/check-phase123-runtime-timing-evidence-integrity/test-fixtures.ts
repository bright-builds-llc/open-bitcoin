import type { TargetFile } from "./constants.ts";

export type CorpusFile = TargetFile | "packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs";

// Testdata only: these Rust snippets are parser input, never executed.
export function completeFiles(): Map<CorpusFile, string> {
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
  return new Map<CorpusFile, string>([
    ["packages/open-bitcoin-node/src/sync/types.rs", "pub enum SyncPeerReceiveOutcome { Message(WireNetworkMessage), Idle, Closed } fn receive() -> Result<SyncPeerReceiveOutcome, SyncRuntimeError>"],
    ["packages/open-bitcoin-node/src/sync/tcp.rs", "fn receive() -> Result<SyncPeerReceiveOutcome, SyncRuntimeError> { Ok(0) if allow_clean_idle && filled == 0 => return Ok(ReadStageOutcome::Closed); allow_clean_idle filled == 0 io::ErrorKind::WouldBlock | io::ErrorKind::TimedOut return Ok(ReadStageOutcome::Idle); unexpected EOF after {filled} of {} frame bytes payload read ended without a complete frame } impl SyncPeerSession for TcpPeerSession { fn send_encoded() { self.stream .write_all(bytes) .map_err(|error| error) } }"],
    ["packages/open-bitcoin-node/src/sync.rs", `pub fn open_with_block_relay_activation() { Self::open_with_runtime_activation( store, config, RelayActivationConfig::default(), block_relay_activation, false, ) } maybe_inbound_metric_status_provider self.network.block_relay_runtime_evidence_snapshot()? let maybe_block_relay_snapshot = self.maybe_authoritative_block_relay_snapshot()?; self.persist_metrics(&summary, maybe_block_relay_snapshot.as_ref(), timestamp); self.write_block_relay_log(&mut summary, maybe_block_relay_snapshot.as_ref(), timestamp); pub fn sync_until_idle_with_clock_and_cancel`],
    ["packages/open-bitcoin-node/src/sync/open_runtime.rs", `pub fn open_with_runtime_activation() { Self::open_with_configured_runtime_activation( store, config, relay_activation, block_relay_activation, inbound_enabled, BasicFilterStartupMode::PreserveSaved, ) }
pub fn open_with_configured_runtime_activation() {
 initialize_configured(&store, now, now, 0, false, u64::MAX, basic_filter_mode)?;
 store.load_chain_meta_for_open()?;
 store.load_all_undo_records()?;
 Chainstate::from_coins_cache(cache, active_chain, undo_by_block, maybe_counts);
 ManagedChainstate::from_recovered_chainstate( FjallChainstateStore::from_store(store.clone()), chainstate, lifecycle, )?;
 let network = Self::prepare_initialized_network(&store, managed, &config, basic_filter_mode, relay_activation, block_relay_activation, inbound_enabled,)?;
 let consensus_params = config.network.consensus_params();
 Self::finish_initialized_runtime(store, network, config, consensus_params) }
pub(super) fn prepare_initialized_network() {
 let maybe_configured_basic_indexing = match basic_filter_mode { BasicFilterStartupMode::Enabled => Some(true), BasicFilterStartupMode::Disabled => Some(false), BasicFilterStartupMode::PreserveSaved => { store.resolved_basic_filter_indexing_after_startup()? } };
 let local_config = progress::local_peer_config(config);
 let mut network = ManagedPeerNetwork::from_initialized_chainstate(managed, local_config, PolicyConfig::default(), config.max_blocks_in_flight_per_peer, relay_activation, block_relay_activation, inbound_enabled,);
 network.record_basic_indexing_configuration(maybe_configured_basic_indexing);
 if let Some(header_store) = store.load_header_store()? { network.seed_header_store(header_store); }
 Ok(network) }`],
    ["packages/open-bitcoin-node/src/sync/session.rs", `
impl DurableSyncRuntime {
fn sync_connected_peer_with_cancel() {
  self.announcement_outboxes.register_peer(peer_id);
  self.network.connect_sync_peer(peer_id, timestamp)?;
  let requests = PeerRequests::new(token);
  self.run_peer_messages(session, turn, outbound, controls);
  maybe_requests = Some(requests);
  self.announcement_outboxes.finish_registration(registration);
  requests.finish(self);
  request_cleanup_result(result, cleanup);
}
fn run_peer_messages() {
outbound.extend(turn.requests.request_missing(self)?); if !turn.requests.renew(self, &outbound)? { return turn.requests.complete(self,
turn.progress); } if !self.send_all_for_registered_peer_with_cancel( session, turn.peer_id, &outbound, controls.1, turn.registration,
turn.requests, )? { return turn.requests.complete(self, turn.progress); } let mut messages_received = 0_usize; while messages_received <
self.config.max_messages_per_peer { if (controls.1)() { return turn.requests.complete(self, turn.progress); } let received = match
session.receive(self.config.network.magic()) { Ok(received) => received, Err(error) => { let reason =
progress::peer_failure_reason_for_error(&error); if reason == PeerFailureReason::MalformedBlock { turn.progress.record_malformed_block();
*turn.maybe_failure_reason_override = Some(reason); } return Err(error); } }; match received { SyncPeerReceiveOutcome::Message(message) => {
let timestamp = (controls.0)(); messages_received = messages_received.saturating_add(1); if !self.process_peer_message(session, turn,
message, timestamp, controls.1)? { return turn.requests.complete(self, turn.progress); } } SyncPeerReceiveOutcome::Idle => { let timestamp =
(controls.0)(); if (controls.1)() { return turn.requests.complete(self, turn.progress); } let outbound = turn.requests.expire(self,
timestamp)?; if !turn.requests.renew(self, &outbound)? { return turn.requests.complete(self, turn.progress); } if
!self.send_all_for_registered_peer_with_cancel( session, turn.peer_id, &outbound, controls.1, turn.registration, turn.requests, )? { return
turn.requests.complete(self, turn.progress); } if !turn.requests.pending(self) { return turn.requests.complete(self, turn.progress); } }
SyncPeerReceiveOutcome::Closed => { return turn.requests.complete(self, turn.progress); } } } turn.progress.state =
PeerSyncState::Connected; turn.requests.observe_capabilities(self, turn.progress)
}
fn process_peer_message() {
let PeerTurn { peer_id, progress, requests, maybe_failure_reason_override, registration, } = turn; let peer_id = *peer_id; if
!requests.renew(self, &[])? { return Ok(false); } let maybe_header_count = match &message { WireNetworkMessage::Headers(headers) =>
Some(headers.headers.len()), _ => None, }; let maybe_terminal_header_hash = match &message { WireNetworkMessage::Headers(headers) =>
headers.headers.last().map(block_hash), _ => None, }; let maybe_block = match &message { WireNetworkMessage::Block(block) =>
Some(block.clone()), _ => None, }; let maybe_block_hash = maybe_block.as_ref().map(|block| block_hash(&block.header)); let
maybe_response_hash = super::block_response::maybe_block_response_hash(&message); let block_response_was_requested = match
maybe_response_hash { Some(block_hash) => requests.block_was_requested(self, block_hash)?, None => false, }; let
block_response_is_best_chain = match maybe_block.as_ref() { Some(block) => { self.block_has_best_chain_header(block_hash(&block.header))? ||
self.block_extends_active_tip(block)? } None => false, }; let notfound_was_requested = requests.notfound_was_requested(self, &message)?; let
receipt_hashes = super::block_response::inflight_receipt_hashes(&message, &self.inflight_blocks); if let Some(block) = maybe_block.as_ref()
&& !block_response_was_requested { requests.record_ignored(self, current_timestamp, progress, &receipt_hashes)?;
self.record_unrequested_block_response(progress, block, block_response_is_best_chain)?; let outbound = requests.request_missing(self)?; if
!requests.renew(self, &outbound)? { return Ok(false); } return self.send_all_for_registered_peer_with_cancel( session, peer_id, &outbound,
should_cancel, registration, requests, ); } let received = requests.dispatch(self, message, current_timestamp, &receipt_hashes, progress)?;
let sync_result = match received { Ok(result) => result, Err(error) => { if maybe_block_hash.is_some() { progress.record_invalid_block();
**maybe_failure_reason_override = Some(PeerFailureReason::InvalidBlock); } return Err(error); } }; let mut outbound = requests.handoff(
self, sync_result .outbound .into_iter() .map(|message| (peer_id, message)) .collect(), )?; if !requests.renew(self, &outbound)? { return
Ok(false); } filter_serving::send_filter_plans( &self.network, session, peer_id, self.config.network.magic(),
sync_result.inbound_response_plan, should_cancel, )?; if let Some(header_count) = maybe_header_count {
progress.record_validated_headers(header_count); let peer_manager = self.network.peer_manager_snapshot()?; tip::record_peer_terminal_tip(
progress, peer_manager.header_store(), maybe_terminal_header_hash, ); } if notfound_was_requested { progress.record_block_notfound(); } let
mut should_persist_progress = maybe_header_count.is_some_and(|count| count > 0); if let Some(disposition) =
sync_result.maybe_block_disposition { should_persist_progress = matches!(disposition, BlockConnectDisposition::Connected(_));
self.record_block_disposition( progress, sync_result .maybe_completed_block .as_ref() .or(maybe_block.as_ref()), disposition,
block_response_was_requested, block_response_is_best_chain, )?; } let reconcile_progress =
block_reconcile::reconcile_best_chain_for_live_session(self, current_timestamp)?; should_persist_progress |=
reconcile_progress.should_persist_progress(); self.record_reconcile_progress(reconcile_progress); if should_persist_progress {
self.persist_progress_and_dispatch_tip()?; } outbound.extend(requests.request_missing(self)?); if !requests.renew(self, &outbound)? { return
Ok(false); } self.send_all_for_registered_peer_with_cancel( session, peer_id, &outbound, should_cancel, registration, requests, )
}
}
fn request_cleanup_result(result, cleanup) {
  match (result, cleanup) {
    (result, Ok(())) => result,
    (Ok(_), Err(error)) => Err(error),
    (Err(error), Err(cleanup)) => Err(SyncRuntimeError::Network { message: format!("{error}; cleanup: {cleanup}"), }),
  }
}
`],
    ["packages/open-bitcoin-node/src/sync/session/emission_terminal.rs", `
impl super::DurableSyncRuntime {
 fn send_all_with_cancel<S: SyncPeerSession>(
        &mut self,
        session: &mut S,
        peer_id: PeerId,
        messages: &[WireNetworkMessage],
        should_cancel: &mut impl FnMut() -> bool,
    ) -> Result<(), SyncRuntimeError> {
        let buffers = messages
            .iter()
            .map(|message| {
                self.network
                    .encode_pending_network_message(peer_id, message)
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(super::filter_serving::transport_error)?;
        for (message, buffer) in messages.iter().zip(buffers) {
            if should_cancel() {
                return Ok(());
            }
            session.send_encoded(buffer.bytes(), self.config.network.magic())?;
            self.network.acknowledge_wire_message_written(message)?;
        }
        Ok(())
    }
 fn send_all_for_registered_peer_with_cancel() {
  self.send_all_for_registration( session, peer_id, messages, should_cancel, Some((registration, requests)), )
 }
 fn send_all_for_registration() {
  self.send_all_with_cancel(session, peer_id, messages, should_cancel)?;
  let emissions = match maybe_registration {
   Some((registration, requests)) => {
    let Some(emissions) = requests.with_outbox(&self.network, |owned_peer| {
     self.announcement_outboxes.take_registration( owned_peer, registration, false, false, )
    })? else { return Ok(false); };
    emissions?
   }
   None => self.announcement_outboxes.take_peer_emissions(peer_id)?,
  };
  send_peer_emissions( &self.network, session, peer_id, self.config.network.magic(), emissions, should_cancel, )?;
  Ok(true)
 }
}
fn send_peer_emissions() {
let mut buffers = VecDeque::new(); let preparation = emissions.iter().try_for_each(|emission| { if emission.peer_id() != peer_id { return
Err(SyncRuntimeError::Network { message: .to_string(), }); } let buffer = network .encode_pending_network_message(peer_id,
emission.message()) .map_err(super::filter_serving::transport_error)?; buffers.push_back(buffer); Ok(()) }); if let Err(error) = preparation
{ drop(buffers); return combined_error(error, abort_emissions(network, emissions)); } let mut emissions = VecDeque::from(emissions); while
let Some(emission) = emissions.pop_front() { let (_target_peer_id, message, capability) = emission.into_parts(); drop(message); if
should_cancel() { drop(buffers); return abort_current_and_suffix(network, capability, emissions); } let Some(buffer) = buffers.pop_front()
else { drop(buffers); return combined_error( SyncRuntimeError::Network { message: .to_string(), }, abort_current_and_suffix(network,
capability, emissions), ); }; if let Err(write_error) = session.send_encoded(buffer.bytes(), network_magic) { drop(buffer); drop(buffers);
return combined_error( write_error, abort_current_and_suffix(network, capability, emissions), ); } drop(buffer); let receipt =
capability.acknowledge_write(); if let Err(completion_error) = network.complete_peer_emission(receipt) { drop(buffers); return
combined_error(completion_error.into(), abort_emissions(network, emissions)); } } Ok(())
}
`],
    ["packages/open-bitcoin-node/src/sync/block_reconcile.rs", `
fn release_unrequested_hashes(
    runtime: &mut DurableSyncRuntime,
    hashes: &[BlockHash],
) -> Result<(), SyncRuntimeError> {
    if hashes.is_empty() {
        return Ok(());
    }
    let manager = runtime.network.peer_manager_snapshot()?;
    let retained: BTreeSet<_> = manager
        .peer_ids()
        .into_iter()
        .filter_map(|peer| manager.peer_state(peer))
        .flat_map(|peer| peer.requested_blocks.iter().copied())
        .collect();
    for hash in hashes {
        if !retained.contains(hash) {
            runtime.inflight_blocks.remove(hash);
        }
    }
    Ok(())
}
#[cfg(test)] fn release_inflight_for_message() { runtime.inflight_blocks.remove(hash); }
`],
    ["packages/open-bitcoin-node/src/sync/block_reconcile/peer_requests.rs", `
fn handoff_core(
    owner: &mut impl RequestOwner,
    inflight: &mut BTreeSet<BlockHash>,
    cap: usize,
    targeted: Vec<(PeerId, WireNetworkMessage)>,
) -> Result<Vec<WireNetworkMessage>, SyncRuntimeError> {
    if targeted.is_empty() {
        return Ok(Vec::new());
    }
    if targeted.iter().any(|(target, _)| *target != owner.peer()) {
        return Err(SyncRuntimeError::Network {
            message: "compact timeout action target does not match connected session".to_owned(),
        });
    }
    let requested: BTreeSet<_> = owner.requested()?.into_iter().collect();
    let mut hashes = BTreeSet::new();
    for (_, message) in &targeted {
        if let WireNetworkMessage::GetData(inventory) = message {
            for item in &inventory.inventory {
                if matches!(
                    item.inventory_type,
                    InventoryType::Block | InventoryType::WitnessBlock
                ) {
                    let hash = BlockHash::from(item.object_hash);
                    if !requested.contains(&hash) {
                        return Err(SyncRuntimeError::Network {
                            message: "request batch lacks retained peer membership".to_owned(),
                        });
                    }
                    hashes.insert(hash);
                }
            }
        }
    }
    if hashes.difference(inflight).count() > cap.saturating_sub(inflight.len()) {
        return Err(SyncRuntimeError::ResourceLimit {
            message: "request batch exceeds global in-flight capacity".to_owned(),
        });
    }
    inflight.extend(hashes);
    Ok(targeted.into_iter().map(|(_, message)| message).collect())
}
fn request_core() {
let requested = hashes .iter() .copied() .filter(|hash| !inflight.contains(hash)) .take(cap.saturating_sub(inflight.len()))
.collect::<Vec<_>>(); let outbound = owner.request(&requested)?; let peer = owner.peer(); handoff_core( owner, inflight, cap, outbound
.into_iter() .map(|message| (peer, message)) .collect(), )
}
impl PeerRequests {
fn expire() {
  self.require_active()?;
  let network = runtime.network.clone();
  network.with_sync_peer_requests(&self.token, |mut owner| {
    let targeted = owner.expire(timestamp).map_err(ManagedNetworkAuthorityError::from)?;
    handoff_core(&mut owner, &mut runtime.inflight_blocks,
      runtime.config.max_blocks_in_flight_total, targeted,)
  })?
}
fn handoff() {
self.require_active()?; let network = runtime.network.clone(); network.with_sync_peer_requests(&self.token, |mut owner| { handoff_core( &mut
owner, &mut runtime.inflight_blocks, runtime.config.max_blocks_in_flight_total, targeted, ) })?
}
fn dispatch() {
  begin_sync_peer_dispatch(token, message, timestamp)?;
  let outcome = continuation.resolve();
  progress.record_activity(timestamp);
  self.settle_dispatch(runtime, outcome, receipt)
}
fn reconcile() {
  let previous = release_unrequested_hashes(runtime, &self.retained);
  let SyncPeerObservation::Present { requests, peer, compact_pending, } = observed else {
    previous?;
    self.maybe_peer = None;
    self.compact_pending = false;
    return Ok(false);
  };
  let current = release_unrequested_hashes(runtime, &requests);
  request_cleanup_result(previous, current)?;
  self.retained = requests;
  self.maybe_peer = Some(*peer);
  self.compact_pending = compact_pending;
  Ok(true)
}
fn settle_authorized() {
  let (observed, result) = authorized.into_parts();
  let observation_cleanup = self.reconcile(runtime, observed).map(|_| ());
  let receipt_cleanup = release_unrequested_hashes(runtime, receipt);
  request_cleanup_result(result.map_err(Into::into),
    request_cleanup_result(observation_cleanup, receipt_cleanup),)
}
fn record_ignored() {
  self.require_active()?;
  let observed = runtime.network.observe_sync_peer(&self.token)?;
  runtime.network.with_sync_peer_requests(&self.token, |_owner| {
    progress.record_activity(timestamp);
  })?;
  let observation_cleanup = self.reconcile(runtime, observed).map(|_| ());
  request_cleanup_result(observation_cleanup,
    release_unrequested_hashes(runtime, receipt),)
}
fn finish() {
  if let Some(denied) = self.maybe_denied_requests.as_ref() {
    runtime.network.require_absent_sync_peer(&self.token)?;
    let retained_cleanup = release_unrequested_hashes(runtime, &self.retained);
    let denied_cleanup = release_unrequested_hashes(runtime, denied);
    request_cleanup_result(retained_cleanup, denied_cleanup)?;
    self.retained.clear();
    self.maybe_denied_requests = None;
    return Ok(());
  }
  let observed = runtime.network.finish_sync_peer(&self.token)?;
  self.reconcile(runtime, observed).map(|_| ())
}
fn pending() { self.compact_pending || self.retained.iter().any(|hash| runtime.inflight_blocks.contains(hash)) }
}
`],
    ["packages/open-bitcoin-node/src/sync/block_response.rs", `impl DurableSyncRuntime { fn block_extends_active_tip() { true } }`],
    ["packages/open-bitcoin-node/src/lib.rs", "SyncPeerReceiveOutcome"],
    ["packages/open-bitcoin-bench/src/runtime_fixtures.rs", "Result<SyncPeerReceiveOutcome, SyncRuntimeError>"],
    ["packages/open-bitcoin-node/src/network/block_relay_evidence.rs", "pub(crate) struct BlockRelayRuntimeEvidenceSnapshot { pub(crate) served_count: u64 } pub(super) struct ManagedBlockRelayEvidenceState { served_count: u64 } record_wire_message_written WireNetworkMessage::Block(_)"],
    ["packages/open-bitcoin-node/src/metrics/block_relay.rs", "served_count: u64 MetricKind::BlockServedCount served_count as f64"],
    ["packages/open-bitcoin-node/src/logging.rs", "pub fn block_relay_log_record served_count: u64 served_count,"],
    ["packages/open-bitcoin-node/src/status/block_relay_evidence.rs", "pub struct BlockRelayEvidenceStatus {}"],
    ["packages/open-bitcoin-rpc/src/context.rs", ""],
    ["packages/open-bitcoin-rpc/src/context/inbound_wire.rs", "\npub(crate) struct EncodedWireResponse { pub(crate) message: WireNetworkMessage, pub(crate) bytes: GenericPendingBuffer }\nimpl ResolvedInboundWireResponses {\n fn push_encoded() {\n  match network.encode_pending_network_message(peer_id, &message) {\n   Ok(bytes) => self.responses.push(InboundWireStep::Encoded(Box::new(EncodedWireResponse { message, bytes, maybe_block_serve_intent, maybe_tx_write_capability, }))),\n   Err(_) => { self.failed = true; }\n  }\n }\n}\nimpl<S: ChainstateStore, V: CoinsView> InboundWireResponsePlan<S, V> {\n fn resolve() {\n  for response in core::mem::take(&mut self.responses) {\n   resolved.push_encoded(message, None, None, &self.network, self.peer_id);\n   self.resolve_block_intent(intent, &mut resolved);\n  }\n  resolved\n }\n fn resolve_block_intent() { resolved.push_encoded(response, Some(intent), None, &self.network, self.peer_id); }\n}\nasync fn resolve_inbound_wire_responses() {\n context.prepare_inbound_wire_message(peer_id, message, timestamp);\n let mut resolved = plan.resolve();\n if resolved.failed { return None; }\n Some(resolved.responses)\n}\nasync fn acknowledge_encoded_wire_response() {\n let context = context.lock().await;\n if let Some(capability) = response.maybe_tx_write_capability.take() {\n  return if was_written { context.complete_peer_emission(capability.acknowledge_write()).is_ok() }\n  else { context.abort_peer_emission(capability).is_ok() };\n }\n let Some(intent) = response.maybe_block_serve_intent.take() else {\n  return !was_written || context .acknowledge_wire_message_written(&response.message) .is_ok();\n };\n let outcome = if was_written { ManagedBlockServeCompletionOutcome::Written } else { ManagedBlockServeCompletionOutcome::TransportFailed };\n context.complete_block_serves(&[intent.completion(outcome)]).is_ok()\n}"],
    ["packages/open-bitcoin-rpc/src/context/network.rs", "ManagedPeerNetwork::new_with_block_relay_activation( config.block_serving let encoded = self.network.encode_messages(&responses)?; .into_iter() .zip(encoded) EncodedWireResponse { message, bytes }"],
    ["packages/open-bitcoin-rpc/src/context/canonical_chain.rs", "pub(super) fn build_context_network() { ManagedPeerNetwork::new_with_block_relay_activation(config.block_serving); }"],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/tests/fixture_runtime.rs", "pub(crate) fn fixture_open_authoritative_network_runtime() { DurableSyncRuntime::open_with_configured_runtime_activation(store, runtime.sync.runtime.clone(), runtime.relay, runtime.block_serving, runtime.inbound.enabled, basic_filter_startup_mode(runtime),); }"],
    ["packages/open-bitcoin-rpc/src/inbound_listener.rs", ""],
    ["packages/open-bitcoin-rpc/src/inbound_listener/connection_runtime.rs", "\nasync fn handle_inbound_stream() {\n let encoded_responses = resolve_inbound_wire_responses();\n let steps = Batch { steps: VecDeque::from(encoded_responses) };\n while let Some(step) = steps.steps.pop_front() {\n  let write_result = tokio::select! { result = write_all_for_state( &stream, response.bytes.bytes(), ) => result };\n  if !acknowledge_inbound_response_write(&write_result, &mut response, &context).await { break; }\n }\n}\nasync fn acknowledge_inbound_response_write() {\n acknowledge_encoded_wire_response(matches!(write_result, Ok(WriteWireMessageOutcome::Written)), response, context,).await\n}"],
    ["packages/open-bitcoin-rpc/src/inbound_listener/tests.rs", "phase123_inbound_encoding_failure_does_not_increment_served phase123_enabled_runtime_config_serves_and_acknowledges_inbound_block phase123_disabled_runtime_config_does_not_serve_inbound_block"],
    ["packages/open-bitcoin-rpc/src/bin/open-bitcoind.rs", `
#[path = "open_bitcoind/canonical_startup.rs"]
mod canonical_startup;
use canonical_startup::{
    open_authoritative_network_runtime, open_configured_history_runtime, open_runtime_store,
};
fn main() {
  if let Some(code) = FjallNodeStore::maybe_run_inspection_worker() { std::process::exit(code); }
  run_application()
}
#[tokio::main]
async fn run_application() {
  let runtime = load_runtime_config()?;
  let opened = match open_configured_history_runtime(&runtime)? {
    Some(opened) => opened,
    None => {
      let maybe_runtime_store = open_runtime_store(&runtime)?;
      open_authoritative_network_runtime(&runtime, maybe_runtime_store)?
    }
  };
  let maybe_runtime_store = match &opened {
    OpenedAuthoritativeRuntime::Durable(authoritative) => authoritative.maybe_sync_runtime.as_ref().map(|sync| sync.store().clone()),
    #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => None,
  };
  match opened {
    OpenedAuthoritativeRuntime::Durable(authoritative_runtime) => serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime),
    #[cfg(test)] OpenedAuthoritativeRuntime::Transient(authoritative_runtime) => serve_authoritative_runtime(runtime, bind_address, auth, maybe_runtime_store, authoritative_runtime),
  }
}
async fn serve_authoritative_runtime() {
  let context = match authoritative_runtime.maybe_prebuilt_context.take() {
    Some(context) => context,
    None => ManagedRpcContext::from_runtime_config_with_network_handle(&runtime, authoritative_runtime.network.clone(), maybe_runtime_store.clone(),)?,
  };
  let context = shared_context.lock().await;
  let mut shutdown_latched = false;
  let mut should_cancel = || daemon_sync_shutdown_requested(&shutdown_receiver);
  sync_until_idle_with_clock_and_cancel();
  if shutdown_latched {}
}
`],
    ["packages/open-bitcoin-rpc/src/bin/open_bitcoind/canonical_startup.rs", `
fn preflight(runtime: &RuntimeConfig) -> Result<CanonicalChain, DaemonSyncPreflightError> {
  runtime.validate_peer_block_filters().map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
  let chain = selected_canonical_chain(runtime).map_err(|error| DaemonSyncPreflightError::new(error.to_string()))?;
  let activation = runtime.sync.is_enabled() || runtime.inbound.enabled || runtime.relay.enabled || runtime.block_serving.block_serving.enabled || runtime.block_serving.compact_relay.enabled;
  if activation { chain.spec().require_ordinary_history().map_err(|_| compatibility_refusal(runtime, chain))?; }
  Ok(chain)
}
pub(super) fn open_authoritative_network_runtime(runtime: &RuntimeConfig, maybe_opened: Option<OpenedCanonicalGenesis>,) -> Result<OpenedAuthoritativeRuntime, DaemonSyncPreflightError> {
  let chain = preflight(runtime)?;
  let Some(opened) = maybe_opened else { return Err(compatibility_refusal(runtime, chain)); };
  let (context, sync_runtime) = ManagedRpcContext::from_canonical_genesis_runtime_config(runtime, opened).map_err(|_| compatibility_refusal(runtime, chain))?;
  match assemble_durable_runtime(runtime, sync_runtime)? {
    OpenedAuthoritativeRuntime::Durable(mut authoritative) => { authoritative.maybe_prebuilt_context = Some(context); Ok(OpenedAuthoritativeRuntime::Durable(authoritative)) }
    #[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => Err(compatibility_refusal(runtime, chain)),
  }
}
pub(super) fn open_configured_history_runtime() {
let chain = preflight(runtime)?; let trigger = runtime.sync.is_enabled() || runtime.inbound.enabled || runtime.prune_mode !=
open_bitcoin_node::core::chainstate::PruneMode::Disabled || runtime.block_filter_index.is_explicit(); if chain != CanonicalChain::Regtest ||
!trigger { return Ok(None); } let (context, sync_runtime) = ManagedRpcContext::from_canonical_history_runtime_config(runtime) .map_err(|_|
compatibility_refusal(runtime, chain))?; match assemble_durable_runtime(runtime, sync_runtime)? { OpenedAuthoritativeRuntime::Durable(mut
authoritative) => { authoritative.maybe_prebuilt_context = Some(context); Ok(Some(OpenedAuthoritativeRuntime::Durable(authoritative))) }
#[cfg(test)] OpenedAuthoritativeRuntime::Transient(_) => Err(compatibility_refusal(runtime, chain)), }
}
`],
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

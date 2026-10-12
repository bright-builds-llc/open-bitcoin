#!/usr/bin/env bun
import { readFileSync } from "node:fs";
import { dirname, join, posix, resolve } from "node:path";
import { compact, hasOrderedCode, maybeFunction, ordinaryRust, rustCode } from "./check-phase157-index-catch-up/rust-evidence.ts";
import { meaningfulAssertions, supportedOwnership } from "./check-phase159-filter-rpcs/evidence.ts";
import { PHASE162_FILES, PHASE162_IMPLEMENTATION, PHASE162_COMPLETE, checkPhase162ClaimState, checkPhase162ExcludedClaims, loadPhase162OptionalSources, phase162ClaimView } from "./check-phase158-validated-reorg.ts";

const N = "packages/open-bitcoin-node/src/";
const R = "packages/open-bitcoin-rpc/src/";
const W = "packages/open-bitcoin-network/src/";
const C = "packages/open-bitcoin-consensus/src/";
const DOC = "docs/parity/catalog/basic-compact-filters.md";
type Contract = readonly [string, string, readonly string[]];

/** Fixed production edges supplement executed tests; lexical checks never certify parity. */
export const CONTRACTS: readonly Contract[] = [
  [W + "peer/message_dispatch.rs", "handle_message", ["WireNetworkMessage::GetCFilters(request)", "!Self::peer_mut(&mut self.peers, peer_id)?.handshake_established()", "PeerAction::ServeBasicFilter(", "FilterRequest::Filters", "WireNetworkMessage::GetCFHeaders(request)", "!Self::peer_mut(&mut self.peers, peer_id)?.handshake_established()", "FilterRequest::Headers", "WireNetworkMessage::GetCFCheckpt(request)", "!Self::peer_mut(&mut self.peers, peer_id)?.handshake_established()", "FilterRequest::Checkpoints", "WireNetworkMessage::MalformedCFRequest(_)", "WireNetworkMessage::CFilter(_)", "WireNetworkMessage::CFHeaders(_)", "WireNetworkMessage::CFCheckpt(_)", "Ok(Vec::new())"]],
  [W + "filter_serving/capability.rs", "from_permissions", ["contains_token(PeerPermissionToken::BlockFilters)", "requested_token(PeerPermissionToken::BlockFilters)"]],
  [W + "filter_serving/capability.rs", "validate_basic_index", ["!basic_enabled && (self.general_configured || self.explicit_permission_requested)", "BasicPeerServingPrerequisite::BasicIndexRequired"]],
  [W + "peer.rs", "snapshot_own_services", ["base.contains(ServiceFlags::COMPACT_FILTERS)", ".with_basic_filter_serving(general_configured)", "record.permission_decision.basic_filter_capability()", "self.own_advertised_services = capability.own_services(base)"]],
  [W + "peer.rs", "add_outbound_peer", ["state.snapshot_own_services(self.local_config.services)", "self.peers.insert(peer_id, state)", "WireNetworkMessage::Version(version)"]],
  [W + "peer/message_dispatch.rs", "handle_version", ["peer.snapshot_own_services(self.local_config.services)", "WireNetworkMessage::Version(local_version)"]],
  [W + "filter_serving/policy.rs", "validate_filter_request", ["filter_type != 0", "own_services.contains(ServiceFlags::COMPACT_FILTERS)", "headers.entry(&request.stop_hash())", "if !allowed", "if start > stop.height", "stop.height - start >= maximum"]],
  [W + "filter_serving/policy.rs", "stale_allowed", ["maybe_best_proof_header()", "maybe_proof_facts(stop)", "i64::from(best.timestamp) - i64::from(stop.timestamp) < 2_592_000", "proof_equivalent_time(best.cumulative, stop.cumulative, best.block_proof, spacing)", "age < 2_592_000"]],
  [W + "filter_serving/policy.rs", "resolve_filter_branch", ["stop.height / 1000", "ancestry_steps > limit.ancestry_steps", "identity_bytes > limit.identity_bytes", "Vec::with_capacity(capacity)", "headers.entry(&current_hash)", "current.height.is_multiple_of(1000)", "current_hash = current.parent", "identities.reverse()"]],
  [N + "network/action_translation.rs", "process_actions", ["PeerAction::ServeBasicFilter(request)", "ManagedInboundResponsePlanItem::FilterRequestIntent(request)"]],
  [N + "network/admission_bridge.rs", "transaction_relay_messages", ["PeerAction::ServeBasicFilter(_)", "=> None"]],
  [N + "network/runtime_authority.rs", "receive_sync_message", ["resolve_basic_filter_intents("]],
  [N + "network/runtime_authority.rs", "receive_message", ["network.receive_message(", "self.resolve_basic_filter_intents(peer_id, result, consensus_params)"]],
  [N + "network/runtime_authority.rs", "receive_message_for_durable_serving", ["network.receive_message_for_durable_serving(", "self.resolve_basic_filter_intents(peer_id, result, consensus_params)"]],
  [N + "network/runtime_authority/filter_index/serving.rs", "resolve_basic_filter_intents", ["Item::FilterRequestIntent(request)", "self.prepare_basic_filter_response(peer, *request, params)?", "Item::PreparedFilterServe(response)", "Item::FilterServeUnavailable(reason)", "self.disconnect_peer(peer)?"]],
  [N + "network/runtime_authority/filter_index/serving.rs", "prepare_filter_locked", ["branch::validate_network(network, peer, request, params)", "reserve_basic_filter_response_locked(network, peer)", "resize_suffix(BASIC_PEER_MAX_SUFFIX_BYTES)", "maybe_basic_filter_query_store()", "with_basic_peer_query(budget", "branch::resolve_network(", "reader.maybe_peer_records(", "encode_complete("]],
  [N + "network/runtime_authority/filter_index/serving.rs", "prepare_basic_filter_response_with_budget", ["self.read(|network|", "self.prepare_filter_locked(", "lease.resize_suffix(", "self.bind_basic_filter_response_locked(", "lease.prepare(frames)"]],
  [N + "network/runtime_authority/filter_index/serving.rs", "encode_complete", ["frame_count as u64 * 2 * std::mem::size_of::<Vec<u8>>() as u64", "Vec::with_capacity(frame_count)", "charge_encoding(", "budget.charge(Axis::LogicalCopyBytes, record.encoded_bytes().len() as u64)?", "record.encoded_bytes().to_vec()", ".encode_wire(magic)"]],
  [N + "network/runtime_authority/filter_index/serving.rs", "charge_encoding", ["frame.checked_sub(24)", ".checked_add(9)", ".checked_add(63)", "n / 64 * 64", ".checked_add(64)", ".checked_mul(3)", ".checked_add(56)", "budget.charge(Axis::HashPaddedBytes, padded)?", "budget.charge(Axis::LogicalCopyBytes, copies)"]],
  [N + "storage/fjall_store/filters/query/peer.rs", "with_basic_peer_query", ["self.filter_publication_guard()?", "budget.charge(Axis::MetadataReads, 1)?", "BasicPeerReadError::Disabled", "inspect("]],
  [N + "storage/fjall_store/filters/query/peer.rs", "maybe_peer_record", ["!self.control.read_integrity", "self.peer_row(", "budget.charge(Axis::EncodedSuffixBytes, frame)?", "charge_envelope(", "codec::parse_record("]],
  [N + "network/runtime_authority/filter_index/transport.rs", "encode_pending_network_message", ["self.lock_authority()", ".local_config", ".magic", "self.basic_filter_transport", ".encode_pending(peer, message, magic)"]],
  [N + "network/runtime_authority/filter_index/transport.rs", "basic_filter_response_is_current", ["self.lock_authority()", "self.filter_current_or_fail(", "response.lease().abort()?"]],
  [N + "network/runtime_authority/filter_index/transport.rs", "is_current", ["identity.authority != network.authority_epoch.raw()", "identity.session != network.peer_session_generation(peer)", "!current_filter_grant(network, peer)", "progress.generation() != identity.generation", "basic_peer_epoch_is_current(&identity.integrity_epoch)"]],
  [N + "network/filter_transport.rs", "next_frame", ["registry.pending_bytes()?", "admit_active_filter_frame(bytes.len() as u64, peer_active, pending)", ".pop_front()", "entry.maybe_frame = Some(frame_bytes)"]],
  [N + "network/filter_transport/pending.rs", "encode_pending", ["WireNetworkMessage::CFilter(_)", "FilterTransportError::FilterResponseRequiresLease", "wire_size(message)?", "self.encode_reserved(peer, size", ".encode_wire(magic)"]],
  [N + "network/filter_transport/pending.rs", "encode_reserved", ["self.reserve_pending(peer, size.peak)?", "let bytes = encode()?", "bytes.len() as u64 != size.frame", "reservation.shrink(size.frame)", "_reservation: reservation"]],
  [N + "network/filter_transport/pending.rs", "reserve_pending", [".filter(|entry| entry.peer == peer)", ">= 128 {", "if registry.generic.len() >= 1024 {", ".pending_bytes()?", ".checked_add(bytes)", "total > BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES)", "registry.generic.insert("]],
  [N + "sync/session.rs", "sync_connected_peer_with_cancel", ["receive_sync_message(", "filter_serving::send_filter_plans("]],
  [N + "sync/session/filter_serving.rs", "send_response", ["response.peer_id() != peer", "basic_filter_response_is_current(response)", "response.next_frame()", "session.send_encoded(frame.bytes(), magic)?", "frame.acknowledge_write()", "complete_basic_filter_frame(receipt)", "response.cancel()"]],
  [N + "sync/tcp.rs", "send_encoded", ["self.stream", ".write_all(bytes)"]],
  [N + "sync/session/emission_terminal.rs", "send_all_with_cancel", [".encode_pending_network_message(peer_id, message)", ".collect::<Result<Vec<_>, _>>()", "session.send_encoded(buffer.bytes()", "acknowledge_wire_message_written(message)"]],
  [N + "sync/session/emission_terminal.rs", "send_peer_emissions", [".encode_pending_network_message(peer_id, emission.message())", "buffers.push_back(buffer)", "session.send_encoded(buffer.bytes(), network_magic)", "abort_current_and_suffix(", "capability.acknowledge_write()", "network.complete_peer_emission(receipt)"]],
  [R + "context/inbound_wire.rs", "resolve", ["ManagedInboundResponsePlanItem::Immediate(message)", "resolved.push_encoded(", "ManagedInboundResponsePlanItem::PreparedFilterServe(response)", "InboundWireStep::Filter(*response)"]],
  [R + "context/inbound_wire.rs", "push_encoded", ["network.encode_pending_network_message(peer_id, &message)", "InboundWireStep::Encoded(Box::new(EncodedWireResponse", "bytes,"]],
  [R + "inbound_listener/connection_runtime.rs", "handle_inbound_stream", ["InboundResponseBatch", "GuardedEncodedResponse", "InboundWireStep::Filter(response)", "execute_inbound_filter(", "response.bytes.bytes()", "acknowledge_inbound_response_write("]],
  [R + "inbound_listener/filter_serving.rs", "execute_inbound_filter", ["basic_filter_response_is_current(", "response.next_frame()", "write.write(frame.bytes()).await", "Ok(WriteWireMessageOutcome::Written)", "network.complete_basic_filter_frame(frame.acknowledge_write())", "response.progress()", "response.cancel().err()"]],
  [R + "inbound_listener/filter_serving.rs", "execute_inbound_emissions", ["pending.executor.encode(&message)", "pending.executor.write(bytes.bytes()).await", "drop(bytes)", "InboundEmissionWriteResult::Written", ".complete(capability.acknowledge_write())"]],
];

/** Actions and specific assertions remain linked inside each executable selector. */
export const TEST_CONTRACTS: readonly Contract[] = [
  [C + "chain_proof/tests.rs", "phase160_peer_evidence_pinned_proof_goldens", ["pinned_proof_cases()", "decimal_proof(", "let actual = block_proof(bits)", "assert!(actual.is_err()", "assert_eq!(actual, Ok(expected_proof)"]],
  [C + "chain_proof/tests.rs", "phase160_peer_evidence_pinned_equivalent_time_goldens", ["pinned_proof_cases()", "decimal_proof(", "let actual = proof_equivalent_time(to, from, proof, spacing)", "assert_eq!(actual, Ok(expected_time)", "assert_eq!(checked, 23"]],
  [W + "peer/message_dispatch.rs", "phase160_dispatch_contract_requests_before_handshake_are_inert", ["for after_version in [false, true]", "for valid_fields in [false, true]", ".handle_message(1, request, 1)", "assert!(actions.is_empty()"]],
  [W + "peer/message_dispatch.rs", "phase160_dispatch_contract_malformed_and_unsolicited_responses_are_inert", ["WireNetworkMessage::CFilter(", "WireNetworkMessage::CFHeaders(", "WireNetworkMessage::CFCheckpt(", ".handle_message(", ".is_empty()"]],
  [W + "filter_serving/capability/tests.rs", "phase160_capability_requested_effective_prerequisite_matrix", ["for general in [false, true]", "BasicPeerServingCapability::from_permissions(", "assert_eq!(capability.effective_permission, effective)", "assert_eq!(capability.explicit_permission_requested, explicit)", "capability.validate_basic_index(false)", "if general || explicit"]],
  [W + "filter_serving/policy/tests.rs", "phase160_range_policy_inclusive_limits_and_genesis", ["1000", "2000", "FilterRequestInvalid::TooManyEntries"]],
  [W + "filter_serving/policy/tests.rs", "phase160_range_policy_checkpoint_positive_intervals_and_local_limits", ["3001", "resolve_filter_branch(", "assert_eq!("]],
  [W + "filter_serving/policy/tests.rs", "phase160_range_policy_stale_signed_timestamp_and_exact_age", ["2_592_000", "FilterRequestInvalid::UnknownOrDisallowedStop"]],
  [W + "filter_serving/policy/tests.rs", "phase160_range_policy_nonuniform_proof_rejects_timestamp_and_height_shortcuts", ["validate_filter_request(", "open_bitcoin_consensus::chain_proof::block_proof(0x207f_ffff)", "assert_eq!(outcome, Err(FilterRequestInvalid::UnknownOrDisallowedStop))"]],
  [N + "network/runtime_authority/filter_index/branch/tests.rs", "phase160_branch_genuine_disconnected_scripts_valid_survives_paired_pruning", ["ValidationProvenance::ScriptsValid", "capture_basic_filter_branch(", "assert_eq!("]],
  [N + "network/runtime_authority/filter_index/serving/tests.rs", "phase160_managed_completeness_missing_corrupt_and_backend_faults_never_expose_prefix", ["managed_receive(", "assert_unavailable("]],
  [N + "network/runtime_authority/filter_index/serving/tests/families.rs", "phase160_complete_preparation_genuine_legal_families_both_managed_directions", ["for (peer, inbound) in [(1, false), (2, true)]", "assert_family_messages("]],
  [N + "sync/tests/filter_index/peer_fixtures.rs", "phase160_complete_preparation_actual_encoding_work_exact_and_one_under", ["measure_basic_filter_preparation_for_test(", "for axis in Axis::ALL", ".with_limit(axis, amount)", "assert!(matches!(exact, Outcome::Prepared(_))", "assert_eq!(work, measured)", ".with_limit(axis, amount - 1)", "Outcome::Unavailable(Unavailable::ResourceRefused)", "measured.get(Axis::LogicalCopyBytes) > read.work.get(Axis::LogicalCopyBytes)", "measured.get(Axis::HashPaddedBytes) > read.work.get(Axis::HashPaddedBytes)"]],
  [N + "network/filter_transport/pending/tests.rs", "phase160_mixed_admission_both_orders_exact_boundary_and_one_over", ["for generic_first in [true, false]", "8 * 1024 * 1024 - 4_000_024", "assert!(owner.reserve_pending(3, 1).is_err())", "response.cancel()", "generic_pending_bytes", "FilterTransportSnapshot::default()"]],
  [N + "network/filter_transport/pending/tests.rs", "phase160_mixed_admission_barrier_race_cannot_over_admit", ["for _ in 0..32", "std::sync::Barrier::new(3)", ".reserve_pending(2, 3 * 1024 * 1024)", "response.next_frame()", "assert_ne!(generic.is_ok(), frame.is_ok())", "7 * 1024 * 1024"]],
  [N + "network/filter_transport/pending/tests.rs", "phase160_mixed_admission_full_encoder_honors_scratch_in_reverse_order", ["response.next_frame()", ".encode_pending(1, &message, NetworkMagic::MAINNET)", ".is_err()", "generic_descriptors", "0"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_transport_generic_first_pressure_refuses_filter_before_socket_write", ["receive_sync_message(", "fill_pending(", "loopback()", "execute(", "assert!(result.is_err())", "assert!(bytes.is_empty())", "assert_eq!(snapshot.generic_pending_bytes, retained)"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_transport_filter_first_write_shares_generic_peak_admission", ["session.control_writes(", "fill_pending(&handle)", "snapshot.generic_pending_bytes + snapshot.active_frame_bytes <= 8 * 1024 * 1024", "stream.write_all(bytes)", "execute(", "assert!(result.is_ok())", "WireNetworkMessage::CFilter(_)"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_transport_actual_caller_all_families", ["WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "sync_connected_peer(", "assert_eq!(filters.len(), 2)", "filter_frames, expected_frames"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_executor_lifecycle_during_write_retains_handed_owner", ["session.control_writes(", "active_frame_bytes", "PhysicalPrefixInvalidated", "assert_eq!("]],
  [R + "inbound_listener/tests/peer_filters.rs", "phase160_inbound_transport_actual_listener_all_three_exact_families", ["fixture.listener().await", "receive_frame(", "assert_eq!(actual, expected)", "WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "assert_eq!("]],
  [R + "inbound_listener/tests/peer_filters.rs", "phase160_inbound_transport_missing_later_record_emits_no_prefix", ["fixture.listener().await", "WireNetworkMessage::GetCFilters(", "receive_frame(", "assert_eq!(", "WireNetworkMessage::Pong { nonce: 160_999 }"]],
  [R + "inbound_listener/tests/peer_filters.rs", "phase160_inbound_transport_all_only_no_backend_emits_no_response", ["assert!(version.services.contains(ServiceFlags::COMPACT_FILTERS))", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "assert_eq!(", "WireNetworkMessage::Pong { nonce: 160_998 }"]],
  [R + "inbound_listener/tests/peer_filters.rs", "phase160_peer_evidence_pre_version_and_pre_verack_requests_are_inert", ["for after_version in [false, true]", "fixture.listener().await", "for filter_type in [0, 255]", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "send_regtest(&stream, WireNetworkMessage::Verack).await", "WireNetworkMessage::CFilter(", "WireNetworkMessage::CFHeaders(", "WireNetworkMessage::CFCheckpt(", "assert!(!matches!(", "WireNetworkMessage::Pong { nonce: 160_116 }"]],
  [R + "inbound_listener/tests/peer_filters/failures.rs", "phase160_inbound_executor_generic_after_cf_obeys_shared_scratch_peak", [".encode_pending_network_message(", "response.next_frame()", "let refused", "FilterTransportError::Capacity(_)", "assert_eq!(before.generic_pending_bytes, baseline.bytes().len() as u64)", "assert_eq!(before.active_frame_bytes, frame.bytes().len() as u64)"]],
  [R + "inbound_listener/tests/peer_filters/failures.rs", "phase160_inbound_executor_generic_suffix_guards_survive_until_batch_drop", ["encode_wire_responses(", "InboundResponseBatch", "GuardedEncodedResponse", "assert_eq!(", "generic_pending_bytes", "drop(batch)", "assert_eq!(", "response.bytes.bytes().len() as u64"]],
  [N + "sync/tests/filter_index/peer_measurements.rs", "phase160_peer_measurements_complete_genuine_persisted_matrix", ["PeerFilterFixture::new(3002)", "[1, 20, 400, 1000]", "BasicPeerServeAxis::ALL", "[(exact, true), (exact - 1, false)]", "assert_eq!(result.is_ok(), expected", "let fixture = fixture.reopen()"]],
  [N + "sync/tests/filter_index/peer_measurements.rs", "phase160_peer_measurements_sparse_genuine_stop_refuses_before_record_reads", ["PeerFilterFixture::sparse(9302)", "measure_basic_peer_reads_for_test(", "BasicPeerServeAxis::AncestryLinks", "assert_eq!(measured.work.get(BasicPeerServeAxis::RecordReads), 0)", "assert_eq!(fixture.store.basic_filter_point_reads_for_test(), before)"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving.rs", "phase160_startup_all_only_actual_loopback_version_without_backend", ["assert!(store.is_none())", "start_inbound_listener_for_runtime_with_context_and_announcements(", "std::net::TcpStream::connect(", "ServiceFlags::COMPACT_FILTERS"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_transport_partial_first_frame_no_complete_credit", ["failed_frame_case(1)"]],
  [N + "sync/tests/peer_filter_transport.rs", "phase160_outbound_transport_partial_second_frame_keeps_prior_prefix", ["failed_frame_case(2)"]],
  [R + "inbound_listener/tests/peer_filters/failures.rs", "phase160_inbound_executor_partial_first_frame_earns_zero", ["partial_failure(1).await"]],
  [R + "inbound_listener/tests/peer_filters/failures.rs", "phase160_inbound_transport_partial_second_frame_keeps_only_complete_prefix", ["partial_failure(2).await"]],
  [N + "sync/tests/peer_filter_transport/wire_goldens.rs", "phase160_peer_evidence_pinned_wire_goldens", ["assert_eq!(cases.len(), 42)", "let message = message(case)", "message.encode_payload()", ".encode_wire(NetworkMagic::MAINNET)", "ParsedNetworkMessage::decode_wire(&frame)", "assert_eq!(actual_payload, payload", "assert_eq!(actual_frame, frame", "assert_eq!(received.message, message"]],
  [N + "sync/tests/peer_filter_transport/wire_goldens.rs", "phase160_peer_evidence_header_only_raw_body_and_unknown_legacy_are_negative", ["PeerFilterFixture::new(16)", "receive_sync_message(", "assert_actual_negative(", ".save_block(", "assert_direct_provenance_negative(", ".save_header_entries(", "ValidationProvenance::UnknownLegacy", "assert_direct_provenance_negative("]],
  [N + "sync/tests/peer_filter_transport/wire_goldens.rs", "phase160_peer_evidence_seeded_old_timestamp_and_old_proof_are_negative_only", ["for old_proof in [false,true]", "with_accepted_stale_for_negative(", "HeaderStore::from_entries(", "2_592_001", "0x1d00_ffff", "network.seed_header_store(headers)", "assert_actual_negative("]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "phase160_peer_evidence_accepted_limits_and_checkpoint_boundaries", ["PeerFilterFixture::new(3002)", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "for stop in [0,999,1000,1001,2000,3001]", "assert_eq!(checkpoint.filter_headers, expected)", "flush_applying_prune_plan(", "assert_eq!(deleted.deleted_block_hashes", "load_block(", "load_undo(", "assert!(result.is_err()", ".inject_query_record_fault_for_test(", "fixture.hash(2000)", "QueryRecordFault::MissingTarget", "assert!(observe(", "WireNetworkMessage::GetCFCheckpt(", "stop_hash: fixture.hash(3001)", ".is_empty()"]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "phase160_peer_evidence_actual_outbound_missing_middle_final_predecessor_and_faults", ["Fault::MissingTarget", "Fault::CorruptTarget", "inject_query_record_fault_for_test(", "assert!(observe(&fixture,request).is_empty())", "set_basic_filter_fault(", "assert_eq!(result.inbound_response_plan", "BasicFilterServeUnavailable::StorageFailure"]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "phase160_peer_evidence_genuine_retained_competing_branch_actual_outbound", ["fixture.apply(&first)", "fixture.apply(&second)", "observe_runtime(", "assert_eq!(actual.len(),8)", "assert_eq!(filter.block_hash,record.identity().block_hash())", "assert_eq!(filter.filter,record.encoded_bytes())", "ValidationProvenance::ScriptsValid"]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "phase160_peer_evidence_genuine_raw_spends_match_independent_pinned_socket_bytes", ["PeerFilterFixture::new(16)", "BasicFilterInputs::from_historical(", "let independent = oracle(", "assert_eq!(row[", "let expected = oracle(", "observe(&fixture", "assert_eq!(frame, &golden)", "assert_eq!(frames.len(), 5)"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs", "phase160_peer_evidence_configured_genuine_catchup_exact_families_and_missing", ["DaemonFixture::open_serving(", "compare_families(", "with_peer(", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "assert_eq!(", "WireNetworkMessage::Pong{nonce:160_117}"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs", "phase160_peer_evidence_configured_all_only_no_backend_all_families", ["DaemonFixture::open_serving(", "with_peer(", "assert!(services.contains(ServiceFlags::COMPACT_FILTERS))", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "assert_eq!(", "WireNetworkMessage::Pong{nonce:160_113}"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs", "phase160_peer_evidence_configured_retained_stale_and_paired_body_absence", ["flush_applying_prune_plan(", "assert_eq!(deleted.deleted_block_hashes,vec![bodyless])", "fixture.store.load_block(bodyless)", ".is_none()", "fixture.store.load_undo(bodyless)", ".is_none()", "compare_families(", "reorg_to_branch(", "compare_families(&fixture,399,stale,expected_stale).await"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "phase160_peer_evidence_cross_direction_generic_blocked_exact_and_one_over", ["for one_over in [false,true]", "prepare_inbound_wire_message(", "fixture.runtime.sync_once(", "pressure(&handle,LIMIT-generic_len-cf_len+u64::from(one_over))", "execute_inbound_filter(", "assert!(result.is_err())", ".progress.messages,0", ".messages,1", "FilterTransportSnapshot::default()"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "phase160_peer_evidence_cross_direction_cf_blocked_generic_peak_exact_and_one_over", ["for one_over in [false,true]", "execute_inbound_filter(", "assert!(held.suffix_bytes>held.active_frame_bytes)", "pressure(&handle,LIMIT-held.active_frame_bytes-peak+u64::from(one_over)", "fixture.runtime.sync_once(", "snapshot.generic_pending_bytes+snapshot.active_frame_bytes,LIMIT-peak+final_len", "assert!(actual.is_empty()", "FilterTransportSnapshot::default()"]],
  [R + "inbound_listener/tests/peer_filters/mixed/race.rs", "phase160_peer_evidence_cross_direction_barrier_race_has_one_physical_winner", ["for _ in 0..8", "Barrier::new(2)", "execute_inbound_filter(", "fixture.runtime.sync_once(", "pressure(&handle,LIMIT-(cf_len+32-1))", "barrier.wait()", "snapshot.generic_pending_bytes+snapshot.active_frame_bytes<=LIMIT", "assert_ne!(cf_won,generic_won", "assert_eq!(pongs,usize::from(generic_won))", "FilterTransportSnapshot::default()"]],
];

export const HELPER_CONTRACTS: readonly Contract[] = [
  [N + "sync/tests/peer_filter_transport/wire_goldens.rs", "assert_direct_provenance_negative", ["connect_outbound_peer(", "complete_peer_handshake(", "assert_eq!(", ".prepare_basic_filter_response(", "FilterRequestInvalid::UnknownOrDisallowedStop", ".receive_sync_message(", ".is_err()", "FilterTransportSnapshot::default()"]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "with_accepted_stale_for_negative", ["ForkFixture::compact(16,0,16)", "fixture.apply(&first)", "fixture.finish()", "fixture.flush()", "fixture.apply(&second)", "fixture.finish()", "fixture.flush()", "action(&mut fixture.runtime,stop,time)"]],
  [N + "sync/tests/peer_filter_transport/wire_goldens.rs", "assert_actual_negative", ["super::loopback()", "WireNetworkMessage::GetCFilters(", "socket.write_all(", "runtime.sync_connected_peer(", ".is_err()", "assert!(!matches!(message,WireNetworkMessage::CFilter(_)|WireNetworkMessage::CFHeaders(_)|WireNetworkMessage::CFCheckpt(_)))", "FilterTransportSnapshot::default()"]],
  [N + "sync/tests/filter_index/peer_evidence.rs", "observe_runtime", ["receive_sync_message(", "peer_filter_transport::loopback()", "filter_serving::send_filter_plans(", "socket.read_to_end(&mut bytes)", "ParsedNetworkMessage::decode_wire("]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs", "compare_families", ["with_peer(", "WireNetworkMessage::GetCFilters(", "WireNetworkMessage::GetCFHeaders(", "WireNetworkMessage::GetCFCheckpt(", "socket.write_all(", "read_frame(socket)", "assert_eq!(actual,expected"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving/evidence.rs", "with_peer", ["start_inbound_listener_for_runtime_with_context_and_announcements(", "std::net::TcpStream::connect(", "action(", "listener.shutdown().await"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "ControlledSession::send_encoded", ["self.entered.send(bytes.len() as u64)", "release.recv_timeout(", "self.inner.send_encoded(bytes,magic)"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "ObservedInbound::write", ["assert_eq!(snapshot.active_frame_bytes,bytes.len() as u64)", "assert_eq!(snapshot.generic_pending_bytes+snapshot.active_frame_bytes,self.expected)", "write_all(self.stream,bytes).await?", "Ok(WriteWireMessageOutcome::Written)"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "HeldInbound::write", ["self.entered.notify_one()", "self.release.notified().await", "write_all(self.stream,bytes).await?", "Ok(WriteWireMessageOutcome::Written)"]],
  [R + "inbound_listener/tests/peer_filters/mixed/race.rs", "RaceSession::send_encoded", ["self.entered.send(())", ".recv_timeout(", "self.inner.send_encoded(bytes,magic)"]],
  [R + "inbound_listener/tests/peer_filters/mixed.rs", "pressure", ["handle.encode_pending_network_message(", "assert_eq!(", "generic_pending_bytes-initial,target"]],
  [N + "network/runtime_authority/filter_index/serving/tests.rs", "assert_unavailable", ["assert!(result.outbound.is_empty())", "assert!(result.targeted_outbound.is_empty())", "assert_eq!(", "vec![PlanItem::FilterServeUnavailable(expected)]", "FilterTransportSnapshot::default()"]],
  [N + "sync/tests/filter_index/peer_measurements.rs", "observe", ["measure_basic_peer_reads_for_test(", "structural(", "assert_eq!("]],
  [N + "sync/tests/filter_index/peer_measurements.rs", "structural", ["BasicPeerServeAxis::ALL", "work.get(axis) <= BasicPeerServeBudget::default().remaining(axis)"]],
  [N + "sync/tests/peer_filter_transport.rs", "failed_frame_case", ["session.control_writes(", "stream.write_all(&bytes[..7])?", "return Err(", "send_response(", "assert!(matches!(result, Err(SyncRuntimeError::Io { .. })))", "fail_on as u64 - 1", "bytes.len() as u64 - 7"]],
  [R + "inbound_listener/tests/peer_filters/failures.rs", "partial_failure", ["execute_inbound_filter(", "PartialWriter", ".expect_err(", "assert_eq!(failure.progress.messages, (fail_on - 1) as u64)", "assert_eq!(failure.progress.bytes, frame.len() as u64)", "assert_eq!(failure.progress.bytes, 0)"]],
];

type Edge = { parent: string; child: string; symbol: string };
const overrides = new Map([
  [R + "inbound_listener/filter_serving.rs", [R + "inbound_listener/connection_runtime.rs", "filter_serving"]],
  [N + "network/runtime_authority/filter_index/serving/tests.rs", [N + "sync/tests/filter_index/peer_fixtures.rs", "phase160_serving_tests"]],
  [R + "bin/open_bitcoind/tests.rs", [R + "bin/open-bitcoind.rs", "tests"]],
  [R + "bin/open_bitcoind/tests/filter_index/peer_serving.rs", [R + "bin/open_bitcoind/tests/filter_index.rs", "peer_serving"]],
]);
function owningEdges(): Edge[] {
  const edges = new Map<string, Edge>();
  for (const [file] of [...CONTRACTS, ...TEST_CONTRACTS, ...HELPER_CONTRACTS]) {
    let child = file;
    while (!child.endsWith("/src/lib.rs") && !child.endsWith("/bin/open-bitcoind.rs") && !edges.has(child)) {
      const maybeOverride = overrides.get(child);
      const directory = dirname(child);
      const parent = maybeOverride?.[0] ?? (directory.endsWith("/src") ? join(directory, "lib.rs") : directory + ".rs");
      const symbol = maybeOverride?.[1] ?? posix.basename(child, ".rs");
      edges.set(child, { parent, child, symbol });
      child = parent;
    }
  }
  return [...edges.values()];
}
export const MODULE_EDGES = owningEdges();
const extraRust = [N + "network/types.rs", N + "sync/types.rs", N + "network/filter_transport/encoded_size.rs", C + "chain_proof.rs", W + "peer.rs", W + "peer/message_dispatch.rs"];
const references = [DOC, "README.md", "AGENTS.md", "scripts/verify.sh", "scripts/generate-peer-filter-fixtures.ts", "scripts/generate-peer-filter-fixtures.test.ts", "scripts/basic-filter-oracle.py", "docs/parity/index.json", "docs/parity/source-breadcrumbs.json", "docs/parity/fixtures/peer-filter-wire.json", "docs/parity/fixtures/peer-filter-proof.json", "docs/parity/fixtures/peer-filter-resource-measurements.json", W + "filter_serving/resource.rs"];
export const CHECK_FILES = [...new Set([...MODULE_EDGES.flatMap(edge => [edge.parent, edge.child]), ...extraRust, ...references, ...PHASE162_FILES])];
type Texts = (file: string) => string;
const sourceChecks = new Map<string, { source: string; failures: string[] }>();
const ownershipChecks = new Map<string, { source: string; failures: string[] }>();

/** Limit repeated trait method names to their concrete implementation. */
export function maybeSelector(source: string, selector: string, production = false): ReturnType<typeof maybeFunction> {
  if (!selector.includes("::")) return maybeFunction(source, selector, production);
  const [owner, symbol] = selector.split("::");
  const code = production ? ordinaryRust(source) : rustCode(source);
  const matches = [...code.matchAll(new RegExp(`\\bimpl\\b[^{}]*\\bfor\\s+${owner}(?:<[^{}]*>)?\\s*\\{`, "g"))];
  if (matches.length !== 1) return undefined;
  const start = matches[0].index;
  let end = start + matches[0][0].length;
  let depth = 1;
  while (end < code.length && depth) {
    if (code[end] === "{") depth++;
    if (code[end] === "}") depth--;
    end++;
  }
  const maybeBody = maybeFunction(source.slice(start, end), symbol, production);
  return maybeBody ? { ...maybeBody, start: maybeBody.start + start, end: maybeBody.end + start } : undefined;
}

/** Fail closed on missing source, executable evidence, registrations or scoped claims. */
export function checkPhase160PeerFilterServing(maybeRoot?: string, maybeSources?: ReadonlyMap<string, string>): string[] {
  const root = resolve(maybeRoot ?? resolve(import.meta.dir, ".."));
  const failures: string[] = [];
  const texts = maybeSources ? new Map(maybeSources) : loadPhase162OptionalSources(root);
  for (const file of CHECK_FILES) {
    if (maybeSources) { if (!texts.has(file)) failures.push(`${file}: required evidence unreadable`); continue; }
    try { texts.set(file, readFileSync(resolve(root, file), "utf8")); }
    catch { failures.push(`${file}: required evidence unreadable`); }
  }
  const text = (file: string) => texts.get(file) ?? "";
  const claimState = checkPhase162ClaimState(text, failures);
  const contracts = [...CONTRACTS, ...HELPER_CONTRACTS, ...TEST_CONTRACTS];
  for (const file of new Set(contracts.map(row => row[0]))) {
    const source = text(file);
    const maybeCached = sourceChecks.get(file);
    if (maybeCached?.source === source) { failures.push(...maybeCached.failures); continue; }
    const found: string[] = [];
    for (const [, symbol, anchors] of contracts.filter(row => row[0] === file)) {
      const production = CONTRACTS.some(row => row[0] === file && row[1] === symbol);
      const maybeBody = maybeSelector(source, symbol, production);
      if (!maybeBody || !supportedOwnership(source, maybeBody.start, !production) || !hasOrderedCode(maybeBody.body, anchors)) found.push(`${file}:${symbol}: ordered actual source/action/assertion links`);
      if (TEST_CONTRACTS.some(row => row[0] === file && row[1] === symbol)
        && (!maybeBody || !/#\[(?:tokio::)?test(?:\]|\()/.test(maybeBody.attributes)
          || !meaningfulAssertions(source, maybeBody.body))) found.push(`${file}:${symbol}: executable meaningful behavioral evidence`);
    }
    // Content equality, never a pathname or mtime, permits reuse across mutation fixtures.
    sourceChecks.set(file, { source, failures: found });
    failures.push(...found);
  }
  checkOwnership(text, failures);
  checkIsolation(text, failures);
  checkResources(text, failures);
  checkReferences(text, failures, claimState);
  return [...new Set(failures)];
}

function checkOwnership(text: Texts, failures: string[]): void {
  for (const { parent, child, symbol } of MODULE_EDGES) {
    const source = text(parent);
    const key = `${parent}:${symbol}`;
    const maybeCached = ownershipChecks.get(key);
    if (maybeCached?.source === source) { failures.push(...maybeCached.failures); continue; }
    const found: string[] = [];
    const code = rustCode(source);
    const declarations = [...code.matchAll(new RegExp(`\\bmod\\s+${symbol}\\s*;`, "g"))];
    const isTest = /\/tests(?:\/|\.rs)/.test(child);
    if (declarations.length !== 1 || !supportedOwnership(source, declarations[0].index, isTest)) {
      found.push(`${parent}: executable owner ${symbol}`);
      ownershipChecks.set(key, { source, failures: found });
      failures.push(...found);
      continue;
    }
    const start = code.lastIndexOf(";", declarations[0].index) + 1;
    const maybePath = source.slice(start, declarations[0].index).match(/#\[path\s*=\s*"([^"]+)"\]/)?.[1];
    const actual = maybePath ? posix.normalize(join(dirname(parent), maybePath)) : join(parent.endsWith("/lib.rs") ? dirname(parent) : parent.slice(0, -3), `${symbol}.rs`);
    if (actual !== child) found.push(`${parent}: actual owner path ${symbol}`);
    ownershipChecks.set(key, { source, failures: found });
    failures.push(...found);
  }
  try {
    const manifest = JSON.parse(text("docs/parity/source-breadcrumbs.json"));
    for (const file of CHECK_FILES.filter(file => file.endsWith(".rs"))) {
      const comments = [...text(file).matchAll(/^\/\/ - (packages\/bitcoin-knots\/[^\n]+)$/gm)].map(match => match[1]);
      const exact = manifest.groups.filter((group: { files?: string[] }) => group.files?.includes(file));
      // New peer modules need exact registrations; established ancestor modules retain their earlier group ownership.
      if (/filter_serving|filter_transport|chain_proof|\/serving(?:\/|\.rs)|\/branch(?:\/|\.rs)|\/query\/peer(?:\/|\.rs)|peer_filters|peer_filter_transport|peer_measurements|peer_serving/.test(file)
        && (exact.length !== 1 || !comments.length || JSON.stringify(comments) !== JSON.stringify(exact[0].breadcrumbs))) failures.push(`docs/parity/source-breadcrumbs.json:${file}: exact pinned breadcrumb registration`);
    }
  } catch { failures.push("docs/parity/source-breadcrumbs.json: invalid inventory"); }
}

function checkIsolation(text: Texts, failures: string[]): void {
  for (const file of [N + "network/types.rs", N + "network/filter_transport.rs", W + "peer.rs"]) {
    if (/use\s+[^;]*runtime_authority[^;]*serving/.test(ordinaryRust(text(file)))) failures.push(`${file}: pure intent imports durable serving`);
  }
  const file = N + "network/filter_transport.rs";
  const code = ordinaryRust(text(file));
  if (!/fn\s+next_frame\s*\(\s*&mut self\s*\)/.test(code)) failures.push(`${file}: no caller pressure snapshots`);
  const physical = ordinaryRust(text(N + "sync/tcp.rs"));
  if (/decode_wire|encode_wire/.test(maybeFunction(physical, "send_encoded", true)?.body ?? "")) failures.push(`${N}sync/tcp.rs: exact physical send cannot decode or reencode`);
  for (const file of [N + "sync/session.rs", N + "sync/session/emission_terminal.rs", N + "sync/session/filter_serving.rs"]) {
    if (/\bsession\s*\.\s*send\s*\(/.test(ordinaryRust(text(file)))) failures.push(`${file}: legacy physical send bypass`);
  }
}

function checkResources(text: Texts, failures: string[]): void {
  const resource = W + "filter_serving/resource.rs";
  const values: [string, string][] = [
    ["BASIC_PEER_MAX_PAYLOAD_BYTES", "4_000_000"], ["BASIC_PEER_MAX_SUFFIX_BYTES", "8 * 1024 * 1024"],
    ["BASIC_PEER_MAX_AGGREGATE_SUFFIX_BYTES", "32 * 1024 * 1024"], ["BASIC_PEER_MAX_REQUESTS", "1"], ["BASIC_PEER_MAX_AGGREGATE_REQUESTS", "4"],
    ["BASIC_PEER_MAX_FRAME_BYTES", "BASIC_PEER_MAX_PAYLOAD_BYTES + 24"], ["BASIC_PEER_MAX_AGGREGATE_FRAME_BYTES", "8 * 1024 * 1024"],
    ["BASIC_PEER_MAX_ANCESTRY_LINKS", "8192"], ["BASIC_PEER_MAX_HEADER_OPERATIONS", "8192"], ["BASIC_PEER_MAX_RECORD_READS", "4096"], ["BASIC_PEER_MAX_METADATA_READS", "256"],
    ["BASIC_PEER_MAX_VALIDATED_ENVELOPE_BYTES", "64 * 1024 * 1024"], ["BASIC_PEER_MAX_HASH_PADDED_BYTES", "128 * 1024 * 1024"], ["BASIC_PEER_MAX_LOGICAL_COPY_BYTES", "64 * 1024 * 1024"],
  ];
  const resourceCode = compact(ordinaryRust(text(resource)));
  for (const [name, value] of values) if (!resourceCode.includes(compact(`${name}: u64 = ${value};`))) failures.push(`${resource}: exact local resource ${name}`);
  const matrix = text(W + "filter_serving/capability/tests.rs");
  for (const expression of ['(&["all"], true, false)', '(&["blockfilters"], true, true)', '(&["all", "blockfilters"], true, true)', '(&["bloomfilter"], false, false)']) if (!compact(matrix).includes(compact(expression))) failures.push(`${W}filter_serving/capability/tests.rs: full all/explicit/excluded matrix`);
}

function checkReferences(text: Texts, failures: string[], claimState: ReturnType<typeof checkPhase162ClaimState>): void {
  for (const file of [DOC, "README.md", "docs/parity/index.json"]) {
    const source = phase162ClaimView(text(file), claimState);
    checkPhase162ExcludedClaims(text(file), file, failures, claimState);
    const positive = "(?:(?:is|are|now|fully|already)\\s+)*(?:complete|shipped|ready|safe|supported|proven|enabled|passed)\\b";
    const subjects = "(?:v2\\.5(?:\\s+milestone)?|CF(?:OP|GR)-\\d+|Phase\\s+16[12]|(?:BIP37|V0)(?:\\s+(?:bloom|filter))?(?:\\s+serving)?|production(?:[- ]funds)?(?:\\s+(?:service|use|safety|operation))?|archive(?:\\s+serving)?)";
    if (new RegExp(`\\b${subjects}\\s+${positive}|\\bpublic\\s+(?:peer\\s+)?serving\\s+(?:(?:is|now)\\s+)*(?:enabled|default)\\b`, "i").test(source)) failures.push(`${file}: unsupported scope claim`);
  }
  for (const term of ["all,blockfilters", "2,592,000", "4,000,024", "33,554,432", "8,388,608", "134,217,728", "67,108,864", "128 descriptors", "1024", "encoder scratch", "Phase 161", "Phase 162", "default-off", "not latency guarantees"]) if (!text(DOC).includes(term)) failures.push(`${DOC}: exact policy/scope disclosure ${term}`);
  for (const command of ["cargo run --manifest-path packages/Cargo.toml -p open-bitcoin-cli --bin open-bitcoin --", "bazel run //packages/open-bitcoin-cli:open_bitcoin --", "command-timings.ts run --key phase160-", "bun test ./scripts/check-phase160-peer-filter-serving.test.ts"]) if (!text(DOC).includes(command)) failures.push(`${DOC}: reproducible repo-local command ${command}`);
  try {
    const index = JSON.parse(text("docs/parity/index.json"));
    const rows = index.checklist.surfaces.filter((row: { requirements?: string[] }) => row.requirements?.some(requirement => /^CFNET-/.test(requirement)));
    const row = rows[0];
    if (index.baseline !== "29.3.knots20260210" || rows.length !== 1 || JSON.stringify(row.requirements) !== '["CFNET-01","CFNET-02","CFNET-03","CFNET-04","CFNET-05","CFNET-06"]') failures.push("docs/parity/index.json: unique scoped CFNET owner");
    for (const file of [DOC, "scripts/check-phase160-peer-filter-serving.ts", "scripts/check-phase160-peer-filter-serving.test.ts", "scripts/generate-peer-filter-fixtures.ts", "docs/parity/fixtures/peer-filter-resource-measurements.json"]) if (!row?.evidence?.includes(file)) failures.push(`docs/parity/index.json: peer evidence ${file}`);
    if (!row?.known_gaps?.some((gap: string) => (claimState !== "complete" && gap.includes("161") && gap.includes("162") && gap.includes("pending"))
      || (claimState === "implementation" && gap === PHASE162_IMPLEMENTATION) || (claimState === "complete" && gap === PHASE162_COMPLETE))) failures.push("docs/parity/index.json: pending operator/integrated scope");
  } catch { failures.push("docs/parity/index.json: invalid peer parity inventory"); }
  for (const file of ["docs/parity/fixtures/peer-filter-wire.json", "docs/parity/fixtures/peer-filter-proof.json"]) if (!text(file).includes("a9aee730466ac67d35a3c03ee24676be5e045878")) failures.push(`${file}: exact Knots pin`);
  const commands = ["bun test ./scripts/check-phase159-filter-rpcs.test.ts", "bun run scripts/check-phase159-filter-rpcs.ts", "bun test ./scripts/check-phase160-peer-filter-serving.test.ts", "bun run scripts/check-phase160-peer-filter-serving.ts", "bun test ./scripts/generate-peer-filter-fixtures.test.ts", "bun run scripts/generate-peer-filter-fixtures.ts --check"];
  const source = text("scripts/verify.sh");
  const documented = source.split(": <<'VERIFY_COMMAND_ORDER'")[1]?.split("VERIFY_COMMAND_ORDER")[0] ?? "";
  const executed = source.split("\n").filter(line => line.startsWith("run_step ")).join("\n");
  for (const [label, section] of [["documented", documented], ["executed", executed]]) {
    let previous = -1;
    for (const command of commands) {
      const matches = section.split("\n").filter(line => line.endsWith(command));
      const at = section.indexOf(command);
      if (matches.length !== 1 || at <= previous) failures.push(`scripts/verify.sh: ordered ${label} peer/independent corpus gate ${command}`);
      previous = at;
    }
  }
}

if (import.meta.main) {
  const failures = checkPhase160PeerFilterServing();
  if (failures.length) {
    console.error("Phase 160 scoped peer source/action/assertion guard failed:");
    for (const failure of failures) console.error(`- ${failure}`);
    process.exit(1);
  }
  console.log(`Phase 160: ${CONTRACTS.length} production edges, ${TEST_CONTRACTS.length} executable selectors, scoped claims and native corpus routing checked; native/review/lifecycle certification remains independent.`);
}

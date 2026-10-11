// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::{
    collections::VecDeque,
    future::pending,
    io,
    net::SocketAddr,
    sync::{Arc, Mutex, atomic::Ordering},
};

use open_bitcoin_network::{
    INBOUND_MESSAGE_HEADER_LEN, InactivePermissionEffectLabel, InboundAdmissionDecision,
    InboundEnvelopePolicy, InboundHandshakeState, PermissionEffectLabel, ResourceGovernancePolicy,
    WireNetworkMessage,
};
use open_bitcoin_node::{
    network::{EffectAbort, PeerEmissionReceipt, PeerEmissionWriteCapability},
    sync::{AnnouncementOutboxNotification, SyncRuntimeError},
};

use crate::{
    ManagedRpcContext,
    context::{
        EncodedWireResponse, InboundWireStep, acknowledge_encoded_wire_response,
        resolve_inbound_wire_responses,
    },
};

#[path = "filter_serving.rs"]
pub(super) mod filter_serving;
use filter_serving::execute_inbound_filter;

use super::{
    InboundAnnouncementTransport, InboundConnectionControl, InboundListenerEvidence,
    resource_runtime::{
        InboundRuntimeCounters, ReadWireMessageOutcome, RuntimeQueuePressureState,
        WriteWireMessageOutcome, current_timestamp, disconnect_admitted_peer, lock_evidence,
        lock_runtime_counters, next_handshake_state, queue_pressure_event,
        read_wire_message_for_state, record_shared_resource_event, resource_timeout_event,
        write_all_for_state,
    },
};

pub(super) async fn handle_inbound_stream<S, V>(
    peer_id: u64,
    remote_addr: SocketAddr,
    stream: tokio::net::TcpStream,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: Arc<Mutex<InboundRuntimeCounters>>,
    connection_control: InboundConnectionControl<S, V>,
) where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let InboundConnectionControl {
        shutdown_requested,
        shutdown_notify,
        maybe_announcement_transport,
    } = connection_control;
    let resource_policy = ResourceGovernancePolicy::default();
    let connected_at_unix_seconds = current_timestamp();
    let mut last_activity_unix_seconds = connected_at_unix_seconds;
    let mut handshake_state = InboundHandshakeState::Accepted;
    let decision = {
        let mut context = context.lock().await;
        context.record_inbound_admission_for_remote_addr(peer_id, remote_addr, false)
    };
    let permission_decision = match decision {
        Ok(InboundAdmissionDecision::Admit(record)) => {
            lock_evidence(&evidence).record_admitted();
            record.permission_decision
        }
        Ok(InboundAdmissionDecision::Reject(rejection)) => {
            lock_evidence(&evidence).record_rejected(rejection.reason);
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            return;
        }
        Err(_error) => {
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            return;
        }
    };
    let mut queue_pressure = RuntimeQueuePressureState::default();
    let mut maybe_outbox_notification = match maybe_announcement_transport.as_ref() {
        Some(transport) => match transport.outboxes.register_peer(peer_id) {
            Ok(notification) => Some(notification),
            Err(_error) => {
                disconnect_admitted_peer(&context, peer_id).await;
                return;
            }
        },
        None => None,
    };

    let Ok(network_info) = context.lock().await.network_info() else {
        if unregister_announcement_peer(&maybe_announcement_transport, peer_id).is_err() {
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
        }
        return;
    };
    let envelope_policy = InboundEnvelopePolicy::new(network_info.network_magic);
    let network = context.lock().await.basic_filter_network_handle();

    'message_loop: loop {
        if shutdown_requested.load(Ordering::Relaxed) {
            break;
        }
        if !drain_inbound_announcements(
            maybe_announcement_transport.as_ref(),
            peer_id,
            &stream,
            network_info.network_magic,
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
            &mut queue_pressure,
            permission_decision.active_effects().to_vec(),
            permission_decision.inactive_effects().to_vec(),
            &context,
            &evidence,
            &runtime_counters,
        )
        .await
        {
            break;
        }
        if let Some(event) = resource_timeout_event(
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            current_timestamp(),
            handshake_state,
        ) {
            record_shared_resource_event(&context, &evidence, event).await;
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        }
        queue_pressure.record_pending_read(INBOUND_MESSAGE_HEADER_LEN);
        if let Some(event) = queue_pressure_event(
            &resource_policy,
            &queue_pressure,
            permission_decision.active_effects().to_vec(),
            permission_decision.inactive_effects().to_vec(),
        ) {
            record_shared_resource_event(&context, &evidence, event).await;
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        }
        let read_future = read_wire_message_for_state(
            &stream,
            &envelope_policy,
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
        );
        tokio::pin!(read_future);
        let maybe_read_result = loop {
            tokio::select! {
                read_result = &mut read_future => break Some(read_result),
                () = wait_for_outbox_notification(&mut maybe_outbox_notification) => {
                    queue_pressure.clear_pending_read();
                    if !drain_inbound_announcements(
                        maybe_announcement_transport.as_ref(),
                        peer_id,
                        &stream,
                        network_info.network_magic,
                        &resource_policy,
                        connected_at_unix_seconds,
                        last_activity_unix_seconds,
                        handshake_state,
                        &mut queue_pressure,
                        permission_decision.active_effects().to_vec(),
                        permission_decision.inactive_effects().to_vec(),
                        &context,
                        &evidence,
                        &runtime_counters,
                    )
                    .await
                    {
                        break 'message_loop;
                    }
                    queue_pressure.record_pending_read(INBOUND_MESSAGE_HEADER_LEN);
                }
                () = shutdown_notify.notified() => break None,
            }
        };
        queue_pressure.clear_pending_read();
        let Some(read_result) = maybe_read_result else {
            break;
        };
        let outcome = match read_result {
            Ok(outcome) => outcome,
            Err(_error) => break,
        };
        let parsed = match outcome {
            ReadWireMessageOutcome::Message(parsed) => parsed,
            ReadWireMessageOutcome::Rejected(event) => {
                record_shared_resource_event(&context, &evidence, event).await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break;
            }
        };
        last_activity_unix_seconds = current_timestamp();
        handshake_state = next_handshake_state(handshake_state, &parsed.message);
        lock_evidence(&evidence).record_handshake(&parsed.message);
        let Some(encoded_responses) = resolve_inbound_wire_responses(
            &context,
            peer_id,
            parsed.message,
            last_activity_unix_seconds,
        )
        .await
        else {
            lock_runtime_counters(&runtime_counters)
                .record_failure(&resource_policy, current_timestamp());
            break;
        };
        let mut steps = filter_serving::InboundResponseBatch {
            steps: VecDeque::from(encoded_responses),
            network: &network,
            evidence: &evidence,
        };
        while let Some(step) = steps.steps.pop_front() {
            let mut response = match step {
                InboundWireStep::Encoded(response) => filter_serving::GuardedEncodedResponse {
                    response,
                    network: &network,
                    evidence: &evidence,
                },
                InboundWireStep::Filter(response) => {
                    let result = execute_inbound_filter(
                        response,
                        &network,
                        filter_serving::SocketFilterWriter {
                            stream: &stream,
                            policy: &resource_policy,
                            connected: connected_at_unix_seconds,
                            activity: last_activity_unix_seconds,
                            handshake: handshake_state,
                            queue: &mut queue_pressure,
                            active: permission_decision.active_effects().to_vec(),
                            inactive: permission_decision.inactive_effects().to_vec(),
                            shutdown: &shutdown_notify,
                        },
                    )
                    .await;
                    if let Err(failure) = result {
                        let clean =
                            abort_inbound_suffix(std::mem::take(&mut steps.steps), &network);
                        let mut event = failure.resource_event();
                        if !clean {
                            event.reason.push_str("; suffix cleanup failed");
                        }
                        record_shared_resource_event(&context, &evidence, event).await;
                        lock_runtime_counters(&runtime_counters)
                            .record_failure(&resource_policy, current_timestamp());
                        break 'message_loop;
                    }
                    continue;
                }
            };
            queue_pressure.record_pending_write(response.bytes.bytes().len());
            if let Some(mut event) = queue_pressure_event(
                &resource_policy,
                &queue_pressure,
                permission_decision.active_effects().to_vec(),
                permission_decision.inactive_effects().to_vec(),
            ) {
                let current_clean =
                    acknowledge_encoded_wire_response(false, &mut response, &context).await;
                drop(response);
                let suffix_clean = abort_inbound_suffix(std::mem::take(&mut steps.steps), &network);
                if !current_clean || !suffix_clean {
                    event.reason.push_str("; unwritten cleanup failed");
                }
                queue_pressure.clear_pending_write();
                record_shared_resource_event(&context, &evidence, event).await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break 'message_loop;
            }
            let write_result = tokio::select! { result = write_all_for_state(
                &stream,
                response.bytes.bytes(),
                &resource_policy,
                connected_at_unix_seconds,
                last_activity_unix_seconds,
                handshake_state,
            ) => result, () = shutdown_notify.notified() => Err(io::Error::from(io::ErrorKind::Interrupted)) };
            queue_pressure.clear_pending_write();
            if !acknowledge_inbound_response_write(&write_result, &mut response, &context).await {
                drop(response);
                let clean = abort_inbound_suffix(std::mem::take(&mut steps.steps), &network);
                let reason =
                    format!("response settlement failed; suffix cleanup successful={clean}");
                record_shared_resource_event(&context, &evidence, terminal_failure_event(reason))
                    .await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break 'message_loop;
            }
            match write_result {
                Ok(WriteWireMessageOutcome::Written) => {}
                Ok(WriteWireMessageOutcome::Rejected(mut event)) => {
                    drop(response);
                    if !abort_inbound_suffix(std::mem::take(&mut steps.steps), &network) {
                        event.reason.push_str("; suffix cleanup failed");
                    }
                    record_shared_resource_event(&context, &evidence, event).await;
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
                Err(error) => {
                    drop(response);
                    let clean = abort_inbound_suffix(std::mem::take(&mut steps.steps), &network);
                    record_shared_resource_event(
                        &context,
                        &evidence,
                        terminal_failure_event(format!(
                            "socket {:?}; suffix cleanup successful={clean}",
                            error.kind()
                        )),
                    )
                    .await;
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
            }
        }
        if matches!(
            context.lock().await.admitted_peer_present(peer_id),
            Ok(false)
        ) {
            break 'message_loop;
        }
        if !drain_inbound_announcements(
            maybe_announcement_transport.as_ref(),
            peer_id,
            &stream,
            network_info.network_magic,
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
            &mut queue_pressure,
            permission_decision.active_effects().to_vec(),
            permission_decision.inactive_effects().to_vec(),
            &context,
            &evidence,
            &runtime_counters,
        )
        .await
        {
            break;
        }
    }
    if unregister_announcement_peer(&maybe_announcement_transport, peer_id).is_err() {
        lock_runtime_counters(&runtime_counters)
            .record_failure(&resource_policy, current_timestamp());
    }
    disconnect_admitted_peer(&context, peer_id).await;
}

fn terminal_failure_event(reason: String) -> open_bitcoin_network::InboundResourceEvent {
    open_bitcoin_network::InboundResourceEvent {
        outcome: "transport_failed".to_string(),
        reason,
        label: "inbound_transport".to_string(),
        source: "source_runtime_write".to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: "disconnect".to_string(),
    }
}

fn abort_inbound_suffix<S, V>(
    steps: VecDeque<InboundWireStep>,
    network: &open_bitcoin_node::ManagedNetworkHandle<S, V>,
) -> bool
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let mut clean = true;
    for step in steps {
        match step {
            InboundWireStep::Encoded(mut response) => {
                clean &= filter_serving::abort_encoded_response(&mut response, network)
            }
            InboundWireStep::Filter(response) => clean &= response.cancel().is_ok(),
        }
    }
    clean
}

async fn wait_for_outbox_notification(
    maybe_notification: &mut Option<AnnouncementOutboxNotification>,
) {
    let Some(notification) = maybe_notification else {
        pending::<()>().await;
        return;
    };
    notification.notified().await;
}

fn unregister_announcement_peer<S, V>(
    maybe_transport: &Option<InboundAnnouncementTransport<S, V>>,
    peer_id: u64,
) -> Result<(), SyncRuntimeError>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    if let Some(transport) = maybe_transport {
        return transport
            .outboxes
            .unregister_peer(&transport.network, peer_id);
    }
    Ok(())
}

pub(super) use filter_serving::{
    InboundEmissionExecutionOutcome, InboundEmissionExecutor, InboundEmissionWriteResult,
    execute_inbound_emissions,
};

#[allow(clippy::too_many_arguments)]
struct SocketInboundEmissionExecutor<'a, S, V: open_bitcoin_node::core::chainstate::CoinsView> {
    transport: &'a InboundAnnouncementTransport<S, V>,
    encoding_network: &'a open_bitcoin_node::ManagedNetworkHandle<S, V>,
    stream: &'a tokio::net::TcpStream,
    peer_id: u64,
    resource_policy: &'a ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    queue_pressure: &'a mut RuntimeQueuePressureState,
    active_permission_effects: Vec<PermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    context: &'a Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    evidence: &'a Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: &'a Arc<Mutex<InboundRuntimeCounters>>,
}

impl<S, V> InboundEmissionExecutor for SocketInboundEmissionExecutor<'_, S, V>
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    fn encode(
        &mut self,
        message: &WireNetworkMessage,
    ) -> Result<open_bitcoin_node::network::filter_transport::GenericPendingBuffer, ()> {
        self.encoding_network
            .encode_pending_network_message(self.peer_id, message)
            .map_err(|_error| ())
    }

    async fn write(&mut self, bytes: &[u8]) -> InboundEmissionWriteResult {
        self.queue_pressure.record_pending_write(bytes.len());
        if let Some(event) = queue_pressure_event(
            self.resource_policy,
            self.queue_pressure,
            self.active_permission_effects.clone(),
            self.inactive_permission_effects.clone(),
        ) {
            self.queue_pressure.clear_pending_write();
            record_shared_resource_event(self.context, self.evidence, event).await;
            lock_runtime_counters(self.runtime_counters)
                .record_failure(self.resource_policy, current_timestamp());
            return InboundEmissionWriteResult::Rejected;
        }
        let write_result = write_all_for_state(
            self.stream,
            bytes,
            self.resource_policy,
            self.connected_at_unix_seconds,
            self.last_activity_unix_seconds,
            self.handshake_state,
        )
        .await;
        self.queue_pressure.clear_pending_write();
        match write_result {
            Ok(WriteWireMessageOutcome::Written) => InboundEmissionWriteResult::Written,
            Ok(WriteWireMessageOutcome::Rejected(event)) => {
                record_shared_resource_event(self.context, self.evidence, event).await;
                lock_runtime_counters(self.runtime_counters)
                    .record_failure(self.resource_policy, current_timestamp());
                InboundEmissionWriteResult::Rejected
            }
            Err(error) => {
                lock_runtime_counters(self.runtime_counters)
                    .record_failure(self.resource_policy, current_timestamp());
                match error.kind() {
                    io::ErrorKind::BrokenPipe
                    | io::ErrorKind::ConnectionAborted
                    | io::ErrorKind::ConnectionReset
                    | io::ErrorKind::NotConnected
                    | io::ErrorKind::UnexpectedEof => InboundEmissionWriteResult::Disconnected,
                    _ => InboundEmissionWriteResult::Failed,
                }
            }
        }
    }

    fn complete(&mut self, receipt: PeerEmissionReceipt) -> Result<(), ()> {
        self.transport
            .network
            .complete_peer_emission(receipt)
            .map(|_outcome| ())
            .map_err(|_error| ())
    }

    fn abort(&mut self, capability: PeerEmissionWriteCapability) -> Result<EffectAbort, ()> {
        self.transport
            .network
            .abort_peer_emission(capability)
            .map_err(|_error| ())
    }

    fn cancellation_cleanup_failed(
        &mut self,
        maybe_cause: Option<InboundEmissionExecutionOutcome>,
    ) {
        lock_evidence(self.evidence).record_resource_event(terminal_failure_event(format!(
            "announcement {maybe_cause:?}; unwritten capability cleanup failed"
        )));
        lock_runtime_counters(self.runtime_counters)
            .record_failure(self.resource_policy, current_timestamp());
    }
}

#[allow(clippy::too_many_arguments)]
async fn drain_inbound_announcements<S, V>(
    maybe_transport: Option<&InboundAnnouncementTransport<S, V>>,
    peer_id: u64,
    stream: &tokio::net::TcpStream,
    _network_magic: open_bitcoin_node::core::primitives::NetworkMagic,
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    queue_pressure: &mut RuntimeQueuePressureState,
    active_permission_effects: Vec<PermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
    evidence: &Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: &Arc<Mutex<InboundRuntimeCounters>>,
) -> bool
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    let Some(transport) = maybe_transport else {
        return true;
    };
    // Capture the CF-configured ledger before taking any affine outbox capability.
    let encoding_network = context.lock().await.basic_filter_network_handle();
    let Ok(emissions) = transport.outboxes.take_peer_emissions(peer_id) else {
        return false;
    };
    let mut executor = SocketInboundEmissionExecutor {
        transport,
        encoding_network: &encoding_network,
        stream,
        peer_id,
        resource_policy,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        handshake_state,
        queue_pressure,
        active_permission_effects,
        inactive_permission_effects,
        context,
        evidence,
        runtime_counters,
    };
    execute_inbound_emissions(emissions, peer_id, &mut executor).await
        == InboundEmissionExecutionOutcome::Complete
}

pub(super) async fn acknowledge_inbound_response_write<S, V>(
    write_result: &io::Result<WriteWireMessageOutcome>,
    response: &mut EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext<S, V>>>,
) -> bool
where
    S: open_bitcoin_node::ChainstateStore + Send + 'static,
    V: open_bitcoin_node::core::chainstate::CoinsView + Send + 'static,
{
    acknowledge_encoded_wire_response(
        matches!(write_result, Ok(WriteWireMessageOutcome::Written)),
        response,
        context,
    )
    .await
}

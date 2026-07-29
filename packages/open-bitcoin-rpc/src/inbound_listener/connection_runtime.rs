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
    core::primitives::NetworkMagic,
    network::{EffectAbort, PeerEmission, PeerEmissionReceipt, PeerEmissionWriteCapability},
    sync::{AnnouncementOutboxNotification, SyncRuntimeError},
};

use crate::{
    ManagedRpcContext,
    context::{
        EncodedWireResponse, acknowledge_encoded_wire_response, resolve_inbound_wire_responses,
    },
};

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

pub(super) async fn handle_inbound_stream(
    peer_id: u64,
    remote_addr: SocketAddr,
    stream: tokio::net::TcpStream,
    context: Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: Arc<Mutex<InboundRuntimeCounters>>,
    connection_control: InboundConnectionControl,
) {
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
        for response in encoded_responses {
            queue_pressure.record_pending_write(response.bytes.len());
            if let Some(event) = queue_pressure_event(
                &resource_policy,
                &queue_pressure,
                permission_decision.active_effects().to_vec(),
                permission_decision.inactive_effects().to_vec(),
            ) {
                acknowledge_encoded_wire_response(false, &response, &context).await;
                record_shared_resource_event(&context, &evidence, event).await;
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break 'message_loop;
            }
            let write_result = write_all_for_state(
                &stream,
                &response.bytes,
                &resource_policy,
                connected_at_unix_seconds,
                last_activity_unix_seconds,
                handshake_state,
            )
            .await;
            queue_pressure.clear_pending_write();
            if !acknowledge_inbound_response_write(&write_result, &response, &context).await {
                lock_runtime_counters(&runtime_counters)
                    .record_failure(&resource_policy, current_timestamp());
                break 'message_loop;
            }
            match write_result {
                Ok(WriteWireMessageOutcome::Written) => {}
                Ok(WriteWireMessageOutcome::Rejected(event)) => {
                    record_shared_resource_event(&context, &evidence, event).await;
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
                Err(_error) => {
                    lock_runtime_counters(&runtime_counters)
                        .record_failure(&resource_policy, current_timestamp());
                    break 'message_loop;
                }
            }
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

async fn wait_for_outbox_notification(
    maybe_notification: &mut Option<AnnouncementOutboxNotification>,
) {
    let Some(notification) = maybe_notification else {
        pending::<()>().await;
        return;
    };
    notification.notified().await;
}

fn unregister_announcement_peer(
    maybe_transport: &Option<InboundAnnouncementTransport>,
    peer_id: u64,
) -> Result<(), SyncRuntimeError> {
    if let Some(transport) = maybe_transport {
        return transport
            .outboxes
            .unregister_peer(&transport.network, peer_id);
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InboundEmissionWriteResult {
    Written,
    Rejected,
    Disconnected,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum InboundEmissionExecutionOutcome {
    Complete,
    TargetMismatch,
    EncodeFailed,
    Rejected,
    Disconnected,
    WriteFailed,
    CompletionFailed,
    AbortFailed,
}

pub(super) trait InboundEmissionExecutor {
    fn encode(&mut self, message: &WireNetworkMessage) -> Result<Vec<u8>, ()>;

    async fn write(&mut self, bytes: &[u8]) -> InboundEmissionWriteResult;

    fn complete(&mut self, receipt: PeerEmissionReceipt) -> Result<(), ()>;

    fn abort(&mut self, capability: PeerEmissionWriteCapability) -> Result<EffectAbort, ()>;
}

pub(super) async fn execute_inbound_emissions<E: InboundEmissionExecutor>(
    emissions: Vec<PeerEmission>,
    peer_id: u64,
    executor: &mut E,
) -> InboundEmissionExecutionOutcome {
    let mut emissions = VecDeque::from(emissions);
    while let Some(emission) = emissions.pop_front() {
        let (target_peer_id, message, capability) = emission.into_parts();
        if target_peer_id != peer_id {
            return abort_current_and_suffix(executor, capability, emissions)
                .map(|()| InboundEmissionExecutionOutcome::TargetMismatch)
                .unwrap_or(InboundEmissionExecutionOutcome::AbortFailed);
        }
        let Ok(bytes) = executor.encode(&message) else {
            return abort_current_and_suffix(executor, capability, emissions)
                .map(|()| InboundEmissionExecutionOutcome::EncodeFailed)
                .unwrap_or(InboundEmissionExecutionOutcome::AbortFailed);
        };
        match executor.write(&bytes).await {
            InboundEmissionWriteResult::Written => {
                if executor.complete(capability.acknowledge_write()).is_err() {
                    if abort_suffix(executor, emissions).is_err() {
                        return InboundEmissionExecutionOutcome::AbortFailed;
                    }
                    return InboundEmissionExecutionOutcome::CompletionFailed;
                }
            }
            InboundEmissionWriteResult::Rejected => {
                return abort_current_and_suffix(executor, capability, emissions)
                    .map(|()| InboundEmissionExecutionOutcome::Rejected)
                    .unwrap_or(InboundEmissionExecutionOutcome::AbortFailed);
            }
            InboundEmissionWriteResult::Disconnected => {
                return abort_current_and_suffix(executor, capability, emissions)
                    .map(|()| InboundEmissionExecutionOutcome::Disconnected)
                    .unwrap_or(InboundEmissionExecutionOutcome::AbortFailed);
            }
            InboundEmissionWriteResult::Failed => {
                return abort_current_and_suffix(executor, capability, emissions)
                    .map(|()| InboundEmissionExecutionOutcome::WriteFailed)
                    .unwrap_or(InboundEmissionExecutionOutcome::AbortFailed);
            }
        }
    }
    InboundEmissionExecutionOutcome::Complete
}

fn abort_current_and_suffix<E: InboundEmissionExecutor>(
    executor: &mut E,
    current: PeerEmissionWriteCapability,
    suffix: VecDeque<PeerEmission>,
) -> Result<(), ()> {
    abort_capabilities(
        executor,
        std::iter::once(current).chain(suffix.into_iter().map(|emission| emission.into_parts().2)),
    )
}

fn abort_suffix<E: InboundEmissionExecutor>(
    executor: &mut E,
    suffix: VecDeque<PeerEmission>,
) -> Result<(), ()> {
    abort_capabilities(
        executor,
        suffix.into_iter().map(|emission| emission.into_parts().2),
    )
}

fn abort_capabilities<E: InboundEmissionExecutor>(
    executor: &mut E,
    capabilities: impl IntoIterator<Item = PeerEmissionWriteCapability>,
) -> Result<(), ()> {
    let mut abort_failed = false;
    for capability in capabilities {
        abort_failed |= !matches!(executor.abort(capability), Ok(EffectAbort::Aborted));
    }
    if abort_failed {
        return Err(());
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
struct SocketInboundEmissionExecutor<'a> {
    transport: &'a InboundAnnouncementTransport,
    stream: &'a tokio::net::TcpStream,
    network_magic: NetworkMagic,
    resource_policy: &'a ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    queue_pressure: &'a mut RuntimeQueuePressureState,
    active_permission_effects: Vec<PermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    context: &'a Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: &'a Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: &'a Arc<Mutex<InboundRuntimeCounters>>,
}

impl InboundEmissionExecutor for SocketInboundEmissionExecutor<'_> {
    fn encode(&mut self, message: &WireNetworkMessage) -> Result<Vec<u8>, ()> {
        message.encode_wire(self.network_magic).map_err(|_error| ())
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
}

#[allow(clippy::too_many_arguments)]
async fn drain_inbound_announcements(
    maybe_transport: Option<&InboundAnnouncementTransport>,
    peer_id: u64,
    stream: &tokio::net::TcpStream,
    network_magic: NetworkMagic,
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    queue_pressure: &mut RuntimeQueuePressureState,
    active_permission_effects: Vec<PermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
    evidence: &Arc<Mutex<InboundListenerEvidence>>,
    runtime_counters: &Arc<Mutex<InboundRuntimeCounters>>,
) -> bool {
    let Some(transport) = maybe_transport else {
        return true;
    };
    let Ok(emissions) = transport.outboxes.take_peer_emissions(peer_id) else {
        return false;
    };
    let mut executor = SocketInboundEmissionExecutor {
        transport,
        stream,
        network_magic,
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

pub(super) async fn acknowledge_inbound_response_write(
    write_result: &io::Result<WriteWireMessageOutcome>,
    response: &EncodedWireResponse,
    context: &Arc<tokio::sync::Mutex<ManagedRpcContext>>,
) -> bool {
    let Ok(WriteWireMessageOutcome::Written) = write_result else {
        return acknowledge_encoded_wire_response(false, response, context).await;
    };
    if response.maybe_block_serve_intent.is_some() {
        return acknowledge_encoded_wire_response(true, response, context).await;
    }
    context
        .lock()
        .await
        .acknowledge_wire_message_written(&response.message)
        .is_ok()
}

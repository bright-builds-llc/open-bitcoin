// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use std::{
    io,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use open_bitcoin_network::{
    INBOUND_MESSAGE_HEADER_LEN, InactivePermissionEffectLabel, InboundAdmissionDecision,
    InboundEnvelopePolicy, InboundHandshakeState, PermissionEffectLabel, ResourceGovernancePolicy,
};
use open_bitcoin_node::core::primitives::NetworkMagic;

use crate::{
    ManagedRpcContext,
    context::{
        EncodedWireResponse, acknowledge_encoded_wire_response, resolve_inbound_wire_responses,
    },
};

use super::{
    InboundAnnouncementTransport, InboundListenerEvidence,
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
    maybe_announcement_transport: Option<InboundAnnouncementTransport>,
) {
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
    if let Some(transport) = maybe_announcement_transport.as_ref()
        && transport.outboxes.register_peer(peer_id).is_err()
    {
        disconnect_admitted_peer(&context, peer_id).await;
        return;
    }

    let Ok(network_info) = context.lock().await.network_info() else {
        unregister_announcement_peer(&maybe_announcement_transport, peer_id);
        return;
    };
    let envelope_policy = InboundEnvelopePolicy::new(network_info.network_magic);

    'message_loop: loop {
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
        let read_result = read_wire_message_for_state(
            &stream,
            &envelope_policy,
            &resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
        )
        .await;
        queue_pressure.clear_pending_read();
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
    unregister_announcement_peer(&maybe_announcement_transport, peer_id);
    disconnect_admitted_peer(&context, peer_id).await;
}

fn unregister_announcement_peer(
    maybe_transport: &Option<InboundAnnouncementTransport>,
    peer_id: u64,
) {
    if let Some(transport) = maybe_transport {
        let _ = transport.outboxes.unregister_peer(peer_id);
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
    for emission in emissions {
        let (target_peer_id, message, receipt) = emission.into_parts();
        if target_peer_id != peer_id {
            return false;
        }
        let Ok(bytes) = message.encode_wire(network_magic) else {
            return false;
        };
        queue_pressure.record_pending_write(bytes.len());
        if let Some(event) = queue_pressure_event(
            resource_policy,
            queue_pressure,
            active_permission_effects.clone(),
            inactive_permission_effects.clone(),
        ) {
            queue_pressure.clear_pending_write();
            record_shared_resource_event(context, evidence, event).await;
            lock_runtime_counters(runtime_counters)
                .record_failure(resource_policy, current_timestamp());
            return false;
        }
        let write_result = write_all_for_state(
            stream,
            &bytes,
            resource_policy,
            connected_at_unix_seconds,
            last_activity_unix_seconds,
            handshake_state,
        )
        .await;
        queue_pressure.clear_pending_write();
        match write_result {
            Ok(WriteWireMessageOutcome::Written) => {
                if transport.network.complete_peer_emission(receipt).is_err() {
                    return false;
                }
            }
            Ok(WriteWireMessageOutcome::Rejected(event)) => {
                record_shared_resource_event(context, evidence, event).await;
                lock_runtime_counters(runtime_counters)
                    .record_failure(resource_policy, current_timestamp());
                return false;
            }
            Err(_error) => {
                lock_runtime_counters(runtime_counters)
                    .record_failure(resource_policy, current_timestamp());
                return false;
            }
        }
    }
    true
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

#[cfg(test)]
mod tests {
    use open_bitcoin_node::sync::AnnouncementOutboxRegistry;

    #[test]
    fn announcement_transport_disconnect_cleanup_is_peer_scoped() {
        // Arrange
        let outboxes = AnnouncementOutboxRegistry::default();
        outboxes.register_peer(41).expect("register first peer");
        outboxes.register_peer(42).expect("register second peer");

        // Act
        outboxes.unregister_peer(41).expect("unregister first peer");
        let snapshots = outboxes.snapshots().expect("outbox snapshots");

        // Assert
        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].peer_id(), 42);
        assert_eq!(snapshots[0].queued_messages(), 0);
    }
}

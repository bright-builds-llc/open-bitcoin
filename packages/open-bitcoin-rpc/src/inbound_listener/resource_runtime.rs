// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

//! Resource-governed socket I/O helpers for the inbound listener.

use std::{io, time::Duration};

use open_bitcoin_codec::parse_message_header;
use open_bitcoin_network::{
    ConnectionChurnInput, INBOUND_MESSAGE_HEADER_LEN, InactivePermissionEffectLabel,
    InboundEnvelopeDecision, InboundEnvelopePolicy, InboundHandshakeState, InboundResourceEvent,
    ParsedNetworkMessage, PermissionEffectLabel, QueuePressureInput, RepeatedFailureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ResourceTimeoutInput, WireNetworkMessage,
};

#[cfg(test)]
use super::current_timestamp;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct InboundRuntimeCounters {
    connection_window_started_unix_seconds: i64,
    connection_attempts_in_window: usize,
    failure_window_started_unix_seconds: i64,
    failures_in_window: usize,
}

impl InboundRuntimeCounters {
    pub(super) fn new(now_unix_seconds: i64) -> Self {
        Self {
            connection_window_started_unix_seconds: now_unix_seconds,
            connection_attempts_in_window: 0,
            failure_window_started_unix_seconds: now_unix_seconds,
            failures_in_window: 0,
        }
    }

    pub(super) fn record_connection_attempt(
        &mut self,
        policy: &ResourceGovernancePolicy,
        now_unix_seconds: i64,
    ) -> ConnectionChurnInput {
        if now_unix_seconds.saturating_sub(self.connection_window_started_unix_seconds)
            > policy.connection_churn_window_seconds
        {
            self.connection_window_started_unix_seconds = now_unix_seconds;
            self.connection_attempts_in_window = 0;
        }
        self.connection_attempts_in_window = self.connection_attempts_in_window.saturating_add(1);
        ConnectionChurnInput {
            window_started_unix_seconds: self.connection_window_started_unix_seconds,
            now_unix_seconds,
            connection_attempts_in_window: self.connection_attempts_in_window,
        }
    }

    pub(super) fn repeated_failure_input(
        &mut self,
        policy: &ResourceGovernancePolicy,
        now_unix_seconds: i64,
    ) -> RepeatedFailureInput {
        if now_unix_seconds.saturating_sub(self.failure_window_started_unix_seconds)
            > policy.repeated_failure_window_seconds
        {
            self.failure_window_started_unix_seconds = now_unix_seconds;
            self.failures_in_window = 0;
        }
        RepeatedFailureInput {
            window_started_unix_seconds: self.failure_window_started_unix_seconds,
            now_unix_seconds,
            failures_in_window: self.failures_in_window,
        }
    }

    pub(super) fn record_failure(
        &mut self,
        policy: &ResourceGovernancePolicy,
        now_unix_seconds: i64,
    ) {
        let _ = self.repeated_failure_input(policy, now_unix_seconds);
        self.failures_in_window = self.failures_in_window.saturating_add(1);
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct RuntimeQueuePressureState {
    peer_read_queue_bytes: usize,
    peer_write_queue_bytes: usize,
    aggregate_read_queue_bytes: usize,
    aggregate_write_queue_bytes: usize,
    peer_queued_messages: usize,
    aggregate_queued_messages: usize,
}

impl RuntimeQueuePressureState {
    pub(super) fn record_pending_read(&mut self, bytes: usize) {
        self.peer_read_queue_bytes = self.peer_read_queue_bytes.saturating_add(bytes);
        self.aggregate_read_queue_bytes = self.aggregate_read_queue_bytes.saturating_add(bytes);
        self.record_queued_message();
    }

    pub(super) fn clear_pending_read(&mut self) {
        self.aggregate_read_queue_bytes = self
            .aggregate_read_queue_bytes
            .saturating_sub(self.peer_read_queue_bytes);
        self.peer_read_queue_bytes = 0;
        self.clear_queued_message();
    }

    pub(super) fn record_pending_write(&mut self, bytes: usize) {
        self.peer_write_queue_bytes = self.peer_write_queue_bytes.saturating_add(bytes);
        self.aggregate_write_queue_bytes = self.aggregate_write_queue_bytes.saturating_add(bytes);
        self.record_queued_message();
    }

    pub(super) fn clear_pending_write(&mut self) {
        self.aggregate_write_queue_bytes = self
            .aggregate_write_queue_bytes
            .saturating_sub(self.peer_write_queue_bytes);
        self.peer_write_queue_bytes = 0;
        self.clear_queued_message();
    }

    pub(super) fn queue_pressure_input(
        &self,
        active_permission_effects: Vec<PermissionEffectLabel>,
        inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    ) -> QueuePressureInput {
        QueuePressureInput {
            peer_read_queue_bytes: self.peer_read_queue_bytes,
            peer_write_queue_bytes: self.peer_write_queue_bytes,
            aggregate_read_queue_bytes: self.aggregate_read_queue_bytes,
            aggregate_write_queue_bytes: self.aggregate_write_queue_bytes,
            peer_queued_messages: self.peer_queued_messages,
            aggregate_queued_messages: self.aggregate_queued_messages,
            active_permission_effects,
            inactive_permission_effects,
        }
    }

    fn record_queued_message(&mut self) {
        self.peer_queued_messages = self.peer_queued_messages.saturating_add(1);
        self.aggregate_queued_messages = self.aggregate_queued_messages.saturating_add(1);
    }

    fn clear_queued_message(&mut self) {
        self.peer_queued_messages = self.peer_queued_messages.saturating_sub(1);
        self.aggregate_queued_messages = self.aggregate_queued_messages.saturating_sub(1);
    }

    #[cfg(test)]
    pub(super) fn record_aggregate_queued_messages(&mut self, messages: usize) {
        self.aggregate_queued_messages = self.aggregate_queued_messages.saturating_add(messages);
    }
}

pub(super) fn queue_pressure_event(
    policy: &ResourceGovernancePolicy,
    state: &RuntimeQueuePressureState,
    active_permission_effects: Vec<PermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
) -> Option<InboundResourceEvent> {
    resource_event_from_decision(policy.decide_queue(
        state.queue_pressure_input(active_permission_effects, inactive_permission_effects),
    ))
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ReadWireMessageOutcome {
    Message(ParsedNetworkMessage),
    Rejected(InboundResourceEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum WriteWireMessageOutcome {
    Written,
    Rejected(InboundResourceEvent),
}

#[cfg(test)]
pub(super) async fn read_wire_message(
    stream: &tokio::net::TcpStream,
    policy: &InboundEnvelopePolicy,
) -> io::Result<ReadWireMessageOutcome> {
    let now_unix_seconds = current_timestamp();
    read_wire_message_for_state(
        stream,
        policy,
        &ResourceGovernancePolicy::default(),
        now_unix_seconds,
        now_unix_seconds,
        InboundHandshakeState::Accepted,
    )
    .await
}

pub(super) async fn read_wire_message_for_state(
    stream: &tokio::net::TcpStream,
    envelope_policy: &InboundEnvelopePolicy,
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
) -> io::Result<ReadWireMessageOutcome> {
    read_wire_message_with_timeout_duration(
        stream,
        envelope_policy,
        resource_policy,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        handshake_state,
        timeout_duration_for_handshake(resource_policy, handshake_state),
    )
    .await
}

pub(super) async fn read_wire_message_with_timeout_duration(
    stream: &tokio::net::TcpStream,
    envelope_policy: &InboundEnvelopePolicy,
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    timeout_duration: Duration,
) -> io::Result<ReadWireMessageOutcome> {
    let mut header = [0_u8; INBOUND_MESSAGE_HEADER_LEN];
    if read_exact_with_timeout(stream, &mut header, timeout_duration).await?
        == SocketIoOutcome::Timeout
    {
        return Ok(ReadWireMessageOutcome::Rejected(
            timeout_event_after_elapsed(
                resource_policy,
                connected_at_unix_seconds,
                last_activity_unix_seconds,
                handshake_state,
            ),
        ));
    }
    let payload_len = match envelope_policy.evaluate_header(&header) {
        InboundEnvelopeDecision::ReadPayload { payload_len } => payload_len,
        InboundEnvelopeDecision::Reject(event) => {
            return Ok(ReadWireMessageOutcome::Rejected(event));
        }
    };
    let header_message = parse_message_header(&header).map_err(|error| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("inbound wire message header rejected after policy evaluation: {error}"),
        )
    })?;
    let mut payload = vec![0_u8; payload_len];
    if read_exact_with_timeout(stream, &mut payload, timeout_duration).await?
        == SocketIoOutcome::Timeout
    {
        return Ok(ReadWireMessageOutcome::Rejected(
            timeout_event_after_elapsed(
                resource_policy,
                connected_at_unix_seconds,
                last_activity_unix_seconds,
                handshake_state,
            ),
        ));
    }
    match envelope_policy.decode_payload(&header_message, &payload) {
        Ok(parsed) => Ok(ReadWireMessageOutcome::Message(parsed)),
        Err(event) => Ok(ReadWireMessageOutcome::Rejected(event)),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SocketIoOutcome {
    Complete,
    Timeout,
}

async fn read_exact_with_timeout(
    stream: &tokio::net::TcpStream,
    buffer: &mut [u8],
    timeout_duration: Duration,
) -> io::Result<SocketIoOutcome> {
    let mut offset = 0;
    while offset < buffer.len() {
        match tokio::time::timeout(timeout_duration, stream.readable()).await {
            Ok(result) => result?,
            Err(_elapsed) => return Ok(SocketIoOutcome::Timeout),
        }
        match stream.try_read(&mut buffer[offset..]) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::UnexpectedEof)),
            Ok(read) => offset += read,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }
    Ok(SocketIoOutcome::Complete)
}

#[cfg(test)]
pub(super) async fn write_all(stream: &tokio::net::TcpStream, buffer: &[u8]) -> io::Result<()> {
    let resource_policy = ResourceGovernancePolicy::default();
    let outcome = write_all_with_timeout_duration(
        stream,
        buffer,
        &resource_policy,
        current_timestamp(),
        current_timestamp(),
        InboundHandshakeState::Accepted,
        Duration::from_secs(resource_policy.slow_handshake_timeout_seconds as u64),
    )
    .await?;
    match outcome {
        WriteWireMessageOutcome::Written => Ok(()),
        WriteWireMessageOutcome::Rejected(_event) => Err(io::Error::from(io::ErrorKind::TimedOut)),
    }
}

pub(super) async fn write_all_for_state(
    stream: &tokio::net::TcpStream,
    buffer: &[u8],
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
) -> io::Result<WriteWireMessageOutcome> {
    write_all_with_timeout_duration(
        stream,
        buffer,
        resource_policy,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        handshake_state,
        timeout_duration_for_handshake(resource_policy, handshake_state),
    )
    .await
}

async fn write_all_with_timeout_duration(
    stream: &tokio::net::TcpStream,
    buffer: &[u8],
    resource_policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
    timeout_duration: Duration,
) -> io::Result<WriteWireMessageOutcome> {
    let mut offset = 0;
    while offset < buffer.len() {
        match tokio::time::timeout(timeout_duration, stream.writable()).await {
            Ok(result) => result?,
            Err(_elapsed) => {
                return Ok(WriteWireMessageOutcome::Rejected(
                    timeout_event_after_elapsed(
                        resource_policy,
                        connected_at_unix_seconds,
                        last_activity_unix_seconds,
                        handshake_state,
                    ),
                ));
            }
        }
        match stream.try_write(&buffer[offset..]) {
            Ok(0) => return Err(io::Error::from(io::ErrorKind::WriteZero)),
            Ok(written) => offset += written,
            Err(error) if error.kind() == io::ErrorKind::WouldBlock => {}
            Err(error) => return Err(error),
        }
    }
    Ok(WriteWireMessageOutcome::Written)
}

pub(super) fn resource_event_from_decision(
    decision: ResourceGovernanceDecision,
) -> Option<InboundResourceEvent> {
    match decision {
        ResourceGovernanceDecision::Accept => None,
        ResourceGovernanceDecision::Backpressure(event)
        | ResourceGovernanceDecision::Disconnect(event)
        | ResourceGovernanceDecision::RecordMisbehavior(event) => Some(event),
    }
}

pub(super) fn resource_timeout_event(
    policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    now_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
) -> Option<InboundResourceEvent> {
    resource_event_from_decision(policy.decide_timeout(ResourceTimeoutInput {
        handshake_state,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        now_unix_seconds,
    }))
}

fn timeout_event_after_elapsed(
    policy: &ResourceGovernancePolicy,
    connected_at_unix_seconds: i64,
    last_activity_unix_seconds: i64,
    handshake_state: InboundHandshakeState,
) -> InboundResourceEvent {
    let synthetic_now = match handshake_state {
        InboundHandshakeState::Established => last_activity_unix_seconds
            .saturating_add(policy.idle_peer_timeout_seconds)
            .saturating_add(1),
        _ => connected_at_unix_seconds
            .saturating_add(policy.slow_handshake_timeout_seconds)
            .saturating_add(1),
    };
    resource_timeout_event(
        policy,
        connected_at_unix_seconds,
        last_activity_unix_seconds,
        synthetic_now,
        handshake_state,
    )
    .unwrap_or_else(|| fallback_timeout_event(handshake_state))
}

fn fallback_timeout_event(handshake_state: InboundHandshakeState) -> InboundResourceEvent {
    let (label, reason) = if handshake_state == InboundHandshakeState::Established {
        (
            "idle_peer",
            "established inbound peer exceeded idle timeout",
        )
    } else {
        (
            "slow_handshake",
            "inbound peer did not complete handshake before timeout",
        )
    };
    InboundResourceEvent {
        outcome: "resource_governance".to_string(),
        reason: reason.to_string(),
        label: label.to_string(),
        source: "source_runtime_read".to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: "timeout_disconnect".to_string(),
    }
}

fn timeout_duration_for_handshake(
    policy: &ResourceGovernancePolicy,
    handshake_state: InboundHandshakeState,
) -> Duration {
    let seconds = if handshake_state == InboundHandshakeState::Established {
        policy.idle_peer_timeout_seconds
    } else {
        policy.slow_handshake_timeout_seconds
    };
    Duration::from_secs(u64::try_from(seconds.max(0)).unwrap_or_default())
}

pub(super) fn next_handshake_state(
    current: InboundHandshakeState,
    message: &WireNetworkMessage,
) -> InboundHandshakeState {
    match (current, message) {
        (InboundHandshakeState::Accepted, WireNetworkMessage::Version(_)) => {
            InboundHandshakeState::Handshaking
        }
        (
            InboundHandshakeState::Accepted | InboundHandshakeState::Handshaking,
            WireNetworkMessage::Verack,
        ) => InboundHandshakeState::Established,
        _ => current,
    }
}

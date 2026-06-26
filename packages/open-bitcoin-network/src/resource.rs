// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use open_bitcoin_codec::{CodecError, parse_message_header};
use open_bitcoin_consensus::crypto::double_sha256;
use open_bitcoin_primitives::{MessageHeader, NetworkMagic};

use crate::message::{MAX_HEADERS_RESULTS, MAX_INV_SIZE};
use crate::peer::DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
use crate::{
    InactivePermissionEffectLabel, InboundHandshakeState, NetworkError, ParsedNetworkMessage,
    PermissionEffectLabel, WireNetworkMessage,
};

pub const INBOUND_MESSAGE_HEADER_LEN: usize = 24;
pub const PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES: usize = 32 * 1024 * 1024;
pub const PHASE94_MAX_PEER_READ_QUEUE_BYTES: usize = 1024 * 1024;
pub const PHASE94_MAX_PEER_WRITE_QUEUE_BYTES: usize = 1024 * 1024;
pub const PHASE94_MAX_AGGREGATE_READ_QUEUE_BYTES: usize = 8 * 1024 * 1024;
pub const PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES: usize = 8 * 1024 * 1024;
pub const PHASE94_MAX_PEER_QUEUED_MESSAGES: usize = 128;
pub const PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES: usize = 1024;
pub const PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER: usize = 1024;
pub const PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER: usize =
    DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
pub const PHASE94_MAX_HEADER_LOCATOR_HASHES: usize = MAX_HEADERS_RESULTS;
pub const PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS: usize = MAX_INV_SIZE;
pub const PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS: i64 = 60;
pub const PHASE94_IDLE_PEER_TIMEOUT_SECONDS: i64 = 1_800;
pub const PHASE94_CONNECTION_CHURN_WINDOW_SECONDS: i64 = 60;
pub const PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW: usize = 16;
pub const PHASE94_REPEATED_FAILURE_WINDOW_SECONDS: i64 = 300;
pub const PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW: usize = 8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceViolationLabel {
    WrongNetworkMagic,
    MalformedHeader,
    PayloadOversized,
    InvalidChecksum,
    UnsupportedCommand,
    MalformedPayload,
    TrailingPayload,
}

impl ResourceViolationLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::WrongNetworkMagic => "wrong_network_magic",
            Self::MalformedHeader => "malformed_header",
            Self::PayloadOversized => "payload_oversized",
            Self::InvalidChecksum => "invalid_checksum",
            Self::UnsupportedCommand => "unsupported_command",
            Self::MalformedPayload => "malformed_payload",
            Self::TrailingPayload => "trailing_payload",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourcePressureLabel {
    ResourcePressureActive,
    ReadQueuePressure,
    WriteQueuePressure,
    RequestCapReached,
}

impl ResourcePressureLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ResourcePressureActive => "resource_pressure_active",
            Self::ReadQueuePressure => "read_queue_pressure",
            Self::WriteQueuePressure => "write_queue_pressure",
            Self::RequestCapReached => "request_cap_reached",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceLifecycleLabel {
    SlowHandshake,
    IdlePeer,
    ConnectionChurnLimited,
    RepeatedFailureLimited,
    ReconnectSuppressedBanned,
    ReconnectSuppressedDiscouraged,
}

impl ResourceLifecycleLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SlowHandshake => "slow_handshake",
            Self::IdlePeer => "idle_peer",
            Self::ConnectionChurnLimited => "connection_churn_limited",
            Self::RepeatedFailureLimited => "repeated_failure_limited",
            Self::ReconnectSuppressedBanned => "reconnect_suppressed_banned",
            Self::ReconnectSuppressedDiscouraged => "reconnect_suppressed_discouraged",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceGovernanceSource {
    EnvelopeGate,
    PayloadDecoder,
    RuntimeRead,
}

impl ResourceGovernanceSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnvelopeGate => "source_envelope_gate",
            Self::PayloadDecoder => "source_payload_decoder",
            Self::RuntimeRead => "source_runtime_read",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResourceGovernanceDecision {
    Accept,
    Backpressure(InboundResourceEvent),
    Disconnect(InboundResourceEvent),
    RecordMisbehavior(InboundResourceEvent),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InboundResourceEvent {
    pub outcome: String,
    pub reason: String,
    pub label: String,
    pub source: String,
    pub message: String,
    pub next_action: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InboundEnvelopeDecision {
    ReadPayload { payload_len: usize },
    Reject(InboundResourceEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InboundEnvelopePolicy {
    pub expected_magic: NetworkMagic,
    pub max_payload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct QueuePressureInput {
    pub peer_read_queue_bytes: usize,
    pub peer_write_queue_bytes: usize,
    pub aggregate_read_queue_bytes: usize,
    pub aggregate_write_queue_bytes: usize,
    pub peer_queued_messages: usize,
    pub aggregate_queued_messages: usize,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RequestPressureInput {
    pub inventory_items: usize,
    pub getdata_items: usize,
    pub header_locator_hashes: usize,
    pub requested_blocks_in_flight: usize,
    pub requested_txids_in_flight: usize,
    pub requested_wtxids_in_flight: usize,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceTimeoutInput {
    pub handshake_state: InboundHandshakeState,
    pub connected_at_unix_seconds: i64,
    pub last_activity_unix_seconds: i64,
    pub now_unix_seconds: i64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConnectionChurnInput {
    pub window_started_unix_seconds: i64,
    pub now_unix_seconds: i64,
    pub connection_attempts_in_window: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RepeatedFailureInput {
    pub window_started_unix_seconds: i64,
    pub now_unix_seconds: i64,
    pub failures_in_window: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct ReconnectSuppressionInput {
    pub banned: bool,
    pub discouraged: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResourceGovernancePolicy {
    pub max_peer_read_queue_bytes: usize,
    pub max_peer_write_queue_bytes: usize,
    pub max_aggregate_read_queue_bytes: usize,
    pub max_aggregate_write_queue_bytes: usize,
    pub max_peer_queued_messages: usize,
    pub max_aggregate_queued_messages: usize,
    pub max_inbound_tx_requests_per_peer: usize,
    pub max_inbound_block_requests_per_peer: usize,
    pub max_header_locator_hashes: usize,
    pub max_inbound_request_inventory_items: usize,
    pub slow_handshake_timeout_seconds: i64,
    pub idle_peer_timeout_seconds: i64,
    pub connection_churn_window_seconds: i64,
    pub max_connections_per_churn_window: usize,
    pub repeated_failure_window_seconds: i64,
    pub max_repeated_failures_per_window: usize,
}

impl Default for ResourceGovernancePolicy {
    fn default() -> Self {
        Self {
            max_peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES,
            max_peer_write_queue_bytes: PHASE94_MAX_PEER_WRITE_QUEUE_BYTES,
            max_aggregate_read_queue_bytes: PHASE94_MAX_AGGREGATE_READ_QUEUE_BYTES,
            max_aggregate_write_queue_bytes: PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES,
            max_peer_queued_messages: PHASE94_MAX_PEER_QUEUED_MESSAGES,
            max_aggregate_queued_messages: PHASE94_MAX_AGGREGATE_QUEUED_MESSAGES,
            max_inbound_tx_requests_per_peer: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
            max_inbound_block_requests_per_peer: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
            max_header_locator_hashes: PHASE94_MAX_HEADER_LOCATOR_HASHES,
            max_inbound_request_inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
            slow_handshake_timeout_seconds: PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS,
            idle_peer_timeout_seconds: PHASE94_IDLE_PEER_TIMEOUT_SECONDS,
            connection_churn_window_seconds: PHASE94_CONNECTION_CHURN_WINDOW_SECONDS,
            max_connections_per_churn_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW,
            repeated_failure_window_seconds: PHASE94_REPEATED_FAILURE_WINDOW_SECONDS,
            max_repeated_failures_per_window: PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW,
        }
    }
}

impl ResourceGovernancePolicy {
    pub fn decide_queue(&self, input: QueuePressureInput) -> ResourceGovernanceDecision {
        if input.peer_read_queue_bytes > self.max_peer_read_queue_bytes
            || input.aggregate_read_queue_bytes > self.max_aggregate_read_queue_bytes
        {
            return ResourceGovernanceDecision::Backpressure(resource_pressure_event(
                ResourcePressureLabel::ReadQueuePressure,
                "read queue resource pressure exceeded configured cap",
                "read_queue_pressure",
            ));
        }

        if input.peer_write_queue_bytes > self.max_peer_write_queue_bytes
            || input.aggregate_write_queue_bytes > self.max_aggregate_write_queue_bytes
        {
            return ResourceGovernanceDecision::Backpressure(resource_pressure_event(
                ResourcePressureLabel::WriteQueuePressure,
                "write queue resource pressure exceeded configured cap",
                "write_queue_pressure",
            ));
        }

        if input.peer_queued_messages > self.max_peer_queued_messages
            || input.aggregate_queued_messages > self.max_aggregate_queued_messages
        {
            return ResourceGovernanceDecision::Backpressure(resource_pressure_event(
                ResourcePressureLabel::ResourcePressureActive,
                "queued message resource pressure exceeded configured cap",
                "resource_pressure_active",
            ));
        }

        ResourceGovernanceDecision::Accept
    }

    pub fn decide_request(&self, input: RequestPressureInput) -> ResourceGovernanceDecision {
        let requested_transactions = input
            .requested_txids_in_flight
            .saturating_add(input.requested_wtxids_in_flight);
        if input.inventory_items > self.max_inbound_request_inventory_items
            || input.getdata_items > self.max_inbound_request_inventory_items
            || input.header_locator_hashes > self.max_header_locator_hashes
            || input.requested_blocks_in_flight > self.max_inbound_block_requests_per_peer
            || requested_transactions > self.max_inbound_tx_requests_per_peer
        {
            return ResourceGovernanceDecision::Disconnect(resource_pressure_event(
                ResourcePressureLabel::RequestCapReached,
                "inbound request or inventory cap exceeded configured limit",
                "request_cap_reached",
            ));
        }

        ResourceGovernanceDecision::Accept
    }

    pub fn decide_timeout(&self, input: ResourceTimeoutInput) -> ResourceGovernanceDecision {
        if matches!(
            input.handshake_state,
            InboundHandshakeState::Accepted | InboundHandshakeState::Handshaking
        ) && elapsed_seconds(input.connected_at_unix_seconds, input.now_unix_seconds)
            > self.slow_handshake_timeout_seconds
        {
            return ResourceGovernanceDecision::Disconnect(resource_lifecycle_event(
                ResourceLifecycleLabel::SlowHandshake,
                "inbound peer did not complete handshake before timeout",
                "timeout_disconnect",
            ));
        }

        if input.handshake_state == InboundHandshakeState::Established
            && elapsed_seconds(input.last_activity_unix_seconds, input.now_unix_seconds)
                > self.idle_peer_timeout_seconds
        {
            return ResourceGovernanceDecision::Disconnect(resource_lifecycle_event(
                ResourceLifecycleLabel::IdlePeer,
                "established inbound peer exceeded idle timeout",
                "timeout_disconnect",
            ));
        }

        ResourceGovernanceDecision::Accept
    }

    pub fn decide_churn(&self, input: ConnectionChurnInput) -> ResourceGovernanceDecision {
        if is_within_window(
            input.window_started_unix_seconds,
            input.now_unix_seconds,
            self.connection_churn_window_seconds,
        ) && input.connection_attempts_in_window > self.max_connections_per_churn_window
        {
            return ResourceGovernanceDecision::Backpressure(resource_lifecycle_event(
                ResourceLifecycleLabel::ConnectionChurnLimited,
                "connection churn exceeded configured window cap",
                "churn_rejected",
            ));
        }

        ResourceGovernanceDecision::Accept
    }

    pub fn decide_repeated_failure(
        &self,
        input: RepeatedFailureInput,
    ) -> ResourceGovernanceDecision {
        if is_within_window(
            input.window_started_unix_seconds,
            input.now_unix_seconds,
            self.repeated_failure_window_seconds,
        ) && input.failures_in_window > self.max_repeated_failures_per_window
        {
            return ResourceGovernanceDecision::Backpressure(resource_lifecycle_event(
                ResourceLifecycleLabel::RepeatedFailureLimited,
                "connection failures exceeded configured window cap",
                "churn_rejected",
            ));
        }

        ResourceGovernanceDecision::Accept
    }

    pub fn decide_reconnect(&self, input: ReconnectSuppressionInput) -> ResourceGovernanceDecision {
        if input.banned {
            return ResourceGovernanceDecision::Disconnect(resource_lifecycle_event(
                ResourceLifecycleLabel::ReconnectSuppressedBanned,
                "active ban suppresses reconnect",
                "reconnect_suppressed",
            ));
        }

        if input.discouraged {
            return ResourceGovernanceDecision::Backpressure(resource_lifecycle_event(
                ResourceLifecycleLabel::ReconnectSuppressedDiscouraged,
                "discouraged peer reconnect is suppressed",
                "reconnect_suppressed",
            ));
        }

        ResourceGovernanceDecision::Accept
    }
}

impl InboundEnvelopePolicy {
    pub const fn new(expected_magic: NetworkMagic) -> Self {
        Self {
            expected_magic,
            max_payload_bytes: PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES,
        }
    }

    pub fn evaluate_header(&self, header_bytes: &[u8]) -> InboundEnvelopeDecision {
        if header_bytes.len() != INBOUND_MESSAGE_HEADER_LEN {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::MalformedHeader,
                ResourceGovernanceSource::EnvelopeGate,
                "message header is not exactly 24 bytes",
            ));
        }

        let header = match parse_message_header(header_bytes) {
            Ok(header) => header,
            Err(_) => {
                return InboundEnvelopeDecision::Reject(Self::event(
                    ResourceViolationLabel::MalformedHeader,
                    ResourceGovernanceSource::EnvelopeGate,
                    "message header parse failed",
                ));
            }
        };

        if header.magic != self.expected_magic {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::WrongNetworkMagic,
                ResourceGovernanceSource::EnvelopeGate,
                "message network magic did not match local network",
            ));
        }

        let payload_len = header.payload_size as usize;
        if payload_len > self.max_payload_bytes {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::PayloadOversized,
                ResourceGovernanceSource::EnvelopeGate,
                "message payload length exceeds inbound runtime cap",
            ));
        }

        if !is_supported_command(header.command.as_str()) {
            return InboundEnvelopeDecision::Reject(Self::event(
                ResourceViolationLabel::UnsupportedCommand,
                ResourceGovernanceSource::EnvelopeGate,
                "message command is outside the supported inbound surface",
            ));
        }

        InboundEnvelopeDecision::ReadPayload { payload_len }
    }

    #[allow(clippy::result_large_err)]
    pub fn decode_payload(
        &self,
        header: &MessageHeader,
        payload: &[u8],
    ) -> Result<ParsedNetworkMessage, InboundResourceEvent> {
        if header.magic != self.expected_magic {
            return Err(Self::event(
                ResourceViolationLabel::WrongNetworkMagic,
                ResourceGovernanceSource::EnvelopeGate,
                "message network magic did not match local network",
            ));
        }

        if !is_supported_command(header.command.as_str()) {
            return Err(Self::event(
                ResourceViolationLabel::UnsupportedCommand,
                ResourceGovernanceSource::EnvelopeGate,
                "message command is outside the supported inbound surface",
            ));
        }

        let expected_payload_len = header.payload_size as usize;
        if expected_payload_len > self.max_payload_bytes {
            return Err(Self::event(
                ResourceViolationLabel::PayloadOversized,
                ResourceGovernanceSource::EnvelopeGate,
                "message payload length exceeds inbound runtime cap",
            ));
        }

        if payload.len() > expected_payload_len {
            return Err(Self::event(
                ResourceViolationLabel::TrailingPayload,
                ResourceGovernanceSource::PayloadDecoder,
                "payload contains trailing bytes beyond header length",
            ));
        }

        if payload.len() < expected_payload_len {
            return Err(Self::event(
                ResourceViolationLabel::MalformedPayload,
                ResourceGovernanceSource::PayloadDecoder,
                "payload ended before header length",
            ));
        }

        if checksum(payload) != header.checksum {
            return Err(Self::event(
                ResourceViolationLabel::InvalidChecksum,
                ResourceGovernanceSource::EnvelopeGate,
                "payload checksum did not match message header",
            ));
        }

        let message = WireNetworkMessage::decode_payload(&header.command, payload)
            .map_err(resource_event_for_decode_error)?;
        Ok(ParsedNetworkMessage {
            header: header.clone(),
            message,
        })
    }

    pub fn event(
        label: ResourceViolationLabel,
        source: ResourceGovernanceSource,
        reason: impl Into<String>,
    ) -> InboundResourceEvent {
        InboundResourceEvent {
            outcome: "rejected".to_string(),
            reason: reason.into(),
            label: label.as_str().to_string(),
            source: source.as_str().to_string(),
            message: "inbound_message_resource_governance".to_string(),
            next_action: "payload_rejected".to_string(),
        }
    }
}

fn resource_pressure_event(
    label: ResourcePressureLabel,
    reason: impl Into<String>,
    next_action: &'static str,
) -> InboundResourceEvent {
    InboundResourceEvent {
        outcome: "resource_governance".to_string(),
        reason: reason.into(),
        label: label.as_str().to_string(),
        source: ResourceGovernanceSource::RuntimeRead.as_str().to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: next_action.to_string(),
    }
}

fn resource_lifecycle_event(
    label: ResourceLifecycleLabel,
    reason: impl Into<String>,
    next_action: &'static str,
) -> InboundResourceEvent {
    InboundResourceEvent {
        outcome: "resource_governance".to_string(),
        reason: reason.into(),
        label: label.as_str().to_string(),
        source: ResourceGovernanceSource::RuntimeRead.as_str().to_string(),
        message: "inbound_resource_governance".to_string(),
        next_action: next_action.to_string(),
    }
}

fn elapsed_seconds(start_unix_seconds: i64, now_unix_seconds: i64) -> i64 {
    now_unix_seconds.saturating_sub(start_unix_seconds)
}

fn is_within_window(start_unix_seconds: i64, now_unix_seconds: i64, window_seconds: i64) -> bool {
    elapsed_seconds(start_unix_seconds, now_unix_seconds) <= window_seconds
}

fn resource_event_for_decode_error(error: NetworkError) -> InboundResourceEvent {
    match error {
        NetworkError::UnknownCommand(_) => InboundEnvelopePolicy::event(
            ResourceViolationLabel::UnsupportedCommand,
            ResourceGovernanceSource::PayloadDecoder,
            "message command is outside the supported inbound surface",
        ),
        NetworkError::InvalidChecksum => InboundEnvelopePolicy::event(
            ResourceViolationLabel::InvalidChecksum,
            ResourceGovernanceSource::EnvelopeGate,
            "payload checksum did not match message header",
        ),
        NetworkError::Codec(CodecError::TrailingData { .. }) => InboundEnvelopePolicy::event(
            ResourceViolationLabel::TrailingPayload,
            ResourceGovernanceSource::PayloadDecoder,
            "payload decoder found trailing bytes",
        ),
        _ => InboundEnvelopePolicy::event(
            ResourceViolationLabel::MalformedPayload,
            ResourceGovernanceSource::PayloadDecoder,
            "payload decoder rejected malformed payload",
        ),
    }
}

fn is_supported_command(command: &str) -> bool {
    matches!(
        command,
        "version"
            | "verack"
            | "wtxidrelay"
            | "sendheaders"
            | "getaddr"
            | "ping"
            | "pong"
            | "getheaders"
            | "headers"
            | "addr"
            | "inv"
            | "getdata"
            | "notfound"
            | "tx"
            | "block"
    )
}

fn checksum(payload: &[u8]) -> [u8; 4] {
    let digest = double_sha256(payload);
    [digest[0], digest[1], digest[2], digest[3]]
}

#[cfg(test)]
mod tests;

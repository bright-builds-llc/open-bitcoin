// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use open_bitcoin_primitives::{MessageCommand, MessageHeader, NetworkMagic};

use super::{
    ConnectionChurnInput, INBOUND_MESSAGE_HEADER_LEN, InboundEnvelopeDecision,
    InboundEnvelopePolicy, PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES,
    PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW, PHASE94_MAX_HEADER_LOCATOR_HASHES,
    PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER, PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
    PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES, PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
    PHASE94_MAX_PEER_QUEUED_MESSAGES, PHASE94_MAX_PEER_READ_QUEUE_BYTES,
    PHASE94_MAX_PEER_WRITE_QUEUE_BYTES, PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW,
    PHASE94_REPEATED_FAILURE_WINDOW_SECONDS, PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS,
    QueuePressureInput, ReconnectSuppressionInput, RepeatedFailureInput, RequestPressureInput,
    ResourceGovernanceDecision, ResourceGovernancePolicy, ResourceGovernanceSource,
    ResourceLifecycleLabel, ResourcePressureLabel, ResourceTimeoutInput, ResourceViolationLabel,
    WireNetworkMessage,
};
use crate::{InactivePermissionEffectLabel, InboundHandshakeState, PermissionEffectLabel};

fn valid_ping_header_and_payload() -> (MessageHeader, Vec<u8>) {
    let wire = WireNetworkMessage::Ping { nonce: 7 }
        .encode_wire(NetworkMagic::MAINNET)
        .expect("wire");
    let header = open_bitcoin_codec::parse_message_header(&wire[..INBOUND_MESSAGE_HEADER_LEN])
        .expect("header");
    let payload = wire[INBOUND_MESSAGE_HEADER_LEN..].to_vec();
    (header, payload)
}

fn assert_rejection(decision: InboundEnvelopeDecision, label: ResourceViolationLabel) {
    let InboundEnvelopeDecision::Reject(event) = decision else {
        panic!("expected rejection");
    };
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, label.as_str());
}

fn empty_queue_input() -> QueuePressureInput {
    QueuePressureInput {
        peer_read_queue_bytes: 0,
        peer_write_queue_bytes: 0,
        aggregate_read_queue_bytes: 0,
        aggregate_write_queue_bytes: 0,
        peer_queued_messages: 0,
        aggregate_queued_messages: 0,
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
    }
}

fn empty_request_input() -> RequestPressureInput {
    RequestPressureInput {
        inventory_items: 0,
        getdata_items: 0,
        header_locator_hashes: 0,
        requested_blocks_in_flight: 0,
        requested_txids_in_flight: 0,
        requested_wtxids_in_flight: 0,
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
    }
}

fn assert_resource_event(
    decision: ResourceGovernanceDecision,
    label: ResourcePressureLabel,
    next_action: &str,
) {
    let event = match decision {
        ResourceGovernanceDecision::Accept => panic!("expected resource event"),
        ResourceGovernanceDecision::Backpressure(event)
        | ResourceGovernanceDecision::Disconnect(event)
        | ResourceGovernanceDecision::RecordMisbehavior(event) => event,
    };

    assert_eq!(event.outcome, "resource_governance");
    assert_eq!(event.label, label.as_str());
    assert_eq!(event.next_action, next_action);
}

fn assert_lifecycle_event(
    decision: ResourceGovernanceDecision,
    label: ResourceLifecycleLabel,
    next_action: &str,
) {
    let event = match decision {
        ResourceGovernanceDecision::Accept => panic!("expected lifecycle event"),
        ResourceGovernanceDecision::Backpressure(event)
        | ResourceGovernanceDecision::Disconnect(event)
        | ResourceGovernanceDecision::RecordMisbehavior(event) => event,
    };

    assert_eq!(event.outcome, "resource_governance");
    assert_eq!(event.label, label.as_str());
    assert_eq!(event.next_action, next_action);
}

mod envelope_cases;
mod lifecycle_cases;
mod pressure_cases;

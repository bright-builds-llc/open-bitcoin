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

#[test]
fn read_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::ReadQueuePressure,
        "read_queue_pressure",
    );
}

#[test]
fn write_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_write_queue_bytes: PHASE94_MAX_PEER_WRITE_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::WriteQueuePressure,
        "write_queue_pressure",
    );
}

#[test]
fn aggregate_write_queue_pressure_returns_stable_label_and_action() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        aggregate_write_queue_bytes: PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::WriteQueuePressure,
        "write_queue_pressure",
    );
}

#[test]
fn queued_message_pressure_returns_resource_pressure_active() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = QueuePressureInput {
        peer_queued_messages: PHASE94_MAX_PEER_QUEUED_MESSAGES + 1,
        ..empty_queue_input()
    };

    // Act
    let decision = policy.decide_queue(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::ResourcePressureActive,
        "resource_pressure_active",
    );
}

#[test]
fn inbound_getdata_inventory_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn inventory_count_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn header_locator_above_cap_returns_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        header_locator_hashes: PHASE94_MAX_HEADER_LOCATOR_HASHES + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn block_requests_above_cap_return_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn transaction_requests_above_cap_return_request_cap_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RequestPressureInput {
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER,
        requested_wtxids_in_flight: 1,
        ..empty_request_input()
    };

    // Act
    let decision = policy.decide_request(input);

    // Assert
    assert_resource_event(
        decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn resource_policy_accepts_inputs_at_configured_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let queue_input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES,
        peer_write_queue_bytes: PHASE94_MAX_PEER_WRITE_QUEUE_BYTES,
        aggregate_write_queue_bytes: PHASE94_MAX_AGGREGATE_WRITE_QUEUE_BYTES,
        peer_queued_messages: PHASE94_MAX_PEER_QUEUED_MESSAGES,
        ..empty_queue_input()
    };
    let request_input = RequestPressureInput {
        inventory_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS,
        header_locator_hashes: PHASE94_MAX_HEADER_LOCATOR_HASHES,
        requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER / 2,
        requested_wtxids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER / 2,
        ..empty_request_input()
    };

    // Act
    let queue_decision = policy.decide_queue(queue_input);
    let request_decision = policy.decide_request(request_input);

    // Assert
    assert_eq!(queue_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(request_decision, ResourceGovernanceDecision::Accept);
}

#[test]
fn inactive_relay_like_effects_do_not_raise_resource_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let inactive_permission_effects = vec![
        InactivePermissionEffectLabel::Relay,
        InactivePermissionEffectLabel::ForceRelay,
        InactivePermissionEffectLabel::Mempool,
        InactivePermissionEffectLabel::BloomFilter,
        InactivePermissionEffectLabel::BlockFilters,
    ];
    let queue_input = QueuePressureInput {
        peer_read_queue_bytes: PHASE94_MAX_PEER_READ_QUEUE_BYTES + 1,
        inactive_permission_effects: inactive_permission_effects.clone(),
        ..empty_queue_input()
    };
    let request_input = RequestPressureInput {
        getdata_items: PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1,
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        inactive_permission_effects,
        ..empty_request_input()
    };

    // Act
    let queue_decision = policy.decide_queue(queue_input);
    let request_decision = policy.decide_request(request_input);

    // Assert
    assert_resource_event(
        queue_decision,
        ResourcePressureLabel::ReadQueuePressure,
        "read_queue_pressure",
    );
    assert_resource_event(
        request_decision,
        ResourcePressureLabel::RequestCapReached,
        "request_cap_reached",
    );
}

#[test]
fn slow_handshake_timeout_disconnects_with_stable_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Handshaking,
        connected_at_unix_seconds: 100,
        last_activity_unix_seconds: 100,
        now_unix_seconds: 100 + PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS + 1,
    };

    // Act
    let decision = policy.decide_timeout(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::SlowHandshake,
        "timeout_disconnect",
    );
}

#[test]
fn established_idle_timeout_disconnects_with_stable_label() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Established,
        connected_at_unix_seconds: 100,
        last_activity_unix_seconds: 200,
        now_unix_seconds: 200 + policy.idle_peer_timeout_seconds + 1,
    };

    // Act
    let decision = policy.decide_timeout(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::IdlePeer,
        "timeout_disconnect",
    );
}

#[test]
fn connection_churn_window_rejects_above_configured_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = ConnectionChurnInput {
        window_started_unix_seconds: 300,
        now_unix_seconds: 300,
        connection_attempts_in_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW + 1,
    };

    // Act
    let decision = policy.decide_churn(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::ConnectionChurnLimited,
        "churn_rejected",
    );
}

#[test]
fn repeated_failure_window_rejects_above_configured_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let input = RepeatedFailureInput {
        window_started_unix_seconds: 400,
        now_unix_seconds: 400 + PHASE94_REPEATED_FAILURE_WINDOW_SECONDS,
        failures_in_window: PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW + 1,
    };

    // Act
    let decision = policy.decide_repeated_failure(input);

    // Assert
    assert_lifecycle_event(
        decision,
        ResourceLifecycleLabel::RepeatedFailureLimited,
        "churn_rejected",
    );
}

#[test]
fn active_ban_and_discouraged_reconnect_are_suppressed() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let banned_input = ReconnectSuppressionInput {
        banned: true,
        discouraged: true,
    };
    let discouraged_input = ReconnectSuppressionInput {
        banned: false,
        discouraged: true,
    };

    // Act
    let banned_decision = policy.decide_reconnect(banned_input);
    let discouraged_decision = policy.decide_reconnect(discouraged_input);

    // Assert
    assert_lifecycle_event(
        banned_decision,
        ResourceLifecycleLabel::ReconnectSuppressedBanned,
        "reconnect_suppressed",
    );
    assert_lifecycle_event(
        discouraged_decision,
        ResourceLifecycleLabel::ReconnectSuppressedDiscouraged,
        "reconnect_suppressed",
    );
}

#[test]
fn lifecycle_policy_accepts_inputs_at_configured_caps() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let handshake_input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Handshaking,
        connected_at_unix_seconds: 500,
        last_activity_unix_seconds: 500,
        now_unix_seconds: 500 + policy.slow_handshake_timeout_seconds,
    };
    let idle_input = ResourceTimeoutInput {
        handshake_state: InboundHandshakeState::Established,
        connected_at_unix_seconds: 500,
        last_activity_unix_seconds: 600,
        now_unix_seconds: 600 + policy.idle_peer_timeout_seconds,
    };
    let churn_input = ConnectionChurnInput {
        window_started_unix_seconds: 700,
        now_unix_seconds: 700 + policy.connection_churn_window_seconds,
        connection_attempts_in_window: policy.max_connections_per_churn_window,
    };
    let expired_failure_input = RepeatedFailureInput {
        window_started_unix_seconds: 800,
        now_unix_seconds: 800 + policy.repeated_failure_window_seconds + 1,
        failures_in_window: policy.max_repeated_failures_per_window + 1,
    };
    let reconnect_input = ReconnectSuppressionInput {
        banned: false,
        discouraged: false,
    };

    // Act
    let handshake_decision = policy.decide_timeout(handshake_input);
    let idle_decision = policy.decide_timeout(idle_input);
    let churn_decision = policy.decide_churn(churn_input);
    let failure_decision = policy.decide_repeated_failure(expired_failure_input);
    let reconnect_decision = policy.decide_reconnect(reconnect_input);

    // Assert
    assert_eq!(handshake_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(idle_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(churn_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(failure_decision, ResourceGovernanceDecision::Accept);
    assert_eq!(reconnect_decision, ResourceGovernanceDecision::Accept);
}

#[test]
fn lifecycle_label_strings_cover_phase94_contract() {
    // Arrange
    let labels = [
        (ResourceLifecycleLabel::SlowHandshake, "slow_handshake"),
        (ResourceLifecycleLabel::IdlePeer, "idle_peer"),
        (
            ResourceLifecycleLabel::ConnectionChurnLimited,
            "connection_churn_limited",
        ),
        (
            ResourceLifecycleLabel::RepeatedFailureLimited,
            "repeated_failure_limited",
        ),
        (
            ResourceLifecycleLabel::ReconnectSuppressedBanned,
            "reconnect_suppressed_banned",
        ),
        (
            ResourceLifecycleLabel::ReconnectSuppressedDiscouraged,
            "reconnect_suppressed_discouraged",
        ),
    ];

    // Act
    let label_strings = labels.map(|(label, _)| label.as_str());

    // Assert
    assert_eq!(
        label_strings,
        labels.map(|(_, expected_label)| expected_label)
    );
}

#[test]
fn wrong_magic_is_rejected_before_payload_allocation() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let header = MessageHeader {
        magic: NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        command: MessageCommand::new("ping").expect("command"),
        payload_size: 0,
        checksum: [0_u8; 4],
    };
    let header_bytes = open_bitcoin_codec::encode_message_header(&header);

    // Act
    let decision = policy.evaluate_header(&header_bytes);

    // Assert
    let InboundEnvelopeDecision::Reject(event) = decision else {
        panic!("wrong network magic must reject");
    };
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "wrong_network_magic");
}

#[test]
fn malformed_header_is_rejected() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let mut malformed_header = [0_u8; INBOUND_MESSAGE_HEADER_LEN];
    malformed_header[..4].copy_from_slice(NetworkMagic::MAINNET.as_bytes());
    malformed_header[4] = b'p';
    malformed_header[6] = b'g';

    // Act
    let decision = policy.evaluate_header(&malformed_header);

    // Assert
    let InboundEnvelopeDecision::Reject(event) = decision else {
        panic!("malformed header must reject");
    };
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "malformed_header");
}

#[test]
fn oversized_payload_is_rejected_before_vec_allocation() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let header = MessageHeader {
        magic: NetworkMagic::MAINNET,
        command: MessageCommand::new("ping").expect("command"),
        payload_size: (PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES + 1) as u32,
        checksum: [0_u8; 4],
    };
    let header_bytes = open_bitcoin_codec::encode_message_header(&header);

    // Act
    let decision = policy.evaluate_header(&header_bytes);

    // Assert
    let InboundEnvelopeDecision::Reject(event) = decision else {
        panic!("oversized payload must reject");
    };
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "payload_oversized");
}

#[test]
fn unsupported_command_is_bounded_evidence() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let header = MessageHeader {
        magic: NetworkMagic::MAINNET,
        command: MessageCommand::from_wire_bytes(*b"mempool\0\0\0\0\0").expect("command"),
        payload_size: 0,
        checksum: [0_u8; 4],
    };
    let header_bytes = open_bitcoin_codec::encode_message_header(&header);

    // Act
    let decision = policy.evaluate_header(&header_bytes);

    // Assert
    let InboundEnvelopeDecision::Reject(event) = decision else {
        panic!("unsupported command must reject");
    };
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "unsupported_command");
}

#[test]
fn invalid_checksum_is_resource_label() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, mut payload) = valid_ping_header_and_payload();
    payload[0] ^= 0x01;

    // Act
    let event = policy
        .decode_payload(&header, &payload)
        .expect_err("invalid checksum must reject");

    // Assert
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "invalid_checksum");
}

#[test]
fn malformed_payload_is_resource_label() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, payload) = valid_ping_header_and_payload();
    let short_payload = &payload[..payload.len() - 1];

    // Act
    let event = policy
        .decode_payload(&header, short_payload)
        .expect_err("malformed payload must reject");

    // Assert
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "malformed_payload");
}

#[test]
fn trailing_payload_is_resource_label() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, mut payload) = valid_ping_header_and_payload();
    payload.push(0x00);

    // Act
    let event = policy
        .decode_payload(&header, &payload)
        .expect_err("trailing payload must reject");

    // Assert
    assert_eq!(event.outcome, "rejected");
    assert_eq!(event.next_action, "payload_rejected");
    assert_eq!(event.label, "trailing_payload");
}

#[test]
fn label_and_source_strings_cover_resource_contract() {
    // Arrange
    let labels = [
        (
            ResourceViolationLabel::WrongNetworkMagic,
            "wrong_network_magic",
        ),
        (ResourceViolationLabel::MalformedHeader, "malformed_header"),
        (
            ResourceViolationLabel::PayloadOversized,
            "payload_oversized",
        ),
        (ResourceViolationLabel::InvalidChecksum, "invalid_checksum"),
        (
            ResourceViolationLabel::UnsupportedCommand,
            "unsupported_command",
        ),
        (
            ResourceViolationLabel::MalformedPayload,
            "malformed_payload",
        ),
        (ResourceViolationLabel::TrailingPayload, "trailing_payload"),
    ];
    let sources = [
        (
            ResourceGovernanceSource::EnvelopeGate,
            "source_envelope_gate",
        ),
        (
            ResourceGovernanceSource::PayloadDecoder,
            "source_payload_decoder",
        ),
        (ResourceGovernanceSource::RuntimeRead, "source_runtime_read"),
    ];

    // Act
    let label_strings = labels.map(|(label, _)| label.as_str());
    let source_strings = sources.map(|(source, _)| source.as_str());

    // Assert
    assert_eq!(
        label_strings,
        labels.map(|(_, expected_label)| expected_label)
    );
    assert_eq!(
        source_strings,
        sources.map(|(_, expected_source)| expected_source)
    );
}

#[test]
fn valid_header_reads_bounded_payload_length() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, _payload) = valid_ping_header_and_payload();
    let header_bytes = open_bitcoin_codec::encode_message_header(&header);

    // Act
    let decision = policy.evaluate_header(&header_bytes);

    // Assert
    assert_eq!(
        decision,
        InboundEnvelopeDecision::ReadPayload { payload_len: 8 }
    );
}

#[test]
fn malformed_header_parse_failures_are_resource_labels() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let short_header = [0_u8; 3];
    let mut bad_padding_header = [0_u8; INBOUND_MESSAGE_HEADER_LEN];
    bad_padding_header[..4].copy_from_slice(NetworkMagic::MAINNET.as_bytes());
    bad_padding_header[4] = b'a';
    bad_padding_header[6] = b'b';

    // Act
    let short_decision = policy.evaluate_header(&short_header);
    let bad_padding_decision = policy.evaluate_header(&bad_padding_header);

    // Assert
    assert_rejection(short_decision, ResourceViolationLabel::MalformedHeader);
    assert_rejection(
        bad_padding_decision,
        ResourceViolationLabel::MalformedHeader,
    );
}

#[test]
fn decode_payload_accepts_valid_bounded_message() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, payload) = valid_ping_header_and_payload();

    // Act
    let parsed = policy.decode_payload(&header, &payload).expect("payload");

    // Assert
    assert_eq!(parsed.header, header);
    assert_eq!(parsed.message, WireNetworkMessage::Ping { nonce: 7 });
}

#[test]
fn decode_payload_pre_decoder_guards_are_resource_labels() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let (header, payload) = valid_ping_header_and_payload();
    let wrong_magic = MessageHeader {
        magic: NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        ..header.clone()
    };
    let unsupported = MessageHeader {
        command: MessageCommand::new("mempool").expect("command"),
        ..header.clone()
    };
    let oversized = MessageHeader {
        payload_size: (PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES + 1) as u32,
        ..header.clone()
    };
    let mut trailing_payload = payload.clone();
    trailing_payload.push(0);
    let short_payload = &payload[..payload.len() - 1];
    let mut bad_checksum_payload = payload.clone();
    bad_checksum_payload[0] ^= 0x01;

    // Act
    let wrong_magic_event = policy
        .decode_payload(&wrong_magic, &payload)
        .expect_err("wrong magic");
    let unsupported_event = policy
        .decode_payload(&unsupported, &payload)
        .expect_err("unsupported");
    let oversized_event = policy
        .decode_payload(&oversized, &payload)
        .expect_err("oversized");
    let trailing_event = policy
        .decode_payload(&header, &trailing_payload)
        .expect_err("trailing");
    let short_event = policy
        .decode_payload(&header, short_payload)
        .expect_err("short");
    let checksum_event = policy
        .decode_payload(&header, &bad_checksum_payload)
        .expect_err("checksum");

    // Assert
    assert_eq!(
        wrong_magic_event.label,
        ResourceViolationLabel::WrongNetworkMagic.as_str()
    );
    assert_eq!(
        unsupported_event.label,
        ResourceViolationLabel::UnsupportedCommand.as_str()
    );
    assert_eq!(
        oversized_event.label,
        ResourceViolationLabel::PayloadOversized.as_str()
    );
    assert_eq!(
        trailing_event.label,
        ResourceViolationLabel::TrailingPayload.as_str()
    );
    assert_eq!(
        short_event.label,
        ResourceViolationLabel::MalformedPayload.as_str()
    );
    assert_eq!(
        checksum_event.label,
        ResourceViolationLabel::InvalidChecksum.as_str()
    );
}

#[test]
fn decode_payload_decoder_errors_are_resource_labels() {
    // Arrange
    let policy = InboundEnvelopePolicy::new(NetworkMagic::MAINNET);
    let trailing_ping_payload = [7_u8; 9];
    let trailing_ping_header = MessageHeader {
        magic: NetworkMagic::MAINNET,
        command: MessageCommand::new("ping").expect("command"),
        payload_size: trailing_ping_payload.len() as u32,
        checksum: super::checksum(&trailing_ping_payload),
    };
    let malformed_version_payload = [];
    let malformed_version_header = MessageHeader {
        magic: NetworkMagic::MAINNET,
        command: MessageCommand::new("version").expect("command"),
        payload_size: malformed_version_payload.len() as u32,
        checksum: super::checksum(&malformed_version_payload),
    };
    let unknown_event = super::resource_event_for_decode_error(
        crate::NetworkError::UnknownCommand("mystery".to_string()),
    );
    let checksum_event =
        super::resource_event_for_decode_error(crate::NetworkError::InvalidChecksum);

    // Act
    let trailing_event = policy
        .decode_payload(&trailing_ping_header, &trailing_ping_payload)
        .expect_err("trailing decoder");
    let malformed_event = policy
        .decode_payload(&malformed_version_header, &malformed_version_payload)
        .expect_err("malformed decoder");

    // Assert
    assert_eq!(
        trailing_event.label,
        ResourceViolationLabel::TrailingPayload.as_str()
    );
    assert_eq!(
        malformed_event.label,
        ResourceViolationLabel::MalformedPayload.as_str()
    );
    assert_eq!(
        unknown_event.label,
        ResourceViolationLabel::UnsupportedCommand.as_str()
    );
    assert_eq!(
        checksum_event.label,
        ResourceViolationLabel::InvalidChecksum.as_str()
    );
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_invalid_messages.py

use open_bitcoin_primitives::{MessageCommand, MessageHeader, NetworkMagic};

use super::{
    INBOUND_MESSAGE_HEADER_LEN, InboundEnvelopeDecision, InboundEnvelopePolicy,
    PHASE94_MAX_INBOUND_RUNTIME_PAYLOAD_BYTES, ResourceGovernanceSource, ResourceViolationLabel,
    WireNetworkMessage,
};

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
    assert_eq!(
        event.label,
        ResourceViolationLabel::WrongNetworkMagic.as_str()
    );
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
    assert_eq!(
        event.label,
        ResourceViolationLabel::PayloadOversized.as_str()
    );
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
    assert_eq!(
        event.label,
        ResourceViolationLabel::UnsupportedCommand.as_str()
    );
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

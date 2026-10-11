// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/protocol.cpp
// - packages/bitcoin-knots/test/functional/test_framework/messages.py

use crate::{
    CFCheckpt, CFHeaders, CFRequestKind, CFilter, GetCFCheckpt, GetCFHeaders, GetCFilters,
    MAX_CF_PAYLOAD_BYTES, NetworkError, ParsedNetworkMessage, ServiceFlags, WireNetworkMessage,
};
use open_bitcoin_codec::CodecError;
use open_bitcoin_primitives::{BlockHash, Hash32, MessageCommand, NetworkMagic};

#[test]
fn phase160_wire_six_pinned_commands_roundtrip() {
    // Arrange
    let raw_hash: Vec<u8> = (0..32).collect();
    let mut ranged = vec![2, 4, 3, 2, 1];
    ranged.extend_from_slice(&raw_hash);
    let mut checkpoint = vec![0];
    checkpoint.extend_from_slice(&raw_hash);
    let mut filter = checkpoint.clone();
    filter.extend_from_slice(&[3, 1, 2, 3]);
    let mut headers = checkpoint.clone();
    let predecessor: Vec<u8> = (64..96).collect();
    headers.extend_from_slice(&predecessor);
    headers.push(1);
    headers.extend(raw_hash.iter().rev());
    let mut checkpoints = checkpoint.clone();
    checkpoints.push(0);
    for (command, payload) in [
        ("getcfilters", ranged.clone()),
        ("getcfheaders", ranged),
        ("getcfcheckpt", checkpoint),
        ("cfilter", filter),
        ("cfheaders", headers),
        ("cfcheckpt", checkpoints),
    ] {
        // Act
        let decoded = WireNetworkMessage::decode_payload(
            &MessageCommand::new(command).expect("command"),
            &payload,
        )
        .expect("pinned payload");
        // Assert
        assert_eq!(decoded.command_name(), command);
        assert_eq!(decoded.encode_payload().expect("encode"), payload);
        if let WireNetworkMessage::CFHeaders(response) = decoded {
            assert_eq!(response.stop_hash.as_bytes().as_slice(), raw_hash);
            assert_eq!(
                response.previous_filter_header.as_bytes().as_slice(),
                predecessor
            );
            assert_eq!(
                response.filter_hashes[0].as_bytes().as_slice(),
                raw_hash.iter().rev().copied().collect::<Vec<_>>()
            );
        }
    }
}

fn roundtrip(message: WireNetworkMessage) {
    let encoded = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let decoded = ParsedNetworkMessage::decode_wire(&encoded).expect("decode");
    assert_eq!(decoded.message, message);
}

#[test]
fn phase160_wire_pinned_genesis_filter_payload() {
    // Arrange: pinned src/test/data/blockfilters.json testnet genesis row, raw wire order.
    let raw_hash = [
        0x43, 0x49, 0x7f, 0xd7, 0xf8, 0x26, 0x95, 0x71, 0x08, 0xf4, 0xa3, 0x0f, 0xd9, 0xce, 0xc3,
        0xae, 0xba, 0x79, 0x97, 0x20, 0x84, 0xe9, 0x0e, 0xad, 0x01, 0xea, 0x33, 0x09, 0, 0, 0, 0,
    ];
    let message = WireNetworkMessage::CFilter(CFilter {
        filter_type: 0,
        block_hash: BlockHash::from_byte_array(raw_hash),
        filter: vec![0x01, 0x9d, 0xfc, 0xa8],
    });
    let mut expected = vec![0];
    expected.extend_from_slice(&raw_hash);
    expected.extend_from_slice(&[4, 0x01, 0x9d, 0xfc, 0xa8]);
    // Act
    let payload = message.encode_payload().expect("genesis cfilter");
    // Assert
    assert_eq!(payload, expected);
    roundtrip(message);
}

#[test]
fn phase160_wire_unknown_command_stays_unknown() {
    // Arrange
    let command = MessageCommand::new("unknown").expect("command");
    // Act
    let result = WireNetworkMessage::decode_payload(&command, &[0; 37]);
    // Assert
    assert!(matches!(result, Err(crate::NetworkError::UnknownCommand(name)) if name == "unknown"));
}

#[test]
fn phase160_wire_request_lengths_and_raw_type() {
    // Arrange
    let stop_hash = BlockHash::from_byte_array(core::array::from_fn(|i| i as u8));
    for filter_type in [0, 2, 255] {
        let messages = [
            (
                WireNetworkMessage::GetCFilters(GetCFilters {
                    filter_type,
                    start_height: 0x01020304,
                    stop_hash,
                }),
                37,
            ),
            (
                WireNetworkMessage::GetCFHeaders(GetCFHeaders {
                    filter_type,
                    start_height: 0,
                    stop_hash,
                }),
                37,
            ),
            (
                WireNetworkMessage::GetCFCheckpt(GetCFCheckpt {
                    filter_type,
                    stop_hash,
                }),
                33,
            ),
        ];
        for (message, len) in messages {
            // Act
            let payload = message.encode_payload().expect("payload");
            // Assert
            assert_eq!(payload.len(), len);
            assert_eq!(payload[0], filter_type);
            assert_eq!(&payload[len - 32..], stop_hash.as_bytes());
            roundtrip(message);
        }
    }
}

#[test]
fn phase160_wire_compact_size_transitions() {
    // Arrange
    let stop_hash = BlockHash::from_byte_array([17; 32]);
    for (count, prefix) in [
        (0, vec![0]),
        (252, vec![252]),
        (253, vec![253, 253, 0]),
        (65535, vec![253, 255, 255]),
        (65536, vec![254, 0, 0, 1, 0]),
    ] {
        let hash = Hash32::from_byte_array(core::array::from_fn(|i| (31 - i) as u8));
        for (message, offset) in [
            (
                WireNetworkMessage::CFilter(CFilter {
                    filter_type: 0,
                    block_hash: stop_hash,
                    filter: vec![0xab; count],
                }),
                33,
            ),
            (
                WireNetworkMessage::CFHeaders(CFHeaders {
                    filter_type: 0,
                    stop_hash,
                    previous_filter_header: Hash32::from_byte_array([0; 32]),
                    filter_hashes: vec![hash; count],
                }),
                65,
            ),
            (
                WireNetworkMessage::CFCheckpt(CFCheckpt {
                    filter_type: 0,
                    stop_hash,
                    filter_headers: vec![hash; count],
                }),
                33,
            ),
        ] {
            // Act
            let payload = message.encode_payload().expect("payload");
            // Assert
            assert_eq!(&payload[offset..offset + prefix.len()], prefix);
            if count > 0
                && matches!(
                    message,
                    WireNetworkMessage::CFHeaders(_) | WireNetworkMessage::CFCheckpt(_)
                )
            {
                assert_eq!(
                    &payload[offset + prefix.len()..offset + prefix.len() + 32],
                    hash.as_bytes()
                );
            }
            roundtrip(message);
        }
    }
}

#[test]
fn phase160_wire_response_count_rejected_before_allocation() {
    // Arrange
    for (name, fixed) in [("cfilter", 33), ("cfheaders", 65), ("cfcheckpt", 33)] {
        let command = MessageCommand::new(name).expect("command");
        for suffix in [
            vec![1],
            vec![253],
            vec![254, 0, 0],
            vec![255, 0, 0, 0],
            vec![253, 1, 0],
            vec![254, 255, 255, 255, 127],
            vec![255, 255, 255, 255, 255, 255, 255, 255, 255],
        ] {
            let mut payload = vec![0; fixed];
            payload.extend_from_slice(&suffix);
            // Act
            let decoded = WireNetworkMessage::decode_payload(&command, &payload);
            // Assert
            assert!(decoded.is_err(), "{name}: {suffix:?}");
        }
    }
}

#[test]
fn phase160_wire_response_fields_cannot_be_truncated() {
    // Arrange
    for (name, fixed) in [("cfilter", 33), ("cfheaders", 65), ("cfcheckpt", 33)] {
        let command = MessageCommand::new(name).expect("command");
        for len in 0..=fixed {
            // Act
            let decoded = WireNetworkMessage::decode_payload(&command, &vec![0; len]);
            // Assert
            assert!(decoded.is_err(), "{name}: {len}");
        }
    }
}

#[test]
fn phase160_wire_payload_envelope_exact_and_one_over() {
    // Arrange
    let mut response = CFilter {
        filter_type: 0,
        block_hash: BlockHash::from_byte_array([0; 32]),
        filter: vec![0; MAX_CF_PAYLOAD_BYTES - 38],
    };
    // Act
    let payload = WireNetworkMessage::CFilter(response.clone())
        .encode_payload()
        .expect("exact envelope");
    // Assert
    assert_eq!(payload.len(), MAX_CF_PAYLOAD_BYTES);
    roundtrip(WireNetworkMessage::CFilter(response.clone()));
    response.filter.push(0);
    assert!(
        WireNetworkMessage::CFilter(response)
            .encode_payload()
            .is_err()
    );
    let oversized = vec![0; MAX_CF_PAYLOAD_BYTES + 1];
    for name in ["cfilter", "cfheaders", "cfcheckpt"] {
        assert!(
            WireNetworkMessage::decode_payload(
                &MessageCommand::new(name).expect("command"),
                &oversized
            )
            .is_err()
        );
    }
}

#[test]
fn phase160_wire_hash_vector_envelope_exact_and_one_over() {
    // Arrange
    let hash = Hash32::from_byte_array([7; 32]);
    let block_hash = BlockHash::from_byte_array([9; 32]);
    for fixed in [33, 65] {
        let max_count = (MAX_CF_PAYLOAD_BYTES - fixed - 5) / 32;
        for count in [max_count, max_count + 1] {
            let message = if fixed == 33 {
                WireNetworkMessage::CFCheckpt(CFCheckpt {
                    filter_type: 0,
                    stop_hash: block_hash,
                    filter_headers: vec![hash; count],
                })
            } else {
                WireNetworkMessage::CFHeaders(CFHeaders {
                    filter_type: 0,
                    stop_hash: block_hash,
                    previous_filter_header: hash,
                    filter_hashes: vec![hash; count],
                })
            };
            // Act
            let payload = message.encode_payload();
            // Assert
            assert_eq!(payload.is_ok(), count == max_count);
            if count == max_count {
                roundtrip(message);
            }
        }
    }
}

#[test]
fn phase160_wire_compact_filters_service_bit() {
    // Arrange
    let services = ServiceFlags::NETWORK_LIMITED | ServiceFlags::COMPACT_FILTERS;
    // Act
    let bits = services.bits();
    // Assert
    assert_eq!(ServiceFlags::COMPACT_FILTERS.bits(), 1 << 6);
    assert_eq!(bits, (1 << 10) | (1 << 6));
    assert!(
        !crate::LocalPeerConfig::default()
            .services
            .contains(ServiceFlags::COMPACT_FILTERS)
    );
}

#[test]
fn phase160_coverage_short_requests_retain_command_identity() {
    // Arrange
    for (name, kind, required) in [
        ("getcfilters", CFRequestKind::Filters, 37),
        ("getcfheaders", CFRequestKind::Headers, 37),
        ("getcfcheckpt", CFRequestKind::Checkpoints, 33),
    ] {
        let command = MessageCommand::new(name).expect("known CF request command");
        for len in 0..required {
            let payload = vec![2; len];

            // Act
            let decoded = WireNetworkMessage::decode_payload(&command, &payload)
                .expect("short complete request yields a receive-only marker");

            // Assert
            assert_eq!(decoded, WireNetworkMessage::MalformedCFRequest(kind));
            assert_eq!(decoded.command_name(), name);
            assert_eq!(decoded.command().expect("diagnostic command"), command);
        }
    }
}

#[test]
fn phase160_coverage_short_request_markers_refuse_outgoing_encoding() {
    // Arrange
    for (name, required) in [
        ("getcfilters", 37),
        ("getcfheaders", 37),
        ("getcfcheckpt", 33),
    ] {
        let command = MessageCommand::new(name).expect("known CF request command");
        let decoded = WireNetworkMessage::decode_payload(&command, &vec![0; required - 1])
            .expect("short complete request yields a receive-only marker");
        let expected = NetworkError::Codec(CodecError::UnexpectedEof {
            needed: required,
            remaining: 0,
        });

        // Act
        let payload = decoded.encode_payload();
        let frame = decoded.encode_wire(NetworkMagic::MAINNET);

        // Assert
        assert_eq!(payload, Err(expected.clone()), "{name}");
        assert_eq!(frame, Err(expected), "{name}");
    }
}

#[test]
#[cfg(target_pointer_width = "64")]
fn phase160_coverage_nine_byte_response_counts_refused_before_allocation() {
    // Arrange: only the fixed fields and a nine-byte count are materialized.
    let count = u64::from(u32::MAX) + 1;
    for (name, fixed, element_len) in [
        ("cfilter", 33, 1),
        ("cfheaders", 65, 32),
        ("cfcheckpt", 33, 32),
    ] {
        let command = MessageCommand::new(name).expect("known CF response command");
        let mut payload = vec![0; fixed];
        payload.push(255);
        payload.extend_from_slice(&count.to_le_bytes());
        let expected_preflight = NetworkError::Codec(CodecError::LengthOutOfRange {
            field: "compact filter payload",
            value: fixed as u64 + 9 + count * element_len as u64,
        });
        let expected_decode = NetworkError::Codec(CodecError::CompactSizeTooLarge(count));

        // Act
        let preflight = super::response_len(fixed, count as usize, element_len);
        let decoded = WireNetworkMessage::decode_payload(&command, &payload);

        // Assert
        // Outgoing sizing bounds complete payload bytes; incoming count parsing has its own ceiling.
        assert_eq!(preflight, Err(expected_preflight), "{name}");
        assert_eq!(decoded, Err(expected_decode), "{name}");
    }
}

#[test]
fn phase160_coverage_response_sizing_overflow_returns_typed_refusal() {
    // Arrange: these counts cannot be backed by allocated response vectors.
    for (fixed, count, element_len) in [
        (33, usize::MAX, 32),
        (65, usize::MAX, 1),
        (33, usize::MAX - 33, 1),
    ] {
        // Act
        let result = super::response_len(fixed, count, element_len);

        // Assert
        assert_eq!(
            result,
            Err(NetworkError::Codec(CodecError::LengthOutOfRange {
                field: "compact filter payload",
                value: count as u64,
            }))
        );
    }
}

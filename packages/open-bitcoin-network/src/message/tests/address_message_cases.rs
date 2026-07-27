// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use super::*;

#[test]
fn getaddr_message_uses_empty_payload() {
    // Arrange
    let message = WireNetworkMessage::GetAddr;
    let command = MessageCommand::new("getaddr").expect("command");

    // Act
    let encoded = message.encode_payload().expect("payload");
    let decoded = WireNetworkMessage::decode_payload(&command, &encoded).expect("decode");

    // Assert
    assert_eq!(message.command_name(), "getaddr");
    assert!(encoded.is_empty());
    assert_eq!(decoded, message);
}

#[test]
fn addr_message_round_trips_legacy_timestamped_addresses() {
    // Arrange
    let address_list = AddressList {
        addresses: vec![
            AddressAnnouncement {
                time_unix_seconds: 1_700_000_001,
                address: sample_peer_address(8333),
            },
            AddressAnnouncement {
                time_unix_seconds: 1_700_000_002,
                address: sample_peer_address(18333),
            },
        ],
    };
    let message = WireNetworkMessage::Addr(address_list.clone());
    let command = MessageCommand::new("addr").expect("command");

    // Act
    let encoded = message.encode_payload().expect("payload");
    let decoded = WireNetworkMessage::decode_payload(&command, &encoded).expect("decode");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");

    // Assert
    assert_eq!(message.command_name(), "addr");
    assert_eq!(encoded[0], 2);
    assert_eq!(&encoded[1..5], &1_700_000_001_u32.to_le_bytes());
    assert_eq!(
        &encoded[5..31],
        &open_bitcoin_codec::encode_network_address(&address_list.addresses[0].address)
    );
    assert_eq!(decoded, message);
    assert_eq!(parsed.message, message);
}

#[test]
fn addr_message_rejects_batches_above_phase92_limit() {
    // Arrange
    let over_limit = PHASE92_ADDR_BATCH_LIMIT + 1;
    let addresses = (0..over_limit)
        .map(|index| AddressAnnouncement {
            time_unix_seconds: 1_700_000_000 + index as u32,
            address: sample_peer_address(8333),
        })
        .collect();
    let message = WireNetworkMessage::Addr(AddressList { addresses });
    let command = MessageCommand::new("addr").expect("command");
    let compact_count_only = [over_limit as u8];

    // Act
    let encode_error = message.encode_payload().expect_err("over-limit encode");
    let decode_error = WireNetworkMessage::decode_payload(&command, &compact_count_only)
        .expect_err("over-limit decode");

    // Assert
    assert_eq!(PHASE92_ADDR_BATCH_LIMIT, 64);
    assert_eq!(
        encode_error.to_string(),
        "addr count length out of range: 65"
    );
    assert_eq!(
        decode_error.to_string(),
        "addr count length out of range: 65"
    );
}

#[test]
fn addrv2_and_sendaddrv2_remain_unknown_commands() {
    for command_name in ["addrv2", "sendaddrv2"] {
        // Arrange
        let command = MessageCommand::new(command_name).expect("command");

        // Act
        let error = WireNetworkMessage::decode_payload(&command, &[]).expect_err("unknown command");

        // Assert
        assert_eq!(
            error.to_string(),
            format!("unknown network command: {command_name}")
        );
    }
}

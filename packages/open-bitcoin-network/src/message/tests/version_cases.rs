// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use super::*;

#[test]
fn wire_message_round_trips_version_and_inventory_payloads() {
    let version = WireNetworkMessage::Version(VersionMessage {
        timestamp: 1_700_000_000,
        nonce: 42,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        receiver: zero_address(),
        sender: zero_address(),
        user_agent: "/open-bitcoin:test/".to_string(),
        start_height: 7,
        relay: true,
        ..VersionMessage::default()
    });

    let encoded = version
        .encode_wire(NetworkMagic::MAINNET)
        .expect("version message should encode");
    let decoded = ParsedNetworkMessage::decode_wire(&encoded).expect("decode");
    assert_eq!(decoded.message, version);
    assert_eq!(decoded.header.magic, NetworkMagic::MAINNET);

    let inventory = WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([9_u8; 32]),
    }]));
    let encoded_inventory = inventory
        .encode_wire(NetworkMagic::MAINNET)
        .expect("inventory should encode");
    let decoded_inventory =
        ParsedNetworkMessage::decode_wire(&encoded_inventory).expect("inventory decode");
    assert_eq!(decoded_inventory.message, inventory);
    assert_eq!(
        WireNetworkMessage::decode_payload(&MessageCommand::new("mystery").expect("command"), &[],)
            .expect_err("unknown command")
            .to_string(),
        "unknown network command: mystery",
    );
}

#[test]
fn local_peer_config_builds_expected_version_message() {
    // Arrange
    let address = sample_peer_address(8333);
    let config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: address.clone(),
        nonce: 5,
        relay: false,
        user_agent: "/open-bitcoin:test/".to_string(),
    };

    // Act
    let version = config.version_message(9, 3);

    // Assert
    assert_eq!(version.timestamp, 9);
    assert_eq!(version.start_height, 3);
    assert!(!version.relay);
    assert_eq!(version.nonce, 5);
    assert_eq!(version.receiver, address);
    assert_eq!(version.sender, address);
}

#[test]
fn version_sender_policy_uses_zero_sender_without_advertisement_candidate() {
    // Arrange
    let address = sample_peer_address(8333);
    let config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: address.clone(),
        nonce: 6,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    };
    let maybe_sender = None;

    // Act
    let version = config.version_message_with_sender_policy(10, 4, maybe_sender);

    // Assert
    assert_eq!(version.receiver, address);
    assert_eq!(version.sender, zero_address());
    assert_eq!(version.timestamp, 10);
    assert_eq!(version.start_height, 4);
}

#[test]
fn version_sender_policy_uses_advertisement_candidate_when_present() {
    // Arrange
    let address = sample_peer_address(8333);
    let sender = sample_peer_address(18333);
    let config = LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: address.clone(),
        nonce: 7,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    };
    let maybe_sender = Some(sender.clone());

    // Act
    let version = config.version_message_with_sender_policy(11, 5, maybe_sender);

    // Assert
    assert_eq!(version.receiver, address);
    assert_eq!(version.sender, sender);
    assert_eq!(version.nonce, 7);
}

#[test]
fn service_flags_defaults_and_remaining_payload_variants_are_covered() {
    let mut flags = ServiceFlags::NONE;
    flags |= ServiceFlags::NETWORK;
    flags |= ServiceFlags::WITNESS;
    assert!(flags.contains(ServiceFlags::NETWORK));
    assert!(flags.contains(ServiceFlags::WITNESS));
    assert!(!flags.contains(ServiceFlags::REPLACE_BY_FEE));

    let default_config = LocalPeerConfig::default();
    assert_eq!(default_config.magic, NetworkMagic::MAINNET);
    assert!(default_config.services.contains(ServiceFlags::NETWORK));
    assert!(default_config.services.contains(ServiceFlags::WITNESS));

    let messages = vec![
        WireNetworkMessage::Verack,
        WireNetworkMessage::WtxidRelay,
        WireNetworkMessage::SendHeaders,
        WireNetworkMessage::Ping { nonce: 9 },
        WireNetworkMessage::Pong { nonce: 8 },
        WireNetworkMessage::GetHeaders {
            locator: open_bitcoin_primitives::BlockLocator {
                block_hashes: vec![Hash32::from_byte_array([2_u8; 32])],
            },
            stop_hash: BlockHash::from_byte_array([3_u8; 32]),
        },
        WireNetworkMessage::Headers(super::super::HeadersMessage {
            headers: vec![sample_block().header.clone()],
        }),
        WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from_byte_array([7_u8; 32]),
        }])),
        WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: Hash32::from_byte_array([8_u8; 32]),
        }])),
        WireNetworkMessage::Tx(sample_transaction()),
        WireNetworkMessage::Block(sample_block()),
    ];

    for message in messages {
        let payload = message.encode_payload().expect("payload");
        let decoded =
            WireNetworkMessage::decode_payload(&message.command().expect("command"), &payload)
                .expect("decode payload");
        assert_eq!(decoded, message);
        let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
        let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");
        assert_eq!(parsed.message, message);
    }

    let tx_payload = WireNetworkMessage::Tx(sample_transaction())
        .encode_payload()
        .expect("tx payload");
    assert!(!tx_payload.is_empty());
}

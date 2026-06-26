// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use open_bitcoin_primitives::{
    Amount, Block, BlockHash, BlockHeader, Hash32, InventoryType, MerkleRoot, MessageCommand,
    NetworkAddress, NetworkMagic, OutPoint, ScriptBuf, ScriptWitness, Transaction,
    TransactionInput, TransactionOutput, Txid,
};

use crate::address::{AddressAnnouncement, AddressList, PHASE92_ADDR_BATCH_LIMIT};

use super::{
    InventoryList, InventoryVector, LocalPeerConfig, ParsedNetworkMessage, ServiceFlags,
    VersionMessage, WireNetworkMessage, zero_address,
};

fn sample_transaction() -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([4_u8; 32]),
                vout: 1,
            },
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x01]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(42).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    }
}

fn sample_block() -> Block {
    Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0_u8; 32]),
            merkle_root: MerkleRoot::from_byte_array([1_u8; 32]),
            time: 2,
            bits: 0x207f_ffff,
            nonce: 3,
        },
        transactions: vec![sample_transaction()],
    }
}

fn sample_peer_address(port: u16) -> NetworkAddress {
    NetworkAddress {
        services: (ServiceFlags::NETWORK | ServiceFlags::WITNESS).bits(),
        address_bytes: [
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xff, 0xff, 0xcb, 0x00,
            0x71, 0x0a,
        ],
        port,
    }
}

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
fn getaddr_rejects_non_empty_payload() {
    // Arrange
    let command = MessageCommand::new("getaddr").expect("command");

    // Act
    let error =
        WireNetworkMessage::decode_payload(&command, &[0x00]).expect_err("payload must fail");

    // Assert
    assert_eq!(error.to_string(), "trailing data: 1 bytes");
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
        WireNetworkMessage::Headers(super::HeadersMessage {
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

#[test]
fn wire_decode_and_cursor_error_paths_are_exercised() {
    assert_eq!(
        ParsedNetworkMessage::decode_wire(&[0_u8; 3])
            .expect_err("short header must fail")
            .to_string(),
        "unexpected EOF: needed 24 bytes, remaining 3",
    );

    let good = WireNetworkMessage::Ping { nonce: 5 }
        .encode_wire(NetworkMagic::MAINNET)
        .expect("wire");
    let mut bad_size = good.clone();
    bad_size[16..20].copy_from_slice(&(9_u32).to_le_bytes());
    assert_eq!(
        ParsedNetworkMessage::decode_wire(&bad_size)
            .expect_err("payload size mismatch must fail")
            .to_string(),
        "payload size length out of range: 8",
    );

    let mut bad_checksum = good;
    *bad_checksum.last_mut().expect("payload byte") ^= 0x01;
    assert_eq!(
        ParsedNetworkMessage::decode_wire(&bad_checksum)
            .expect_err("checksum mismatch must fail")
            .to_string(),
        "invalid network payload checksum",
    );

    let mut relay_optional =
        super::encode_version_payload(&VersionMessage::default()).expect("payload");
    relay_optional.pop();
    let decoded = super::decode_version_payload(&relay_optional).expect("optional relay");
    assert!(!decoded.relay);

    let invalid_user_agent = {
        let mut payload = Vec::new();
        payload.extend_from_slice(&super::PROTOCOL_VERSION.to_le_bytes());
        payload.extend_from_slice(&0_u64.to_le_bytes());
        payload.extend_from_slice(&0_i64.to_le_bytes());
        payload.extend_from_slice(&open_bitcoin_codec::encode_network_address(&zero_address()));
        payload.extend_from_slice(&open_bitcoin_codec::encode_network_address(&zero_address()));
        payload.extend_from_slice(&0_u64.to_le_bytes());
        open_bitcoin_codec::write_compact_size(&mut payload, 1).expect("compact size");
        payload.push(0xff);
        payload.extend_from_slice(&0_i32.to_le_bytes());
        payload
    };
    assert_eq!(
        super::decode_version_payload(&invalid_user_agent)
            .expect_err("invalid user agent encoding must fail")
            .to_string(),
        "version message user agent is not valid UTF-8",
    );

    let mut cursor = super::Cursor::new(&[0x01, 0x02]);
    assert_eq!(
        cursor
            .read_slice(3)
            .expect_err("read past end must fail")
            .to_string(),
        "unexpected EOF: needed 3 bytes, remaining 2",
    );
    let cursor = super::Cursor::new(&[0x01]);
    assert_eq!(
        cursor
            .finish()
            .expect_err("trailing data must fail")
            .to_string(),
        "trailing data: 1 bytes",
    );
    let mut cursor = super::Cursor::new(&[0xfd, 0x01, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::Cursor::new(&[0xfe, 0x01, 0x00, 0x00, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::Cursor::new(&[0xfe, 0x00, 0x00, 0x01, 0x00]);
    assert_eq!(cursor.read_compact_size().expect("canonical value"), 65_536);
    let mut cursor = super::Cursor::new(&[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::Cursor::new(&[0xff, 0, 0, 0, 0, 1, 0, 0, 0]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("oversized compact size")
            .to_string(),
        "compact size too large: 4294967296",
    );
}

#[test]
fn decode_helpers_cover_headers_inventory_and_nonce_failures() {
    let mut headers_payload = Vec::new();
    open_bitcoin_codec::write_compact_size(&mut headers_payload, 1).expect("count");
    headers_payload.extend_from_slice(&open_bitcoin_codec::encode_block_header(
        &sample_block().header,
    ));
    open_bitcoin_codec::write_compact_size(&mut headers_payload, 1).expect("txn count");
    assert_eq!(
        super::decode_headers_payload(&headers_payload)
            .expect_err("headers payload with txns must fail")
            .to_string(),
        "headers message included non-zero transaction count: 1",
    );

    let mut too_many_headers = Vec::new();
    open_bitcoin_codec::write_compact_size(
        &mut too_many_headers,
        (super::MAX_HEADERS_RESULTS + 1) as u64,
    )
    .expect("count");
    assert_eq!(
        super::decode_headers_payload(&too_many_headers)
            .expect_err("header count overflow")
            .to_string(),
        "headers count length out of range: 2001",
    );

    let mut too_many_inventory = Vec::new();
    open_bitcoin_codec::write_compact_size(
        &mut too_many_inventory,
        (super::MAX_INV_SIZE + 1) as u64,
    )
    .expect("count");
    assert_eq!(
        super::decode_inventory_payload(&too_many_inventory)
            .expect_err("inventory count overflow")
            .to_string(),
        "inventory count length out of range: 50001",
    );

    let expected_inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: Hash32::from_byte_array([5_u8; 32]),
    }]);
    let mut inventory_payload = Vec::new();
    open_bitcoin_codec::write_compact_size(&mut inventory_payload, 1).expect("compact size");
    inventory_payload.extend_from_slice(&open_bitcoin_codec::encode_inventory_vector(
        &expected_inventory.inventory[0],
    ));
    assert_eq!(
        super::decode_inventory_payload(&inventory_payload).expect("inventory payload"),
        expected_inventory,
    );
    let encoded_tx = WireNetworkMessage::Tx(sample_transaction())
        .encode_payload()
        .expect("tx payload");
    assert!(matches!(
        WireNetworkMessage::decode_payload(
            &MessageCommand::new("tx").expect("command"),
            &encoded_tx,
        )
        .expect("decode tx"),
        WireNetworkMessage::Tx(_)
    ));

    assert_eq!(
        super::decode_nonce_payload(&[1, 0, 0, 0, 0, 0, 0, 0, 1])
            .expect_err("trailing nonce payload must fail")
            .to_string(),
        "trailing data: 1 bytes",
    );
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use super::*;

#[test]
fn phase112_wire_sendcmpct_round_trips_payload_and_wire() {
    // Arrange
    let send_compact = SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    };
    let message = WireNetworkMessage::SendCompact(send_compact);

    // Act
    let encoded = message.encode_payload().expect("sendcmpct payload");
    let decoded =
        WireNetworkMessage::decode_payload(&message.command().expect("command"), &encoded)
            .expect("sendcmpct decode");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");

    // Assert
    assert_eq!(message.command_name(), "sendcmpct");
    assert_eq!(encoded.len(), 9);
    assert_eq!(decoded, message);
    assert_eq!(parsed.message, message);
}

#[test]
fn phase112_sendcmpct_unsupported_versions_are_messages() {
    for version in [1_u64, 3] {
        // Arrange
        let command = MessageCommand::new("sendcmpct").expect("command");
        let send_compact = SendCompactMessage {
            announce: true,
            version,
        };
        let payload = encode_send_compact_payload(&send_compact);

        // Act
        let decoded = WireNetworkMessage::decode_payload(&command, &payload)
            .expect("unsupported versions decode as messages");

        // Assert
        assert_eq!(decoded, WireNetworkMessage::SendCompact(send_compact));
    }
}

#[test]
fn phase112_other_unknown_bip152_adjacent_commands_stay_unknown() {
    for command_name in ["sendcmpct2", "cmpctblock2"] {
        // Arrange
        let command = MessageCommand::new(command_name).expect("command");

        // Act
        let error = WireNetworkMessage::decode_payload(&command, &[])
            .expect_err("adjacent BIP152 commands remain unknown");

        // Assert
        assert_eq!(
            error.to_string(),
            format!("unknown network command: {command_name}"),
        );
    }
}

#[test]
fn phase112_wire_cmpctblock_round_trips_payload_and_wire() {
    // Arrange
    let payload = sample_compact_block_payload();
    let message = WireNetworkMessage::CompactBlock(payload.clone());

    // Act
    let encoded = message.encode_payload().expect("cmpctblock payload");
    let decoded =
        WireNetworkMessage::decode_payload(&message.command().expect("command"), &encoded)
            .expect("cmpctblock decode");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");

    // Assert
    assert_eq!(message.command_name(), "cmpctblock");
    assert_eq!(decoded, WireNetworkMessage::CompactBlock(payload));
    assert_eq!(parsed.message, message);
}

#[test]
fn phase112_wire_cmpctblock_rejects_malformed_payload_before_message() {
    // Arrange
    let command = MessageCommand::new("cmpctblock").expect("command");
    let mut malformed_payload = open_bitcoin_codec::encode_block_header(&sample_block().header);
    malformed_payload.extend_from_slice(&1_u64.to_le_bytes());
    open_bitcoin_codec::write_compact_size(&mut malformed_payload, 0).expect("short id count");
    open_bitcoin_codec::write_compact_size(&mut malformed_payload, 0).expect("prefilled count");

    // Act
    let error = WireNetworkMessage::decode_payload(&command, &malformed_payload)
        .expect_err("empty compact block payload should fail before message creation");

    // Assert
    assert_eq!(
        error.to_string(),
        "compact block has no short ids or prefilled transactions",
    );
}

#[test]
fn phase112_compact_block_inventory_getdata_still_uses_inventory_message() {
    // Arrange
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: Hash32::from_byte_array([11_u8; 32]),
    }]);
    let message = WireNetworkMessage::GetData(inventory.clone());

    // Act
    let encoded = message.encode_payload().expect("getdata payload");
    let decoded = WireNetworkMessage::decode_payload(
        &MessageCommand::new("getdata").expect("command"),
        &encoded,
    )
    .expect("getdata should decode");

    // Assert
    assert_eq!(message.command_name(), "getdata");
    assert_eq!(decoded, WireNetworkMessage::GetData(inventory));
}

#[test]
fn phase112_message_decode_surfaces_malformed_bip152_errors() {
    let cases = [
        (
            "cmpctblock",
            malformed_empty_cmpctblock_payload(),
            "compact block has no short ids or prefilled transactions",
        ),
        (
            "getblocktxn",
            malformed_getblocktxn_overflow_payload(),
            "differential index overflow",
        ),
        (
            "blocktxn",
            malformed_blocktxn_superfluous_witness_payload(),
            "superfluous witness record",
        ),
    ];

    for (command_name, payload, expected) in cases {
        // Arrange
        let command = MessageCommand::new(command_name).expect("command");

        // Act
        let error = WireNetworkMessage::decode_payload(&command, &payload)
            .expect_err("malformed BIP152 payload should be surfaced");

        // Assert
        assert_eq!(error.to_string(), expected);
    }
}

#[test]
fn phase112_wire_getblocktxn_round_trips_payload_and_wire() {
    // Arrange
    let request = BlockTransactionsRequest {
        block_hash: BlockHash::from_byte_array([12_u8; 32]),
        index_deltas: vec![0, 2, 4],
    };
    let message = WireNetworkMessage::GetBlockTxn(request.clone());

    // Act
    let encoded = message.encode_payload().expect("getblocktxn payload");
    let decoded =
        WireNetworkMessage::decode_payload(&message.command().expect("command"), &encoded)
            .expect("getblocktxn decode");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");

    // Assert
    assert_eq!(message.command_name(), "getblocktxn");
    assert_eq!(decoded, WireNetworkMessage::GetBlockTxn(request));
    assert_eq!(parsed.message, message);
}

#[test]
fn phase112_wire_blocktxn_round_trips_witness_transactions() {
    // Arrange
    let response = BlockTransactions {
        block_hash: BlockHash::from_byte_array([13_u8; 32]),
        transactions: vec![sample_transaction()],
    };
    let message = WireNetworkMessage::BlockTxn(response.clone());

    // Act
    let encoded = message.encode_payload().expect("blocktxn payload");
    let decoded =
        WireNetworkMessage::decode_payload(&message.command().expect("command"), &encoded)
            .expect("blocktxn decode");
    let wire = message.encode_wire(NetworkMagic::MAINNET).expect("wire");
    let parsed = ParsedNetworkMessage::decode_wire(&wire).expect("decode wire");

    // Assert
    assert_eq!(message.command_name(), "blocktxn");
    assert_eq!(decoded, WireNetworkMessage::BlockTxn(response));
    assert_eq!(parsed.message, message);
    let WireNetworkMessage::BlockTxn(parsed_response) = parsed.message else {
        panic!("parsed message should be blocktxn");
    };
    assert!(parsed_response.transactions[0].has_witness());
}

#[test]
fn phase112_all_bip152_commands_are_explicit_network_messages() {
    // Arrange
    let messages = [
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }),
        WireNetworkMessage::CompactBlock(sample_compact_block_payload()),
        WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
            block_hash: BlockHash::from_byte_array([14_u8; 32]),
            index_deltas: vec![0],
        }),
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash: BlockHash::from_byte_array([15_u8; 32]),
            transactions: vec![sample_transaction()],
        }),
    ];

    // Act
    let decoded = messages.clone().map(|message| {
        let payload = message.encode_payload().expect("BIP152 payload");
        WireNetworkMessage::decode_payload(&message.command().expect("command"), &payload)
            .expect("BIP152 message should decode")
    });
    let adjacent_command = MessageCommand::new("sendcmpct2").expect("command");
    let adjacent_error = WireNetworkMessage::decode_payload(&adjacent_command, &[])
        .expect_err("adjacent command remains unknown");

    // Assert
    assert_eq!(decoded, messages);
    assert_eq!(
        adjacent_error.to_string(),
        "unknown network command: sendcmpct2",
    );
}

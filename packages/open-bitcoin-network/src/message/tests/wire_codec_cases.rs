// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/netaddress.h
// - packages/bitcoin-knots/src/netbase.h

use super::*;

#[test]
fn empty_payload_messages_reject_non_empty_payload() {
    // Arrange
    let command_names = ["verack", "wtxidrelay", "sendheaders", "getaddr"];

    // Act
    let errors = command_names.map(|command_name| {
        let command = MessageCommand::new(command_name).expect("command");
        WireNetworkMessage::decode_payload(&command, &[0x00]).expect_err("payload must fail")
    });

    // Assert
    assert!(
        errors
            .iter()
            .all(|error| error.to_string() == "trailing data: 1 bytes")
    );
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
        super::super::encode_version_payload(&VersionMessage::default()).expect("payload");
    relay_optional.pop();
    let decoded = super::super::decode_version_payload(&relay_optional).expect("optional relay");
    assert!(!decoded.relay);

    let invalid_user_agent = {
        let mut payload = Vec::new();
        payload.extend_from_slice(&super::super::PROTOCOL_VERSION.to_le_bytes());
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
        super::super::decode_version_payload(&invalid_user_agent)
            .expect_err("invalid user agent encoding must fail")
            .to_string(),
        "version message user agent is not valid UTF-8",
    );

    let mut cursor = super::super::Cursor::new(&[0x01, 0x02]);
    assert_eq!(
        cursor
            .read_slice(3)
            .expect_err("read past end must fail")
            .to_string(),
        "unexpected EOF: needed 3 bytes, remaining 2",
    );
    let cursor = super::super::Cursor::new(&[0x01]);
    assert_eq!(
        cursor
            .finish()
            .expect_err("trailing data must fail")
            .to_string(),
        "trailing data: 1 bytes",
    );
    let mut cursor = super::super::Cursor::new(&[0xfd, 0x01, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::super::Cursor::new(&[0xfe, 0x01, 0x00, 0x00, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::super::Cursor::new(&[0xfe, 0x00, 0x00, 0x01, 0x00]);
    assert_eq!(cursor.read_compact_size().expect("canonical value"), 65_536);
    let mut cursor =
        super::super::Cursor::new(&[0xff, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]);
    assert_eq!(
        cursor
            .read_compact_size()
            .expect_err("non-canonical compact size")
            .to_string(),
        "non-canonical compact size for value 1",
    );
    let mut cursor = super::super::Cursor::new(&[0xff, 0, 0, 0, 0, 1, 0, 0, 0]);
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
        super::super::decode_headers_payload(&headers_payload)
            .expect_err("headers payload with txns must fail")
            .to_string(),
        "headers message included non-zero transaction count: 1",
    );

    let mut too_many_headers = Vec::new();
    open_bitcoin_codec::write_compact_size(
        &mut too_many_headers,
        (super::super::MAX_HEADERS_RESULTS + 1) as u64,
    )
    .expect("count");
    assert_eq!(
        super::super::decode_headers_payload(&too_many_headers)
            .expect_err("header count overflow")
            .to_string(),
        "headers count length out of range: 2001",
    );

    let mut too_many_inventory = Vec::new();
    open_bitcoin_codec::write_compact_size(
        &mut too_many_inventory,
        (super::super::MAX_INV_SIZE + 1) as u64,
    )
    .expect("count");
    assert_eq!(
        super::super::decode_inventory_payload(&too_many_inventory)
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
        super::super::decode_inventory_payload(&inventory_payload).expect("inventory payload"),
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
        super::super::decode_nonce_payload(&[1, 0, 0, 0, 0, 0, 0, 0, 1])
            .expect_err("trailing nonce payload must fail")
            .to_string(),
        "trailing data: 1 bytes",
    );
}

use open_bitcoin_primitives::{
    BLOCK_LOCATOR_DUMMY_VERSION, BlockLocator, Hash32, InventoryType, InventoryVector,
    MessageCommand, MessageHeader, NetworkAddress, NetworkMagic,
};

use crate::compact_size::{compact_size_to_usize, read_compact_size, write_compact_size};
use crate::error::CodecError;
use crate::primitives::{Reader, write_i32_le, write_u16_be, write_u32_le, write_u64_le};

pub fn parse_message_header(bytes: &[u8]) -> Result<MessageHeader, CodecError> {
    let mut reader = Reader::new(bytes);
    let header = parse_message_header_from_reader(&mut reader)?;
    reader.finish()?;
    Ok(header)
}

pub(crate) fn parse_message_header_from_reader(
    reader: &mut Reader<'_>,
) -> Result<MessageHeader, CodecError> {
    Ok(MessageHeader {
        magic: NetworkMagic::from_bytes(reader.read_array::<4>()?),
        command: MessageCommand::from_wire_bytes(
            reader.read_array::<{ open_bitcoin_primitives::MESSAGE_TYPE_SIZE }>()?,
        )?,
        payload_size: reader.read_u32_le()?,
        checksum: reader.read_array::<4>()?,
    })
}

pub fn encode_message_header(header: &MessageHeader) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(header.magic.as_bytes());
    out.extend_from_slice(&header.command.to_wire_bytes());
    write_u32_le(&mut out, header.payload_size);
    out.extend_from_slice(&header.checksum);
    out
}

pub fn parse_network_address(bytes: &[u8]) -> Result<NetworkAddress, CodecError> {
    let mut reader = Reader::new(bytes);
    let address = NetworkAddress {
        services: reader.read_u64_le()?,
        address_bytes: reader.read_array::<16>()?,
        port: reader.read_u16_be()?,
    };
    reader.finish()?;
    Ok(address)
}

pub fn encode_network_address(address: &NetworkAddress) -> Vec<u8> {
    let mut out = Vec::new();
    write_u64_le(&mut out, address.services);
    out.extend_from_slice(&address.address_bytes);
    write_u16_be(&mut out, address.port);
    out
}

pub fn parse_inventory_vector(bytes: &[u8]) -> Result<InventoryVector, CodecError> {
    let mut reader = Reader::new(bytes);
    let vector = InventoryVector {
        inventory_type: InventoryType::from_raw(reader.read_u32_le()?),
        object_hash: Hash32::from_byte_array(reader.read_array::<32>()?),
    };
    reader.finish()?;
    Ok(vector)
}

pub fn encode_inventory_vector(vector: &InventoryVector) -> Vec<u8> {
    let mut out = Vec::new();
    write_u32_le(&mut out, vector.inventory_type.to_raw());
    out.extend_from_slice(vector.object_hash.as_bytes());
    out
}

pub fn parse_block_locator(bytes: &[u8]) -> Result<BlockLocator, CodecError> {
    let mut reader = Reader::new(bytes);
    let _dummy_version = reader.read_i32_le()?;
    let count = compact_size_to_usize(read_compact_size(&mut reader)?, "block locator count");
    let mut hashes = Vec::with_capacity(count);
    for _ in 0..count {
        hashes.push(Hash32::from_byte_array(reader.read_array::<32>()?));
    }
    reader.finish()?;
    Ok(BlockLocator {
        block_hashes: hashes,
    })
}

pub fn encode_block_locator(locator: &BlockLocator) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::new();
    write_i32_le(&mut out, BLOCK_LOCATOR_DUMMY_VERSION);
    write_compact_size(&mut out, locator.block_hashes.len() as u64)?;
    for hash in &locator.block_hashes {
        out.extend_from_slice(hash.as_bytes());
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use open_bitcoin_primitives::InventoryType;

    use crate::test_support::decode_hex;

    use super::{
        BlockLocator, Hash32, InventoryVector, encode_block_locator, encode_inventory_vector,
        encode_message_header, encode_network_address, parse_block_locator, parse_inventory_vector,
        parse_message_header, parse_network_address,
    };

    const MESSAGE_HEADER_HEX: &str = include_str!("../testdata/message_header.hex");

    #[test]
    fn message_header_round_trips() {
        let bytes = decode_hex(MESSAGE_HEADER_HEX);
        let header = parse_message_header(&bytes).expect("fixture should decode");
        let reencoded = encode_message_header(&header);

        assert_eq!(reencoded, bytes);
    }

    #[test]
    fn block_locator_round_trips() {
        let locator = BlockLocator {
            block_hashes: vec![Hash32::from_byte_array([8_u8; 32])],
        };

        let encoded = encode_block_locator(&locator).expect("locator should encode");
        assert_eq!(parse_block_locator(&encoded), Ok(locator));
    }

    #[test]
    fn inventory_vector_encodes_expected_type_tag() {
        let vector = InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: Hash32::from_byte_array([7_u8; 32]),
        };

        let encoded = encode_inventory_vector(&vector);
        let decoded = parse_inventory_vector(&encoded).expect("vector should decode");

        assert_eq!(decoded, vector);
        assert_eq!(&encoded[..4], &2_u32.to_le_bytes());
    }

    #[test]
    fn network_address_round_trips() {
        let address = open_bitcoin_primitives::NetworkAddress {
            services: 1,
            address_bytes: [0; 16],
            port: 8333,
        };

        let encoded = encode_network_address(&address);
        let decoded = parse_network_address(&encoded).expect("address should decode");

        assert_eq!(decoded, address);
    }

    #[test]
    fn parse_message_header_reports_trailing_data() {
        let mut bytes = decode_hex(MESSAGE_HEADER_HEX);
        bytes.push(0x00);

        let error = parse_message_header(&bytes).expect_err("trailing data must be rejected");
        assert_eq!(error.to_string(), "trailing data: 1 bytes");
    }

    #[test]
    fn parse_message_header_rejects_non_ascii_commands() {
        let mut bytes = decode_hex(MESSAGE_HEADER_HEX);
        bytes[4] = 0xff;

        let error = parse_message_header(&bytes).expect_err("non-ascii command must be rejected");
        assert_eq!(
            error.to_string(),
            "message command contains non-ascii byte: 255"
        );
    }
}

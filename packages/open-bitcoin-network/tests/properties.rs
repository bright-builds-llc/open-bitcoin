// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use open_bitcoin_network::{
    InventoryList, LocalPeerConfig, ParsedNetworkMessage, ServiceFlags, VersionMessage,
    WireNetworkMessage,
};
use open_bitcoin_primitives::{
    BlockHash, BlockLocator, Hash32, InventoryType, InventoryVector, NetworkAddress, NetworkMagic,
};

#[derive(Debug, Clone)]
struct DeterministicGenerator {
    state: u64,
}

impl DeterministicGenerator {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    fn next_u8(&mut self) -> u8 {
        self.state = self
            .state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1);
        (self.state >> 32) as u8
    }

    fn next_u64(&mut self) -> u64 {
        let mut bytes = [0_u8; 8];
        for byte in &mut bytes {
            *byte = self.next_u8();
        }
        u64::from_le_bytes(bytes)
    }

    fn hash(&mut self) -> Hash32 {
        let mut bytes = [0_u8; 32];
        for byte in &mut bytes {
            *byte = self.next_u8();
        }
        Hash32::from_byte_array(bytes)
    }
}

#[test]
fn generated_wire_messages_round_trip_through_protocol_codec() {
    // Arrange
    let mut generator = DeterministicGenerator::new(0xface_feed);
    let magic = NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]);

    for case in 0..64 {
        let message = generated_message(case, &mut generator);

        // Act
        let encoded = message.encode_wire(magic).expect("message encodes");
        let parsed = ParsedNetworkMessage::decode_wire(&encoded).expect("message decodes");

        // Assert
        assert_eq!(parsed.header.magic, magic);
        assert_eq!(parsed.message, message);
    }
}

#[test]
fn generated_wire_messages_reject_mutated_checksums() {
    // Arrange
    let mut generator = DeterministicGenerator::new(0xdec0_de00);
    let magic = NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]);

    for case in 0..16 {
        let message = generated_message(case, &mut generator);
        let mut encoded = message.encode_wire(magic).expect("message encodes");
        encoded[20] ^= 0xff;

        // Act
        let result = ParsedNetworkMessage::decode_wire(&encoded);

        // Assert
        assert!(matches!(
            result,
            Err(open_bitcoin_network::NetworkError::InvalidChecksum)
        ));
    }
}

fn generated_message(case: usize, generator: &mut DeterministicGenerator) -> WireNetworkMessage {
    match case % 7 {
        0 => WireNetworkMessage::Version(generated_version(generator)),
        1 => WireNetworkMessage::Verack,
        2 => WireNetworkMessage::WtxidRelay,
        3 => WireNetworkMessage::Ping {
            nonce: generator.next_u64(),
        },
        4 => WireNetworkMessage::Pong {
            nonce: generator.next_u64(),
        },
        5 => WireNetworkMessage::Inv(InventoryList::new(vec![generated_inventory(generator)])),
        _ => WireNetworkMessage::GetHeaders {
            locator: BlockLocator {
                block_hashes: vec![generator.hash()],
            },
            stop_hash: BlockHash::from_byte_array(generator.hash().to_byte_array()),
        },
    }
}

fn generated_version(generator: &mut DeterministicGenerator) -> VersionMessage {
    let config = LocalPeerConfig {
        magic: NetworkMagic::from_bytes([0xfa, 0xbf, 0xb5, 0xda]),
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 1,
            address_bytes: [0_u8; 16],
            port: 18_444,
        },
        nonce: generator.next_u64(),
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    };
    config.version_message(generator.next_u64() as i64, 0)
}

fn generated_inventory(generator: &mut DeterministicGenerator) -> InventoryVector {
    InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: generator.hash(),
    }
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn local_config() -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: super::super::super::message::zero_address(),
        nonce: 7,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    }
}

pub(super) fn complete_outbound_handshake(
    manager: &mut PeerManager,
    peer_id: PeerId,
    start_height: i32,
) {
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");
    manager
        .handle_message(peer_id, WireNetworkMessage::Verack, 12)
        .expect("verack");
}

pub(super) fn assert_phase94_block_cap_matches_peer_default() {
    assert_eq!(
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER
    );
}

pub(super) fn phase111_block_witness_compact_inventory(count: usize) -> InventoryList {
    let inventory_types = [
        InventoryType::Block,
        InventoryType::WitnessBlock,
        InventoryType::CompactBlock,
    ];
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: inventory_types[index % inventory_types.len()],
                object_hash: hash_from_index(111_000 + index),
            })
            .collect(),
    )
}

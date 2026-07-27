// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

pub(super) fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_231_006_500 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

pub(super) fn mined_header(previous_block_hash: BlockHash, seed: u32) -> BlockHeader {
    let mut header = header(previous_block_hash, seed);
    let nonce = (0..=u32::MAX)
        .find(|nonce| {
            header.nonce = *nonce;
            check_block_header(&header).is_ok()
        })
        .expect("expected nonce at easy target");
    header.nonce = nonce;
    header
}

pub(super) fn assert_resource_limit_disconnect(actions: &[PeerAction]) {
    let [PeerAction::ResourceGovernanceDisconnect(event)] = actions else {
        panic!("expected resource-governance disconnect action, got {actions:?}");
    };
    assert_eq!(event.label, "request_cap_reached");
    assert_eq!(event.next_action, "request_cap_reached");
}

pub(super) fn transaction_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

pub(super) fn block_inventory(count: usize) -> InventoryList {
    InventoryList::new(
        (0..count)
            .map(|index| InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: hash_from_index(index),
            })
            .collect(),
    )
}

pub(super) fn cleanup_label_for(
    cause: BlockInFlightCleanupCause,
    blocks_in_flight_before: usize,
    released_blocks: usize,
    remaining_blocks_in_flight: usize,
) -> &'static str {
    classify_block_inflight_cleanup(&BlockInFlightCleanupInput {
        cause,
        blocks_in_flight_before,
        released_blocks,
        remaining_blocks_in_flight,
        max_blocks_in_flight_per_peer: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        max_blocks_in_flight_total: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    })
    .label
    .as_str()
}

pub(super) fn header_chain(count: usize) -> Vec<BlockHeader> {
    let mut headers = Vec::new();
    let mut previous = BlockHash::from_byte_array([0_u8; 32]);
    for index in 0..count {
        let next = header(previous, index as u32 + 1);
        previous = open_bitcoin_consensus::block_hash(&next);
        headers.push(next);
    }
    headers
}

pub(super) fn hash_from_index(index: usize) -> Hash32 {
    let mut bytes = [0_u8; 32];
    bytes[..8].copy_from_slice(&(index as u64).to_le_bytes());
    Hash32::from_byte_array(bytes)
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! `pruned_count` moves only when durable have_pruned is true and the payload is gone.
//! Knots `IsBlockPruned` is `m_have_pruned` plus missing block data.

use open_bitcoin_core::chainstate::PruneMode;

use crate::status::{BlockServingStatusCounters, FieldAvailability};

use super::*;

fn serving_peer(peer_id: u64) -> ManagedPeerNetwork<MemoryChainstateStore> {
    let mut network = block_serving_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
}

fn connect_active_pair(network: &mut ManagedPeerNetwork<MemoryChainstateStore>) -> (Block, Block) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&child, verify_flags(), consensus_params())
        .expect("connect child");
    (genesis, child)
}

fn status_after_getdata(
    network: &mut ManagedPeerNetwork<MemoryChainstateStore>,
    peer_id: u64,
    inventory: InventoryList,
) -> BlockServingStatusCounters {
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("getdata");
    let FieldAvailability::Available(status) =
        network.block_relay_evidence_status().block_serving.status
    else {
        panic!("block-serving status counters should be available after getdata");
    };
    status
}

#[test]
fn missing_payload_without_have_pruned_counts_unavailable() {
    // Arrange
    let peer_id = 149_401;
    let mut network = serving_peer(peer_id);
    let (genesis, _child) = connect_active_pair(&mut network);
    network.set_serving_have_pruned(false);
    let genesis_hash = block_hash(&genesis.header);
    network.blocks_by_hash.remove(&genesis_hash);
    let inventory = block_getdata_inventory(&genesis);

    // Act
    let status = status_after_getdata(&mut network, peer_id, inventory);

    // Assert
    assert_eq!(status.unavailable_count, 1);
    assert_eq!(status.pruned_count, 0);
}

#[test]
fn have_pruned_active_chain_gap_counts_pruned() {
    // Arrange
    let peer_id = 149_402;
    let mut network = serving_peer(peer_id);
    let (genesis, _child) = connect_active_pair(&mut network);
    network.set_serving_have_pruned(true);
    let genesis_hash = block_hash(&genesis.header);
    network.blocks_by_hash.remove(&genesis_hash);
    let inventory = block_getdata_inventory(&genesis);

    // Act
    let status = status_after_getdata(&mut network, peer_id, inventory);

    // Assert
    assert_eq!(status.pruned_count, 1);
    assert_eq!(status.unavailable_count, 0);
}

#[test]
fn present_payload_with_have_pruned_counts_available() {
    // Arrange
    let peer_id = 149_403;
    let mut network = serving_peer(peer_id);
    let (genesis, _child) = connect_active_pair(&mut network);
    network.set_serving_have_pruned(true);
    let inventory = block_getdata_inventory(&genesis);

    // Act
    let status = status_after_getdata(&mut network, peer_id, inventory);

    // Assert
    assert!(status.available_count >= 1);
    assert_eq!(status.pruned_count, 0);
}

#[test]
fn manual_only_without_have_pruned_does_not_count_pruned() {
    // Arrange
    let peer_id = 149_404;
    let mut network = serving_peer(peer_id);
    let (genesis, _child) = connect_active_pair(&mut network);
    network.set_prune_mode(PruneMode::ManualOnly);
    network.set_serving_have_pruned(false);
    let genesis_hash = block_hash(&genesis.header);
    network.blocks_by_hash.remove(&genesis_hash);
    let inventory = block_getdata_inventory(&genesis);

    // Act
    let status = status_after_getdata(&mut network, peer_id, inventory);

    // Assert
    assert_eq!(status.unavailable_count, 1);
    assert_eq!(status.pruned_count, 0);
}

#[test]
fn unknown_hash_with_have_pruned_does_not_count_pruned() {
    // Arrange
    let peer_id = 149_405;
    let mut network = serving_peer(peer_id);
    let (_genesis, _child) = connect_active_pair(&mut network);
    network.set_serving_have_pruned(true);
    let unknown = BlockHash::from_byte_array([9_u8; 32]);
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: unknown.into(),
    }]);

    // Act
    let status = status_after_getdata(&mut network, peer_id, inventory);

    // Assert
    assert_eq!(status.pruned_count, 0);
}

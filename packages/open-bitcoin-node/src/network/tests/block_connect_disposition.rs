// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::consensus::block_hash;

use super::*;

#[test]
fn connect_stored_block_returns_connected_disposition() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(101),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let disposition = network
        .connect_stored_block(
            &child,
            2,
            i64::from(child.header.time),
            verify_flags(),
            consensus_params(),
        )
        .expect("connect stored child");

    // Assert
    assert_eq!(
        disposition,
        BlockConnectDisposition::Connected(
            network
                .maybe_chain_tip()
                .expect("child should become active tip")
        )
    );
    assert_eq!(network.maybe_chain_tip().expect("tip").height, 1);
}

#[test]
fn connect_stored_block_returns_duplicate_disposition() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(102),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let child = build_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    network
        .connect_local_block(&child, verify_flags(), consensus_params())
        .expect("connect child");
    let child_hash = block_hash(&child.header);

    // Act
    let disposition = network
        .connect_stored_block(
            &child,
            2,
            i64::from(child.header.time),
            verify_flags(),
            consensus_params(),
        )
        .expect("classify duplicate");

    // Assert
    assert_eq!(disposition, BlockConnectDisposition::Duplicate(child_hash));
    assert_eq!(network.chainstate_snapshot().active_chain.len(), 2);
    assert_eq!(network.maybe_chain_tip().expect("tip").height, 1);
}

#[test]
fn connect_stored_block_returns_non_extending_disposition() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(103),
        PolicyConfig::default(),
    );
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let unknown_parent = BlockHash::from_byte_array([42_u8; 32]);
    let side_block = build_block(unknown_parent, 1, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let side_hash = block_hash(&side_block.header);

    // Act
    let disposition = network
        .connect_stored_block(
            &side_block,
            2,
            i64::from(side_block.header.time),
            verify_flags(),
            consensus_params(),
        )
        .expect("classify non-extending block");

    // Assert
    assert_eq!(
        disposition,
        BlockConnectDisposition::NonExtending {
            block_hash: side_hash,
            previous_block_hash: unknown_parent,
        }
    );
    assert_eq!(network.maybe_chain_tip().expect("tip").height, 0);
}

#[test]
fn connect_stored_block_returns_disconnected_disposition() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(104),
        PolicyConfig::default(),
    );
    let unknown_parent = BlockHash::from_byte_array([7_u8; 32]);
    let disconnected = build_block(unknown_parent, 1, 500_000_000);
    let disconnected_hash = block_hash(&disconnected.header);

    // Act
    let disposition = network
        .connect_stored_block(
            &disconnected,
            1,
            i64::from(disconnected.header.time),
            verify_flags(),
            consensus_params(),
        )
        .expect("classify disconnected block");

    // Assert
    assert_eq!(
        disposition,
        BlockConnectDisposition::Disconnected {
            block_hash: disconnected_hash,
        }
    );
    assert!(network.maybe_chain_tip().is_none());
}

#[test]
fn receive_sync_message_reports_block_disposition() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(105),
        PolicyConfig::default(),
    );
    let disconnected = build_block(BlockHash::from_byte_array([9_u8; 32]), 1, 500_000_000);
    network.add_inbound_peer(55).expect("peer");

    // Act
    let result = network
        .receive_sync_message(
            55,
            WireNetworkMessage::Block(disconnected),
            1_231_006_501,
            verify_flags(),
            consensus_params(),
        )
        .expect("receive sync block");

    // Assert
    assert!(result.maybe_block_disposition.is_some());
}

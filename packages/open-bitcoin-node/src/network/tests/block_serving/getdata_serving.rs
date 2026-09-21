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

use super::*;

#[test]
fn phase111_disabled_block_serving_suppresses_cached_active_block() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        local_config(111_201),
        PolicyConfig::default(),
    );
    network
        .connect_outbound_peer(111_201, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let outbound = network
        .receive_message(
            111_201,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata")
        .outbound;

    // Assert
    assert_eq!(
        outbound,
        vec![WireNetworkMessage::NotFound(block_getdata_inventory(
            &genesis
        ))]
    );
}

#[test]
fn phase111_block_getdata_serves_active_validated_block_when_enabled() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_202);
    network
        .connect_outbound_peer(111_202, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let outbound = network
        .receive_message(
            111_202,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::Block(genesis)]);
}

#[test]
fn phase111_witness_block_getdata_preserves_witness_payload() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_203);
    network
        .connect_outbound_peer(111_203, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let mut cached_genesis = genesis.clone();
    cached_genesis.transactions[0].inputs[0].witness =
        ScriptWitness::new(vec![vec![0x01, 0x02], vec![0x51]]);
    let genesis_hash = block_hash(&genesis.header);
    network.blocks_by_hash.insert(genesis_hash, cached_genesis);
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessBlock,
        object_hash: genesis_hash.into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            111_203,
            WireNetworkMessage::GetData(inventory),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("witness block getdata")
        .outbound;

    // Assert
    let [WireNetworkMessage::Block(served)] = outbound.as_slice() else {
        panic!("expected served witness block, got {outbound:?}");
    };
    let encoded = encode_block(served).expect("served block should encode");
    let decoded = parse_block(&encoded).expect("served block should decode");
    assert_eq!(
        decoded.transactions[0].inputs[0].witness.stack(),
        &[vec![0x01, 0x02], vec![0x51]]
    );
}

#[test]
fn phase111_compact_block_getdata_is_suppressed_without_block_payload() {
    // Arrange
    let mut network = block_serving_enabled_managed_network(111_204);
    network
        .connect_outbound_peer(111_204, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: block_hash(&genesis.header).into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            111_204,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("compact block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert!(
        !outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::Block(_)))
    );
}

#[test]
fn phase113_compact_getdata_remains_suppressed_after_negotiation_policy() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(113_301);
    network
        .connect_outbound_peer(113_301, 1)
        .expect("connect outbound");
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::CompactBlock,
        object_hash: block_hash(&genesis.header).into(),
    }]);

    // Act
    let outbound = network
        .receive_message(
            113_301,
            WireNetworkMessage::GetData(inventory.clone()),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("compact block getdata")
        .outbound;

    // Assert
    assert_eq!(outbound, vec![WireNetworkMessage::NotFound(inventory)]);
    assert!(
        !outbound
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::CompactBlock(_)))
    );
    assert!(!outbound.iter().any(|message| {
        matches!(
            message,
            WireNetworkMessage::GetBlockTxn(_) | WireNetworkMessage::BlockTxn(_)
        )
    }));
    assert_eq!(network.mempool_info().transaction_count, 0);
    assert_eq!(network.maybe_chain_tip().expect("tip").height, 0);
}

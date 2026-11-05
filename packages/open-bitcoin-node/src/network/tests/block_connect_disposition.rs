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

use super::compact_cleanup_cases::{
    assert_in_flight, assert_no_in_flight_hash, build_block_with_transactions,
    compact_payload_from_block, start_shared_compact_on_peer, tip_chain, txid,
};
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
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
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
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
            i64::from(child.header.time),
            verify_flags(),
            consensus_params(),
        )
        .expect("classify duplicate");

    // Assert
    assert_eq!(disposition, BlockConnectDisposition::Duplicate(child_hash));
    assert_eq!(
        network
            .chainstate_snapshot()
            .expect("chainstate snapshot")
            .active_chain
            .len(),
        2
    );
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
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(2),
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
            open_bitcoin_core::chainstate::ChainPosition::untrusted_fixture_work(1),
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

#[test]
fn phase167_compact_stored_body_fixture_stale_epoch_preserves_partial_request() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(167_671);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    start_shared_compact_on_peer(
        &mut network,
        167_671,
        compact_payload_from_block(&block, 17),
        i64::from(block.header.time),
    );
    network
        .retain_checked_candidate(&block, consensus_params(), i64::from(block.header.time))
        .expect("actual retained checked candidate");
    let epoch = network
        .peer_manager()
        .header_store()
        .body_eligibility_epoch();
    network.peer_manager_mut().mark_body_unavailable(hash);
    assert!(!epoch.is_current());
    let before = network.chainstate_snapshot().expect("accepted before");

    // Act
    let finished = network.finish_bookkeeping(&block);

    // Assert
    assert!(!finished);
    assert_eq!(
        network.chainstate_snapshot().expect("accepted after"),
        before
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_671]
    );
    assert_in_flight(&network, 167_671, hash);
}

#[test]
fn phase167_compact_stored_body_consumed_fallback_finishes_once_on_local_success() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(167_681);
    let (_, spendable) = tip_chain(&mut network);
    let spend = spend_transaction(txid(&spendable.transactions[0]), 499_999_000);
    let block = build_block_with_transactions(block_hash(&spendable.header), 2, vec![spend]);
    let hash = block_hash(&block.header);
    let unrelated_spend = spend_transaction(txid(&spendable.transactions[0]), 499_997_000);
    let unrelated =
        build_block_with_transactions(block_hash(&spendable.header), 3, vec![unrelated_spend]);
    let unrelated_hash = block_hash(&unrelated.header);
    let now = i64::from(block.header.time);
    start_shared_compact_on_peer(
        &mut network,
        167_681,
        compact_payload_from_block(&block, 17),
        now,
    );
    start_shared_compact_on_peer(
        &mut network,
        167_682,
        compact_payload_from_block(&block, 19),
        now,
    );
    start_shared_compact_on_peer(
        &mut network,
        167_683,
        compact_payload_from_block(&unrelated, 23),
        now,
    );
    let wrong = spend_transaction(txid(&spendable.transactions[0]), 499_998_000);
    let mismatch = WireNetworkMessage::BlockTxn(open_bitcoin_codec::BlockTransactions {
        block_hash: hash,
        transactions: vec![wrong],
    });

    // Act
    let first = network
        .receive_message(
            167_681,
            mismatch,
            now + 1,
            verify_flags(),
            consensus_params(),
        )
        .expect("real first fallback");
    let repeated = network
        .receive_message(
            167_681,
            WireNetworkMessage::CompactBlock(compact_payload_from_block(&block, 17)),
            now + 2,
            verify_flags(),
            consensus_params(),
        )
        .expect("consumed attempt suppresses repeat compact fallback");
    assert!(
        first
            .outbound
            .iter()
            .any(|item| matches!(item, WireNetworkMessage::GetData(_)))
    );
    assert!(
        !repeated
            .outbound
            .iter()
            .any(|item| matches!(item, WireNetworkMessage::GetData(_)))
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_681, 167_682]
    );
    network
        .connect_local_block(&block, verify_flags(), consensus_params())
        .expect("real retained local completion");

    // Assert
    assert!(
        network
            .peer_manager()
            .block_request_peer_order(hash)
            .is_empty()
    );
    for peer in [167_681, 167_682] {
        assert!(
            !network
                .peer_manager()
                .peer_state(peer)
                .expect("live peer")
                .requested_blocks
                .contains(&hash)
        );
        assert_no_in_flight_hash(&network, peer, hash);
    }
    assert_eq!(
        network
            .peer_manager()
            .block_request_peer_order(unrelated_hash),
        vec![167_683]
    );
    assert_in_flight(&network, 167_683, unrelated_hash);
    let reentered = network
        .receive_message(
            167_681,
            WireNetworkMessage::CompactBlock(compact_payload_from_block(&block, 17)),
            now + 3,
            verify_flags(),
            consensus_params(),
        )
        .expect("refunded compact slot");
    assert!(
        reentered
            .outbound
            .iter()
            .any(|item| matches!(item, WireNetworkMessage::GetBlockTxn(_)))
    );
    assert_eq!(
        network.peer_manager().block_request_peer_order(hash),
        vec![167_681]
    );
    assert!(network.finish_bookkeeping(&block));
    assert!(
        network
            .peer_manager()
            .block_request_peer_order(hash)
            .is_empty()
    );
    assert_eq!(
        network
            .peer_manager()
            .block_request_peer_order(unrelated_hash),
        vec![167_683]
    );
}

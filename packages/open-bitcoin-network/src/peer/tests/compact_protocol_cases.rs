// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase112_bip152_non_compact_dispatch_emits_no_unconditional_actions() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(112_003, 0).expect("peer");
    let block_hash = BlockHash::from_byte_array([112_u8; 32]);
    let messages = [
        WireNetworkMessage::SendCompact(SendCompactMessage {
            announce: true,
            version: 2,
        }),
        WireNetworkMessage::BlockTxn(BlockTransactions {
            block_hash,
            transactions: Vec::new(),
        }),
    ];

    for message in messages {
        // Act
        let actions = manager
            .handle_message(112_003, message, 1)
            .expect("BIP152 message should be accepted");

        // Assert
        assert!(actions.is_empty());
    }
}

#[test]
fn phase122_compact_announced_getblocktxn_dispatches_ordered_absolute_indexes_for_that_peer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let peer_id = 122_002;
    let other_peer_id = 122_003;
    let block_hash = BlockHash::from_byte_array([0x42; 32]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    manager
        .add_outbound_peer(other_peer_id, 0)
        .expect("other peer");
    manager
        .record_compact_block_announcement(peer_id, block_hash)
        .expect("announcement");
    let request = WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
        block_hash,
        index_deltas: vec![2, 0, 3],
    });

    // Act
    let actions = manager
        .handle_message(peer_id, request.clone(), 1)
        .expect("announced request");
    let other_actions = manager
        .handle_message(other_peer_id, request, 1)
        .expect("unannounced request");

    // Assert
    assert_eq!(
        actions,
        vec![PeerAction::ServeCompactBlockTransactions(
            super::super::CompactBlockTransactionsRequest {
                block_hash,
                indexes: vec![2, 3, 7],
            }
        )]
    );
    assert!(other_actions.is_empty());
}

#[test]
fn phase122_unannounced_getblocktxn_over_request_cap_disconnects_before_suppression() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let peer_id = 122_005;
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    let request = WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
        block_hash: BlockHash::from_byte_array([0x45; 32]),
        index_deltas: vec![0; PHASE94_MAX_INBOUND_REQUEST_INVENTORY_ITEMS + 1],
    });

    // Act
    let actions = manager
        .handle_message(peer_id, request, 1)
        .expect("request pressure should produce a disconnect action");

    // Assert
    assert_resource_limit_disconnect(&actions);
    assert!(!actions
        .iter()
        .any(|action| { matches!(action, PeerAction::ServeCompactBlockTransactions(_)) }));
}

#[test]
fn phase122_compact_overflowing_getblocktxn_disconnects_and_peer_cleanup_drops_provenance() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let peer_id = 122_004;
    let block_hash = BlockHash::from_byte_array([0x44; 32]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    manager
        .record_compact_block_announcement(peer_id, block_hash)
        .expect("announcement");

    // Act
    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::GetBlockTxn(BlockTransactionsRequest {
                block_hash,
                index_deltas: vec![u64::MAX],
            }),
            1,
        )
        .expect("malformed request");
    manager.remove_peer(peer_id).expect("remove peer");

    // Assert
    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
    assert!(manager.peer_state(peer_id).is_none());
}

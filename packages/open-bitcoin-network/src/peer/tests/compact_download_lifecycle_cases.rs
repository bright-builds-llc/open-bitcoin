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
fn generic_compact_block_dispatch_requires_adapter_receive_facts() {
    // Arrange
    let peer_id = 126_001;
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    let payload = CompactBlockPayload {
        header: header(BlockHash::from_byte_array([0_u8; 32]), 126),
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };

    // Act
    let error = manager
        .handle_message(peer_id, WireNetworkMessage::CompactBlock(payload), 1)
        .expect_err("generic compact dispatch must require adapter facts");

    // Assert
    assert_eq!(error, NetworkError::CompactBlockReceiveFactsRequired);
    assert!(manager
        .compact_download_peer_state(peer_id)
        .is_none_or(|state| state.in_flight.is_empty()));
}

#[test]
fn phase115_handle_compact_block_download_with_activation_enabled() {
    let peer_id = 115_001;
    let (mut manager, payload, _, block_hash) = phase115_prepare_compact_download_manager(peer_id);

    let actions = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("compact block should process");

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        actions[0],
        PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))
    ));
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(download_state.in_flight.contains_key(&block_hash));
    assert_eq!(
        manager.block_relay_activation_policy(),
        compact_announcement_activation(true)
    );
}

#[test]
fn phase115_expire_compact_download_timeouts_requests_full_blocks() {
    let peer_id = 115_002;
    let (mut manager, payload, _, block_hash) = phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
        )
        .expect("compact block should start download");

    let actions = manager
        .expire_compact_download_timeouts(100 + crate::COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1);

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        (
            returned_peer_id,
            PeerAction::Send(WireNetworkMessage::GetData(inventory))
        ) if *returned_peer_id == peer_id
            && inventory.inventory.len() == 1
            && inventory.inventory[0].inventory_type == InventoryType::Block
            && inventory.inventory[0].object_hash == block_hash.into()
    ));
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(download_state.in_flight.is_empty());
}

#[test]
fn phase115_handle_block_transactions_message_completes_download() {
    let peer_id = 115_003;
    let (mut manager, payload, missing, block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("compact block should start download");

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("blocktxn should process");

    assert_eq!(actions.len(), 1);
    assert!(matches!(actions[0], PeerAction::ReceivedBlock(_)));
    assert!(
        manager.compact_download_peer_state(peer_id).is_none()
            || manager
                .compact_download_peer_state(peer_id)
                .expect("download state")
                .in_flight
                .is_empty()
    );
}

#[test]
fn phase115_cleanup_all_compact_downloads() {
    let peer_a = 115_004;
    let peer_b = 115_005;
    let (mut manager, payload_a, _, _) = phase115_prepare_compact_download_manager(peer_a);
    let (payload_b, _, _) = {
        let (_, payload, missing, block_hash) = phase115_prepare_compact_download_manager(peer_b);
        (payload, missing, block_hash)
    };

    let _ = manager
        .handle_compact_block_download(
            peer_a,
            payload_a,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("peer a compact block");
    manager.add_outbound_peer(peer_b, 0).expect("peer b");
    complete_outbound_handshake(&mut manager, peer_b, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_b);
    let _ = manager
        .handle_compact_block_download(
            peer_b,
            payload_b,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("peer b compact block");

    assert_eq!(
        manager
            .cleanup_compact_download_for_peer(peer_a, CompactDownloadCleanupCause::Timeout)
            .expect("peer a cleanup"),
        1
    );
    manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::PeerDisconnect);
    assert!(manager
        .compact_download_peer_state(peer_a)
        .is_none_or(|state| state.in_flight.is_empty()));
    assert!(manager
        .compact_download_peer_state(peer_b)
        .is_none_or(|state| state.in_flight.is_empty()));
}

#[test]
fn phase115_on_compact_download_block_connected_clears_matching_in_flight() {
    let peer_id = 115_006;
    let (mut manager, payload, _, connected_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("compact block");
    manager.on_compact_download_block_connected(connected_hash);
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    assert!(!download_state.in_flight.contains_key(&connected_hash));
}

#[test]
fn peer_manager_on_mempool_transaction_removed_clears_matching_partial_slots() {
    // Arrange
    let peer_id = 119_001;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let (payload, matched, matched_wtxid, _, _) =
        phase119_compact_payload_with_one_matched_and_one_missing();
    let block_hash = block_hash(&payload.header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let facts = CompactBlockReceiveFacts {
        candidates: &[(&matched_wtxid, &matched)],
        extra: &[],
    };
    let _ = manager
        .handle_compact_block_download(peer_id, payload, facts, 1_000)
        .expect("compact block with one candidate match");

    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));
    assert!(!in_flight.partial.is_transaction_available(2));

    let unrelated_wtxid = Wtxid::from_byte_array([0xaa; 32]);

    // Act — unrelated wtxid leaves matched slot unchanged
    manager.on_mempool_transaction_removed(&unrelated_wtxid);
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(in_flight.partial.is_transaction_available(1));

    // Act — matching wtxid clears the volatile slot
    manager.on_mempool_transaction_removed(&matched_wtxid);

    // Assert
    let download_state = manager
        .compact_download_peer_state(peer_id)
        .expect("download state");
    let in_flight = download_state
        .in_flight
        .get(&block_hash)
        .expect("in-flight partial");
    assert!(!in_flight.partial.is_transaction_available(1));
    assert_eq!(in_flight.partial.missing_transaction_indexes(), vec![1, 2]);
}

#[test]
fn phase115_cleanup_compact_download_for_peer_without_state_is_noop() {
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(115_007, 0).expect("peer");

    assert_eq!(
        manager
            .cleanup_compact_download_for_peer(115_007, CompactDownloadCleanupCause::Timeout)
            .expect("cleanup should succeed"),
        0
    );
}

#[test]
fn phase115_block_transactions_without_download_state_is_ignored() {
    let peer_id = 115_008;
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 0);

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: BlockHash::from_byte_array([0x88; 32]),
                transactions: Vec::new(),
            }),
            1,
        )
        .expect("blocktxn without download state");

    assert!(actions.is_empty());
}

#[test]
fn phase120_compact_block_duplicate_blocktxn_disconnects_peer() {
    let peer_id = 120_201;
    let (mut manager, payload, missing, block_hash) =
        phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("compact block should start download");

    let in_flight = manager
        .compact_download_peer_state_mut(peer_id)
        .expect("download state")
        .in_flight
        .get_mut(&block_hash)
        .expect("in-flight entry");
    in_flight.getblocktxn_in_flight = false;

    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![missing],
            }),
            1_001,
        )
        .expect("duplicate blocktxn should process");

    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
}

#[test]
fn phase120_compact_block_out_of_bounds_blocktxn_disconnects_peer() {
    let peer_id = 120_202;
    let (mut manager, payload, _, block_hash) = phase115_prepare_compact_download_manager(peer_id);
    let _ = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("compact block should start download");

    let null_transaction = Transaction {
        version: 2,
        inputs: Vec::new(),
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("valid amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("valid script"),
        }],
        lock_time: 0,
    };
    let actions = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash,
                transactions: vec![null_transaction],
            }),
            1_001,
        )
        .expect("oob blocktxn should process");

    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
}

#[test]
fn phase120_compact_block_invalid_init_disconnects_peer() {
    let peer_id = 120_203;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let payload = CompactBlockPayload {
        header,
        nonce: 1,
        short_ids: Vec::new(),
        prefilled_transactions: Vec::new(),
    };
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let actions = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("invalid compact should process");

    assert_eq!(
        actions,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockHeaderViolation
        )]
    );
}

#[test]
fn phase120_compact_block_short_id_collision_falls_back_to_getdata() {
    let peer_id = 120_204;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let header = mined_header(block_hash(&tip), 3);
    let colliding = ShortId::from_wire_bytes([0xaa, 0xbb, 0xcc, 0xdd, 0x11, 0x00]);
    let payload = CompactBlockPayload {
        header: header.clone(),
        nonce: 1,
        short_ids: vec![colliding, colliding],
        prefilled_transactions: vec![PrefilledTransaction {
            index_delta: 0,
            transaction: phase115_coinbase_transaction(),
        }],
    };
    let block_hash = block_hash(&header);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    let actions = manager
        .handle_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            1_000,
        )
        .expect("collision should process");

    assert_eq!(actions.len(), 1);
    assert!(matches!(
        &actions[0],
        PeerAction::Send(WireNetworkMessage::GetData(inventory))
            if inventory.inventory.len() == 1
                && inventory.inventory[0].inventory_type == InventoryType::Block
                && inventory.inventory[0].object_hash == block_hash.into()
    ));
}

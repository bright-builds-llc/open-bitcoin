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
fn phase167_compact_provenance_headers_all_producer_permutations_and_scoped_cleanup() {
    let cases: [([u8; 3], [PeerId; 3]); 6] = [
        ([0, 1, 2], [90, 2, 7]),
        ([0, 2, 1], [90, 7, 2]),
        ([1, 0, 2], [2, 90, 7]),
        ([1, 2, 0], [2, 7, 90]),
        ([2, 0, 1], [7, 90, 2]),
        ([2, 1, 0], [7, 2, 90]),
    ];
    for (producers, expected) in cases {
        // Arrange
        let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(90);
        for peer in [2, 7] {
            manager.add_outbound_peer(peer, 0).expect("peer");
            complete_outbound_handshake(&mut manager, peer, 2);
            process_high_bandwidth_sendcmpct(&mut manager, peer);
        }
        let unrelated = BlockHash::from(hash_from_index(991));
        manager
            .request_missing_blocks(7, &[unrelated])
            .expect("unrelated request");
        let fixture_enforce_segwit = false;
        // Act
        for (index, producer) in producers.into_iter().enumerate() {
            match producer {
                0 => {
                    manager
                        .request_missing_blocks(90, &[hash])
                        .expect("explicit");
                }
                1 => {
                    manager
                        .handle_fixture_compact_block_download(
                            2,
                            payload.clone(),
                            explicit_empty_compact_receive_facts(),
                            [100, 100, -100][index],
                            fixture_enforce_segwit,
                        )
                        .expect("compact");
                }
                _ => {
                    manager
                        .handle_headers_with_policy(
                            7,
                            HeadersMessage {
                                headers: vec![payload.header.clone()],
                            },
                            HeaderSyncPolicy::HeadersAndBlocks,
                            |store, header| store.insert_header(header.clone()),
                        )
                        .expect("header-driven request");
                }
            }
        }
        // Assert
        assert_eq!(manager.block_request_peer_order(hash), expected.to_vec());
        for peer in [90, 2, 7] {
            assert!(
                manager
                    .peer_requested_blocks(peer)
                    .expect("membership")
                    .contains(&hash)
            );
        }
        manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::Reorg);
        let mut surviving: Vec<_> = expected.into_iter().filter(|peer| *peer != 2).collect();
        assert_eq!(manager.block_request_peer_order(hash), surviving);
        assert!(
            manager
                .compact_download_peer_state(2)
                .expect("cleared partial")
                .in_flight
                .is_empty()
        );
        manager
            .request_missing_blocks(2, &[hash])
            .expect("ordinary re-enrollment");
        surviving.push(2);
        assert_eq!(manager.block_request_peer_order(hash), surviving);
        manager
            .handle_message(
                7,
                WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                    inventory_type: InventoryType::WitnessBlock,
                    object_hash: hash.into(),
                }])),
                -100,
            )
            .expect("sender-scoped release");
        surviving.retain(|peer| *peer != 7);
        assert_eq!(manager.block_request_peer_order(hash), surviving);
        manager.remove_peer(90).expect("disconnect");
        assert_eq!(manager.block_request_peer_order(hash), vec![2]);
        assert_eq!(manager.block_request_peer_order(unrelated), vec![7]);
        assert_eq!(
            manager
                .peer_requested_blocks(7)
                .expect("unrelated membership"),
            vec![unrelated]
        );
    }
}

#[test]
fn phase110_block_notfound_releases_requested_block_without_clearing_tx_inflight() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 112);
    complete_outbound_handshake(&mut manager, 112, 1);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    manager
        .handle_message(
            112,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            13,
        )
        .expect("transaction inventory");
    let block_hash = BlockHash::from(hash_from_index(112_000));
    manager
        .request_missing_blocks(112, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            112,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessBlock,
                object_hash: block_hash.into(),
            }])),
            14,
        )
        .expect("notfound");
    let retry = manager
        .request_missing_blocks(112, &[block_hash])
        .expect("retry request");

    // Assert
    assert!(actions.is_empty());
    assert_eq!(manager.transaction_request_snapshot(112).in_flight_count, 1);
    assert!(matches!(retry, Some(WireNetworkMessage::GetData(_))));
}

#[test]
fn phase110_block_response_clears_requested_block_before_received_action() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 113, 1);
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([113_u8; 32]), 1),
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    manager
        .request_missing_blocks(113, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(113, WireNetworkMessage::Block(block), 14)
        .expect("block");

    // Assert
    assert!(matches!(actions.as_slice(), [PeerAction::ReceivedBlock(_)]));
    assert!(
        manager
            .peer_requested_blocks(113)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase110_block_peer_removal_drops_requested_blocks_and_preserves_tx_cleanup() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 114..=115 {
        add_relay_outbound_peer(&mut manager, peer_id);
        complete_outbound_handshake(&mut manager, peer_id, 1);
    }
    let block_hash = BlockHash::from(hash_from_index(114_000));
    manager
        .request_missing_blocks(114, &[block_hash])
        .expect("block request")
        .expect("getdata");
    let txid = TxRelayId::Txid(txid_from_byte(114));
    seed_duplicate_announcements(&mut manager, 114, 115, txid, 20);

    // Act
    let actions = manager
        .remove_peer_with_transaction_cleanup(114, 30)
        .expect("peer cleanup");

    // Assert
    assert_eq!(
        actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 114 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 115,
                relay_id: txid,
            }),
        ]
    );
    assert!(manager.peer_state(114).is_none());
    assert!(manager.peer_requested_blocks(114).is_err());
}

#[test]
fn phase111_notfound_releases_block_and_witness_block_requested_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_104, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_104, 2);
    let block_hash = BlockHash::from(hash_from_index(111_104));
    let witness_block_hash = BlockHash::from(hash_from_index(111_105));
    manager
        .request_missing_blocks(111_104, &[block_hash, witness_block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            111_104,
            WireNetworkMessage::NotFound(InventoryList::new(vec![
                InventoryVector {
                    inventory_type: InventoryType::Block,
                    object_hash: block_hash.into(),
                },
                InventoryVector {
                    inventory_type: InventoryType::WitnessBlock,
                    object_hash: witness_block_hash.into(),
                },
            ])),
            14,
        )
        .expect("notfound");

    // Assert
    assert!(actions.is_empty());
    assert!(
        manager
            .peer_requested_blocks(111_104)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase111_received_block_releases_requested_block_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_105, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_105, 1);
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([111_u8; 32]), 105),
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    manager
        .request_missing_blocks(111_105, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(111_105, WireNetworkMessage::Block(block), 14)
        .expect("block");

    // Assert
    assert!(matches!(actions.as_slice(), [PeerAction::ReceivedBlock(_)]));
    assert!(
        manager
            .peer_requested_blocks(111_105)
            .expect("requested blocks")
            .is_empty()
    );
}

#[test]
fn phase111_peer_removal_drops_block_request_state_without_compact_state() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 111_106..=111_107 {
        add_relay_outbound_peer(&mut manager, peer_id);
        complete_outbound_handshake(&mut manager, peer_id, 1);
    }
    let block_hash = BlockHash::from(hash_from_index(111_106));
    let txid = TxRelayId::Txid(txid_from_byte(106));
    manager
        .request_missing_blocks(111_106, &[block_hash])
        .expect("block request")
        .expect("getdata");
    manager
        .handle_message(
            111_106,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: hash_from_index(111_107),
            }])),
            20,
        )
        .expect("compact getdata");
    seed_duplicate_announcements(&mut manager, 111_106, 111_107, txid, 20);

    // Act
    let actions = manager
        .remove_peer_with_transaction_cleanup(111_106, 30)
        .expect("peer cleanup");

    // Assert
    assert_eq!(
        actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 111_106 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 111_107,
                relay_id: txid,
            }),
        ]
    );
    assert!(manager.peer_state(111_106).is_none());
    assert!(manager.peer_requested_blocks(111_106).is_err());
}

#[test]
fn phase111_compact_notfound_does_not_create_or_release_block_inflight_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(111_108, 10).expect("peer");
    complete_outbound_handshake(&mut manager, 111_108, 1);
    let block_hash = BlockHash::from(hash_from_index(111_108));
    manager
        .request_missing_blocks(111_108, &[block_hash])
        .expect("block request")
        .expect("getdata");

    // Act
    let actions = manager
        .handle_message(
            111_108,
            WireNetworkMessage::NotFound(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: block_hash.into(),
            }])),
            14,
        )
        .expect("compact notfound");

    // Assert
    assert!(actions.is_empty());
    assert_eq!(
        manager
            .peer_requested_blocks(111_108)
            .expect("requested blocks"),
        vec![block_hash]
    );
}

#[test]
fn ping_block_announcement_and_duplicate_add_paths_are_exercised() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(5).expect("peer");
    assert_eq!(
        manager
            .add_inbound_peer(5)
            .expect_err("duplicate peer")
            .to_string(),
        "peer already exists: 5",
    );
    assert_eq!(
        manager
            .add_outbound_peer(5, 1)
            .expect_err("duplicate peer")
            .to_string(),
        "peer already exists: 5",
    );

    let ping = manager.request_ping(5, 123).expect("ping");
    assert_eq!(ping, WireNetworkMessage::Ping { nonce: 123 });
    manager
        .handle_message(5, WireNetworkMessage::Pong { nonce: 123 }, 1)
        .expect("pong");
    assert!(
        manager
            .peer_state(5)
            .expect("state")
            .last_ping_nonce
            .is_none()
    );

    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };
    let inv_message = manager
        .announce_block(5, &block)
        .expect("announce")
        .expect("inv");
    assert!(matches!(
        inv_message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::Block
    ));

    manager
        .handle_message(5, WireNetworkMessage::SendHeaders, 2)
        .expect("sendheaders");
    let headers_message = manager
        .announce_block(5, &block)
        .expect("announce")
        .expect("headers");
    assert!(matches!(
        headers_message,
        WireNetworkMessage::Headers(HeadersMessage { headers }) if headers.len() == 1
    ));

    let transaction = open_bitcoin_primitives::Transaction::default();
    let announcement = manager
        .announce_transaction(5, &transaction)
        .expect("announce")
        .expect("message");
    assert!(matches!(
        announcement,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::Transaction
    ));

    manager.remove_peer(5).expect("remove peer");
    assert!(manager.peer_state(5).is_none());
}

#[test]
fn announce_block_with_action_emits_compact_block_for_valid_coinbase_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(50, 0).expect("peer");
    let block = announce_with_action_coinbase_block();
    let compact_nonce = 0x1122_3344_5566_7788_u64;

    // Act
    let message = manager
        .announce_block_with_action(
            50,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            compact_nonce,
        )
        .expect("announce")
        .expect("message");

    // Assert
    let WireNetworkMessage::CompactBlock(payload) = message else {
        panic!("expected CompactBlock, got {message:?}");
    };
    assert_eq!(payload.header, block.header);
    assert_eq!(payload.nonce, compact_nonce);
    assert_eq!(payload.prefilled_transactions.len(), 1);
    assert_eq!(payload.prefilled_transactions[0].index_delta, 0);
    assert!(payload.short_ids.is_empty());
}

#[test]
fn announce_block_with_action_emits_headers_when_action_is_headers() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(51, 0).expect("peer");
    // Peer prefers inv by default; action must still force Headers.
    assert!(
        !manager
            .peer_state(51)
            .expect("state")
            .remote_prefers_headers
    );
    let block = announce_with_action_coinbase_block();

    // Act
    let message = manager
        .announce_block_with_action(51, &block, CompactAnnouncementAction::AnnounceHeaders, 0)
        .expect("announce")
        .expect("message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Headers(HeadersMessage { headers }) if headers.len() == 1
            && headers[0] == block.header
    ));
}

#[test]
fn announce_block_with_action_emits_inventory_when_action_is_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(52, 0).expect("peer");
    manager
        .handle_message(52, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    assert!(
        manager
            .peer_state(52)
            .expect("state")
            .remote_prefers_headers
    );
    let block = announce_with_action_coinbase_block();
    let expected_hash = block_hash(&block.header);

    // Act
    let message = manager
        .announce_block_with_action(52, &block, CompactAnnouncementAction::AnnounceInventory, 0)
        .expect("announce")
        .expect("message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory.len() == 1
            && inventory[0].inventory_type == InventoryType::Block
            && inventory[0].object_hash == expected_hash.into()
    ));
}

#[test]
fn announce_block_with_action_suppress_returns_none() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(53, 0).expect("peer");
    let block = announce_with_action_coinbase_block();

    // Act
    let maybe_message = manager
        .announce_block_with_action(53, &block, CompactAnnouncementAction::Suppress, 0)
        .expect("announce");

    // Assert
    assert!(maybe_message.is_none());
}

#[test]
fn announce_block_with_action_unknown_peer_returns_error() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let block = announce_with_action_coinbase_block();

    // Act
    let error = manager
        .announce_block_with_action(99, &block, CompactAnnouncementAction::AnnounceHeaders, 0)
        .expect_err("unknown peer");

    // Assert
    assert_eq!(error, NetworkError::UnknownPeer(99));
}

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
fn announce_block_with_action_construction_failure_falls_back_to_inv() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(54, 0).expect("peer");
    assert!(
        !manager
            .peer_state(54)
            .expect("state")
            .remote_prefers_headers
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };

    // Act
    let message = manager
        .announce_block_with_action(
            54,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            7,
        )
        .expect("announce")
        .expect("fallback message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Inv(InventoryList { ref inventory })
        if inventory.len() == 1 && inventory[0].inventory_type == InventoryType::Block
    ));
    assert!(!matches!(message, WireNetworkMessage::CompactBlock(_)));
}

#[test]
fn announce_block_with_action_construction_failure_falls_back_to_headers() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(55, 0).expect("peer");
    manager
        .handle_message(55, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders");
    assert!(
        manager
            .peer_state(55)
            .expect("state")
            .remote_prefers_headers
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 4),
        transactions: Vec::new(),
    };

    // Act
    let message = manager
        .announce_block_with_action(
            55,
            &block,
            CompactAnnouncementAction::AnnounceCompactBlock,
            7,
        )
        .expect("announce")
        .expect("fallback message");

    // Assert
    assert!(matches!(
        message,
        WireNetworkMessage::Headers(HeadersMessage { ref headers }) if headers.len() == 1
    ));
    assert!(!matches!(message, WireNetworkMessage::CompactBlock(_)));
}

#[test]
fn inventory_requests_and_notfound_paths_cover_tx_and_block_modes() {
    let mut manager = relay_download_manager(true);
    add_relay_permissioned_inbound_peer(&mut manager, 6);
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Inv(InventoryList::default()), 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let txid_inv = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Transaction,
        object_hash: Hash32::from_byte_array([2_u8; 32]),
    }]);
    let txid_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([2_u8; 32])));
    let txid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(txid_inv), 1)
        .expect("txid inventory");
    assert_transaction_relay_request(&txid_actions, 6, txid_relay_id);

    manager
        .handle_message(6, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::WtxidRelay, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::SendHeaders, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Pong { nonce: 1 }, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let wtxid_inv = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::WitnessTransaction,
        object_hash: Hash32::from_byte_array([3_u8; 32]),
    }]);
    let wtxid_relay_id = TxRelayId::Wtxid(open_bitcoin_primitives::Wtxid::from(
        Hash32::from_byte_array([3_u8; 32]),
    ));
    let wtxid_actions = manager
        .handle_message(6, WireNetworkMessage::Inv(wtxid_inv), 2)
        .expect("wtxid inventory");
    assert_transaction_relay_request(&wtxid_actions, 6, wtxid_relay_id);
    let ignored_inventory = manager
        .handle_message(
            6,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::CompactBlock,
                object_hash: Hash32::from_byte_array([4_u8; 32]),
            }])),
            2,
        )
        .expect("ignored inventory");
    assert!(ignored_inventory.is_empty());

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 5);
    manager.seed_local_chain(&[ChainPosition::new(genesis.clone(), 0, 1, 0)]);
    let next = mined_header(open_bitcoin_consensus::block_hash(&genesis), 6);
    manager
        .handle_message(
            6,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![next.clone()],
            }),
            3,
        )
        .expect("headers");

    let not_found = InventoryList::new(vec![
        InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: Hash32::from_byte_array([2_u8; 32]),
        },
        InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: Hash32::from_byte_array([3_u8; 32]),
        },
        InventoryVector {
            inventory_type: InventoryType::Block,
            object_hash: open_bitcoin_consensus::block_hash(&next).into(),
        },
        InventoryVector {
            inventory_type: InventoryType::CompactBlock,
            object_hash: Hash32::from_byte_array([4_u8; 32]),
        },
    ]);
    manager
        .handle_message(6, WireNetworkMessage::NotFound(not_found), 4)
        .expect("notfound");
    let peer = manager.peer_state(6).expect("peer");
    let snapshot = manager.transaction_request_snapshot(6);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
    assert!(peer.requested_blocks.is_empty());
}

#[test]
fn received_tx_and_block_clear_requested_inventory() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(8).expect("peer");

    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    manager
        .handle_message(
            8,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            1,
        )
        .expect("txid inventory");
    manager
        .handle_message(8, WireNetworkMessage::WtxidRelay, 2)
        .expect("wtxidrelay");
    manager
        .handle_message(
            8,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            3,
        )
        .expect("wtxid inventory");

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 7);
    manager.seed_local_chain(&[ChainPosition::new(genesis.clone(), 0, 1, 0)]);
    let next = mined_header(open_bitcoin_consensus::block_hash(&genesis), 8);
    manager
        .handle_message(
            8,
            WireNetworkMessage::Headers(crate::HeadersMessage {
                headers: vec![next.clone()],
            }),
            4,
        )
        .expect("headers");

    // Act
    manager
        .handle_message(8, WireNetworkMessage::Tx(transaction), 5)
        .expect("transaction");
    manager
        .handle_message(
            8,
            WireNetworkMessage::Block(Block {
                header: next,
                transactions: Vec::new(),
            }),
            6,
        )
        .expect("block");

    // Assert
    let peer = manager.peer_state(8).expect("peer");
    let snapshot = manager.transaction_request_snapshot(8);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
    assert!(peer.requested_blocks.is_empty());
}

#[test]
fn getheaders_headers_tx_and_block_paths_are_explicit() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(7).expect("peer");

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 7);
    let genesis_position = ChainPosition::new(genesis.clone(), 0, 1, 0);
    manager.seed_local_chain(std::slice::from_ref(&genesis_position));

    let getheaders_actions = manager
        .handle_message(
            7,
            WireNetworkMessage::GetHeaders {
                locator: open_bitcoin_primitives::BlockLocator::default(),
                stop_hash: BlockHash::from_byte_array([0_u8; 32]),
            },
            1,
        )
        .expect("getheaders");
    assert!(matches!(
        getheaders_actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::Headers(HeadersMessage { headers }))]
        if headers.len() == 1
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![genesis.clone()],
                }),
                1,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let missing_parent = mined_header(BlockHash::from_byte_array([8_u8; 32]), 8);
    assert_eq!(
        manager
            .handle_message(
                7,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![missing_parent],
                }),
                2,
            )
            .expect_err("missing ancestor")
            .to_string(),
        format!(
            "missing header ancestor: {:?}",
            BlockHash::from_byte_array([8_u8; 32]).to_byte_array()
        ),
    );
    let invalid_pow_header = header(genesis_position.block_hash, 99);
    assert_eq!(
        manager
            .handle_message(
                7,
                WireNetworkMessage::Headers(crate::HeadersMessage {
                    headers: vec![invalid_pow_header],
                }),
                2,
            )
            .expect_err("invalid pow")
            .to_string(),
        "invalid header: high-hash (proof of work failed)",
    );
    let empty_headers = manager
        .handle_message(
            7,
            WireNetworkMessage::Headers(crate::HeadersMessage { headers: vec![] }),
            3,
        )
        .expect("empty headers");
    assert!(empty_headers.is_empty());

    let served = manager
        .handle_message(
            7,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Block,
                object_hash: genesis_position.block_hash.into(),
            }])),
            3,
        )
        .expect("getdata");
    assert!(matches!(served.as_slice(), [PeerAction::ServeInventory(_)]));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::NotFound(InventoryList::default()),
                3
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = open_bitcoin_consensus::transaction_txid(&transaction).expect("txid");
    let wtxid = open_bitcoin_consensus::transaction_wtxid(&transaction).expect("wtxid");
    let tx_actions = manager
        .handle_message(7, WireNetworkMessage::Tx(transaction), 4)
        .expect("tx");
    assert!(matches!(
        tx_actions.as_slice(),
        [
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup { .. }),
            PeerAction::ReceivedTransaction { .. },
        ]
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Tx(open_bitcoin_primitives::Transaction::default()),
                4,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let block = Block {
        header: genesis,
        transactions: Vec::new(),
    };
    let block_hash = open_bitcoin_consensus::block_hash(&block.header);
    let block_actions = manager
        .handle_message(7, WireNetworkMessage::Block(block), 5)
        .expect("block");
    assert!(matches!(
        block_actions.as_slice(),
        [PeerAction::ReceivedBlock(_)]
    ));
    assert_eq!(
        manager
            .handle_message(
                99,
                WireNetworkMessage::Block(Block {
                    header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 10),
                    transactions: Vec::new(),
                }),
                5,
            )
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let peer = manager.peer_state(7).expect("peer");
    let _ = txid;
    let _ = wtxid;
    let snapshot = manager.transaction_request_snapshot(7);
    assert_eq!(snapshot.in_flight_count, 0);
    assert_eq!(snapshot.candidate_count, 0);
    assert!(!peer.requested_blocks.contains(&block_hash));
}

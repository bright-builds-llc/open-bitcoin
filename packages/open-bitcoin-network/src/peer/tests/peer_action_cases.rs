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
fn deferred_relay_commands_remain_absent_from_peer_message_surface() {
    // Arrange
    let deferred_commands = [
        "mempool",
        "getcfilters",
        "cfilter",
        "getcfheaders",
        "getcfcheckpt",
        "filterload",
        "filteradd",
        "filterclear",
    ];

    for command in deferred_commands {
        let command = MessageCommand::new(command).expect("test command should be valid");

        // Act
        let result = WireNetworkMessage::decode_payload(&command, &[]);

        // Assert
        assert!(matches!(
            result,
            Err(NetworkError::UnknownCommand(rejected)) if rejected == command.as_str()
        ));
    }
}

#[test]
fn peer_manager_active_tip_change_resets_both_reject_evidence_domains() {
    // Arrange
    let mut manager =
        PeerManager::with_reject_evidence_tweak(local_config(), RejectEvidenceTweak::new(11));
    let hard_reject = Wtxid::from(Hash32::from_byte_array([91_u8; 32]));
    let reconsiderable = Wtxid::from(Hash32::from_byte_array([92_u8; 32]));
    manager.record_hard_reject(hard_reject);
    manager.record_reconsiderable_transaction(reconsiderable);

    // Act
    manager.on_active_tip_changed(RejectEvidenceTweak::new(12));

    // Assert
    assert!(!manager.hard_reject_contains(hard_reject));
    assert!(!manager.reconsiderable_transaction_contains(reconsiderable));
}

#[test]
fn peer_manager_parent_request_does_not_hash_txid_as_hard_reject_wtxid() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 221);
    add_relay_outbound_peer(&mut manager, 222);
    add_relay_outbound_peer(&mut manager, 223);
    let local_transaction = open_bitcoin_primitives::Transaction::default();
    let local_txid = transaction_txid(&local_transaction).expect("txid");
    let rejected_txid = txid_from_byte(102);
    let rejected_wtxid = Wtxid::from(Hash32::from(rejected_txid));
    let mempool_txid = txid_from_byte(103);
    manager
        .note_local_transaction(&local_transaction)
        .expect("local transaction");
    manager.record_hard_reject(rejected_wtxid);
    manager.note_mempool_known(TxRelayId::Txid(mempool_txid));

    // Act
    let already_have_actions = manager
        .request_orphan_parent(221, local_txid, 1)
        .expect("already-have parent request");
    let unresolved_reject_actions = manager
        .request_orphan_parent(222, rejected_txid, 2)
        .expect("unresolved reject parent request");
    let mempool_known_actions = manager
        .request_orphan_parent(223, mempool_txid, 3)
        .expect("mempool-known parent request");

    // Assert
    assert_eq!(
        already_have_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressAlreadyHave {
                peer_id: 221,
                relay_id: TxRelayId::Txid(local_txid),
            },
        )],
    );
    assert_eq!(
        unresolved_reject_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::RequestGetData {
                peer_id: 222,
                relay_id: TxRelayId::Txid(rejected_txid),
            },
        )],
    );
    assert_eq!(
        mempool_known_actions,
        vec![PeerAction::TransactionRelay(TxDownloadAction::Suppress {
            peer_id: 223,
            relay_id: TxRelayId::Txid(mempool_txid),
            reason: TxDownloadSuppressionReason::MempoolKnown,
        })],
    );
}

#[test]
fn helper_methods_and_unknown_peer_errors_are_covered() {
    let mut manager = PeerManager::new(local_config());
    assert!(manager.peer_state(99).is_none());
    assert_eq!(
        manager
            .peer_requested_blocks(99)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .remove_peer(99)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Version(Default::default()), 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .request_ping(99, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    let block = Block {
        header: mined_header(BlockHash::from_byte_array([0_u8; 32]), 1),
        transactions: Vec::new(),
    };
    assert_eq!(
        manager
            .announce_block(99, &block)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .announce_transaction(99, &open_bitcoin_primitives::Transaction::default())
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::Verack, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );
    assert_eq!(
        manager
            .handle_message(99, WireNetworkMessage::GetAddr, 1)
            .expect_err("unknown peer")
            .to_string(),
        "unknown peer: 99",
    );

    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 2);
    let position = ChainPosition::new(genesis, 0, 1, 0);
    manager.seed_local_chain(std::slice::from_ref(&position));
    manager.note_local_position(&position);
    manager
        .note_local_transaction(&open_bitcoin_primitives::Transaction::default())
        .expect("local transaction");
    assert_eq!(manager.header_store().best_height(), 0);

    let mut restored_headers = HeaderStore::default();
    restored_headers.seed_from_chain(std::slice::from_ref(&position));
    let mut restored_manager = PeerManager::new(local_config());
    restored_manager.seed_header_store(restored_headers);
    assert_eq!(restored_manager.header_store().best_height(), 0);
}

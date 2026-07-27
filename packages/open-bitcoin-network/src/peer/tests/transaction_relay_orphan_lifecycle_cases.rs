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
fn peer_manager_transaction_relay_semantic_reject_evidence_suppresses_without_punishment() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 207);
    add_relay_outbound_peer(&mut manager, 208);
    add_relay_outbound_peer(&mut manager, 209);
    manager
        .handle_message(208, WireNetworkMessage::WtxidRelay, 0)
        .expect("hard-reject peer wtxidrelay");
    manager
        .handle_message(209, WireNetworkMessage::WtxidRelay, 0)
        .expect("reconsiderable peer wtxidrelay");
    let local_transaction = open_bitcoin_primitives::Transaction::default();
    let local_txid = transaction_txid(&local_transaction).expect("txid");
    let rejected_wtxid = Wtxid::from(Hash32::from_byte_array([88_u8; 32]));
    let reconsiderable_wtxid = Wtxid::from(Hash32::from_byte_array([89_u8; 32]));
    let package_fingerprint = [90_u8; 32];
    manager
        .note_local_transaction(&local_transaction)
        .expect("local transaction");
    assert!(!manager.reconsiderable_package_contains(package_fingerprint));
    manager.record_hard_reject(rejected_wtxid);
    manager.record_reconsiderable_transaction(reconsiderable_wtxid);
    manager.record_reconsiderable_package(package_fingerprint);

    // Act
    let already_have_actions = manager
        .handle_message(
            207,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(local_txid))),
            1,
        )
        .expect("already-have inventory");
    let hard_reject_actions = manager
        .handle_message(
            208,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(
                rejected_wtxid,
            ))),
            2,
        )
        .expect("hard-reject inventory");
    let reconsiderable_actions = manager
        .handle_message(
            209,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(
                reconsiderable_wtxid,
            ))),
            3,
        )
        .expect("reconsiderable inventory");

    // Assert
    assert_eq!(
        already_have_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressAlreadyHave {
                peer_id: 207,
                relay_id: TxRelayId::Txid(local_txid),
            },
        )],
    );
    assert_eq!(
        hard_reject_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRecentReject {
                peer_id: 208,
                relay_id: TxRelayId::Wtxid(rejected_wtxid),
            },
        )],
    );
    assert_eq!(
        reconsiderable_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRecentReject {
                peer_id: 209,
                relay_id: TxRelayId::Wtxid(reconsiderable_wtxid),
            },
        )],
    );
    assert!(manager.reconsiderable_package_contains(package_fingerprint));
}

#[test]
fn peer_manager_orphan_parent_request_uses_transaction_scheduler() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 219);
    let parent_txid = txid_from_byte(101);

    // Act
    let actions = manager
        .request_orphan_parent(219, parent_txid, 10)
        .expect("parent request");

    // Assert
    assert_transaction_relay_request(&actions, 219, TxRelayId::Txid(parent_txid));
    assert_eq!(manager.transaction_request_snapshot(219).in_flight_count, 1);
}

#[test]
fn peer_manager_transaction_relay_orphan_parent_request_uses_relay_download_eligibility() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(235).expect("peer");
    let parent_txid = txid_from_byte(113);

    // Act
    let actions = manager
        .request_orphan_parent(235, parent_txid, 10)
        .expect("parent request");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        235,
        TxRelayId::Txid(parent_txid),
        TxDownloadSuppressionReason::RelayDisabled,
    );
    assert_eq!(manager.transaction_request_snapshot(235).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(235).candidate_count, 0);
}

#[test]
fn peer_manager_orphan_parent_request_respects_inflight_cap() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 220);

    // Act
    for index in 0..PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER {
        let parent_txid = txid_from_byte(index as u8);
        let actions = manager
            .request_orphan_parent(220, parent_txid, index as i64)
            .expect("parent request");
        assert_transaction_relay_request(&actions, 220, TxRelayId::Txid(parent_txid));
    }
    let capped_txid = txid_from_byte(250);
    let capped_actions = manager
        .request_orphan_parent(220, capped_txid, 500)
        .expect("capped parent request");

    // Assert
    assert_eq!(
        manager.transaction_request_snapshot(220).in_flight_count,
        PHASE101_MAX_TX_REQUESTS_IN_FLIGHT_PER_PEER
    );
    assert_eq!(
        capped_actions,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::SuppressRequestCap {
                peer_id: 220,
                relay_id: TxRelayId::Txid(capped_txid),
            },
        )],
    );
}

#[test]
fn peer_manager_orphan_parent_request_counts_toward_resource_governance() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_permissioned_inbound_peer(&mut manager, 224);
    let parent_txid = txid_from_byte(104);

    // Act
    let actions = manager
        .request_orphan_parent(224, parent_txid, 10)
        .expect("parent request");
    let snapshot = manager.transaction_request_snapshot(224);
    let candidates = manager.eviction_candidate_inputs();
    let [candidate] = candidates.as_slice() else {
        panic!("expected one eviction candidate");
    };
    let decision = ResourceGovernancePolicy::default().decide_request(RequestPressureInput {
        requested_txids_in_flight: PHASE94_MAX_INBOUND_TX_REQUESTS_PER_PEER
            .saturating_add(snapshot.in_flight_count),
        ..RequestPressureInput::default()
    });

    // Assert
    assert_transaction_relay_request(&actions, 224, TxRelayId::Txid(parent_txid));
    assert_eq!(snapshot.in_flight_count, 1);
    assert_eq!(candidate.requested_inventory_count, 1);
    assert!(
        matches!(decision, ResourceGovernanceDecision::Disconnect(event) if event.label == "request_cap_reached"),
        "expected orphan parent request to contribute to resource governance cap"
    );
}

#[test]
fn peer_manager_orphan_parent_request_rejects_unknown_peer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let parent_txid = txid_from_byte(105);

    // Act
    let error = manager
        .request_orphan_parent(225, parent_txid, 10)
        .expect_err("unknown peer should fail");

    // Assert
    assert_eq!(error, NetworkError::UnknownPeer(225));
}

#[test]
fn peer_manager_transaction_relay_notfound_timeout_and_disconnect_cleanup_fallback() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 209..=214 {
        add_relay_outbound_peer(&mut manager, peer_id);
    }
    let notfound_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([91_u8; 32])));
    let timeout_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([92_u8; 32])));
    let disconnect_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([93_u8; 32])));
    seed_duplicate_announcements(&mut manager, 209, 210, notfound_relay_id, 1);
    seed_duplicate_announcements(&mut manager, 211, 212, timeout_relay_id, 10);
    seed_duplicate_announcements(&mut manager, 213, 214, disconnect_relay_id, 20);

    // Act
    let notfound_actions = manager
        .handle_message(
            209,
            WireNetworkMessage::NotFound(transaction_relay_inventory(notfound_relay_id)),
            30,
        )
        .expect("notfound");
    let timeout_actions =
        manager.expire_transaction_requests(10 + PHASE101_GETDATA_TX_INTERVAL_SECONDS);
    let disconnect_actions = manager
        .remove_peer_with_transaction_cleanup(213, 40)
        .expect("disconnect cleanup");

    // Assert
    assert_eq!(
        notfound_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::NotFoundCleanup {
                peer_id: 209,
                relay_id: notfound_relay_id,
            }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 210,
                relay_id: notfound_relay_id,
            }),
        ],
    );
    assert_eq!(
        timeout_actions,
        vec![
            (
                211,
                PeerAction::TransactionRelay(TxDownloadAction::RequestExpired {
                    peer_id: 211,
                    relay_id: timeout_relay_id,
                }),
            ),
            (
                212,
                PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                    peer_id: 212,
                    relay_id: timeout_relay_id,
                }),
            ),
        ],
    );
    assert_eq!(
        disconnect_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::PeerCleanup { peer_id: 213 }),
            PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                peer_id: 214,
                relay_id: disconnect_relay_id,
            }),
        ],
    );
    assert!(manager.peer_state(213).is_none());
}

#[test]
fn peer_manager_transaction_relay_received_transaction_cleanup_waits_for_admission() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 215..=217 {
        add_relay_outbound_peer(&mut manager, peer_id);
    }
    manager
        .handle_message(217, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    manager
        .handle_message(
            215,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("request inventory");

    // Act
    let received_actions = manager
        .handle_message(215, WireNetworkMessage::Tx(transaction.clone()), 3)
        .expect("received transaction");
    let txid_again = manager
        .handle_message(
            216,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            4,
        )
        .expect("txid inventory after receipt");
    let wtxid_again = manager
        .handle_message(
            217,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            5,
        )
        .expect("wtxid inventory after receipt");

    // Assert
    assert_eq!(
        received_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup {
                peer_id: 215,
                txid,
                wtxid,
            }),
            PeerAction::ReceivedTransaction {
                transaction,
                provenance: ReceivedTransactionProvenance {
                    delivered_by: 215,
                    announcers: vec![215],
                },
            },
        ],
    );
    assert_eq!(
        txid_again,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::RequestGetData {
                peer_id: 216,
                relay_id: TxRelayId::Txid(txid),
            },
        )],
    );
    assert_eq!(
        wtxid_again,
        vec![PeerAction::TransactionRelay(
            TxDownloadAction::RequestGetData {
                peer_id: 217,
                relay_id: TxRelayId::Wtxid(wtxid),
            },
        )],
    );
}

#[test]
fn peer_manager_orphan_owner_routes_late_inventory_and_disconnects_announcers_coherently() {
    // Arrange
    let mut manager = relay_download_manager(true);
    for peer_id in 230..=232 {
        add_relay_outbound_peer(&mut manager, peer_id);
    }
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let _ = manager.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction,
            txid,
            wtxid,
            missing_parents: vec![txid_from_byte(99)],
            now_unix_seconds: 2,
        },
        ReceivedTransactionProvenance {
            delivered_by: 230,
            announcers: vec![230, 231],
        },
    );

    // Act
    manager
        .remove_peer_with_transaction_cleanup(230, 3)
        .expect("first announcer disconnect");
    let late_inventory = manager
        .handle_message(
            232,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            4,
        )
        .expect("late orphan inventory");
    manager
        .remove_peer_with_transaction_cleanup(231, 5)
        .expect("second announcer disconnect");

    // Assert
    assert!(late_inventory.is_empty());
    assert_eq!(manager.orphan_count(), 1);
    assert_eq!(manager.orphan_peer_len(230), 0);
    assert_eq!(manager.orphan_peer_len(231), 0);
    assert_eq!(manager.orphan_peer_len(232), 1);
    manager
        .remove_peer_with_transaction_cleanup(232, 6)
        .expect("late announcer disconnect");
    assert_eq!(manager.orphan_count(), 0);
}

#[test]
fn peer_manager_orphan_owner_delegates_reconsideration_feedback_and_expiry() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 4,
        max_orphans_per_peer: 4,
        max_announcers_per_orphan: 2,
        max_retained_bytes: crate::PHASE133_MAX_ORPHAN_RETAINED_BYTES,
        orphan_ttl_seconds: 2,
        max_reconsiderations_per_parent: 1,
    });
    let parent_txid = txid_from_byte(180);
    let child_wtxids = [wtxid_from_byte(181), wtxid_from_byte(182)];
    for (index, child_wtxid) in child_wtxids.into_iter().enumerate() {
        let _ = manager.stage_missing_parent_with_provenance(
            OrphanStageInput {
                transaction: Transaction {
                    version: index as i32,
                    ..Transaction::default()
                },
                txid: txid_from_byte(181 + index as u8),
                wtxid: child_wtxid,
                missing_parents: vec![parent_txid],
                now_unix_seconds: 0,
            },
            ReceivedTransactionProvenance {
                delivered_by: 240,
                announcers: vec![240],
            },
        );
    }

    // Act
    let first = manager.reconsider_orphans_after_parent(parent_txid, 1);
    let second = manager.drain_pending_orphan_reconsiderations(1);
    let terminal = manager.record_orphan_reconsideration_outcome(
        child_wtxids[0],
        OrphanReconsiderationStatus::Rejected,
    );
    let expired = manager.expire_orphans(2);

    // Assert
    assert_eq!(first.len(), 1);
    assert_eq!(second.len(), 1);
    assert_eq!(terminal.len(), 1);
    assert_eq!(expired.len(), 1);
    assert_eq!(manager.orphan_count(), 0);
}

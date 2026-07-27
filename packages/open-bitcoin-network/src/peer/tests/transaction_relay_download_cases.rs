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
fn announce_transaction_uses_wtxidrelay_when_peer_negotiates_it() {
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(4).expect("peer");
    manager
        .handle_message(
            4,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            20,
        )
        .expect("version");
    manager
        .handle_message(4, WireNetworkMessage::WtxidRelay, 20)
        .expect("wtxidrelay");

    let transaction = open_bitcoin_primitives::Transaction::default();
    let announcement = manager
        .announce_transaction(4, &transaction)
        .expect("announce")
        .expect("message");

    assert!(matches!(
        announcement,
        WireNetworkMessage::Inv(InventoryList { inventory })
        if inventory[0].inventory_type == InventoryType::WitnessTransaction
    ));
}

#[test]
fn scoped_relay_permission_effects_remain_policy_only_for_transaction_paths() {
    // Arrange
    let permission_decision = permission_decision(["in", "relay", "forcerelay", "mempool"]);
    assert!(active_permission_labels(&permission_decision).is_empty());
    assert_eq!(
        relay_permission_labels(&permission_decision),
        vec![
            "transaction_relay_policy_input",
            "force_relay_policy_input",
            "mempool_policy_input",
        ],
    );
    assert_eq!(
        inactive_permission_labels(&permission_decision),
        Vec::<&'static str>::new(),
    );
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let mut manager = relay_download_manager(true);
    manager
        .add_inbound_peer_record(permissioned_inbound_record(91, permission_decision))
        .expect("permissioned inbound peer should be added");

    // Act
    let txid_inventory_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            1,
        )
        .expect("transaction inventory");
    let wtxidrelay_actions = manager
        .handle_message(91, WireNetworkMessage::WtxidRelay, 2)
        .expect("wtxidrelay");
    let wtxid_inventory_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            3,
        )
        .expect("witness transaction inventory");
    let tx_actions = manager
        .handle_message(91, WireNetworkMessage::Tx(transaction.clone()), 4)
        .expect("transaction");
    let getdata_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            5,
        )
        .expect("getdata");
    let wtxid_getdata_actions = manager
        .handle_message(
            91,
            WireNetworkMessage::GetData(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::WitnessTransaction,
                object_hash: wtxid.into(),
            }])),
            6,
        )
        .expect("witness transaction getdata");

    // Assert
    assert_transaction_relay_request(&txid_inventory_actions, 91, TxRelayId::Txid(txid));
    assert!(wtxidrelay_actions.is_empty());
    assert_transaction_relay_request(&wtxid_inventory_actions, 91, TxRelayId::Wtxid(wtxid));
    assert_eq!(
        tx_actions,
        vec![
            PeerAction::TransactionRelay(TxDownloadAction::ReceivedTxCleanup {
                peer_id: 91,
                txid,
                wtxid,
            }),
            PeerAction::ReceivedTransaction {
                transaction,
                provenance: ReceivedTransactionProvenance {
                    delivered_by: 91,
                    announcers: vec![91],
                },
            },
        ],
    );
    assert_eq!(
        getdata_actions,
        vec![PeerAction::ServeInventory(vec![InventoryVector {
            inventory_type: InventoryType::Transaction,
            object_hash: txid.into(),
        }])]
    );
    assert_eq!(
        wtxid_getdata_actions,
        vec![PeerAction::ServeInventory(vec![InventoryVector {
            inventory_type: InventoryType::WitnessTransaction,
            object_hash: wtxid.into(),
        }])]
    );
}

#[test]
fn peer_manager_transaction_relay_default_off_download_suppresses_without_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(226).expect("peer");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let actions = manager
        .handle_message(
            226,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("default-off transaction inventory");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        226,
        TxRelayId::Txid(txid),
        TxDownloadSuppressionReason::RelayDisabled,
    );
    let snapshot = manager.transaction_request_snapshot(226);
    assert_eq!(snapshot.candidate_count, 0);
    assert_eq!(snapshot.in_flight_count, 0);
}

#[test]
fn peer_manager_transaction_relay_enabled_outbound_and_permissioned_inbound_schedule_downloads() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 227);
    add_relay_permissioned_inbound_peer(&mut manager, 228);
    let txid = txid_from_byte(106);
    let wtxid = open_bitcoin_primitives::Wtxid::from_byte_array([107; 32]);
    manager
        .handle_message(228, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");

    // Act
    let txid_actions = manager
        .handle_message(
            227,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("outbound txid inventory");
    let wtxid_actions = manager
        .handle_message(
            228,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            3,
        )
        .expect("permissioned inbound wtxid inventory");

    // Assert
    assert_transaction_relay_request(&txid_actions, 227, TxRelayId::Txid(txid));
    assert_transaction_relay_request(&wtxid_actions, 228, TxRelayId::Wtxid(wtxid));
}

#[test]
fn peer_manager_transaction_relay_enabled_inbound_classes_require_scoped_relay_permission() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager
        .add_inbound_peer(229)
        .expect("ordinary inbound peer");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            230,
            protected_permission_decision(),
        ))
        .expect("protected inbound peer");
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            231,
            permission_decision(["in", "download"]),
        ))
        .expect("permissioned inbound peer without relay scope");

    // Act
    let ordinary_actions = manager
        .handle_message(
            229,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(108),
            ))),
            1,
        )
        .expect("ordinary inbound inventory");
    let protected_actions = manager
        .handle_message(
            230,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(109),
            ))),
            2,
        )
        .expect("protected inbound inventory");
    let no_relay_actions = manager
        .handle_message(
            231,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(
                txid_from_byte(110),
            ))),
            3,
        )
        .expect("permissioned inbound without relay inventory");

    // Assert
    assert_transaction_relay_suppression(
        &ordinary_actions,
        229,
        TxRelayId::Txid(txid_from_byte(108)),
        TxDownloadSuppressionReason::PermissionRequired,
    );
    assert_transaction_relay_suppression(
        &protected_actions,
        230,
        TxRelayId::Txid(txid_from_byte(109)),
        TxDownloadSuppressionReason::ProtectedNotRelay,
    );
    assert_transaction_relay_suppression(
        &no_relay_actions,
        231,
        TxRelayId::Txid(txid_from_byte(110)),
        TxDownloadSuppressionReason::PermissionRequired,
    );
}

#[test]
fn peer_manager_transaction_relay_inbound_serving_disabled_blocks_permissioned_downloads() {
    // Arrange
    let mut manager = relay_download_manager(false);
    add_relay_permissioned_inbound_peer(&mut manager, 232);
    let txid = txid_from_byte(111);

    // Act
    let actions = manager
        .handle_message(
            232,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("permissioned inbound inventory");

    // Assert
    assert_transaction_relay_suppression(
        &actions,
        232,
        TxRelayId::Txid(txid),
        TxDownloadSuppressionReason::InboundServingRequired,
    );
}

#[test]
fn peer_manager_transaction_relay_ineligible_first_announcement_does_not_block_eligible_second() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager
        .add_inbound_peer(233)
        .expect("ordinary inbound peer");
    add_relay_outbound_peer(&mut manager, 234);
    let relay_id = TxRelayId::Txid(txid_from_byte(112));

    // Act
    let first_actions = manager
        .handle_message(
            233,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            1,
        )
        .expect("ordinary inbound inventory");
    let second_actions = manager
        .handle_message(
            234,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            2,
        )
        .expect("outbound inventory");

    // Assert
    assert_transaction_relay_suppression(
        &first_actions,
        233,
        relay_id,
        TxDownloadSuppressionReason::PermissionRequired,
    );
    assert_transaction_relay_request(&second_actions, 234, relay_id);
    assert_eq!(manager.transaction_request_snapshot(233).candidate_count, 0);
    assert_eq!(manager.transaction_request_snapshot(233).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(234).in_flight_count, 1);
}

#[test]
fn peer_manager_transaction_relay_txid_inv_emits_typed_request_action() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 201);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let actions = manager
        .handle_message(
            201,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            1,
        )
        .expect("txid inventory");

    // Assert
    assert_transaction_relay_request(&actions, 201, TxRelayId::Txid(txid));
}

#[test]
fn peer_manager_transaction_relay_wtxid_inv_emits_typed_request_action() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 202);
    manager
        .handle_message(202, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    // Act
    let actions = manager
        .handle_message(
            202,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            2,
        )
        .expect("wtxid inventory");

    // Assert
    assert_transaction_relay_request(&actions, 202, TxRelayId::Wtxid(wtxid));
}

#[test]
fn peer_manager_transaction_relay_mismatch_emits_suppression_without_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(203).expect("txid-only peer");
    manager.add_inbound_peer(204).expect("wtxid peer");
    manager
        .handle_message(204, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    // Act
    let txid_to_wtxid_peer = manager
        .handle_message(
            204,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(txid))),
            2,
        )
        .expect("mismatched txid inventory");
    let wtxid_to_txid_peer = manager
        .handle_message(
            203,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(wtxid))),
            3,
        )
        .expect("mismatched wtxid inventory");

    // Assert
    assert_transaction_relay_identity_mismatch(&txid_to_wtxid_peer, 204);
    assert_transaction_relay_identity_mismatch(&wtxid_to_txid_peer, 203);
    assert_eq!(manager.transaction_request_snapshot(204).in_flight_count, 0);
    assert_eq!(manager.transaction_request_snapshot(203).in_flight_count, 0);
}

#[test]
fn peer_manager_transaction_relay_duplicate_inv_suppresses_second_getdata_but_keeps_fallback() {
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 205);
    add_relay_outbound_peer(&mut manager, 206);
    let transaction = open_bitcoin_primitives::Transaction::default();
    let relay_id = TxRelayId::Txid(transaction_txid(&transaction).expect("txid"));

    // Act
    let first_actions = manager
        .handle_message(
            205,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            1,
        )
        .expect("first inventory");
    let duplicate_actions = manager
        .handle_message(
            206,
            WireNetworkMessage::Inv(transaction_relay_inventory(relay_id)),
            2,
        )
        .expect("duplicate inventory");
    let fallback_actions =
        manager.expire_transaction_requests(1 + PHASE101_GETDATA_TX_INTERVAL_SECONDS);

    // Assert
    assert_transaction_relay_request(&first_actions, 205, relay_id);
    assert_transaction_relay_duplicate(&duplicate_actions, 206, relay_id);
    assert_eq!(
        fallback_actions,
        vec![
            (
                205,
                PeerAction::TransactionRelay(TxDownloadAction::RequestExpired {
                    peer_id: 205,
                    relay_id,
                }),
            ),
            (
                206,
                PeerAction::TransactionRelay(TxDownloadAction::FallbackRequest {
                    peer_id: 206,
                    relay_id,
                }),
            ),
        ],
    );
}

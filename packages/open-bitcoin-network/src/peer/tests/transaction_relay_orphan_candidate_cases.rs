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
fn peer_manager_orphan_owner_constructs_and_advances_same_peer_candidates() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 4,
        max_orphans_per_peer: 4,
        max_announcers_per_orphan: 2,
        max_retained_bytes: crate::PHASE133_MAX_ORPHAN_RETAINED_BYTES,
        orphan_ttl_seconds: 120,
        max_reconsiderations_per_parent: 2,
    });
    let parent = Transaction {
        version: 10,
        ..Transaction::default()
    };
    let parent_txid = txid_from_byte(190);
    let parent_wtxid = wtxid_from_byte(191);
    manager.record_reconsiderable_transaction(parent_wtxid);
    for (index, child_version) in [20, 21].into_iter().enumerate() {
        let _ = manager.stage_missing_parent_with_provenance(
            OrphanStageInput {
                transaction: Transaction {
                    version: child_version,
                    ..Transaction::default()
                },
                txid: txid_from_byte(192 + index as u8),
                wtxid: wtxid_from_byte(194 + index as u8),
                missing_parents: vec![parent_txid],
                now_unix_seconds: index as i64,
            },
            ReceivedTransactionProvenance {
                delivered_by: 241,
                announcers: vec![241],
            },
        );
    }

    // Act
    let first = manager
        .begin_same_peer_candidate(parent.clone(), parent_txid, parent_wtxid, 241)
        .expect("newest child candidate");
    let second = manager
        .advance_same_peer_candidate(parent_wtxid, 241)
        .expect("older child candidate");
    let (first_members, first_origins) = first.into_ordered_parts();
    let (second_members, second_origins) = second.into_ordered_parts();

    // Assert
    assert_eq!(
        first_members,
        [
            parent.clone(),
            Transaction {
                version: 21,
                ..Transaction::default()
            }
        ]
    );
    assert_eq!(
        second_members,
        [
            parent,
            Transaction {
                version: 20,
                ..Transaction::default()
            }
        ]
    );
    assert_eq!(first_origins, [241, 241]);
    assert_eq!(second_origins, [241, 241]);
}

#[test]
fn peer_manager_transaction_relay_received_transaction_mismatch_does_not_satisfy_unrelated_request()
{
    // Arrange
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 218);
    let requested_relay_id = TxRelayId::Txid(Txid::from(Hash32::from_byte_array([94_u8; 32])));
    manager
        .handle_message(
            218,
            WireNetworkMessage::Inv(transaction_relay_inventory(requested_relay_id)),
            1,
        )
        .expect("request inventory");
    let unrelated_transaction = open_bitcoin_primitives::Transaction::default();

    // Act
    let actions = manager
        .handle_message(218, WireNetworkMessage::Tx(unrelated_transaction), 2)
        .expect("unrelated transaction");

    // Assert
    assert_transaction_relay_identity_mismatch(&actions, 218);
    assert!(
        !actions
            .iter()
            .any(|action| matches!(action, PeerAction::ReceivedTransaction { .. }))
    );
    assert_eq!(manager.transaction_request_snapshot(218).in_flight_count, 1);
}

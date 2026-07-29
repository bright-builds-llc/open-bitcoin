// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use std::collections::BTreeSet;

use super::*;

fn alias_identity(txid_byte: u8, wtxid_byte: u8) -> PeerTransactionIdentity {
    PeerTransactionIdentity::new(txid_from_byte(txid_byte), wtxid_from_byte(wtxid_byte))
}

fn alias_cursor_manager(
    canonical: PeerTransactionIdentity,
    stored_aliases: &[PeerTransactionIdentity],
) -> PeerManager {
    let parent = identity(100);
    let sibling = identity(101);
    let mut manager = relay_download_manager(true);
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 8,
        max_orphans_per_peer: 8,
        max_reconsiderations_per_parent: 8,
        ..OrphanPolicy::default()
    });
    manager.record_reconsiderable_transaction(parent.wtxid());
    for (index, stored_alias) in stored_aliases.iter().copied().enumerate() {
        stage_orphan(
            &mut manager,
            20_000,
            stored_alias,
            parent.txid(),
            i64::try_from(index).expect("small fixture index"),
        );
    }
    stage_orphan(
        &mut manager,
        20_000,
        sibling,
        parent.txid(),
        i64::try_from(stored_aliases.len()).expect("small fixture length"),
    );
    let _ = manager
        .begin_same_peer_candidate(
            Transaction {
                version: 100,
                ..Transaction::default()
            },
            parent.txid(),
            parent.wtxid(),
            20_000,
        )
        .expect("candidate cursor should retain the alias suffix");
    assert_eq!(manager.debug_candidate_cursor_count(), 1);

    for (index, identity) in std::iter::once(canonical)
        .chain(stored_aliases.iter().copied())
        .enumerate()
    {
        let peer_id = 21_000 + u64::try_from(index).expect("small fixture index");
        add_relay_outbound_peer(&mut manager, peer_id);
        manager
            .handle_message(
                peer_id,
                WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(
                    identity.wtxid(),
                ))),
                i64::try_from(index + 1).expect("small fixture index"),
            )
            .expect("alias request should be tracked");
    }

    manager
}

#[test]
fn txid_teardown_removes_stored_orphan_wtxid_and_all_alias_cursors() {
    // Arrange
    let canonical = alias_identity(110, 111);
    let stored_alias = alias_identity(110, 112);
    let mut manager = alias_cursor_manager(canonical, &[stored_alias]);
    let prepared = manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![canonical]))
        .expect("canonical teardown should resolve the stored alias");

    // Act
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(manager.orphan_count(), 1);
    assert_eq!(manager.debug_candidate_cursor_count(), 0);
    assert_eq!(
        manager.transaction_request_snapshot(21_000).in_flight_count,
        0
    );
    assert_eq!(
        manager.transaction_request_snapshot(21_001).in_flight_count,
        0
    );
    assert_eq!(
        manager.mempool_lifecycle_mismatch_count(&BTreeSet::new()),
        0
    );
}

#[test]
fn txid_teardown_removes_every_stored_wtxid_alias() {
    // Arrange
    let canonical = alias_identity(120, 121);
    let first_alias = alias_identity(120, 122);
    let second_alias = alias_identity(120, 123);
    let mut manager = alias_cursor_manager(canonical, &[first_alias, second_alias]);
    let prepared = manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![canonical]))
        .expect("canonical teardown should resolve every stored alias");

    // Act
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(manager.orphan_count(), 1);
    assert_eq!(manager.debug_candidate_cursor_count(), 0);
    for peer_id in 21_000..=21_002 {
        assert_eq!(
            manager
                .transaction_request_snapshot(peer_id)
                .in_flight_count,
            0
        );
    }
}

#[test]
fn reconciliation_detects_cursor_for_removed_txid_alias() {
    // Arrange
    let canonical = alias_identity(130, 131);
    let stored_alias = alias_identity(130, 132);
    let mut manager = PeerManager::new(local_config());
    let admission = manager
        .prepare_transaction_lifecycle(lifecycle_input(vec![canonical], Vec::new()))
        .expect("canonical admission should prepare");
    manager.apply_prepared_transaction_lifecycle(admission);
    let parent = identity(133);
    let sibling = identity(134);
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 4,
        max_orphans_per_peer: 4,
        max_reconsiderations_per_parent: 2,
        ..OrphanPolicy::default()
    });
    manager.record_reconsiderable_transaction(parent.wtxid());
    stage_orphan(&mut manager, 22_000, stored_alias, parent.txid(), 1);
    stage_orphan(&mut manager, 22_000, sibling, parent.txid(), 2);
    let _ = manager
        .begin_same_peer_candidate(
            Transaction {
                version: 133,
                ..Transaction::default()
            },
            parent.txid(),
            parent.wtxid(),
            22_000,
        )
        .expect("candidate cursor should retain the alias suffix");
    manager
        .orphanage
        .remove_orphan_without_candidate_scan(stored_alias.wtxid());
    let canonical_members = BTreeSet::from([canonical]);

    // Act
    let mismatch_count = manager.mempool_lifecycle_mismatch_count(&canonical_members);

    // Assert
    assert_eq!(mismatch_count, 1);
}

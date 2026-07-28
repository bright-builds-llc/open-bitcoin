// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use std::collections::BTreeSet;

use super::*;

#[test]
fn lifecycle_reconciliation_snapshot_matches_clean_canonical_membership() {
    // Arrange
    let target = identity(8);
    let mut manager = PeerManager::new(local_config());
    let admission = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            vec![target],
            Vec::new(),
            vec![AcceptedPeerPackageFingerprint::new([9; 32], vec![target])],
        ))
        .expect("bounded admission should prepare");
    manager.apply_prepared_transaction_lifecycle(admission);
    let canonical = BTreeSet::from([target]);

    // Act
    let snapshot = manager.mempool_lifecycle_snapshot();
    let mismatch_count = manager.mempool_lifecycle_mismatch_count(&canonical);

    // Assert
    assert_eq!(snapshot.requests.already_have_count, 2);
    assert_eq!(snapshot.known_identities, 1);
    assert_eq!(snapshot.orphan_transactions, 0);
    assert_eq!(snapshot.candidate_cursors, 0);
    assert_eq!(snapshot.accepted_packages, 1);
    assert_eq!(snapshot.compact_download_peers, 0);
    assert_eq!(mismatch_count, 0);
}

#[test]
fn lifecycle_reconciliation_counts_orphan_candidate_package_and_scheduler_divergence() {
    // Arrange
    let accepted = identity(13);
    let parent = identity(14);
    let first_child = identity(15);
    let second_child = identity(17);
    let mut manager = PeerManager::new(local_config());
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 4,
        max_orphans_per_peer: 4,
        max_reconsiderations_per_parent: 2,
        ..OrphanPolicy::default()
    });
    let admission = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            vec![accepted],
            Vec::new(),
            vec![AcceptedPeerPackageFingerprint::new(
                [16; 32],
                vec![accepted],
            )],
        ))
        .expect("bounded admission should prepare");
    manager.apply_prepared_transaction_lifecycle(admission);
    manager.record_reconsiderable_transaction(parent.wtxid());
    stage_orphan(&mut manager, 8_000, first_child, parent.txid(), 1);
    stage_orphan(&mut manager, 8_000, second_child, parent.txid(), 2);
    let _ = manager
        .begin_same_peer_candidate(
            Transaction {
                version: 14,
                ..Transaction::default()
            },
            parent.txid(),
            parent.wtxid(),
            8_000,
        )
        .expect("candidate cursor should be retained");
    assert_eq!(manager.debug_candidate_cursor_count(), 1);
    let canonical = BTreeSet::from([second_child]);

    // Act
    let mismatch_count = manager.mempool_lifecycle_mismatch_count(&canonical);

    // Assert
    assert!(mismatch_count >= 8);
}

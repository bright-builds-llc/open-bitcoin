// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;

#[test]
fn orphanage_labels_are_fixed_low_cardinality_values() {
    // Arrange / Act
    let evidence_labels = [
        OrphanEvidenceLabel::Orphaned,
        OrphanEvidenceLabel::ParentRequested,
        OrphanEvidenceLabel::OrphanEvicted,
        OrphanEvidenceLabel::OrphanExpired,
        OrphanEvidenceLabel::OrphanReconsidered,
    ]
    .map(OrphanEvidenceLabel::as_str);
    let statuses = [
        OrphanReconsiderationStatus::Accepted,
        OrphanReconsiderationStatus::StillMissingParent,
        OrphanReconsiderationStatus::Rejected,
        OrphanReconsiderationStatus::Expired,
        OrphanReconsiderationStatus::Evicted,
    ]
    .map(OrphanReconsiderationStatus::as_str);

    // Assert
    assert_eq!(
        evidence_labels,
        [
            "orphaned",
            "parent_requested",
            "orphan_evicted",
            "orphan_expired",
            "orphan_reconsidered",
        ],
    );
    assert_eq!(
        statuses,
        [
            "accepted_child",
            "still_missing_parent",
            "rejected_child",
            "expired_child",
            "evicted_child",
        ],
    );
}

#[test]
fn default_policy_matches_phase102_orphan_bounds() {
    // Arrange / Act
    let policy = OrphanPolicy::default();

    // Assert
    assert_eq!(policy.max_total_orphans, PHASE102_MAX_ORPHAN_TRANSACTIONS);
    assert_eq!(policy.max_orphans_per_peer, PHASE102_MAX_ORPHANS_PER_PEER);
    assert_eq!(
        policy.max_announcers_per_orphan,
        PHASE133_MAX_ANNOUNCERS_PER_ORPHAN,
    );
    assert_eq!(
        policy.max_retained_bytes,
        PHASE133_MAX_ORPHAN_RETAINED_BYTES,
    );
    assert_eq!(policy.orphan_ttl_seconds, PHASE102_ORPHAN_TTL_SECONDS);
    assert_eq!(
        policy.max_reconsiderations_per_parent,
        PHASE102_MAX_RECONSIDERATIONS_PER_PARENT,
    );
}

#[test]
fn missing_parent_stage_requests_each_unique_parent_by_txid() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));

    // Act
    let actions = stage_singleton(&mut orphanage, 42, orphan_input(42, 8, 9, [3, 1, 3, 2], 10));

    // Assert
    assert_eq!(
        actions,
        [
            parent_request(42, 1),
            parent_request(42, 2),
            parent_request(42, 3),
        ],
    );
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(42), 1);
}

#[test]
fn parent_acceptance_reconsiders_ready_children_with_work_cap() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 2));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 50, 3, [7], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 51, 1, [7], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 52, 2, [7], 0));

    // Act
    let first_batch = orphanage.reconsider_after_parent(TxRelayId::Txid(txid(7)), 1);
    let second_batch = orphanage.drain_pending_reconsiderations(2);

    // Assert
    assert_eq!(first_batch.len(), 2);
    assert!(matches!(
        &first_batch[0],
        OrphanAction::Reconsider { candidate, label }
            if candidate.wtxid == wtxid(1)
                && candidate.missing_parents.is_empty()
                && *label == OrphanEvidenceLabel::OrphanReconsidered
    ));
    assert!(matches!(
        &first_batch[1],
        OrphanAction::Reconsider { candidate, label }
            if candidate.wtxid == wtxid(2)
                && candidate.missing_parents.is_empty()
                && *label == OrphanEvidenceLabel::OrphanReconsidered
    ));
    assert!(matches!(
        &second_batch[..],
        [OrphanAction::Reconsider { candidate, label }]
            if candidate.wtxid == wtxid(3)
                && candidate.missing_parents.is_empty()
                && *label == OrphanEvidenceLabel::OrphanReconsidered
    ));
    assert_eq!(orphanage.len(), 3);
}

#[test]
fn wtxid_parent_acceptance_does_not_reconsider_children() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 55, 56, [7], 0));

    // Act
    let wtxid_actions = orphanage.reconsider_after_parent(TxRelayId::Wtxid(wtxid(7)), 1);
    let unrelated_txid_actions = orphanage.reconsider_after_parent(TxRelayId::Txid(txid(99)), 1);

    // Assert
    assert!(wtxid_actions.is_empty());
    assert!(unrelated_txid_actions.is_empty());
    assert_eq!(orphanage.len(), 1);
}

#[test]
fn still_missing_parent_child_remains_staged() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 60, 61, [7, 8], 0));

    // Act
    let first_parent = orphanage.reconsider_after_parent(TxRelayId::Txid(txid(7)), 1);
    let second_parent = orphanage.reconsider_after_parent(TxRelayId::Txid(txid(8)), 2);

    // Assert
    assert!(first_parent.is_empty());
    assert_eq!(orphanage.len(), 1);
    assert!(matches!(
        &second_parent[..],
        [OrphanAction::Reconsider { candidate, .. }] if candidate.wtxid == wtxid(61)
    ));
}

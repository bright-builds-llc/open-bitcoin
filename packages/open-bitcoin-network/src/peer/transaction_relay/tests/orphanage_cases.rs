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
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use open_bitcoin_primitives::Transaction;

use super::*;

fn policy(
    max_total_orphans: usize,
    max_orphans_per_peer: usize,
    orphan_ttl_seconds: i64,
    max_reconsiderations_per_parent: usize,
) -> OrphanPolicy {
    OrphanPolicy {
        max_total_orphans,
        max_orphans_per_peer,
        orphan_ttl_seconds,
        max_reconsiderations_per_parent,
    }
}

fn orphan_input(
    peer_id: PeerId,
    tx_byte: u8,
    wtx_byte: u8,
    missing_parent_bytes: impl IntoIterator<Item = u8>,
    now_unix_seconds: i64,
) -> OrphanStageInput {
    OrphanStageInput {
        peer_id,
        transaction: Transaction::default(),
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        missing_parents: missing_parent_bytes.into_iter().map(txid).collect(),
        now_unix_seconds,
    }
}

fn parent_request(peer_id: PeerId, parent_byte: u8) -> OrphanAction {
    OrphanAction::RequestParent {
        peer_id,
        relay_id: TxRelayId::Txid(txid(parent_byte)),
        label: OrphanEvidenceLabel::ParentRequested,
    }
}

fn evicted(peer_id: PeerId, tx_byte: u8, wtx_byte: u8) -> OrphanAction {
    OrphanAction::Evicted {
        peer_id,
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        label: OrphanEvidenceLabel::OrphanEvicted,
    }
}

fn expired(peer_id: PeerId, tx_byte: u8, wtx_byte: u8) -> OrphanAction {
    OrphanAction::Expired {
        peer_id,
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        label: OrphanEvidenceLabel::OrphanExpired,
    }
}

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
    let actions = orphanage.stage_missing_parent(orphan_input(42, 8, 9, [3, 1, 3, 2], 10));

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
fn total_cap_eviction_is_deterministic() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(2, 10, 120, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(2, 20, 2, [1], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 21, 3, [1], 0));

    // Act
    let actions = orphanage.stage_missing_parent(orphan_input(1, 22, 1, [1], 0));

    // Assert
    assert!(actions.contains(&evicted(1, 22, 1)));
    assert_eq!(orphanage.len(), 2);
    assert_eq!(orphanage.peer_len(1), 1);
    assert_eq!(orphanage.peer_len(2), 1);
}

#[test]
fn per_peer_cap_eviction_is_deterministic() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 2, 120, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(7, 30, 3, [1], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(7, 31, 2, [1], 0));

    // Act
    let actions = orphanage.stage_missing_parent(orphan_input(7, 32, 1, [1], 0));

    // Assert
    assert!(actions.contains(&evicted(7, 32, 1)));
    assert_eq!(orphanage.len(), 2);
    assert_eq!(orphanage.peer_len(7), 2);
}

#[test]
fn expiry_uses_injected_time_without_sleeping() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 5, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(8, 40, 41, [1], 10));

    // Act
    let too_early = orphanage.expire(14);
    let expired_at_deadline = orphanage.expire(15);
    let mut zero_ttl_orphanage = TxOrphanage::new(policy(10, 10, 0, 10));
    let immediately_expired =
        zero_ttl_orphanage.stage_missing_parent(orphan_input(9, 42, 43, [1], 20));

    // Assert
    assert!(too_early.is_empty());
    assert_eq!(expired_at_deadline, [expired(8, 40, 41)]);
    assert!(orphanage.is_empty());
    assert_eq!(immediately_expired, [expired(9, 42, 43)]);
    assert!(zero_ttl_orphanage.is_empty());
}

#[test]
fn parent_acceptance_reconsiders_ready_children_with_work_cap() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 2));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 50, 3, [7], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 51, 1, [7], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 52, 2, [7], 0));

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
    let _ = orphanage.stage_missing_parent(orphan_input(1, 55, 56, [7], 0));

    // Act
    let actions = orphanage.reconsider_after_parent(TxRelayId::Wtxid(wtxid(7)), 1);

    // Assert
    assert!(actions.is_empty());
    assert_eq!(orphanage.len(), 1);
}

#[test]
fn still_missing_parent_child_remains_staged() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 60, 61, [7, 8], 0));

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

#[test]
fn missing_reconsideration_outcome_is_noop() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));

    // Act
    let actions =
        orphanage.record_reconsideration_outcome(wtxid(99), OrphanReconsiderationStatus::Accepted);

    // Assert
    assert!(actions.is_empty());
    assert!(orphanage.is_empty());
}

#[test]
fn accepted_rejected_expired_and_evicted_reconsideration_outcomes_remove_children() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    for byte in 70..=73 {
        let _ = orphanage.stage_missing_parent(orphan_input(1, byte, byte, [7], 0));
    }
    let _ = orphanage.reconsider_after_parent(TxRelayId::Txid(txid(7)), 1);

    // Act
    let accepted =
        orphanage.record_reconsideration_outcome(wtxid(70), OrphanReconsiderationStatus::Accepted);
    let rejected =
        orphanage.record_reconsideration_outcome(wtxid(71), OrphanReconsiderationStatus::Rejected);
    let expired =
        orphanage.record_reconsideration_outcome(wtxid(72), OrphanReconsiderationStatus::Expired);
    let evicted =
        orphanage.record_reconsideration_outcome(wtxid(73), OrphanReconsiderationStatus::Evicted);

    // Assert
    assert!(matches!(
        &accepted[..],
        [OrphanAction::Reconsidered {
            status: OrphanReconsiderationStatus::Accepted,
            ..
        }]
    ));
    assert!(matches!(
        &rejected[..],
        [OrphanAction::Reconsidered {
            status: OrphanReconsiderationStatus::Rejected,
            ..
        }]
    ));
    assert!(matches!(
        &expired[..],
        [OrphanAction::Reconsidered {
            status: OrphanReconsiderationStatus::Expired,
            ..
        }]
    ));
    assert!(matches!(
        &evicted[..],
        [OrphanAction::Reconsidered {
            status: OrphanReconsiderationStatus::Evicted,
            ..
        }]
    ));
    assert!(orphanage.is_empty());
}

#[test]
fn peer_cleanup_removes_owned_orphans() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 80, 81, [7], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(1, 82, 83, [7], 0));
    let _ = orphanage.stage_missing_parent(orphan_input(2, 84, 85, [7], 0));

    // Act
    let actions = orphanage.cleanup_peer(1);

    // Assert
    assert_eq!(
        actions,
        [OrphanAction::PeerCleanup {
            peer_id: 1,
            removed: 2,
            label: OrphanEvidenceLabel::OrphanEvicted,
        }],
    );
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(1), 0);
    assert_eq!(orphanage.peer_len(2), 1);
}

#[test]
fn peer_cleanup_without_owned_orphans_is_noop() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = orphanage.stage_missing_parent(orphan_input(2, 90, 91, [7], 0));

    // Act
    let actions = orphanage.cleanup_peer(1);

    // Assert
    assert!(actions.is_empty());
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(2), 1);
}

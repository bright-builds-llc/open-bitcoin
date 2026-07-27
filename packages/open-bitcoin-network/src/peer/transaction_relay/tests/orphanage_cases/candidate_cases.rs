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
        let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, byte, byte, [7], 0));
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
fn newest_same_peer_candidate_skips_wrong_peer_and_hard_rejected_child() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let parent = transaction(50);
    let parent_txid = txid(110);
    let parent_wtxid = wtxid(111);
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(1),
            ..orphan_input(2, 112, 113, [110], 0)
        },
        provenance(2, [2]),
    );
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(2),
            ..orphan_input(1, 114, 115, [110], 1)
        },
        provenance(2, [1, 2]),
    );
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(3),
            ..orphan_input(1, 116, 117, [110], 2)
        },
        provenance(1, [1]),
    );
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(1));
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let mut hard = HardRejectEvidence::new(RejectEvidenceTweak::new(2));
    hard.record(wtxid(117));

    // Act
    let candidate = orphanage
        .begin_same_peer_candidate(
            parent.clone(),
            parent_txid,
            parent_wtxid,
            1,
            &reconsiderable,
            &hard,
        )
        .expect("older same-peer child remains eligible");
    let (members, origins, provenances) = candidate.into_ordered_parts_with_provenance();

    // Assert
    assert_eq!(members, [parent, transaction(2)]);
    assert_eq!(origins, [1, 1]);
    assert_eq!(provenances[1], provenance(2, [1, 2]));
}

#[test]
fn candidate_requires_reconsiderable_parent_and_matching_announcer() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let parent = transaction(60);
    let parent_txid = txid(120);
    let parent_wtxid = wtxid(121);
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(4),
            ..orphan_input(2, 122, 123, [120], 0)
        },
        provenance(2, [2]),
    );
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(3));
    let hard = HardRejectEvidence::new(RejectEvidenceTweak::new(4));

    // Act
    let without_evidence = orphanage.begin_same_peer_candidate(
        parent.clone(),
        parent_txid,
        parent_wtxid,
        2,
        &reconsiderable,
        &hard,
    );
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let wrong_peer = orphanage.begin_same_peer_candidate(
        parent,
        parent_txid,
        parent_wtxid,
        1,
        &reconsiderable,
        &hard,
    );

    // Assert
    assert!(without_evidence.is_none());
    assert!(wrong_peer.is_none());
}

#[test]
fn max_reconsiderations_cursor_advances_newest_first_without_graph_aggregation() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 2));
    let parent = transaction(70);
    let parent_txid = txid(130);
    let parent_wtxid = wtxid(131);
    for (sequence, child_version) in [(0, 5), (1, 6), (2, 7)] {
        let _ = orphanage.stage_missing_parent_with_provenance(
            OrphanStageInput {
                transaction: transaction(child_version),
                ..orphan_input(
                    1,
                    132 + child_version as u8,
                    142 + child_version as u8,
                    [130],
                    sequence,
                )
            },
            provenance(1, [1]),
        );
    }
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(8),
            ..orphan_input(1, 150, 151, [130, 200], 3)
        },
        provenance(1, [1]),
    );
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction(9),
            ..orphan_input(1, 152, 153, [150], 4)
        },
        provenance(1, [1]),
    );
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(5));
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let hard = HardRejectEvidence::new(RejectEvidenceTweak::new(6));

    // Act
    let first = orphanage
        .begin_same_peer_candidate(
            parent.clone(),
            parent_txid,
            parent_wtxid,
            1,
            &reconsiderable,
            &hard,
        )
        .expect("newest sibling");
    let _ =
        orphanage.record_reconsideration_outcome(wtxid(148), OrphanReconsiderationStatus::Rejected);
    let second = orphanage
        .advance_same_peer_candidate(parent_wtxid, 1, &hard)
        .expect("next sibling");
    let capped = orphanage.advance_same_peer_candidate(parent_wtxid, 1, &hard);
    let (first_members, _) = first.into_ordered_parts();
    let (second_members, _) = second.into_ordered_parts();

    // Assert
    assert_eq!(first_members, [parent.clone(), transaction(7)]);
    assert_eq!(second_members, [parent, transaction(5)]);
    assert!(capped.is_none());
}

#[test]
fn persistent_candidate_cursor_retains_child_identities_not_child_bodies() {
    // Arrange
    let body_bytes = 8 * 1_024;
    let child_count = 3;
    let retained_cap = body_bytes * (child_count + 1);
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: retained_cap,
        ..policy(10, 10, 120, child_count)
    });
    let parent = transaction(90);
    let parent_txid = txid(210);
    let parent_wtxid = wtxid(211);
    for index in 0..child_count {
        let child_byte = 212 + index as u8;
        let _ = orphanage.stage_missing_parent_with_provenance(
            OrphanStageInput {
                transaction: transaction_with_body_bytes(index as i32, body_bytes),
                ..orphan_input(1, child_byte, child_byte, [210], index as i64)
            },
            provenance(1, [1]),
        );
    }
    let canonical_bytes = orphanage.debug_retained_bytes();
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(9));
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let hard = HardRejectEvidence::new(RejectEvidenceTweak::new(10));

    // Act
    let candidate = orphanage.begin_same_peer_candidate(
        parent,
        parent_txid,
        parent_wtxid,
        1,
        &reconsiderable,
        &hard,
    );
    let (cursor_count, retained_child_identities, cursor_bytes) =
        orphanage.debug_candidate_cursor_retention();

    // Assert
    assert!(candidate.is_some());
    assert_eq!(cursor_count, 1);
    assert_eq!(retained_child_identities, child_count);
    assert!(
        cursor_bytes < body_bytes,
        "cursor must retain one small parent and child identities, not large child bodies"
    );
    assert_eq!(
        orphanage.debug_retained_bytes(),
        canonical_bytes + cursor_bytes
    );
    assert!(orphanage.debug_retained_bytes() <= retained_cap);
}

#[test]
fn candidate_cursor_creation_respects_aggregate_retained_byte_budget() {
    // Arrange
    let parent = transaction(91);
    let parent_txid = txid(220);
    let parent_wtxid = wtxid(221);
    let child_input = OrphanStageInput {
        transaction: transaction(1),
        ..orphan_input(1, 222, 223, [220], 0)
    };
    let mut probe = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = probe.stage_missing_parent_with_provenance(child_input.clone(), provenance(1, [1]));
    let canonical_bytes = probe.debug_retained_bytes();
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: canonical_bytes + 1,
        ..policy(10, 10, 120, 10)
    });
    let _ = orphanage.stage_missing_parent_with_provenance(child_input, provenance(1, [1]));
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(11));
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let hard = HardRejectEvidence::new(RejectEvidenceTweak::new(12));

    // Act
    let candidate = orphanage.begin_same_peer_candidate(
        parent,
        parent_txid,
        parent_wtxid,
        1,
        &reconsiderable,
        &hard,
    );

    // Assert
    assert!(candidate.is_none());
    assert_eq!(orphanage.debug_candidate_cursor_retention(), (0, 0, 0));
    assert_eq!(orphanage.debug_retained_bytes(), canonical_bytes);
}

#[test]
fn bounded_parent_traversal_stops_before_an_older_eligible_child() {
    // Arrange
    let traversal_cap = PHASE102_MAX_RECONSIDERATIONS_PER_PARENT;
    let mut orphanage = TxOrphanage::new(policy(100, 100, 120, traversal_cap));
    let parent = transaction(80);
    let parent_txid = txid(170);
    let parent_wtxid = wtxid(171);
    for index in 0..=traversal_cap {
        let child_byte = 180 + index as u8;
        let _ = orphanage.stage_missing_parent_with_provenance(
            OrphanStageInput {
                transaction: transaction(index as i32),
                ..orphan_input(1, child_byte, child_byte, [170], index as i64)
            },
            provenance(1, [1]),
        );
    }
    let mut reconsiderable = ReconsiderableRejectEvidence::new(RejectEvidenceTweak::new(7));
    reconsiderable.record(ReconsiderableEvidenceKey::Transaction(parent_wtxid));
    let mut hard = HardRejectEvidence::new(RejectEvidenceTweak::new(8));
    for index in 1..=traversal_cap {
        hard.record(wtxid(180 + index as u8));
    }

    // Act
    let candidate = orphanage.begin_same_peer_candidate(
        parent,
        parent_txid,
        parent_wtxid,
        1,
        &reconsiderable,
        &hard,
    );

    // Assert
    assert!(
        candidate.is_none(),
        "the eligible oldest child is beyond the configured traversal cap"
    );
    assert_eq!(orphanage.len(), traversal_cap + 1);
    assert!(orphanage.debug_indexes_match_oracle());
}

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

use open_bitcoin_primitives::{
    Amount, OutPoint, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
};

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
        max_announcers_per_orphan: 4,
        max_retained_bytes: PHASE133_MAX_ORPHAN_RETAINED_BYTES,
        orphan_ttl_seconds,
        max_reconsiderations_per_parent,
    }
}

fn orphan_input(
    _peer_id: PeerId,
    tx_byte: u8,
    wtx_byte: u8,
    missing_parent_bytes: impl IntoIterator<Item = u8>,
    now_unix_seconds: i64,
) -> OrphanStageInput {
    OrphanStageInput {
        transaction: Transaction::default(),
        txid: txid(tx_byte),
        wtxid: wtxid(wtx_byte),
        missing_parents: missing_parent_bytes.into_iter().map(txid).collect(),
        now_unix_seconds,
    }
}

fn stage_singleton(
    orphanage: &mut TxOrphanage,
    peer_id: PeerId,
    input: OrphanStageInput,
) -> Vec<OrphanAction> {
    orphanage.stage_missing_parent_with_provenance(input, provenance(peer_id, [peer_id]))
}

fn provenance(
    delivered_by: PeerId,
    announcers: impl IntoIterator<Item = PeerId>,
) -> ReceivedTransactionProvenance {
    ReceivedTransactionProvenance {
        delivered_by,
        announcers: announcers.into_iter().collect(),
    }
}

fn transaction(version: i32) -> Transaction {
    Transaction {
        version,
        ..Transaction::default()
    }
}

fn transaction_with_body_bytes(version: i32, body_bytes: usize) -> Transaction {
    Transaction {
        version,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(vec![0x51]).expect("bounded test script"),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::new(vec![vec![0x51; 16]]),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::ZERO,
            script_pubkey: ScriptBuf::from_bytes(vec![0x51; body_bytes])
                .expect("bounded test script"),
        }],
        ..Transaction::default()
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
fn total_cap_eviction_is_deterministic() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(2, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 20, 2, [1], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 21, 3, [1], 0));

    // Act
    let actions = stage_singleton(&mut orphanage, 1, orphan_input(1, 22, 1, [1], 0));

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
    let _ = stage_singleton(&mut orphanage, 7, orphan_input(7, 30, 3, [1], 0));
    let _ = stage_singleton(&mut orphanage, 7, orphan_input(7, 31, 2, [1], 0));

    // Act
    let actions = stage_singleton(&mut orphanage, 7, orphan_input(7, 32, 1, [1], 0));

    // Assert
    assert!(actions.contains(&evicted(7, 32, 1)));
    assert_eq!(orphanage.len(), 2);
    assert_eq!(orphanage.peer_len(7), 2);
}

#[test]
fn expiry_uses_injected_time_without_sleeping() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 5, 10));
    let _ = stage_singleton(&mut orphanage, 8, orphan_input(8, 40, 41, [1], 10));

    // Act
    let too_early = orphanage.expire(14);
    let expired_at_deadline = orphanage.expire(15);
    let mut zero_ttl_orphanage = TxOrphanage::new(policy(10, 10, 0, 10));
    let immediately_expired =
        stage_singleton(&mut zero_ttl_orphanage, 9, orphan_input(9, 42, 43, [1], 20));

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
fn peer_cleanup_removes_owned_orphans() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 80, 81, [7], 0));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 82, 83, [7], 0));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 84, 85, [7], 0));

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
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 90, 91, [7], 0));

    // Act
    let actions = orphanage.cleanup_peer(1);

    // Assert
    assert!(actions.is_empty());
    assert_eq!(orphanage.len(), 1);
    assert_eq!(orphanage.peer_len(2), 1);
}

#[test]
fn different_deliverer_provenance_is_bounded_deduplicated_and_retains_delivered_by() {
    // Arrange
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_announcers_per_orphan: 3,
        ..policy(10, 10, 120, 10)
    });
    let input = orphan_input(9, 100, 101, [7], 0);

    // Act
    let _ = orphanage.stage_missing_parent_with_provenance(input, provenance(9, [5, 3, 5, 1, 7]));

    // Assert
    assert_eq!(orphanage.peer_len(9), 1);
    assert_eq!(orphanage.peer_len(1), 1);
    assert_eq!(orphanage.peer_len(3), 1);
    assert_eq!(orphanage.peer_len(5), 0);
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn announcer_cap_keeps_one_shared_body_under_adversarial_peer_churn() {
    // Arrange
    let configured_cap = PHASE133_MAX_ANNOUNCERS_PER_ORPHAN;
    let mut orphanage = TxOrphanage::new(OrphanPolicy::default());
    let retained_wtxid = wtxid(102);
    let announced_peers = 1..=(configured_cap as u64 + 4);

    // Act
    let _ = orphanage.stage_missing_parent_with_provenance(
        orphan_input(99, 101, 102, [7], 0),
        provenance(99, announced_peers.clone()),
    );
    for peer_id in announced_peers {
        let _ = orphanage.add_announcer(retained_wtxid, peer_id);
    }

    // Assert
    let retained_associations: usize = (1..=99).map(|peer_id| orphanage.peer_len(peer_id)).sum();
    assert_eq!(orphanage.len(), 1, "announcers must share one body");
    assert_eq!(retained_associations, configured_cap);
    assert!(orphanage.contains(retained_wtxid));
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn retained_byte_budget_evicts_large_orphan_bodies_before_count_cap() {
    // Arrange
    let body_bytes = 4_096;
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: body_bytes * 2,
        ..policy(10, 10, 120, 10)
    });
    let _ = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction_with_body_bytes(1, body_bytes),
            ..orphan_input(1, 101, 102, [7], 0)
        },
        provenance(1, [1]),
    );

    // Act
    let actions = orphanage.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: transaction_with_body_bytes(2, body_bytes),
            ..orphan_input(2, 103, 104, [7], 1)
        },
        provenance(2, [2]),
    );

    // Assert
    assert_eq!(orphanage.len(), 1, "byte budget must precede count cap");
    assert_eq!(actions, [evicted(1, 101, 102), parent_request(2, 7)]);
    assert!(orphanage.debug_retained_bytes() <= body_bytes * 2);
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn retained_byte_budget_rejects_late_announcer_state_growth() {
    // Arrange
    let input = OrphanStageInput {
        transaction: transaction_with_body_bytes(1, 4_096),
        ..orphan_input(1, 105, 106, [7], 0)
    };
    let mut probe = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = probe.stage_missing_parent_with_provenance(input.clone(), provenance(1, [1]));
    let retained_cap = probe.debug_retained_bytes();
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_retained_bytes: retained_cap,
        ..policy(10, 10, 120, 10)
    });
    let _ = orphanage.stage_missing_parent_with_provenance(input, provenance(1, [1]));

    // Act
    let added = orphanage.add_announcer(wtxid(106), 2);

    // Assert
    assert!(!added);
    assert_eq!(orphanage.peer_len(2), 0);
    assert_eq!(orphanage.debug_retained_bytes(), retained_cap);
}

#[test]
fn late_announcer_missing_body_is_noop_and_existing_body_does_not_refresh_ttl() {
    // Arrange
    let mut orphanage = TxOrphanage::new(OrphanPolicy {
        max_announcers_per_orphan: 2,
        ..policy(10, 10, 5, 10)
    });
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 102, 103, [7], 10));

    // Act
    let missing_body_added = orphanage.add_announcer(wtxid(200), 2);
    let existing_body_added = orphanage.add_announcer(wtxid(103), 2);
    let over_cap_added = orphanage.add_announcer(wtxid(103), 3);
    let expired_actions = orphanage.expire(15);

    // Assert
    assert!(!missing_body_added);
    assert!(existing_body_added);
    assert!(!over_cap_added);
    assert_eq!(expired_actions, [expired(1, 102, 103)]);
    assert!(orphanage.is_empty());
    assert!(orphanage.debug_indexes_match_oracle());
}

#[test]
fn disconnect_removes_only_one_announcer_and_deletes_body_at_zero() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(10, 10, 120, 10));
    let _ = orphanage.stage_missing_parent_with_provenance(
        orphan_input(1, 104, 105, [7], 0),
        provenance(1, [1, 2]),
    );

    // Act
    let first_cleanup = orphanage.cleanup_peer(1);
    let second_cleanup = orphanage.cleanup_peer(2);

    // Assert
    assert!(matches!(
        &first_cleanup[..],
        [OrphanAction::PeerCleanup {
            peer_id: 1,
            removed: 0,
            ..
        }]
    ));
    assert_eq!(second_cleanup.len(), 1);
    assert!(orphanage.is_empty());
    assert!(orphanage.debug_indexes_match_oracle());
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

#[test]
fn coherent_disconnect_expiry_eviction_cleanup_preserves_index_oracle() {
    // Arrange
    let mut orphanage = TxOrphanage::new(policy(2, 10, 2, 10));
    let _ = stage_singleton(&mut orphanage, 1, orphan_input(1, 160, 161, [7], 0));
    let _ = stage_singleton(&mut orphanage, 2, orphan_input(2, 160, 161, [8], 1));
    let _ = stage_singleton(&mut orphanage, 3, orphan_input(3, 162, 163, [8], 1));
    let _ = stage_singleton(&mut orphanage, 4, orphan_input(4, 164, 165, [9], 1));

    // Act / Assert
    assert_eq!(orphanage.len(), 2, "the total cap must evict one body");
    assert!(orphanage.debug_indexes_match_oracle());
    let _ =
        orphanage.record_reconsideration_outcome(wtxid(161), OrphanReconsiderationStatus::Rejected);
    assert!(orphanage.debug_indexes_match_oracle());
    let _ = orphanage.expire(3);
    assert!(orphanage.debug_indexes_match_oracle());
    let _ = orphanage.cleanup_peer(3);
    assert!(orphanage.debug_indexes_match_oracle());
}

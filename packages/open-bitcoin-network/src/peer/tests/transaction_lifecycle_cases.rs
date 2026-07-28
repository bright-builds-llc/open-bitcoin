// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use super::*;
use crate::{
    AcceptedPeerPackageFingerprint, PeerTransactionIdentity, PeerTransactionLifecycleInput,
    PeerTransactionLifecyclePreparationError,
};

mod reconciliation;

fn identity(byte: u8) -> PeerTransactionIdentity {
    PeerTransactionIdentity::new(
        txid_from_byte(byte),
        wtxid_from_byte(byte.wrapping_add(128)),
    )
}

fn lifecycle_input(
    admissions: Vec<PeerTransactionIdentity>,
    teardowns: Vec<PeerTransactionIdentity>,
) -> PeerTransactionLifecycleInput {
    PeerTransactionLifecycleInput::new(admissions, teardowns, Vec::new())
}

fn stage_orphan(
    manager: &mut PeerManager,
    peer_id: PeerId,
    orphan_identity: PeerTransactionIdentity,
    parent_txid: Txid,
    sequence: i64,
) {
    let _ = manager.stage_missing_parent_with_provenance(
        OrphanStageInput {
            transaction: Transaction {
                version: i32::from(orphan_identity.txid().to_byte_array()[0]),
                ..Transaction::default()
            },
            txid: orphan_identity.txid(),
            wtxid: orphan_identity.wtxid(),
            missing_parents: vec![parent_txid],
            now_unix_seconds: sequence,
        },
        ReceivedTransactionProvenance {
            delivered_by: peer_id,
            announcers: vec![peer_id],
        },
    );
}

#[test]
fn lifecycle_input_preserves_explicit_parent_first_admission_order() {
    // Arrange
    let parent = PeerTransactionIdentity::new(txid_from_byte(1), wtxid_from_byte(2));
    let child = PeerTransactionIdentity::new(txid_from_byte(3), wtxid_from_byte(4));

    // Act
    let input = PeerTransactionLifecycleInput::new(vec![parent, child], Vec::new(), Vec::new());

    // Assert
    assert_eq!(input.admissions(), &[parent, child]);
    assert!(input.teardowns().is_empty());
    assert!(input.accepted_packages().is_empty());
    assert_eq!(parent.txid(), txid_from_byte(1));
    assert_eq!(parent.wtxid(), wtxid_from_byte(2));
}

#[test]
fn prepared_lifecycle_consumes_known_and_fingerprint_operations() {
    // Arrange
    let identity = PeerTransactionIdentity::new(txid_from_byte(5), wtxid_from_byte(6));
    let fingerprint = [7; 32];
    let mut manager = PeerManager::new(local_config());
    let admission = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            vec![identity],
            Vec::new(),
            vec![AcceptedPeerPackageFingerprint::new(
                fingerprint,
                vec![identity],
            )],
        ))
        .expect("bounded admission should prepare");

    // Act
    manager.apply_prepared_transaction_lifecycle(admission);

    // Assert
    assert!(manager.debug_mempool_identity_known(identity));
    assert!(manager.debug_accepted_package_fingerprint_contains(fingerprint));

    // Arrange
    let teardown = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            vec![identity],
            Vec::new(),
        ))
        .expect("bounded teardown should prepare");

    // Act
    manager.apply_prepared_transaction_lifecycle(teardown);

    // Assert
    assert!(!manager.debug_mempool_identity_known(identity));
    assert!(!manager.debug_accepted_package_fingerprint_contains(fingerprint));
}

#[test]
fn lifecycle_getters_preserve_descendant_first_teardown_and_package_members() {
    // Arrange
    let parent = identity(10);
    let child = identity(11);
    let fingerprint = [12; 32];
    let package = AcceptedPeerPackageFingerprint::new(fingerprint, vec![parent, child]);
    let input =
        PeerTransactionLifecycleInput::new(Vec::new(), vec![child, parent], vec![package.clone()]);
    let manager = PeerManager::new(local_config());

    // Act
    let prepared = manager
        .prepare_transaction_lifecycle(input.clone())
        .expect("ordered lifecycle should prepare");

    // Assert
    assert_eq!(input.teardowns(), &[child, parent]);
    assert_eq!(input.accepted_packages(), std::slice::from_ref(&package));
    assert_eq!(package.fingerprint(), fingerprint);
    assert_eq!(package.members(), &[parent, child]);
    assert!(prepared.admission_order().is_empty());
    assert_eq!(prepared.teardown_order(), &[child, parent]);
}

#[test]
fn lifecycle_preparation_error_messages_cover_every_bounded_failure() {
    // Arrange
    let errors = [
        PeerTransactionLifecyclePreparationError::IdentityWorkLimit {
            count: 101,
            maximum: 100,
        },
        PeerTransactionLifecyclePreparationError::PackageMemberLimit {
            count: 26,
            maximum: 25,
        },
        PeerTransactionLifecyclePreparationError::FingerprintLimit {
            count: 101,
            maximum: 100,
        },
        PeerTransactionLifecyclePreparationError::FingerprintRetirementLimit {
            count: 33,
            maximum: 32,
        },
        PeerTransactionLifecyclePreparationError::OrphanPeerLimit {
            count: 26,
            maximum: 25,
        },
        PeerTransactionLifecyclePreparationError::CandidateLimit {
            count: 33,
            maximum: 32,
        },
        PeerTransactionLifecyclePreparationError::TxidAliasConflict {
            txid: txid_from_byte(20),
        },
        PeerTransactionLifecyclePreparationError::WtxidAliasConflict {
            wtxid: wtxid_from_byte(21),
        },
        PeerTransactionLifecyclePreparationError::FinalMembershipConflict {
            txid: txid_from_byte(22),
            wtxid: wtxid_from_byte(23),
        },
        PeerTransactionLifecyclePreparationError::FingerprintMembersConflict {
            fingerprint: [24; 32],
        },
    ];

    // Act
    let rendered = errors.map(|error| error.to_string());

    // Assert
    for message in rendered {
        assert!(!message.is_empty());
    }
}

#[test]
fn lifecycle_rejects_alias_and_final_membership_conflicts_without_mutation() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let baseline = format!("{manager:?}");
    let same_txid_a = PeerTransactionIdentity::new(txid_from_byte(30), wtxid_from_byte(31));
    let same_txid_b = PeerTransactionIdentity::new(txid_from_byte(30), wtxid_from_byte(32));
    let same_wtxid_a = PeerTransactionIdentity::new(txid_from_byte(33), wtxid_from_byte(34));
    let same_wtxid_b = PeerTransactionIdentity::new(txid_from_byte(35), wtxid_from_byte(34));
    let overlap = identity(36);

    // Act
    let txid_error = manager
        .prepare_transaction_lifecycle(lifecycle_input(vec![same_txid_a, same_txid_b], Vec::new()));
    let wtxid_error = manager.prepare_transaction_lifecycle(lifecycle_input(
        vec![same_wtxid_a, same_wtxid_b],
        Vec::new(),
    ));
    let overlap_error =
        manager.prepare_transaction_lifecycle(lifecycle_input(vec![overlap], vec![overlap]));

    // Assert
    assert!(matches!(
        txid_error,
        Err(PeerTransactionLifecyclePreparationError::TxidAliasConflict { .. })
    ));
    assert!(matches!(
        wtxid_error,
        Err(PeerTransactionLifecyclePreparationError::WtxidAliasConflict { .. })
    ));
    assert!(matches!(
        overlap_error,
        Err(PeerTransactionLifecyclePreparationError::FinalMembershipConflict { .. })
    ));
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn lifecycle_accepts_exact_identity_and_package_caps_then_rejects_cap_plus_one() {
    // Arrange
    let manager = PeerManager::new(local_config());
    let identities = (0_u8..=100).map(identity).collect::<Vec<_>>();
    let exact_package = AcceptedPeerPackageFingerprint::new([40; 32], identities[..25].to_vec());
    let oversized_package =
        AcceptedPeerPackageFingerprint::new([41; 32], identities[..26].to_vec());
    let baseline = format!("{manager:?}");

    // Act
    let exact_identities =
        manager.prepare_transaction_lifecycle(lifecycle_input(identities[..100].to_vec(), vec![]));
    let too_many_identities =
        manager.prepare_transaction_lifecycle(lifecycle_input(identities.clone(), vec![]));
    let exact_members = manager.prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
        Vec::new(),
        Vec::new(),
        vec![exact_package],
    ));
    let too_many_members = manager.prepare_transaction_lifecycle(
        PeerTransactionLifecycleInput::new(Vec::new(), Vec::new(), vec![oversized_package]),
    );

    // Assert
    assert!(exact_identities.is_ok());
    assert!(matches!(
        too_many_identities,
        Err(
            PeerTransactionLifecyclePreparationError::IdentityWorkLimit {
                count: 101,
                maximum: 100
            }
        )
    ));
    assert!(exact_members.is_ok());
    assert!(matches!(
        too_many_members,
        Err(
            PeerTransactionLifecyclePreparationError::PackageMemberLimit {
                count: 26,
                maximum: 25
            }
        )
    ));
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn lifecycle_bounds_active_fingerprints_and_rejects_conflicting_members() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let packages = (0_u8..=100)
        .map(|byte| AcceptedPeerPackageFingerprint::new([byte; 32], Vec::new()))
        .collect::<Vec<_>>();
    let exact = manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            Vec::new(),
            packages[..100].to_vec(),
        ))
        .expect("one hundred fingerprints should prepare");
    manager.apply_prepared_transaction_lifecycle(exact);
    let baseline = format!("{manager:?}");
    let member_a = identity(50);
    let member_b = identity(51);

    // Act
    let overflow = manager.prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
        Vec::new(),
        Vec::new(),
        vec![packages[100].clone()],
    ));
    let conflict = manager.prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
        Vec::new(),
        Vec::new(),
        vec![
            AcceptedPeerPackageFingerprint::new([101; 32], vec![member_a]),
            AcceptedPeerPackageFingerprint::new([101; 32], vec![member_b]),
        ],
    ));

    // Assert
    assert!(matches!(
        overflow,
        Err(PeerTransactionLifecyclePreparationError::FingerprintLimit {
            count: 101,
            maximum: 100
        })
    ));
    assert!(matches!(
        conflict,
        Err(PeerTransactionLifecyclePreparationError::FingerprintMembersConflict { .. })
    ));
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn lifecycle_accepts_thirty_two_fingerprint_retirements_and_rejects_thirty_three() {
    // Arrange
    let target = identity(60);
    let packages = (0_u8..33)
        .map(|byte| AcceptedPeerPackageFingerprint::new([byte; 32], vec![target]))
        .collect::<Vec<_>>();
    let mut exact_manager = PeerManager::new(local_config());
    let exact = exact_manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            Vec::new(),
            packages[..32].to_vec(),
        ))
        .expect("fingerprints should prepare");
    exact_manager.apply_prepared_transaction_lifecycle(exact);
    let exact_teardown = exact_manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![target]))
        .expect("thirty-two retirements should prepare");

    let mut overflow_manager = PeerManager::new(local_config());
    let overflow_seed = overflow_manager
        .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
            Vec::new(),
            Vec::new(),
            packages,
        ))
        .expect("thirty-three active fingerprints remain below the active cap");
    overflow_manager.apply_prepared_transaction_lifecycle(overflow_seed);
    let baseline = format!("{overflow_manager:?}");

    // Act
    exact_manager.apply_prepared_transaction_lifecycle(exact_teardown);
    let overflow =
        overflow_manager.prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![target]));

    // Assert
    assert!(!exact_manager.debug_accepted_package_fingerprint_contains([0; 32]));
    assert!(matches!(
        overflow,
        Err(
            PeerTransactionLifecyclePreparationError::FingerprintRetirementLimit {
                count: 33,
                maximum: 32
            }
        )
    ));
    assert_eq!(format!("{overflow_manager:?}"), baseline);
}

#[test]
fn lifecycle_rejects_preexisting_orphan_total_and_per_peer_cap_plus_one() {
    // Arrange
    let mut total_manager = PeerManager::new(local_config());
    total_manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 101,
        max_orphans_per_peer: 1,
        ..OrphanPolicy::default()
    });
    for byte in 0_u8..=100 {
        stage_orphan(
            &mut total_manager,
            10_000 + u64::from(byte),
            identity(byte),
            txid_from_byte(byte.wrapping_add(1)),
            i64::from(byte),
        );
    }
    let total_baseline = format!("{total_manager:?}");

    let mut peer_manager = PeerManager::new(local_config());
    peer_manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 26,
        max_orphans_per_peer: 26,
        ..OrphanPolicy::default()
    });
    for byte in 0_u8..26 {
        stage_orphan(
            &mut peer_manager,
            11_000,
            identity(byte),
            txid_from_byte(byte.wrapping_add(1)),
            i64::from(byte),
        );
    }
    let peer_baseline = format!("{peer_manager:?}");

    // Act
    let total_error = total_manager.prepare_transaction_lifecycle(
        PeerTransactionLifecycleInput::new(Vec::new(), Vec::new(), Vec::new()),
    );
    let peer_error = peer_manager.prepare_transaction_lifecycle(
        PeerTransactionLifecycleInput::new(Vec::new(), Vec::new(), Vec::new()),
    );

    // Assert
    assert!(matches!(
        total_error,
        Err(
            PeerTransactionLifecyclePreparationError::IdentityWorkLimit {
                count: 101,
                maximum: 100
            }
        )
    ));
    assert!(matches!(
        peer_error,
        Err(PeerTransactionLifecyclePreparationError::OrphanPeerLimit {
            count: 26,
            maximum: 25
        })
    ));
    assert_eq!(format!("{total_manager:?}"), total_baseline);
    assert_eq!(format!("{peer_manager:?}"), peer_baseline);
}

#[test]
fn lifecycle_rejects_candidate_cursor_cap_plus_one_without_mutation() {
    // Arrange
    let parent = identity(70);
    let mut manager = PeerManager::new(local_config());
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 33,
        max_orphans_per_peer: 33,
        max_reconsiderations_per_parent: 33,
        ..OrphanPolicy::default()
    });
    manager.record_reconsiderable_transaction(parent.wtxid());
    for byte in 0_u8..33 {
        stage_orphan(
            &mut manager,
            12_000,
            identity(byte),
            parent.txid(),
            i64::from(byte),
        );
    }
    let _ = manager
        .begin_same_peer_candidate(
            Transaction {
                version: 70,
                ..Transaction::default()
            },
            parent.txid(),
            parent.wtxid(),
            12_000,
        )
        .expect("candidate cursor should be retained");
    let baseline = format!("{manager:?}");

    // Act
    let error = manager.prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ));

    // Assert
    assert!(matches!(
        error,
        Err(PeerTransactionLifecyclePreparationError::CandidateLimit {
            count: 33,
            maximum: 32
        })
    ));
    assert_eq!(format!("{manager:?}"), baseline);
}

#[test]
fn lifecycle_apply_cleans_orphan_provenance_and_same_peer_candidate_cursor() {
    // Arrange
    let parent = identity(80);
    let first_child = identity(81);
    let second_child = identity(82);
    let mut manager = PeerManager::new(local_config());
    manager.replace_orphan_policy_for_testing(OrphanPolicy {
        max_total_orphans: 4,
        max_orphans_per_peer: 4,
        max_reconsiderations_per_parent: 2,
        ..OrphanPolicy::default()
    });
    manager.record_reconsiderable_transaction(parent.wtxid());
    stage_orphan(&mut manager, 13_000, first_child, parent.txid(), 1);
    stage_orphan(&mut manager, 13_000, second_child, parent.txid(), 2);
    let _ = manager
        .begin_same_peer_candidate(
            Transaction {
                version: 80,
                ..Transaction::default()
            },
            parent.txid(),
            parent.wtxid(),
            13_000,
        )
        .expect("first candidate");
    assert_eq!(manager.debug_candidate_cursor_count(), 1);
    assert_eq!(manager.orphan_count(), 2);
    let prepared = manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![second_child]))
        .expect("child teardown should prepare exact orphan and cursor removal");

    // Act
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(manager.orphan_count(), 1);
    assert_eq!(manager.orphan_peer_len(13_000), 1);
    assert_eq!(manager.debug_candidate_cursor_count(), 0);
}

#[test]
fn lifecycle_apply_cleans_txid_and_wtxid_request_aliases() {
    // Arrange
    let target = identity(90);
    let mut manager = relay_download_manager(true);
    add_relay_outbound_peer(&mut manager, 14_000);
    add_relay_outbound_peer(&mut manager, 14_001);
    manager
        .handle_message(
            14_000,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Txid(target.txid()))),
            1,
        )
        .expect("txid request");
    manager
        .handle_message(
            14_001,
            WireNetworkMessage::Inv(transaction_relay_inventory(TxRelayId::Wtxid(
                target.wtxid(),
            ))),
            2,
        )
        .expect("wtxid request");
    let prepared = manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![target]))
        .expect("request cleanup should prepare");

    // Act
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    assert_eq!(
        manager.transaction_request_snapshot(14_000).in_flight_count,
        0
    );
    assert_eq!(
        manager.transaction_request_snapshot(14_001).in_flight_count,
        0
    );
}

#[test]
fn lifecycle_apply_cleans_partial_compact_slots() {
    // Arrange
    let peer_id = 15_000;
    let genesis = mined_header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    let (payload, matched, matched_wtxid, _, _) =
        phase119_compact_payload_with_one_matched_and_one_missing();
    let compact_block_hash = block_hash(&payload.header);
    let matched_txid = transaction_txid(&matched).expect("matched txid");
    let target = PeerTransactionIdentity::new(matched_txid, matched_wtxid);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    complete_outbound_handshake(&mut manager, peer_id, 2);
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);
    let facts = CompactBlockReceiveFacts {
        candidates: &[(&matched_wtxid, &matched)],
        extra: &[],
    };
    let _ = manager
        .handle_compact_block_download(peer_id, payload, facts, 1_000)
        .expect("compact download");
    let prepared = manager
        .prepare_transaction_lifecycle(lifecycle_input(Vec::new(), vec![target]))
        .expect("compact cleanup should prepare");

    // Act
    manager.apply_prepared_transaction_lifecycle(prepared);

    // Assert
    let state = manager
        .compact_download_peer_state(peer_id)
        .expect("compact state");
    let in_flight = state
        .in_flight
        .get(&compact_block_hash)
        .expect("in-flight partial");
    assert!(!in_flight.partial.is_transaction_available(1));
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

mod peer_abort;
mod peer_sessions;
mod snapshot_abort;

#[test]
fn family_caps_match_the_resolved_peer_and_snapshot_bounds() {
    // Arrange
    let peer_cap = PHASE94_MAX_PEER_QUEUED_MESSAGES;

    // Act
    let actual = [
        MAX_PENDING_PEER_EFFECTS,
        MAX_COMPLETED_PEER_EFFECTS,
        MAX_PENDING_SNAPSHOT_EFFECTS,
        MAX_COMPLETED_SNAPSHOT_EFFECTS,
    ];

    // Assert
    assert_eq!(actual, [peer_cap, peer_cap, 1, 2]);
}

#[test]
fn peer_capability_and_receipt_bind_every_peer_identity_dimension() {
    // Arrange
    let authority_epoch = AuthorityEpoch::INITIAL;
    let generation = LifecycleGeneration::INITIAL;
    let effect_id = PeerEffectId::new(41);
    let peer_id: PeerId = 134_081;
    let session_generation = PeerSessionGeneration::new(7);
    let capability = PeerEffectCapability::new(
        authority_epoch,
        generation,
        effect_id,
        peer_id,
        session_generation,
    );

    // Act
    let receipt = capability.acknowledge_write();

    // Assert
    assert_eq!(receipt.authority_epoch(), authority_epoch);
    assert_eq!(receipt.lifecycle_generation(), generation);
    assert_eq!(receipt.effect_id(), effect_id);
    assert_eq!(receipt.peer_id(), peer_id);
    assert_eq!(receipt.peer_session_generation(), session_generation);
}

#[test]
fn snapshot_capability_and_receipt_bind_every_snapshot_identity_dimension() {
    // Arrange
    let authority_epoch = AuthorityEpoch::INITIAL;
    let generation = LifecycleGeneration::INITIAL;
    let effect_id = SnapshotEffectId::new(42);
    let snapshot_identity = SnapshotIdentity::new(9);
    let prepared = PreparedSnapshotWrite::new(
        authority_epoch,
        generation,
        effect_id,
        snapshot_identity,
        MempoolSnapshot::default(),
    );

    // Act
    let (snapshot, capability) = prepared.into_parts();
    let receipt = capability.acknowledge_write();

    // Assert
    assert!(snapshot.records.is_empty());
    assert_eq!(receipt.authority_epoch(), authority_epoch);
    assert_eq!(receipt.persistence_generation(), generation);
    assert_eq!(receipt.effect_id(), effect_id);
    assert_eq!(receipt.snapshot_identity(), snapshot_identity);
}

#[test]
fn independently_constructed_handles_use_distinct_non_initial_incarnations() {
    // Arrange
    let first = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let second = ManagedNetworkHandle::from_network_fixture(network_fixture());

    // Act
    let first_peer = first
        .prepare_peer_relay_effect(134_140)
        .expect("first peer effect should prepare")
        .acknowledge_write();
    let second_peer = second
        .prepare_peer_relay_effect(134_140)
        .expect("second peer effect should prepare")
        .acknowledge_write();
    let first_snapshot = first
        .prepare_mempool_snapshot_write()
        .expect("first snapshot should prepare")
        .into_parts()
        .1
        .acknowledge_write();
    let second_snapshot = second
        .prepare_mempool_snapshot_write()
        .expect("second snapshot should prepare")
        .into_parts()
        .1
        .acknowledge_write();

    // Assert
    assert_eq!(first_peer.effect_id(), second_peer.effect_id());
    assert_eq!(first_snapshot.effect_id(), second_snapshot.effect_id());
    assert_ne!(first_peer.authority_epoch(), AuthorityEpoch::INITIAL);
    assert_ne!(second_peer.authority_epoch(), AuthorityEpoch::INITIAL);
    assert_ne!(first_peer.authority_epoch(), second_peer.authority_epoch());
    assert_eq!(
        first_peer.authority_epoch(),
        first_snapshot.authority_epoch()
    );
    assert_eq!(
        second_peer.authority_epoch(),
        second_snapshot.authority_epoch()
    );
}

#[test]
fn independently_constructed_handles_reject_each_others_same_id_receipts() {
    // Arrange
    let first = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let second = ManagedNetworkHandle::from_network_fixture(network_fixture());
    let foreign_peer = first
        .prepare_peer_relay_effect(134_146)
        .expect("first peer effect should prepare")
        .acknowledge_write();
    let local_peer = second
        .prepare_peer_relay_effect(134_146)
        .expect("second peer effect should prepare")
        .acknowledge_write();
    let foreign_snapshot = first
        .prepare_mempool_snapshot_write()
        .expect("first snapshot should prepare")
        .into_parts()
        .1
        .acknowledge_write();
    let local_snapshot = second
        .prepare_mempool_snapshot_write()
        .expect("second snapshot should prepare")
        .into_parts()
        .1
        .acknowledge_write();

    // Act
    let foreign_peer_result = second.complete_peer_effect(foreign_peer);
    let foreign_snapshot_result = second.complete_snapshot_write(foreign_snapshot);
    let local_peer_result = second.complete_peer_effect(local_peer);
    let local_snapshot_result = second.complete_snapshot_write(local_snapshot);

    // Assert
    assert!(foreign_peer_result.is_err());
    assert!(foreign_snapshot_result.is_err());
    assert_eq!(
        local_peer_result.expect("local peer receipt should remain pending"),
        EffectCompletion::Applied
    );
    assert_eq!(
        local_snapshot_result.expect("local snapshot receipt should remain pending"),
        EffectCompletion::Applied
    );
}

#[test]
fn peer_ledger_consumes_only_the_complete_pending_binding() {
    // Arrange
    let mut ledger = PeerEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    let capability = ledger
        .reserve_next(
            authority_epoch,
            LifecycleGeneration::INITIAL,
            134_141,
            PeerSessionGeneration::new(3),
        )
        .expect("peer binding should reserve");
    let exact = capability.acknowledge_write();
    let replay = exact.duplicate_for_test();
    let mismatch = PeerEffectCapability::new(
        authority_epoch,
        LifecycleGeneration::INITIAL,
        exact.effect_id(),
        134_142,
        PeerSessionGeneration::new(3),
    )
    .acknowledge_write();

    // Act
    let mismatch_completion = ledger.complete_exact(&mismatch);
    let exact_completion = ledger.complete_exact(&exact);
    let replay_completion = ledger.complete_exact(&replay);

    // Assert
    assert_eq!(mismatch_completion, ExactEffectLedgerCompletion::NotPending);
    assert_eq!(exact_completion, ExactEffectLedgerCompletion::Recorded);
    assert_eq!(
        replay_completion,
        ExactEffectLedgerCompletion::AlreadyRecorded
    );
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.completed_len(), 1);
}

#[test]
fn snapshot_ledger_consumes_only_the_complete_pending_binding() {
    // Arrange
    let mut ledger = SnapshotEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    let prepared = ledger
        .reserve_next(
            authority_epoch,
            LifecycleGeneration::INITIAL,
            MempoolSnapshot::default(),
        )
        .expect("snapshot binding should reserve");
    let exact = prepared.into_parts().1.acknowledge_write();
    let replay = exact.duplicate_for_test();
    let mismatch = PreparedSnapshotWrite::new(
        authority_epoch,
        LifecycleGeneration::INITIAL,
        exact.effect_id(),
        SnapshotIdentity::new(99),
        MempoolSnapshot::default(),
    )
    .into_parts()
    .1
    .acknowledge_write();

    // Act
    let mismatch_completion = ledger.complete_exact(&mismatch);
    let exact_completion = ledger.complete_exact(&exact);
    let replay_completion = ledger.complete_exact(&replay);

    // Assert
    assert_eq!(mismatch_completion, ExactEffectLedgerCompletion::NotPending);
    assert_eq!(exact_completion, ExactEffectLedgerCompletion::Recorded);
    assert_eq!(
        replay_completion,
        ExactEffectLedgerCompletion::AlreadyRecorded
    );
    assert_eq!(ledger.pending_len(), 0);
    assert_eq!(ledger.completed_len(), 1);
}

#[test]
fn peer_completed_ledger_evicts_the_oldest_exact_binding_at_cap_plus_one() {
    // Arrange
    let mut ledger = PeerEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    let generation = LifecycleGeneration::INITIAL;
    let session_generation = PeerSessionGeneration::new(1);
    for raw_id in 0..MAX_COMPLETED_PEER_EFFECTS {
        let receipt = PeerEffectCapability::new(
            authority_epoch,
            generation,
            PeerEffectId::new(raw_id as u64),
            raw_id as PeerId,
            session_generation,
        )
        .acknowledge_write();
        ledger.record_completed_for_test(&receipt);
    }
    let oldest = PeerEffectCapability::new(
        authority_epoch,
        generation,
        PeerEffectId::new(0),
        0,
        session_generation,
    )
    .acknowledge_write();
    let newest = PeerEffectCapability::new(
        authority_epoch,
        generation,
        PeerEffectId::new(MAX_COMPLETED_PEER_EFFECTS as u64),
        MAX_COMPLETED_PEER_EFFECTS as PeerId,
        session_generation,
    )
    .acknowledge_write();

    // Act
    ledger.record_completed_for_test(&newest);

    // Assert
    assert_eq!(ledger.completed_len(), MAX_COMPLETED_PEER_EFFECTS);
    assert!(!ledger.is_completed_exact(&oldest));
    assert!(ledger.is_completed_exact(&newest));
}

#[test]
fn snapshot_completed_ledger_evicts_the_oldest_exact_binding_at_cap_plus_one() {
    // Arrange
    let mut ledger = SnapshotEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    let generation = LifecycleGeneration::INITIAL;
    for raw_id in 0..MAX_COMPLETED_SNAPSHOT_EFFECTS {
        let receipt = PreparedSnapshotWrite::new(
            authority_epoch,
            generation,
            SnapshotEffectId::new(raw_id as u64),
            SnapshotIdentity::new(raw_id as u64),
            MempoolSnapshot::default(),
        )
        .into_parts()
        .1
        .acknowledge_write();
        ledger.record_completed_for_test(&receipt);
    }
    let oldest = PreparedSnapshotWrite::new(
        authority_epoch,
        generation,
        SnapshotEffectId::new(0),
        SnapshotIdentity::new(0),
        MempoolSnapshot::default(),
    )
    .into_parts()
    .1
    .acknowledge_write();
    let newest = PreparedSnapshotWrite::new(
        authority_epoch,
        generation,
        SnapshotEffectId::new(MAX_COMPLETED_SNAPSHOT_EFFECTS as u64),
        SnapshotIdentity::new(MAX_COMPLETED_SNAPSHOT_EFFECTS as u64),
        MempoolSnapshot::default(),
    )
    .into_parts()
    .1
    .acknowledge_write();

    // Act
    ledger.record_completed_for_test(&newest);

    // Assert
    assert_eq!(ledger.completed_len(), MAX_COMPLETED_SNAPSHOT_EFFECTS);
    assert!(!ledger.is_completed_exact(&oldest));
    assert!(ledger.is_completed_exact(&newest));
}

#[test]
fn pending_ledgers_fail_closed_at_exact_family_caps() {
    // Arrange
    let mut peer_ledger = PeerEffectLedger::default();
    let authority_epoch = AuthorityEpoch::INITIAL
        .checked_next()
        .expect("test authority epoch should advance");
    for raw_id in 0..MAX_PENDING_PEER_EFFECTS {
        assert!(
            peer_ledger
                .try_reserve_for_test(PeerEffectCapability::new(
                    authority_epoch,
                    LifecycleGeneration::INITIAL,
                    PeerEffectId::new(raw_id as u64),
                    raw_id as PeerId,
                    PeerSessionGeneration::new(1),
                ))
                .is_ok()
        );
    }
    let mut snapshot_ledger = SnapshotEffectLedger::default();
    snapshot_ledger
        .try_reserve_for_test(PreparedSnapshotWrite::new(
            authority_epoch,
            LifecycleGeneration::INITIAL,
            SnapshotEffectId::new(0),
            SnapshotIdentity::new(0),
            MempoolSnapshot::default(),
        ))
        .expect("one snapshot may be pending");

    // Act
    let peer_overflow = peer_ledger.try_reserve_for_test(PeerEffectCapability::new(
        authority_epoch,
        LifecycleGeneration::INITIAL,
        PeerEffectId::new(MAX_PENDING_PEER_EFFECTS as u64),
        MAX_PENDING_PEER_EFFECTS as PeerId,
        PeerSessionGeneration::new(1),
    ));
    let snapshot_overflow = snapshot_ledger.try_reserve_for_test(PreparedSnapshotWrite::new(
        authority_epoch,
        LifecycleGeneration::INITIAL,
        SnapshotEffectId::new(1),
        SnapshotIdentity::new(1),
        MempoolSnapshot::default(),
    ));

    // Assert
    assert!(peer_overflow.is_err());
    assert!(snapshot_overflow.is_err());
    assert_eq!(peer_ledger.pending_len(), MAX_PENDING_PEER_EFFECTS);
    assert_eq!(snapshot_ledger.pending_len(), MAX_PENDING_SNAPSHOT_EFFECTS);
}

#[test]
fn dispatcher_rejects_every_foreign_peer_binding_without_mutation() {
    // Arrange
    let mut network = network_fixture();
    let capability = match apply_lifecycle_command(
        &mut network,
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_143)),
    )
    .expect("peer effect should prepare")
    {
        crate::network::runtime_authority::LifecycleCommandResult::RelayPrepared(capability) => {
            capability
        }
        _ => panic!("relay preparation returned the wrong command result"),
    };
    let exact = capability.acknowledge_write();
    let exact_completion = exact.duplicate_for_test();
    let next_authority = exact
        .authority_epoch()
        .checked_next()
        .expect("test authority epoch should advance");
    let next_lifecycle = exact
        .lifecycle_generation()
        .checked_next()
        .expect("test lifecycle generation should advance");
    let next_session = exact
        .peer_session_generation()
        .checked_next()
        .expect("test session generation should advance");
    let mismatches = [
        PeerEffectCapability::new(
            next_authority,
            exact.lifecycle_generation(),
            exact.effect_id(),
            exact.peer_id(),
            exact.peer_session_generation(),
        )
        .acknowledge_write(),
        PeerEffectCapability::new(
            exact.authority_epoch(),
            next_lifecycle,
            exact.effect_id(),
            exact.peer_id(),
            exact.peer_session_generation(),
        )
        .acknowledge_write(),
        PeerEffectCapability::new(
            exact.authority_epoch(),
            exact.lifecycle_generation(),
            PeerEffectId::new(134_143),
            exact.peer_id(),
            exact.peer_session_generation(),
        )
        .acknowledge_write(),
        PeerEffectCapability::new(
            exact.authority_epoch(),
            exact.lifecycle_generation(),
            exact.effect_id(),
            134_144,
            exact.peer_session_generation(),
        )
        .acknowledge_write(),
        PeerEffectCapability::new(
            exact.authority_epoch(),
            exact.lifecycle_generation(),
            exact.effect_id(),
            exact.peer_id(),
            next_session,
        )
        .acknowledge_write(),
    ];
    let state_before = format!("{network:?}");

    // Act
    for mismatch in mismatches {
        let result =
            apply_lifecycle_command(&mut network, LifecycleCommand::CompletePeerEffect(mismatch));

        // Assert
        assert!(result.is_err());
        assert_eq!(format!("{network:?}"), state_before);
    }
    let completion = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::CompletePeerEffect(exact_completion),
    )
    .expect("the exact pending peer receipt should remain valid");
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::PeerEffectCompleted(
            EffectCompletion::Applied
        )
    ));
}

#[test]
fn dispatcher_rejects_every_foreign_snapshot_binding_without_mutation() {
    // Arrange
    let mut network = network_fixture();
    let prepared = prepare_snapshot(&mut network);
    let exact = prepared.into_parts().1.acknowledge_write();
    let exact_completion = exact.duplicate_for_test();
    let next_authority = exact
        .authority_epoch()
        .checked_next()
        .expect("test authority epoch should advance");
    let next_persistence = exact
        .persistence_generation()
        .checked_next()
        .expect("test persistence generation should advance");
    let mismatches = [
        PreparedSnapshotWrite::new(
            next_authority,
            exact.persistence_generation(),
            exact.effect_id(),
            exact.snapshot_identity(),
            MempoolSnapshot::default(),
        )
        .into_parts()
        .1
        .acknowledge_write(),
        PreparedSnapshotWrite::new(
            exact.authority_epoch(),
            next_persistence,
            exact.effect_id(),
            exact.snapshot_identity(),
            MempoolSnapshot::default(),
        )
        .into_parts()
        .1
        .acknowledge_write(),
        PreparedSnapshotWrite::new(
            exact.authority_epoch(),
            exact.persistence_generation(),
            SnapshotEffectId::new(134_144),
            exact.snapshot_identity(),
            MempoolSnapshot::default(),
        )
        .into_parts()
        .1
        .acknowledge_write(),
        PreparedSnapshotWrite::new(
            exact.authority_epoch(),
            exact.persistence_generation(),
            exact.effect_id(),
            SnapshotIdentity::new(134_145),
            MempoolSnapshot::default(),
        )
        .into_parts()
        .1
        .acknowledge_write(),
    ];
    let state_before = format!("{network:?}");

    // Act
    for mismatch in mismatches {
        let result = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::CompleteSnapshotEffect(mismatch),
        );

        // Assert
        assert!(result.is_err());
        assert_eq!(format!("{network:?}"), state_before);
    }
    let completion = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::CompleteSnapshotEffect(exact_completion),
    )
    .expect("the exact pending snapshot receipt should remain valid");
    assert!(matches!(
        completion,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectCompleted(
            EffectCompletion::Applied
        )
    ));
}

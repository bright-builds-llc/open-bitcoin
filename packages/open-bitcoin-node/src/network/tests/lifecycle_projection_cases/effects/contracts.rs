// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

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
fn peer_completed_ledger_evicts_the_oldest_id_at_cap_plus_one() {
    // Arrange
    let mut ledger = PeerEffectLedger::default();
    for raw_id in 0..MAX_COMPLETED_PEER_EFFECTS {
        ledger.record_completed(PeerEffectId::new(raw_id as u64));
    }
    let oldest = PeerEffectId::new(0);
    let newest = PeerEffectId::new(MAX_COMPLETED_PEER_EFFECTS as u64);

    // Act
    ledger.record_completed(newest);

    // Assert
    assert_eq!(ledger.completed_len(), MAX_COMPLETED_PEER_EFFECTS);
    assert!(!ledger.is_completed(oldest));
    assert!(ledger.is_completed(newest));
}

#[test]
fn snapshot_completed_ledger_evicts_the_oldest_id_at_cap_plus_one() {
    // Arrange
    let mut ledger = SnapshotEffectLedger::default();
    for raw_id in 0..MAX_COMPLETED_SNAPSHOT_EFFECTS {
        ledger.record_completed(SnapshotEffectId::new(raw_id as u64));
    }
    let oldest = SnapshotEffectId::new(0);
    let newest = SnapshotEffectId::new(MAX_COMPLETED_SNAPSHOT_EFFECTS as u64);

    // Act
    ledger.record_completed(newest);

    // Assert
    assert_eq!(ledger.completed_len(), MAX_COMPLETED_SNAPSHOT_EFFECTS);
    assert!(!ledger.is_completed(oldest));
    assert!(ledger.is_completed(newest));
}

#[test]
fn pending_ledgers_fail_closed_at_exact_family_caps() {
    // Arrange
    let mut peer_ledger = PeerEffectLedger::default();
    for raw_id in 0..MAX_PENDING_PEER_EFFECTS {
        assert!(
            peer_ledger
                .try_reserve(PeerEffectId::new(raw_id as u64))
                .is_ok()
        );
    }
    let mut snapshot_ledger = SnapshotEffectLedger::default();
    snapshot_ledger
        .try_reserve(SnapshotEffectId::new(0))
        .expect("one snapshot may be pending");

    // Act
    let peer_overflow = peer_ledger.try_reserve(PeerEffectId::new(MAX_PENDING_PEER_EFFECTS as u64));
    let snapshot_overflow = snapshot_ledger.try_reserve(SnapshotEffectId::new(1));

    // Assert
    assert!(peer_overflow.is_err());
    assert!(snapshot_overflow.is_err());
    assert_eq!(peer_ledger.pending_len(), MAX_PENDING_PEER_EFFECTS);
    assert_eq!(snapshot_ledger.pending_len(), MAX_PENDING_SNAPSHOT_EFFECTS);
}

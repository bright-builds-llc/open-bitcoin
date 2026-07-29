// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::network::EffectAbort;

fn snapshot_capability(
    authority_epoch: AuthorityEpoch,
    persistence_generation: LifecycleGeneration,
    effect_id: SnapshotEffectId,
    snapshot_identity: SnapshotIdentity,
) -> crate::network::SnapshotWriteCapability {
    PreparedSnapshotWrite::new(
        authority_epoch,
        persistence_generation,
        effect_id,
        snapshot_identity,
        MempoolSnapshot::default(),
    )
    .into_parts()
    .1
}

#[test]
fn exact_snapshot_abort_releases_pending_without_recording_achievement() {
    // Arrange
    let mut network = network_fixture();
    let prepared = prepare_snapshot(&mut network);
    let (_, capability) = prepared.into_parts();
    let dirty_before = network.dirty_generation;

    // Act
    let result = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::AbortSnapshotEffect(capability),
    )
    .expect("exact snapshot abort should dispatch");

    // Assert
    assert!(matches!(
        result,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert_eq!(network.snapshot_effect_ledger.pending_len(), 0);
    assert_eq!(network.snapshot_effect_ledger.completed_len(), 0);
    assert_eq!(network.dirty_generation, dirty_before);
}

#[test]
fn exact_snapshot_abort_ignores_newer_lifecycle_and_dirty_freshness() {
    // Arrange
    let (mut network, coinbase_txid) = network_with_spendable_coinbase(PolicyConfig::default());
    let prepared = prepare_snapshot(&mut network);
    let (_, capability) = prepared.into_parts();
    apply_local_spend(&mut network, coinbase_txid, 134_170);
    let current_generation = network.lifecycle_generation;
    let dirty_before = network.dirty_generation;

    // Act
    let result = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::AbortSnapshotEffect(capability),
    )
    .expect("stale exact snapshot abort should dispatch");

    // Assert
    assert!(matches!(
        result,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert_eq!(network.lifecycle_generation, current_generation);
    assert_eq!(network.dirty_generation, dirty_before);
    assert_eq!(network.snapshot_effect_ledger.pending_len(), 0);
    assert_eq!(network.snapshot_effect_ledger.completed_len(), 0);
}

#[test]
fn snapshot_abort_rejects_every_immutable_binding_mismatch_without_mutation() {
    // Arrange
    let mut network = network_fixture();
    let exact = prepare_snapshot(&mut network).into_parts().1;
    let authority_epoch = network.authority_epoch;
    let persistence_generation = network.lifecycle_generation;
    let mismatches = [
        snapshot_capability(
            authority_epoch
                .checked_next()
                .expect("test authority epoch should advance"),
            persistence_generation,
            SnapshotEffectId::new(0),
            SnapshotIdentity::new(0),
        ),
        snapshot_capability(
            authority_epoch,
            persistence_generation
                .checked_next()
                .expect("test persistence generation should advance"),
            SnapshotEffectId::new(0),
            SnapshotIdentity::new(0),
        ),
        snapshot_capability(
            authority_epoch,
            persistence_generation,
            SnapshotEffectId::new(134_170),
            SnapshotIdentity::new(0),
        ),
        snapshot_capability(
            authority_epoch,
            persistence_generation,
            SnapshotEffectId::new(0),
            SnapshotIdentity::new(134_170),
        ),
    ];
    let state_before = format!("{network:?}");

    // Act / Assert
    for mismatch in mismatches {
        let result = apply_lifecycle_command(
            &mut network,
            LifecycleCommand::AbortSnapshotEffect(mismatch),
        )
        .expect("binding mismatch should classify");
        assert!(matches!(
            result,
            crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
                EffectAbort::NotPending
            )
        ));
        assert_eq!(format!("{network:?}"), state_before);
    }

    let exact_result =
        apply_lifecycle_command(&mut network, LifecycleCommand::AbortSnapshotEffect(exact))
            .expect("exact snapshot reservation should remain pending");
    assert!(matches!(
        exact_result,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
            EffectAbort::Aborted
        )
    ));
}

#[test]
fn snapshot_abort_replay_is_a_typed_noop() {
    // Arrange
    let mut network = network_fixture();
    let exact = prepare_snapshot(&mut network).into_parts().1;
    let replay = snapshot_capability(
        network.authority_epoch,
        network.lifecycle_generation,
        SnapshotEffectId::new(0),
        SnapshotIdentity::new(0),
    );
    apply_lifecycle_command(&mut network, LifecycleCommand::AbortSnapshotEffect(exact))
        .expect("first abort should dispatch");
    let state_before = format!("{network:?}");

    // Act
    let replay_result =
        apply_lifecycle_command(&mut network, LifecycleCommand::AbortSnapshotEffect(replay))
            .expect("replayed abort should classify");

    // Assert
    assert!(matches!(
        replay_result,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
            EffectAbort::NotPending
        )
    ));
    assert_eq!(format!("{network:?}"), state_before);
}

#[test]
fn snapshot_abort_restores_the_single_pending_slot() {
    // Arrange
    let mut network = network_fixture();
    let capability = prepare_snapshot(&mut network).into_parts().1;
    assert!(matches!(
        apply_lifecycle_command(
            &mut network,
            LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        ),
        Err(crate::network::lifecycle_projection::LifecycleProjectionError::EffectPreparation(_))
    ));

    // Act
    let abort = apply_lifecycle_command(
        &mut network,
        LifecycleCommand::AbortSnapshotEffect(capability),
    )
    .expect("snapshot abort should dispatch");
    let retry = prepare_snapshot(&mut network);

    // Assert
    assert!(matches!(
        abort,
        crate::network::runtime_authority::LifecycleCommandResult::SnapshotEffectAborted(
            EffectAbort::Aborted
        )
    ));
    assert_eq!(retry.snapshot(), &MempoolSnapshot::default());
}

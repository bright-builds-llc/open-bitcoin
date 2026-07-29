// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::any::TypeId;

use open_bitcoin_core::primitives::BlockHash;
use open_bitcoin_mempool::{PolicyConfig, PolicyTime, PreparedLifecycleFacts};
use open_bitcoin_network::{HeadersMessage, LocalPeerConfig, WireNetworkMessage};

use super::{
    AuthorityEpoch, LifecycleCommand, LifecycleCommandKind, LifecycleEvidenceSnapshot,
    LifecycleGeneration, LifecyclePreparationError, LifecycleProjectionPlan,
    PeerRelayPreparationRequest, PreparedCompactProjection, PreparedFanoutProjection,
    PreparedLifecycleEvidence, PreparedPeerLifecycleProjection, PreparedPersistenceProjection,
    PreparedServingProjection, PreparedUnbroadcastProjection, ProjectionShape,
    SnapshotPreparationRequest,
};
use crate::network::lifecycle_effects::{
    PeerEffectCapability, PeerEffectId, PeerEffectReceipt, PeerSessionGeneration,
    PreparedSnapshotWrite, SnapshotEffectId, SnapshotIdentity, SnapshotWriteReceipt,
};
use crate::{ManagedPeerNetwork, MemoryChainstateStore};

fn projection_plan() -> LifecycleProjectionPlan {
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    let core = network
        .mempool()
        .prepare_expiry(PolicyTime::new(0))
        .expect("empty mempool expiry should prepare");
    LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, core)
        .expect("coherent lifecycle facts should prepare every projection")
}

#[test]
fn command_family_names_every_lifecycle_and_effect_path() {
    // Arrange
    let expected = [
        LifecycleCommandKind::SingletonAdmission,
        LifecycleCommandKind::PackageAdmission,
        LifecycleCommandKind::Pressure,
        LifecycleCommandKind::Expiry,
        LifecycleCommandKind::ConnectedBlock,
        LifecycleCommandKind::ReorgStep,
        LifecycleCommandKind::Maintenance,
        LifecycleCommandKind::PrepareSnapshot,
        LifecycleCommandKind::PrepareRelay,
        LifecycleCommandKind::AbortPeerEffect,
        LifecycleCommandKind::CompletePeerEffect,
        LifecycleCommandKind::CompletePeerEmission,
        LifecycleCommandKind::CompleteSnapshotEffect,
    ];
    let epoch = AuthorityEpoch::INITIAL;
    let generation = LifecycleGeneration::INITIAL;
    let peer_receipt = PeerEffectCapability::new(
        epoch,
        generation,
        PeerEffectId::new(0),
        134_080,
        PeerSessionGeneration::INITIAL,
    )
    .acknowledge_write();
    let peer_abort = PeerEffectCapability::new(
        epoch,
        generation,
        PeerEffectId::new(2),
        134_080,
        PeerSessionGeneration::INITIAL,
    );
    let snapshot_receipt = PreparedSnapshotWrite::new(
        epoch,
        generation,
        SnapshotEffectId::new(0),
        SnapshotIdentity::new(0),
        crate::storage::MempoolSnapshot::default(),
    )
    .into_parts()
    .1
    .acknowledge_write();
    let emission_receipt = crate::network::PeerEmission::new(
        134_080,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: Vec::new(),
        }),
        BlockHash::from_byte_array([0x15; 32]),
        PeerEffectCapability::new(
            epoch,
            generation,
            PeerEffectId::new(1),
            134_080,
            PeerSessionGeneration::INITIAL,
        ),
    )
    .expect("headers emission should prepare")
    .into_parts()
    .2
    .acknowledge_write();
    let commands = [
        LifecycleCommand::SingletonAdmission(projection_plan()),
        LifecycleCommand::PackageAdmission(projection_plan()),
        LifecycleCommand::Pressure(projection_plan()),
        LifecycleCommand::Expiry(projection_plan()),
        LifecycleCommand::ConnectedBlock(projection_plan()),
        LifecycleCommand::ReorgStep(projection_plan()),
        LifecycleCommand::Maintenance(projection_plan()),
        LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new()),
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(134_080)),
        LifecycleCommand::AbortPeerEffect(peer_abort),
        LifecycleCommand::CompletePeerEffect(peer_receipt),
        LifecycleCommand::CompletePeerEmission(emission_receipt),
        LifecycleCommand::CompleteSnapshotEffect(snapshot_receipt),
    ];

    // Act
    let actual = commands.map(|command| command.kind());

    // Assert
    assert_eq!(actual, expected);
}

#[test]
fn epoch_and_generation_have_distinct_checked_sequences() {
    // Arrange
    let epoch = AuthorityEpoch::INITIAL;
    let generation = LifecycleGeneration::INITIAL;

    // Act
    let next_epoch = epoch.checked_next().expect("epoch should advance");
    let next_generation = generation
        .checked_next()
        .expect("generation should advance");

    // Assert
    assert_eq!(next_epoch.raw(), 2);
    assert_eq!(next_generation.raw(), 1);
    assert!(AuthorityEpoch::MAX.checked_next().is_err());
    assert!(LifecycleGeneration::MAX.checked_next().is_err());
}

#[test]
fn facts_plans_owned_effects_and_receipts_are_distinct_types() {
    // Arrange
    let type_ids = [
        TypeId::of::<PreparedLifecycleFacts>(),
        TypeId::of::<LifecycleProjectionPlan>(),
        TypeId::of::<PeerEffectCapability>(),
        TypeId::of::<PreparedSnapshotWrite>(),
        TypeId::of::<PeerEffectReceipt>(),
        TypeId::of::<SnapshotWriteReceipt>(),
    ];

    // Act
    let unique_count = type_ids
        .iter()
        .enumerate()
        .filter(|(index, type_id)| !type_ids[..*index].contains(type_id))
        .count();

    // Assert
    assert_eq!(unique_count, type_ids.len());
}

#[test]
fn construction_requires_authority_core_and_every_projection_target() {
    // Arrange
    let plan = projection_plan();

    // Act
    let LifecycleProjectionPlan {
        authority_epoch,
        core,
        compact,
        serving,
        fanout,
        peers,
        unbroadcast,
        persistence,
        evidence,
    } = plan;

    // Assert
    assert_eq!(authority_epoch, AuthorityEpoch::INITIAL);
    assert!(core.facts().delta().is_empty());
    assert_eq!(compact.replacement.iter_available().count(), 0);
    assert!(serving.transactions_by_txid.is_empty());
    assert!(serving.transactions_by_wtxid.is_empty());
    assert_eq!(serving.relay_serving.info().serveable_transactions, 0);
    assert_eq!(fanout.replacement.info().known_transactions, 0);
    assert!(peers.prepared.admission_order().is_empty());
    assert!(peers.prepared.teardown_order().is_empty());
    assert!(unbroadcast.replacement.is_empty());
    assert_eq!(
        persistence.lifecycle_generation,
        LifecycleGeneration::INITIAL
    );
    assert_eq!(persistence.dirty_generation, None);
    assert_eq!(evidence.replacement, LifecycleEvidenceSnapshot::default());
}

#[test]
fn every_projection_target_has_a_distinct_concrete_type() {
    // Arrange
    let type_ids = [
        TypeId::of::<PreparedCompactProjection>(),
        TypeId::of::<PreparedServingProjection>(),
        TypeId::of::<PreparedFanoutProjection>(),
        TypeId::of::<PreparedPeerLifecycleProjection>(),
        TypeId::of::<PreparedUnbroadcastProjection>(),
        TypeId::of::<PreparedPersistenceProjection>(),
        TypeId::of::<PreparedLifecycleEvidence>(),
    ];

    // Act
    let unique_count = type_ids
        .iter()
        .enumerate()
        .filter(|(index, type_id)| !type_ids[..*index].contains(type_id))
        .count();

    // Assert
    assert_eq!(unique_count, type_ids.len());
}

#[test]
fn preparation_rejects_incoherent_lifecycle_orders() {
    // Arrange
    let final_present_mismatch = ProjectionShape::checked_from_counts(1, 0, 0, 0, 0);
    let teardown_mismatch = ProjectionShape::checked_from_counts(0, 0, 1, 0, 0);

    // Act
    let maybe_final_present_error = final_present_mismatch.expect_err("order must match");
    let maybe_teardown_error = teardown_mismatch.expect_err("order must match");

    // Assert
    assert_eq!(
        maybe_final_present_error,
        LifecyclePreparationError::FinalPresentOrderMismatch {
            final_present: 1,
            admitted_order: 0,
        }
    );
    assert_eq!(
        maybe_teardown_error,
        LifecyclePreparationError::TeardownOrderMismatch {
            removed: 1,
            teardown_order: 0,
        }
    );
}

#[test]
fn empty_projection_applies_every_prepared_target_without_failure() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    let prepared = projection_plan();
    let LifecycleProjectionPlan {
        authority_epoch: _,
        core: _,
        compact,
        serving,
        fanout,
        peers,
        unbroadcast: _,
        persistence: _,
        evidence: _,
    } = prepared;

    // Act
    network.apply_prepared_compact(compact);
    network.apply_prepared_serving(serving);
    network.apply_prepared_fanout(fanout);
    network.apply_prepared_peer_lifecycle(peers);

    // Assert
    assert_eq!(network.compact_extra_txn.iter_available().count(), 0);
    assert_eq!(network.relay_serving.info().serveable_transactions, 0);
    assert_eq!(network.relay_fanout.info().known_transactions, 0);
}

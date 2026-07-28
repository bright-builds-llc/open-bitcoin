// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::any::TypeId;

use open_bitcoin_mempool::{PolicyConfig, PolicyTime, PreparedLifecycleFacts};
use open_bitcoin_network::LocalPeerConfig;

use super::{
    AuthorityEpoch, LifecycleCommand, LifecycleCommandKind, LifecycleGeneration,
    LifecycleProjectionPlan, OwnedPeerRelayEffects, OwnedSnapshotEffect, PeerEffectReceipt,
    PeerRelayPreparationRequest, SnapshotEffectReceipt, SnapshotPreparationRequest,
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
        LifecycleCommandKind::CompletePeerEffect,
        LifecycleCommandKind::CompleteSnapshotEffect,
    ];
    let epoch = AuthorityEpoch::INITIAL;
    let generation = LifecycleGeneration::INITIAL;
    let commands = [
        LifecycleCommand::SingletonAdmission(projection_plan()),
        LifecycleCommand::PackageAdmission(projection_plan()),
        LifecycleCommand::Pressure(projection_plan()),
        LifecycleCommand::Expiry(projection_plan()),
        LifecycleCommand::ConnectedBlock(projection_plan()),
        LifecycleCommand::ReorgStep(projection_plan()),
        LifecycleCommand::Maintenance(projection_plan()),
        LifecycleCommand::PrepareSnapshot(SnapshotPreparationRequest::new(epoch, generation)),
        LifecycleCommand::PrepareRelay(PeerRelayPreparationRequest::new(epoch, generation)),
        LifecycleCommand::CompletePeerEffect(PeerEffectReceipt::new(epoch, generation)),
        LifecycleCommand::CompleteSnapshotEffect(SnapshotEffectReceipt::new(epoch, generation)),
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
        TypeId::of::<OwnedPeerRelayEffects>(),
        TypeId::of::<OwnedSnapshotEffect>(),
        TypeId::of::<PeerEffectReceipt>(),
        TypeId::of::<SnapshotEffectReceipt>(),
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

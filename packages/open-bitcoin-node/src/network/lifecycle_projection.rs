// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sealed command vocabulary for authoritative mempool lifecycle projection.

use std::fmt;

use open_bitcoin_mempool::PreparedMempoolTransition;

/// Identifies one incarnation of the sole managed-network authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct AuthorityEpoch(u64);

impl AuthorityEpoch {
    pub(super) const INITIAL: Self = Self(1);
    pub(super) const MAX: Self = Self(u64::MAX);

    pub(super) const fn raw(self) -> u64 {
        self.0
    }

    pub(super) fn checked_next(self) -> Result<Self, LifecyclePreparationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(LifecyclePreparationError::AuthorityEpochExhausted)
    }
}

/// Identifies one committed, non-empty authoritative lifecycle transition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct LifecycleGeneration(u64);

impl LifecycleGeneration {
    pub(super) const INITIAL: Self = Self(0);
    pub(super) const MAX: Self = Self(u64::MAX);

    pub(super) const fn raw(self) -> u64 {
        self.0
    }

    pub(super) fn checked_next(self) -> Result<Self, LifecyclePreparationError> {
        self.0
            .checked_add(1)
            .map(Self)
            .ok_or(LifecyclePreparationError::LifecycleGenerationExhausted)
    }
}

/// Failures detected while preparing authority-bound lifecycle work.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LifecyclePreparationError {
    AuthorityEpochExhausted,
    LifecycleGenerationExhausted,
}

impl fmt::Display for LifecyclePreparationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorityEpochExhausted => formatter.write_str("authority epoch exhausted"),
            Self::LifecycleGenerationExhausted => {
                formatter.write_str("lifecycle generation exhausted")
            }
        }
    }
}

impl std::error::Error for LifecyclePreparationError {}

/// Prepared in-memory consequences remain distinct from committed lifecycle facts.
pub(super) struct LifecycleProjectionPlan {
    core: PreparedMempoolTransition,
}

impl LifecycleProjectionPlan {
    fn from_core(core: PreparedMempoolTransition) -> Self {
        Self { core }
    }
}

/// Owned snapshot work that may leave the authority lock in a later plan.
pub(super) struct OwnedSnapshotEffect {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

/// Owned peer-relay work that may leave the authority lock in a later plan.
pub(super) struct OwnedPeerRelayEffects {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

pub(super) struct SnapshotPreparationRequest {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

impl SnapshotPreparationRequest {
    fn new(authority_epoch: AuthorityEpoch, generation: LifecycleGeneration) -> Self {
        Self {
            authority_epoch,
            generation,
        }
    }
}

pub(super) struct PeerRelayPreparationRequest {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

impl PeerRelayPreparationRequest {
    fn new(authority_epoch: AuthorityEpoch, generation: LifecycleGeneration) -> Self {
        Self {
            authority_epoch,
            generation,
        }
    }
}

/// Proof that one owned peer effect achieved its external write.
pub(super) struct PeerEffectReceipt {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

impl PeerEffectReceipt {
    fn new(authority_epoch: AuthorityEpoch, generation: LifecycleGeneration) -> Self {
        Self {
            authority_epoch,
            generation,
        }
    }
}

/// Proof that one owned snapshot effect achieved its durable write.
pub(super) struct SnapshotEffectReceipt {
    authority_epoch: AuthorityEpoch,
    generation: LifecycleGeneration,
}

impl SnapshotEffectReceipt {
    fn new(authority_epoch: AuthorityEpoch, generation: LifecycleGeneration) -> Self {
        Self {
            authority_epoch,
            generation,
        }
    }
}

/// The sole typed vocabulary for lifecycle mutation and effect preparation/completion.
pub(super) enum LifecycleCommand {
    SingletonAdmission(LifecycleProjectionPlan),
    PackageAdmission(LifecycleProjectionPlan),
    Pressure(LifecycleProjectionPlan),
    Expiry(LifecycleProjectionPlan),
    ConnectedBlock(LifecycleProjectionPlan),
    ReorgStep(LifecycleProjectionPlan),
    Maintenance(LifecycleProjectionPlan),
    PrepareSnapshot(SnapshotPreparationRequest),
    PrepareRelay(PeerRelayPreparationRequest),
    CompletePeerEffect(PeerEffectReceipt),
    CompleteSnapshotEffect(SnapshotEffectReceipt),
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LifecycleCommandKind {
    SingletonAdmission,
    PackageAdmission,
    Pressure,
    Expiry,
    ConnectedBlock,
    ReorgStep,
    Maintenance,
    PrepareSnapshot,
    PrepareRelay,
    CompletePeerEffect,
    CompleteSnapshotEffect,
}

#[cfg(test)]
impl LifecycleCommand {
    const fn kind(&self) -> LifecycleCommandKind {
        match self {
            Self::SingletonAdmission(_) => LifecycleCommandKind::SingletonAdmission,
            Self::PackageAdmission(_) => LifecycleCommandKind::PackageAdmission,
            Self::Pressure(_) => LifecycleCommandKind::Pressure,
            Self::Expiry(_) => LifecycleCommandKind::Expiry,
            Self::ConnectedBlock(_) => LifecycleCommandKind::ConnectedBlock,
            Self::ReorgStep(_) => LifecycleCommandKind::ReorgStep,
            Self::Maintenance(_) => LifecycleCommandKind::Maintenance,
            Self::PrepareSnapshot(_) => LifecycleCommandKind::PrepareSnapshot,
            Self::PrepareRelay(_) => LifecycleCommandKind::PrepareRelay,
            Self::CompletePeerEffect(_) => LifecycleCommandKind::CompletePeerEffect,
            Self::CompleteSnapshotEffect(_) => LifecycleCommandKind::CompleteSnapshotEffect,
        }
    }
}

#[cfg(test)]
mod command_contract {
    use std::any::TypeId;

    use open_bitcoin_mempool::{Mempool, PolicyTime, PreparedLifecycleFacts};

    use super::{
        AuthorityEpoch, LifecycleCommand, LifecycleCommandKind, LifecycleGeneration,
        LifecycleProjectionPlan, OwnedPeerRelayEffects, OwnedSnapshotEffect, PeerEffectReceipt,
        PeerRelayPreparationRequest, SnapshotEffectReceipt, SnapshotPreparationRequest,
    };

    fn projection_plan() -> LifecycleProjectionPlan {
        let core = Mempool::default()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty mempool expiry should prepare");
        LifecycleProjectionPlan::from_core(core)
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
}

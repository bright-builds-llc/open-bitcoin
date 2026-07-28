// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sealed command vocabulary for authoritative mempool lifecycle projection.

use std::fmt;

use open_bitcoin_mempool::{PreparedLifecycleFacts, PreparedMempoolTransition};

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
    FinalPresentOrderMismatch {
        final_present: usize,
        admitted_order: usize,
    },
    TeardownOrderMismatch {
        removed: usize,
        teardown_order: usize,
    },
}

impl fmt::Display for LifecyclePreparationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorityEpochExhausted => formatter.write_str("authority epoch exhausted"),
            Self::LifecycleGenerationExhausted => {
                formatter.write_str("lifecycle generation exhausted")
            }
            Self::FinalPresentOrderMismatch {
                final_present,
                admitted_order,
            } => write!(
                formatter,
                "final-present count {final_present} does not match admitted order {admitted_order}"
            ),
            Self::TeardownOrderMismatch {
                removed,
                teardown_order,
            } => write!(
                formatter,
                "removed count {removed} does not match teardown order {teardown_order}"
            ),
        }
    }
}

impl std::error::Error for LifecyclePreparationError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ProjectionShape {
    admitted: usize,
    removed: usize,
    retry_clears: usize,
}

impl ProjectionShape {
    fn prepare(facts: &PreparedLifecycleFacts) -> Result<Self, LifecyclePreparationError> {
        Self::checked_from_counts(
            facts.final_present().len(),
            facts.admitted_order().len(),
            facts.removed().len(),
            facts.teardown_order().len(),
            facts.delta().retry_clears.len(),
        )
    }

    fn checked_from_counts(
        final_present: usize,
        admitted_order: usize,
        removed: usize,
        teardown_order: usize,
        retry_clears: usize,
    ) -> Result<Self, LifecyclePreparationError> {
        if final_present != admitted_order {
            return Err(LifecyclePreparationError::FinalPresentOrderMismatch {
                final_present,
                admitted_order,
            });
        }
        if removed != teardown_order {
            return Err(LifecyclePreparationError::TeardownOrderMismatch {
                removed,
                teardown_order,
            });
        }
        Ok(Self {
            admitted: admitted_order,
            removed: teardown_order,
            retry_clears,
        })
    }
}

/// Exact compact-reconstruction projection shape.
pub(super) struct PreparedCompactProjection(ProjectionShape);

/// Exact accepted-serving projection shape.
pub(super) struct PreparedServingProjection(ProjectionShape);

/// Exact announcement/fanout projection shape.
pub(super) struct PreparedFanoutProjection(ProjectionShape);

/// Exact peer request, known-set, orphan, candidate, and fingerprint projection shape.
pub(super) struct PreparedPeerLifecycleProjection(ProjectionShape);

/// Exact unbroadcast bookkeeping projection shape.
pub(super) struct PreparedUnbroadcastProjection(ProjectionShape);

/// Exact persistence-view projection shape.
pub(super) struct PreparedPersistenceProjection(ProjectionShape);

/// Exact lifecycle status, metrics, and log-evidence projection shape.
pub(super) struct PreparedLifecycleEvidence(ProjectionShape);

/// Prepared in-memory consequences remain distinct from committed lifecycle facts.
pub(super) struct LifecycleProjectionPlan {
    authority_epoch: AuthorityEpoch,
    core: PreparedMempoolTransition,
    compact: PreparedCompactProjection,
    serving: PreparedServingProjection,
    fanout: PreparedFanoutProjection,
    peers: PreparedPeerLifecycleProjection,
    unbroadcast: PreparedUnbroadcastProjection,
    persistence: PreparedPersistenceProjection,
    evidence: PreparedLifecycleEvidence,
}

impl LifecycleProjectionPlan {
    fn prepare(
        authority_epoch: AuthorityEpoch,
        core: PreparedMempoolTransition,
    ) -> Result<Self, LifecyclePreparationError> {
        let shape = ProjectionShape::prepare(core.facts())?;
        Ok(Self {
            authority_epoch,
            core,
            compact: PreparedCompactProjection(shape),
            serving: PreparedServingProjection(shape),
            fanout: PreparedFanoutProjection(shape),
            peers: PreparedPeerLifecycleProjection(shape),
            unbroadcast: PreparedUnbroadcastProjection(shape),
            persistence: PreparedPersistenceProjection(shape),
            evidence: PreparedLifecycleEvidence(shape),
        })
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
        LifecycleProjectionPlan::prepare(AuthorityEpoch::INITIAL, core)
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
}

#[cfg(test)]
mod closed_plan {
    use std::any::TypeId;

    use open_bitcoin_mempool::PolicyTime;

    use crate::ManagedMempool;

    use super::{
        AuthorityEpoch, LifecyclePreparationError, LifecycleProjectionPlan,
        PreparedCompactProjection, PreparedFanoutProjection, PreparedLifecycleEvidence,
        PreparedPeerLifecycleProjection, PreparedPersistenceProjection, PreparedServingProjection,
        PreparedUnbroadcastProjection, ProjectionShape,
    };

    #[test]
    fn construction_requires_authority_core_and_every_projection_target() {
        // Arrange
        let core = ManagedMempool::default()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty mempool expiry should prepare");

        // Act
        let plan = LifecycleProjectionPlan::prepare(AuthorityEpoch::INITIAL, core)
            .expect("coherent lifecycle facts should prepare every projection");
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
        assert_eq!(compact.0, serving.0);
        assert_eq!(fanout.0, peers.0);
        assert_eq!(unbroadcast.0, persistence.0);
        assert_eq!(evidence.0, compact.0);
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
}

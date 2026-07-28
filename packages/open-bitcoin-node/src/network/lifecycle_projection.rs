// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sealed command vocabulary for authoritative mempool lifecycle projection.

use std::collections::BTreeMap;
use std::fmt;

use open_bitcoin_core::primitives::{Transaction, Txid, Wtxid};
use open_bitcoin_mempool::{MempoolError, PreparedLifecycleFacts, PreparedMempoolTransition};
use open_bitcoin_network::{
    AcceptedPeerPackageFingerprint, PeerTransactionIdentity, PeerTransactionLifecycleInput,
    PeerTransactionLifecyclePreparationError, PreparedPeerTransactionLifecycle,
};

use super::ManagedPeerNetwork;
use super::compact_receive_candidates::CompactExtraTxnBuffer;
use super::relay_fanout::ManagedRelayFanoutState;
use super::relay_serving::RelayServingCache;
use crate::ChainstateStore;

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
#[derive(Debug, Clone, PartialEq, Eq)]
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
    CompactTransactionBody(MempoolError),
    PeerLifecycle(PeerTransactionLifecyclePreparationError),
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
            Self::CompactTransactionBody(error) => {
                write!(formatter, "compact transaction preparation failed: {error}")
            }
            Self::PeerLifecycle(error) => {
                write!(formatter, "peer lifecycle preparation failed: {error}")
            }
        }
    }
}

impl std::error::Error for LifecyclePreparationError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CompactTransactionBody(error) => Some(error),
            Self::PeerLifecycle(error) => Some(error),
            Self::AuthorityEpochExhausted
            | Self::LifecycleGenerationExhausted
            | Self::FinalPresentOrderMismatch { .. }
            | Self::TeardownOrderMismatch { .. } => None,
        }
    }
}

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

/// Exact compact-reconstruction projection.
pub(super) struct PreparedCompactProjection {
    pub(super) replacement: CompactExtraTxnBuffer,
}

/// Exact accepted-serving and body-index projection.
pub(super) struct PreparedServingProjection {
    pub(super) transactions_by_txid: BTreeMap<Txid, Transaction>,
    pub(super) transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
    pub(super) relay_serving: RelayServingCache,
}

/// Exact announcement/fanout projection.
pub(super) struct PreparedFanoutProjection {
    pub(super) replacement: ManagedRelayFanoutState,
}

/// Exact peer request, known-set, orphan, candidate, and fingerprint projection.
pub(super) struct PreparedPeerLifecycleProjection {
    prepared: PreparedPeerTransactionLifecycle,
}

/// Exact unbroadcast bookkeeping projection shape.
pub(super) struct PreparedUnbroadcastProjection(ProjectionShape);

/// Exact persistence-view projection shape.
pub(super) struct PreparedPersistenceProjection(ProjectionShape);

/// Exact lifecycle status, metrics, and log-evidence projection shape.
pub(super) struct PreparedLifecycleEvidence(ProjectionShape);

/// Prepared in-memory consequences remain distinct from committed lifecycle facts.
pub(super) struct LifecycleProjectionPlan {
    pub(super) authority_epoch: AuthorityEpoch,
    pub(super) core: PreparedMempoolTransition,
    pub(super) compact: PreparedCompactProjection,
    pub(super) serving: PreparedServingProjection,
    pub(super) fanout: PreparedFanoutProjection,
    pub(super) peers: PreparedPeerLifecycleProjection,
    pub(super) unbroadcast: PreparedUnbroadcastProjection,
    pub(super) persistence: PreparedPersistenceProjection,
    pub(super) evidence: PreparedLifecycleEvidence,
}

impl LifecycleProjectionPlan {
    pub(super) fn prepare<S: ChainstateStore>(
        network: &ManagedPeerNetwork<S>,
        authority_epoch: AuthorityEpoch,
        core: PreparedMempoolTransition,
    ) -> Result<Self, LifecyclePreparationError> {
        let facts = core.facts();
        let shape = ProjectionShape::prepare(facts)?;
        let compact = network
            .prepare_compact_projection(facts)
            .map_err(LifecyclePreparationError::CompactTransactionBody)?;
        let serving = network.prepare_serving_projection(facts);
        let fanout = network.prepare_fanout_projection(facts);
        let peers = prepare_peer_projection(network, facts)?;
        Ok(Self {
            authority_epoch,
            core,
            compact,
            serving,
            fanout,
            peers,
            unbroadcast: PreparedUnbroadcastProjection(shape),
            persistence: PreparedPersistenceProjection(shape),
            evidence: PreparedLifecycleEvidence(shape),
        })
    }
}

fn prepare_peer_projection<S: ChainstateStore>(
    network: &ManagedPeerNetwork<S>,
    facts: &PreparedLifecycleFacts,
) -> Result<PreparedPeerLifecycleProjection, LifecyclePreparationError> {
    let admissions = facts
        .final_present()
        .iter()
        .map(|member| peer_identity(member.member))
        .collect::<Vec<_>>();
    let teardowns = facts
        .teardown_order()
        .iter()
        .copied()
        .map(peer_identity)
        .collect::<Vec<_>>();
    let accepted_packages = facts
        .maybe_package_report()
        .filter(|_| !admissions.is_empty())
        .map(|report| {
            vec![AcceptedPeerPackageFingerprint::new(
                *report.fingerprint().as_bytes(),
                admissions.clone(),
            )]
        })
        .unwrap_or_default();
    let input = PeerTransactionLifecycleInput::new(admissions, teardowns, accepted_packages);
    let prepared = network
        .peer_manager
        .prepare_transaction_lifecycle(input)
        .map_err(LifecyclePreparationError::PeerLifecycle)?;
    Ok(PreparedPeerLifecycleProjection { prepared })
}

const fn peer_identity(
    member: open_bitcoin_mempool::MempoolMemberIdentity,
) -> PeerTransactionIdentity {
    PeerTransactionIdentity::new(member.txid, member.wtxid)
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn apply_prepared_peer_lifecycle(
        &mut self,
        prepared: PreparedPeerLifecycleProjection,
    ) {
        self.peer_manager
            .apply_prepared_transaction_lifecycle(prepared.prepared);
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
mod tests;

#[cfg(test)]
mod closed_plan {
    use std::any::TypeId;

    use open_bitcoin_mempool::{PolicyConfig, PolicyTime};
    use open_bitcoin_network::LocalPeerConfig;

    use crate::{ManagedPeerNetwork, MemoryChainstateStore};

    use super::{
        AuthorityEpoch, LifecyclePreparationError, LifecycleProjectionPlan,
        PreparedCompactProjection, PreparedFanoutProjection, PreparedLifecycleEvidence,
        PreparedPeerLifecycleProjection, PreparedPersistenceProjection, PreparedServingProjection,
        PreparedUnbroadcastProjection, ProjectionShape,
    };

    #[test]
    fn construction_requires_authority_core_and_every_projection_target() {
        // Arrange
        let network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig::default(),
            PolicyConfig::default(),
        );
        let core = network
            .mempool()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty mempool expiry should prepare");

        // Act
        let plan = LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, core)
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
        assert_eq!(compact.replacement.iter_available().count(), 0);
        assert!(serving.transactions_by_txid.is_empty());
        assert!(serving.transactions_by_wtxid.is_empty());
        assert_eq!(serving.relay_serving.info().serveable_transactions, 0);
        assert_eq!(fanout.replacement.info().known_transactions, 0);
        assert!(peers.prepared.admission_order().is_empty());
        assert!(peers.prepared.teardown_order().is_empty());
        assert_eq!(unbroadcast.0, persistence.0);
        assert_eq!(evidence.0, unbroadcast.0);
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

#[cfg(test)]
mod target_operations {
    use open_bitcoin_mempool::{PolicyConfig, PolicyTime};
    use open_bitcoin_network::LocalPeerConfig;

    use crate::{ManagedPeerNetwork, MemoryChainstateStore};

    use super::{AuthorityEpoch, LifecycleProjectionPlan};

    #[test]
    fn empty_projection_applies_every_prepared_target_without_failure() {
        // Arrange
        let mut network = ManagedPeerNetwork::new(
            MemoryChainstateStore::default(),
            LocalPeerConfig::default(),
            PolicyConfig::default(),
        );
        let core = network
            .mempool()
            .prepare_expiry(PolicyTime::new(0))
            .expect("empty mempool expiry should prepare");
        let prepared = LifecycleProjectionPlan::prepare(&network, AuthorityEpoch::INITIAL, core)
            .expect("empty target work should prepare");
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
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Sealed command vocabulary for authoritative mempool lifecycle projection.

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_core::primitives::{Transaction, Txid, Wtxid};
use open_bitcoin_mempool::{
    MempoolError, MempoolMemberIdentity, PreparedLifecycleFacts, PreparedMempoolTransition,
};
use open_bitcoin_network::{
    AcceptedPeerPackageFingerprint, PeerId, PeerTransactionIdentity, PeerTransactionLifecycleInput,
    PeerTransactionLifecyclePreparationError, PreparedPeerTransactionLifecycle,
};

use super::ManagedPeerNetwork;
use super::announcement_transport::PeerEmissionReceipt;
use super::compact_receive_candidates::CompactExtraTxnBuffer;
use super::lifecycle_effects::{
    EffectPreparationError, PeerEffectCapability, PeerEffectReceipt, SnapshotWriteReceipt,
};
use super::relay_fanout::ManagedRelayFanoutState;
use super::relay_serving::RelayServingCache;
use crate::ChainstateStore;

mod authority;
mod reconciliation;

#[cfg(test)]
pub(in crate::network) use reconciliation::LifecycleReconciliationReport;

pub(super) const MAX_UNBROADCAST_MEMBERS: usize = 5_000;

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
    UnbroadcastCapacity {
        attempted: usize,
        capacity: usize,
    },
    #[cfg(test)]
    InjectedFailure(LifecyclePreparationFailurePoint),
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
            Self::UnbroadcastCapacity {
                attempted,
                capacity,
            } => write!(
                formatter,
                "unbroadcast membership count {attempted} exceeds capacity {capacity}"
            ),
            #[cfg(test)]
            Self::InjectedFailure(point) => {
                write!(
                    formatter,
                    "injected lifecycle preparation failure at {point:?}"
                )
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
            | Self::TeardownOrderMismatch { .. }
            | Self::UnbroadcastCapacity { .. } => None,
            #[cfg(test)]
            Self::InjectedFailure(_) => None,
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::network) enum LifecyclePreparationFailurePoint {
    Identity,
    Body,
    Fingerprint,
    FinalMembership,
    Serving,
    Fanout,
    Peer,
    Compact,
    Unbroadcast,
    Generation,
    Evidence,
}

#[cfg(test)]
impl LifecyclePreparationFailurePoint {
    pub(in crate::network) const ALL: [Self; 11] = [
        Self::Identity,
        Self::Body,
        Self::Fingerprint,
        Self::FinalMembership,
        Self::Serving,
        Self::Fanout,
        Self::Peer,
        Self::Compact,
        Self::Unbroadcast,
        Self::Generation,
        Self::Evidence,
    ];
}

#[cfg(test)]
thread_local! {
    static INJECTED_PREPARATION_FAILURE: Cell<Option<LifecyclePreparationFailurePoint>> =
        const { Cell::new(None) };
}

#[cfg(test)]
pub(in crate::network) struct LifecyclePreparationFailureGuard {
    previous: Option<LifecyclePreparationFailurePoint>,
}

#[cfg(test)]
impl LifecyclePreparationFailureGuard {
    pub(in crate::network) fn inject(point: LifecyclePreparationFailurePoint) -> Self {
        let previous = INJECTED_PREPARATION_FAILURE.replace(Some(point));
        Self { previous }
    }
}

#[cfg(test)]
impl Drop for LifecyclePreparationFailureGuard {
    fn drop(&mut self) {
        INJECTED_PREPARATION_FAILURE.set(self.previous);
    }
}

#[cfg(test)]
fn fail_preparation_at(
    point: LifecyclePreparationFailurePoint,
) -> Result<(), LifecyclePreparationError> {
    if INJECTED_PREPARATION_FAILURE.get() == Some(point) {
        return Err(LifecyclePreparationError::InjectedFailure(point));
    }
    Ok(())
}

/// Failures detected by the sole authority validation boundary.
#[derive(Debug)]
pub(super) enum LifecycleProjectionError {
    AuthorityUnavailable,
    StaleAuthorityEpoch {
        expected: AuthorityEpoch,
        actual: AuthorityEpoch,
    },
    EffectPreparation(EffectPreparationError),
    InvalidEffectReceipt(&'static str),
    PeerEvidence(super::types::ManagedNetworkError),
    Mempool(MempoolError),
}

impl fmt::Display for LifecycleProjectionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthorityUnavailable => {
                formatter.write_str("authoritative lifecycle state is unavailable")
            }
            Self::StaleAuthorityEpoch { expected, actual } => write!(
                formatter,
                "stale lifecycle authority epoch: expected {}, actual {}",
                expected.raw(),
                actual.raw()
            ),
            Self::EffectPreparation(error) => match error {
                EffectPreparationError::PeerEffectsAtCapacity => {
                    formatter.write_str("pending peer lifecycle effects are at capacity")
                }
                EffectPreparationError::SnapshotEffectPending => {
                    formatter.write_str("a mempool snapshot write is already pending")
                }
                EffectPreparationError::EffectIdentityCollision => {
                    formatter.write_str("lifecycle effect identity collision")
                }
                EffectPreparationError::EffectIdentityExhausted => {
                    formatter.write_str("lifecycle effect identity exhausted")
                }
            },
            Self::InvalidEffectReceipt(family) => {
                write!(formatter, "foreign or mismatched {family} effect receipt")
            }
            Self::PeerEvidence(error) => error.fmt(formatter),
            Self::Mempool(error) => error.fmt(formatter),
        }
    }
}

impl std::error::Error for LifecycleProjectionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Mempool(error) => Some(error),
            Self::PeerEvidence(error) => Some(error),
            Self::AuthorityUnavailable
            | Self::StaleAuthorityEpoch { .. }
            | Self::InvalidEffectReceipt(_)
            | Self::EffectPreparation(_) => None,
        }
    }
}

impl From<EffectPreparationError> for LifecycleProjectionError {
    fn from(value: EffectPreparationError) -> Self {
        Self::EffectPreparation(value)
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

/// Fixed low-cardinality lifecycle evidence.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(super) struct LifecycleEvidenceSnapshot {
    pub(super) committed_transitions: u64,
    pub(super) admitted_members: u64,
    pub(super) removed_members: u64,
    pub(super) retry_clears: u64,
    pub(super) replacement_removals: u64,
    pub(super) expiry_removals: u64,
    pub(super) pressure_removals: u64,
    pub(super) block_confirmation_removals: u64,
    pub(super) block_conflict_removals: u64,
    pub(super) reorg_removals: u64,
}

/// Exact unbroadcast bookkeeping projection.
pub(super) struct PreparedUnbroadcastProjection {
    replacement: BTreeSet<MempoolMemberIdentity>,
}

/// Exact persistence-view projection.
pub(super) struct PreparedPersistenceProjection {
    lifecycle_generation: LifecycleGeneration,
    dirty_generation: Option<LifecycleGeneration>,
}

/// Exact lifecycle status, metrics, and log-evidence projection.
pub(super) struct PreparedLifecycleEvidence {
    replacement: LifecycleEvidenceSnapshot,
}

/// Exact admission provenance used only while preparing fanout consequences.
pub(super) enum AdmissionProjectionSource {
    None,
    Local,
    Peer(BTreeMap<MempoolMemberIdentity, PeerId>),
}

impl AdmissionProjectionSource {
    pub(super) fn peer(origins: impl IntoIterator<Item = (MempoolMemberIdentity, PeerId)>) -> Self {
        Self::Peer(origins.into_iter().collect())
    }

    pub(super) fn maybe_origin_peer(&self, member: MempoolMemberIdentity) -> Option<PeerId> {
        let Self::Peer(origins) = self else {
            return None;
        };
        origins.get(&member).copied()
    }
}

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
        Self::prepare_admission(
            network,
            authority_epoch,
            core,
            AdmissionProjectionSource::None,
        )
    }

    pub(super) fn prepare_admission<S: ChainstateStore>(
        network: &ManagedPeerNetwork<S>,
        authority_epoch: AuthorityEpoch,
        core: PreparedMempoolTransition,
        source: AdmissionProjectionSource,
    ) -> Result<Self, LifecyclePreparationError> {
        let facts = core.facts();
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Identity)?;
        ProjectionShape::prepare(facts)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Body)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Fingerprint)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::FinalMembership)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Compact)?;
        let compact = network
            .prepare_compact_projection(facts)
            .map_err(LifecyclePreparationError::CompactTransactionBody)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Serving)?;
        let serving = network.prepare_serving_projection(facts);
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Fanout)?;
        let fanout = network.prepare_fanout_projection(facts, &source);
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Peer)?;
        let peers = prepare_peer_projection(network, facts)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Unbroadcast)?;
        let unbroadcast = network.prepare_unbroadcast_projection(facts)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Generation)?;
        let persistence = network.prepare_persistence_projection(facts)?;
        #[cfg(test)]
        fail_preparation_at(LifecyclePreparationFailurePoint::Evidence)?;
        let evidence = network.prepare_lifecycle_evidence(facts);
        Ok(Self {
            authority_epoch,
            core,
            compact,
            serving,
            fanout,
            peers,
            unbroadcast,
            persistence,
            evidence,
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

pub(in crate::network) struct SnapshotPreparationRequest;

impl SnapshotPreparationRequest {
    pub(in crate::network) const fn new() -> Self {
        Self
    }
}

pub(in crate::network) struct PeerRelayPreparationRequest {
    pub(in crate::network) peer_id: PeerId,
}

impl PeerRelayPreparationRequest {
    pub(in crate::network) const fn new(peer_id: PeerId) -> Self {
        Self { peer_id }
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
    AbortPeerEffect(PeerEffectCapability),
    CompletePeerEffect(PeerEffectReceipt),
    CompletePeerEmission(PeerEmissionReceipt),
    CompleteSnapshotEffect(SnapshotWriteReceipt),
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
    AbortPeerEffect,
    CompletePeerEffect,
    CompletePeerEmission,
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
            Self::AbortPeerEffect(_) => LifecycleCommandKind::AbortPeerEffect,
            Self::CompletePeerEffect(_) => LifecycleCommandKind::CompletePeerEffect,
            Self::CompletePeerEmission(_) => LifecycleCommandKind::CompletePeerEmission,
            Self::CompleteSnapshotEffect(_) => LifecycleCommandKind::CompleteSnapshotEffect,
        }
    }
}

#[cfg(test)]
mod tests;

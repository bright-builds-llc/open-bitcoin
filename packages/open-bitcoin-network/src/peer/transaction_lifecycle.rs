// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

//! Bounded, consuming peer-local mempool lifecycle operations.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_primitives::{Txid, Wtxid};

use crate::compact_download::CompactDownloadPeerState;
use crate::error::PeerId;

use super::{
    PHASE102_MAX_ORPHAN_TRANSACTIONS, PHASE102_MAX_ORPHANS_PER_PEER,
    PHASE102_MAX_RECONSIDERATIONS_PER_PARENT, PeerManager, TxRelayId,
};

mod reconciliation;

pub use reconciliation::PeerMempoolLifecycleSnapshot;

/// One canonical transaction identity supplied by authoritative lifecycle facts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PeerTransactionIdentity {
    txid: Txid,
    wtxid: Wtxid,
}

impl PeerTransactionIdentity {
    /// Creates one already-derived txid/wtxid pair.
    pub const fn new(txid: Txid, wtxid: Wtxid) -> Self {
        Self { txid, wtxid }
    }

    /// Returns the legacy transaction identifier.
    pub const fn txid(self) -> Txid {
        self.txid
    }

    /// Returns the witness transaction identifier.
    pub const fn wtxid(self) -> Wtxid {
        self.wtxid
    }

    const fn relay_ids(self) -> [TxRelayId; 2] {
        [TxRelayId::Txid(self.txid), TxRelayId::Wtxid(self.wtxid)]
    }
}

/// One accepted package alias and the canonical members it represents.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AcceptedPeerPackageFingerprint {
    fingerprint: [u8; 32],
    members: Vec<PeerTransactionIdentity>,
}

impl AcceptedPeerPackageFingerprint {
    /// Creates a forward fingerprint-to-members association.
    pub fn new(fingerprint: [u8; 32], members: Vec<PeerTransactionIdentity>) -> Self {
        Self {
            fingerprint,
            members,
        }
    }

    /// Returns the accepted fingerprint.
    pub const fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    /// Returns request-ordered package members.
    pub fn members(&self) -> &[PeerTransactionIdentity] {
        &self.members
    }
}

/// Ordered authoritative facts used to prepare one peer-local operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PeerTransactionLifecycleInput {
    admissions: Vec<PeerTransactionIdentity>,
    teardowns: Vec<PeerTransactionIdentity>,
    accepted_packages: Vec<AcceptedPeerPackageFingerprint>,
}

impl PeerTransactionLifecycleInput {
    /// Creates an ordered input. Preparation validates identities and all bounds.
    pub fn new(
        admissions: Vec<PeerTransactionIdentity>,
        teardowns: Vec<PeerTransactionIdentity>,
        accepted_packages: Vec<AcceptedPeerPackageFingerprint>,
    ) -> Self {
        Self {
            admissions,
            teardowns,
            accepted_packages,
        }
    }

    /// Returns parent-first final-present identities.
    pub fn admissions(&self) -> &[PeerTransactionIdentity] {
        &self.admissions
    }

    /// Returns descendant-first final-absent identities.
    pub fn teardowns(&self) -> &[PeerTransactionIdentity] {
        &self.teardowns
    }

    /// Returns accepted package aliases.
    pub fn accepted_packages(&self) -> &[AcceptedPeerPackageFingerprint] {
        &self.accepted_packages
    }
}

/// A peer-local lifecycle preparation failure detected before mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeerTransactionLifecyclePreparationError {
    IdentityWorkLimit { count: usize, maximum: usize },
    PackageMemberLimit { count: usize, maximum: usize },
    FingerprintLimit { count: usize, maximum: usize },
    FingerprintRetirementLimit { count: usize, maximum: usize },
    OrphanPeerLimit { count: usize, maximum: usize },
    CandidateLimit { count: usize, maximum: usize },
    TxidAliasConflict { txid: Txid },
    WtxidAliasConflict { wtxid: Wtxid },
    FinalMembershipConflict { txid: Txid, wtxid: Wtxid },
    FingerprintMembersConflict { fingerprint: [u8; 32] },
}

impl fmt::Display for PeerTransactionLifecyclePreparationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IdentityWorkLimit { count, maximum } => {
                write!(
                    formatter,
                    "peer identity work {count} exceeds limit {maximum}"
                )
            }
            Self::PackageMemberLimit { count, maximum } => {
                write!(formatter, "package members {count} exceeds limit {maximum}")
            }
            Self::FingerprintLimit { count, maximum } => {
                write!(
                    formatter,
                    "active fingerprints {count} exceeds limit {maximum}"
                )
            }
            Self::FingerprintRetirementLimit { count, maximum } => write!(
                formatter,
                "fingerprint retirements {count} exceeds limit {maximum}"
            ),
            Self::OrphanPeerLimit { count, maximum } => {
                write!(
                    formatter,
                    "peer orphan associations {count} exceeds limit {maximum}"
                )
            }
            Self::CandidateLimit { count, maximum } => {
                write!(
                    formatter,
                    "candidate members {count} exceeds limit {maximum}"
                )
            }
            Self::TxidAliasConflict { txid } => {
                write!(formatter, "txid {txid:?} maps to conflicting wtxids")
            }
            Self::WtxidAliasConflict { wtxid } => {
                write!(formatter, "wtxid {wtxid:?} maps to conflicting txids")
            }
            Self::FinalMembershipConflict { txid, wtxid } => write!(
                formatter,
                "identity ({txid:?}, {wtxid:?}) is both admitted and removed"
            ),
            Self::FingerprintMembersConflict { fingerprint } => {
                write!(
                    formatter,
                    "fingerprint {fingerprint:?} maps to conflicting members"
                )
            }
        }
    }
}

impl std::error::Error for PeerTransactionLifecyclePreparationError {}

struct PreparedOrphanLifecycle {
    orphan_removals: Box<[PeerTransactionIdentity]>,
    candidate_removals: Box<[(Wtxid, PeerId)]>,
    fingerprint_retirements: Box<[[u8; 32]]>,
    fingerprint_admissions: Box<[([u8; 32], BTreeSet<Wtxid>)]>,
}

struct PreparedCompactPartialLifecycle {
    replacements: Box<[(PeerId, CompactDownloadPeerState)]>,
}

/// Non-forgeable, exact peer-local lifecycle work consumed by [`PeerManager`].
pub struct PreparedPeerTransactionLifecycle {
    admission_order: Box<[PeerTransactionIdentity]>,
    teardown_order: Box<[PeerTransactionIdentity]>,
    request_admissions: Box<[[TxRelayId; 2]]>,
    request_teardowns: Box<[[TxRelayId; 2]]>,
    known_admissions: Box<[PeerTransactionIdentity]>,
    known_teardowns: Box<[PeerTransactionIdentity]>,
    orphan: PreparedOrphanLifecycle,
    compact: PreparedCompactPartialLifecycle,
}

impl PreparedPeerTransactionLifecycle {
    /// Returns the preserved parent-first admission order.
    pub fn admission_order(&self) -> &[PeerTransactionIdentity] {
        &self.admission_order
    }

    /// Returns the preserved descendant-first teardown order.
    pub fn teardown_order(&self) -> &[PeerTransactionIdentity] {
        &self.teardown_order
    }
}

impl PeerManager {
    /// Prepares every peer-local consequence without mutating peer state.
    pub fn prepare_transaction_lifecycle(
        &self,
        input: PeerTransactionLifecycleInput,
    ) -> Result<PreparedPeerTransactionLifecycle, PeerTransactionLifecyclePreparationError> {
        validate_identity_work_bounds(&input)?;
        validate_identity_aliases(&input)?;
        validate_orphan_bounds(self)?;

        let admission_set: BTreeSet<_> = input.admissions.iter().copied().collect();
        let teardown_set: BTreeSet<_> = input.teardowns.iter().copied().collect();
        if let Some(identity) = admission_set.intersection(&teardown_set).next() {
            return Err(
                PeerTransactionLifecyclePreparationError::FinalMembershipConflict {
                    txid: identity.txid,
                    wtxid: identity.wtxid,
                },
            );
        }

        let request_admissions = input
            .admissions
            .iter()
            .copied()
            .map(PeerTransactionIdentity::relay_ids)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let affected_orphan_identities = admission_set
            .union(&teardown_set)
            .copied()
            .collect::<BTreeSet<_>>();
        let orphan = prepare_orphan_lifecycle(self, &input, &affected_orphan_identities)?;
        let mut teardown_identities = input.teardowns.clone();
        let mut teardown_identity_set =
            teardown_identities.iter().copied().collect::<BTreeSet<_>>();
        teardown_identities.extend(
            orphan
                .orphan_removals
                .iter()
                .copied()
                .filter(|identity| teardown_identity_set.insert(*identity)),
        );
        let request_teardowns = teardown_identities
            .iter()
            .copied()
            .map(PeerTransactionIdentity::relay_ids)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let compact = prepare_compact_lifecycle(self, &teardown_set);

        Ok(PreparedPeerTransactionLifecycle {
            admission_order: input.admissions.clone().into_boxed_slice(),
            teardown_order: input.teardowns.clone().into_boxed_slice(),
            request_admissions,
            request_teardowns,
            known_admissions: input.admissions.into_boxed_slice(),
            known_teardowns: teardown_identities.into_boxed_slice(),
            orphan,
            compact,
        })
    }

    /// Consumes exact validated work without derivation, scanning, decoding, or I/O.
    pub fn apply_prepared_transaction_lifecycle(
        &mut self,
        prepared: PreparedPeerTransactionLifecycle,
    ) {
        let PreparedPeerTransactionLifecycle {
            admission_order: _,
            teardown_order: _,
            request_admissions,
            request_teardowns,
            known_admissions,
            known_teardowns,
            orphan,
            compact,
        } = prepared;

        for relay_ids in request_teardowns {
            for relay_id in relay_ids {
                self.tx_download.forget_lifecycle_identity(relay_id);
            }
        }
        for identity in known_teardowns {
            self.known_txids.remove(&identity.txid);
            self.known_wtxids.remove(&identity.wtxid);
            if self.known_wtxids_by_txid.get(&identity.txid) == Some(&identity.wtxid) {
                self.known_wtxids_by_txid.remove(&identity.txid);
            }
            for relay_id in identity.relay_ids() {
                self.mempool_known.remove(&relay_id);
            }
        }
        apply_prepared_orphan_lifecycle(&mut self.orphanage, orphan);
        for (peer_id, replacement) in compact.replacements {
            self.compact_download_states.insert(peer_id, replacement);
        }
        for relay_ids in request_admissions {
            for relay_id in relay_ids {
                self.tx_download.mark_already_have(relay_id);
            }
        }
        for identity in known_admissions {
            self.known_txids.insert(identity.txid);
            self.known_wtxids.insert(identity.wtxid);
            self.known_wtxids_by_txid
                .insert(identity.txid, identity.wtxid);
            for relay_id in identity.relay_ids() {
                self.mempool_known.insert(relay_id);
            }
        }
    }

    #[cfg(test)]
    pub(crate) fn debug_accepted_package_fingerprint_contains(
        &self,
        fingerprint: [u8; 32],
    ) -> bool {
        self.orphanage
            .debug_accepted_package_fingerprint_contains(fingerprint)
    }

    #[cfg(test)]
    pub(crate) fn debug_candidate_cursor_count(&self) -> usize {
        self.orphanage.debug_candidate_cursor_count()
    }
}

fn validate_identity_work_bounds(
    input: &PeerTransactionLifecycleInput,
) -> Result<(), PeerTransactionLifecyclePreparationError> {
    for count in [input.admissions.len(), input.teardowns.len()] {
        if count > PHASE102_MAX_ORPHAN_TRANSACTIONS {
            return Err(
                PeerTransactionLifecyclePreparationError::IdentityWorkLimit {
                    count,
                    maximum: PHASE102_MAX_ORPHAN_TRANSACTIONS,
                },
            );
        }
    }
    for package in &input.accepted_packages {
        if package.members.len() > PHASE102_MAX_ORPHANS_PER_PEER {
            return Err(
                PeerTransactionLifecyclePreparationError::PackageMemberLimit {
                    count: package.members.len(),
                    maximum: PHASE102_MAX_ORPHANS_PER_PEER,
                },
            );
        }
    }
    Ok(())
}

fn validate_identity_aliases(
    input: &PeerTransactionLifecycleInput,
) -> Result<(), PeerTransactionLifecyclePreparationError> {
    let mut wtxid_by_txid = BTreeMap::new();
    let mut txid_by_wtxid = BTreeMap::new();
    for identity in input
        .admissions
        .iter()
        .chain(&input.teardowns)
        .chain(
            input
                .accepted_packages
                .iter()
                .flat_map(|package| package.members.iter()),
        )
        .copied()
    {
        if wtxid_by_txid
            .insert(identity.txid, identity.wtxid)
            .is_some_and(|wtxid| wtxid != identity.wtxid)
        {
            return Err(
                PeerTransactionLifecyclePreparationError::TxidAliasConflict {
                    txid: identity.txid,
                },
            );
        }
        if txid_by_wtxid
            .insert(identity.wtxid, identity.txid)
            .is_some_and(|txid| txid != identity.txid)
        {
            return Err(
                PeerTransactionLifecyclePreparationError::WtxidAliasConflict {
                    wtxid: identity.wtxid,
                },
            );
        }
    }
    Ok(())
}

fn validate_orphan_bounds(
    manager: &PeerManager,
) -> Result<(), PeerTransactionLifecyclePreparationError> {
    if manager.orphanage.len() > PHASE102_MAX_ORPHAN_TRANSACTIONS {
        return Err(
            PeerTransactionLifecyclePreparationError::IdentityWorkLimit {
                count: manager.orphanage.len(),
                maximum: PHASE102_MAX_ORPHAN_TRANSACTIONS,
            },
        );
    }
    if let Some(count) = manager
        .orphanage
        .candidate_cursors()
        .map(|(_, _, child_identities, visited)| child_identities.len().max(visited))
        .find(|count| *count > PHASE102_MAX_RECONSIDERATIONS_PER_PARENT)
    {
        return Err(PeerTransactionLifecyclePreparationError::CandidateLimit {
            count,
            maximum: PHASE102_MAX_RECONSIDERATIONS_PER_PARENT,
        });
    }
    if let Some(count) = manager
        .orphanage
        .orphan_count_by_peer_values()
        .find(|count| *count > PHASE102_MAX_ORPHANS_PER_PEER)
    {
        return Err(PeerTransactionLifecyclePreparationError::OrphanPeerLimit {
            count,
            maximum: PHASE102_MAX_ORPHANS_PER_PEER,
        });
    }
    Ok(())
}

fn prepare_orphan_lifecycle(
    manager: &PeerManager,
    input: &PeerTransactionLifecycleInput,
    affected_identities: &BTreeSet<PeerTransactionIdentity>,
) -> Result<PreparedOrphanLifecycle, PeerTransactionLifecyclePreparationError> {
    let affected_txids: BTreeSet<_> = affected_identities
        .iter()
        .map(|identity| identity.txid)
        .collect();
    let affected_wtxids: BTreeSet<_> = affected_identities
        .iter()
        .map(|identity| identity.wtxid)
        .collect();
    let teardown_wtxids: BTreeSet<_> = input
        .teardowns
        .iter()
        .map(|identity| identity.wtxid)
        .collect();

    let orphan_removals = manager
        .orphanage
        .orphan_identities()
        .filter_map(|(txid, wtxid)| {
            (affected_txids.contains(&txid) || affected_wtxids.contains(&wtxid))
                .then_some(PeerTransactionIdentity::new(txid, wtxid))
        })
        .collect::<Vec<_>>();
    let candidate_removals = manager
        .orphanage
        .candidate_cursors()
        .filter_map(|(key, parent_txid, child_identities, _)| {
            (affected_txids.contains(&parent_txid)
                || affected_wtxids.contains(&key.0)
                || child_identities.iter().any(|identity| {
                    affected_txids.contains(&identity.txid())
                        || affected_wtxids.contains(&identity.wtxid())
                }))
            .then_some(key)
        })
        .collect::<Vec<_>>();

    let current_fingerprints: BTreeMap<_, _> = manager
        .orphanage
        .accepted_package_fingerprints()
        .map(|(fingerprint, members)| (*fingerprint, members.clone()))
        .collect();
    let mut prospective_fingerprints = current_fingerprints.clone();
    let mut fingerprint_admissions = Vec::new();
    for package in &input.accepted_packages {
        let members: BTreeSet<_> = package
            .members
            .iter()
            .map(|identity| identity.wtxid)
            .collect();
        if prospective_fingerprints
            .get(&package.fingerprint)
            .is_some_and(|current| current != &members)
        {
            return Err(
                PeerTransactionLifecyclePreparationError::FingerprintMembersConflict {
                    fingerprint: package.fingerprint,
                },
            );
        }
        if members.is_disjoint(&teardown_wtxids) {
            prospective_fingerprints.insert(package.fingerprint, members.clone());
            fingerprint_admissions.push((package.fingerprint, members));
        }
    }
    if prospective_fingerprints.len() > PHASE102_MAX_ORPHAN_TRANSACTIONS {
        return Err(PeerTransactionLifecyclePreparationError::FingerprintLimit {
            count: prospective_fingerprints.len(),
            maximum: PHASE102_MAX_ORPHAN_TRANSACTIONS,
        });
    }
    let fingerprint_retirements = current_fingerprints
        .iter()
        .filter_map(|(fingerprint, members)| {
            (!members.is_disjoint(&teardown_wtxids)).then_some(*fingerprint)
        })
        .collect::<Vec<_>>();
    if fingerprint_retirements.len() > PHASE102_MAX_RECONSIDERATIONS_PER_PARENT {
        return Err(
            PeerTransactionLifecyclePreparationError::FingerprintRetirementLimit {
                count: fingerprint_retirements.len(),
                maximum: PHASE102_MAX_RECONSIDERATIONS_PER_PARENT,
            },
        );
    }

    Ok(PreparedOrphanLifecycle {
        orphan_removals: orphan_removals.into_boxed_slice(),
        candidate_removals: candidate_removals.into_boxed_slice(),
        fingerprint_retirements: fingerprint_retirements.into_boxed_slice(),
        fingerprint_admissions: fingerprint_admissions.into_boxed_slice(),
    })
}

fn prepare_compact_lifecycle(
    manager: &PeerManager,
    teardowns: &BTreeSet<PeerTransactionIdentity>,
) -> PreparedCompactPartialLifecycle {
    let mut replacements = Vec::new();
    for (peer_id, state) in &manager.compact_download_states {
        let mut replacement = state.clone();
        for teardown in teardowns {
            for in_flight in replacement.in_flight.values_mut() {
                in_flight
                    .partial
                    .on_mempool_transaction_removed(&teardown.wtxid);
            }
        }
        if &replacement != state {
            replacements.push((*peer_id, replacement));
        }
    }
    PreparedCompactPartialLifecycle {
        replacements: replacements.into_boxed_slice(),
    }
}

fn apply_prepared_orphan_lifecycle(
    orphanage: &mut super::TxOrphanage,
    prepared: PreparedOrphanLifecycle,
) {
    for key in prepared.candidate_removals {
        orphanage.remove_candidate_cursor(key);
    }
    for identity in prepared.orphan_removals {
        orphanage.remove_orphan_without_candidate_scan(identity.wtxid);
    }
    for fingerprint in prepared.fingerprint_retirements {
        orphanage.retire_accepted_package_fingerprint(fingerprint);
    }
    for (fingerprint, members) in prepared.fingerprint_admissions {
        orphanage.record_accepted_package_fingerprint(fingerprint, members);
    }
}

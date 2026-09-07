// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Complete startup recovery projection prepared before any authority mutation.

#[cfg(test)]
use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use open_bitcoin_core::chainstate::CoinsView;
use open_bitcoin_core::primitives::{Transaction, Txid, Wtxid};
use open_bitcoin_mempool::{Mempool, MempoolMemberIdentity, PolicyTime};
use open_bitcoin_network::{
    PeerTransactionIdentity, PeerTransactionLifecycleInput, PreparedPeerTransactionLifecycle,
};

use super::{LifecycleEvidenceSnapshot, LifecycleGeneration};
use crate::network::compact_receive_candidates::CompactExtraTxnBuffer;
use crate::network::recovery::{ManagedMempoolRecoverySummary, PreparedMempoolRecovery};
use crate::network::relay_fanout::ManagedRelayFanoutState;
use crate::network::relay_serving::RelayServingCache;
use crate::{ChainstateStore, ManagedPeerNetwork};

#[derive(Debug)]
pub(in crate::network) enum RecoveryInstallError {
    StaleAuthorityEpoch,
    StaleChainstate,
    NonFreshAuthority,
    PendingLifecycleEffects,
    InvalidPreparedRecovery,
    PeerProjection(open_bitcoin_network::PeerTransactionLifecyclePreparationError),
    #[cfg(test)]
    InjectedFailure(RecoveryInstallFailurePoint),
}

impl fmt::Display for RecoveryInstallError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaleAuthorityEpoch => {
                formatter.write_str("prepared recovery belongs to another authority incarnation")
            }
            Self::StaleChainstate => {
                formatter.write_str("chainstate changed after mempool recovery preparation")
            }
            Self::NonFreshAuthority => {
                formatter.write_str("mempool recovery requires a fresh startup authority")
            }
            Self::PendingLifecycleEffects => {
                formatter.write_str("mempool recovery cannot replace pending lifecycle effects")
            }
            Self::InvalidPreparedRecovery => {
                formatter.write_str("prepared mempool recovery is internally inconsistent")
            }
            Self::PeerProjection(error) => {
                write!(formatter, "recovery peer projection failed: {error}")
            }
            #[cfg(test)]
            Self::InjectedFailure(point) => {
                write!(formatter, "injected recovery install failure at {point:?}")
            }
        }
    }
}

impl std::error::Error for RecoveryInstallError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PeerProjection(error) => Some(error),
            Self::StaleAuthorityEpoch
            | Self::StaleChainstate
            | Self::NonFreshAuthority
            | Self::PendingLifecycleEffects
            | Self::InvalidPreparedRecovery => None,
            #[cfg(test)]
            Self::InjectedFailure(_) => None,
        }
    }
}

pub(in crate::network) struct PreparedRecoveryProjection {
    pub(super) staged_mempool: Mempool,
    pub(super) compact: CompactExtraTxnBuffer,
    pub(super) transactions_by_txid: BTreeMap<Txid, Transaction>,
    pub(super) transactions_by_wtxid: BTreeMap<Wtxid, Transaction>,
    pub(super) relay_serving: RelayServingCache,
    pub(super) relay_fanout: ManagedRelayFanoutState,
    pub(super) peer: PreparedPeerTransactionLifecycle,
    pub(super) unbroadcast_members: BTreeSet<MempoolMemberIdentity>,
    pub(super) generation: LifecycleGeneration,
    pub(super) maybe_captured_at: Option<PolicyTime>,
    pub(super) lifecycle_evidence: LifecycleEvidenceSnapshot,
    pub(super) summary: ManagedMempoolRecoverySummary,
}

impl PreparedRecoveryProjection {
    pub(super) fn prepare<S: ChainstateStore, V: open_bitcoin_core::chainstate::CoinsView>(
        network: &ManagedPeerNetwork<S, V>,
        prepared: PreparedMempoolRecovery,
    ) -> Result<Self, RecoveryInstallError> {
        #[cfg(test)]
        fail_install_at(RecoveryInstallFailurePoint::AuthorityEpoch)?;
        if prepared.authority_epoch != network.authority_epoch {
            return Err(RecoveryInstallError::StaleAuthorityEpoch);
        }
        if prepared.maybe_chainstate_tip != network.chainstate_snapshot().tip().cloned() {
            return Err(RecoveryInstallError::StaleChainstate);
        }

        #[cfg(test)]
        fail_install_at(RecoveryInstallFailurePoint::Generation)?;
        let generation = prepared
            .captured_generation
            .map_or(LifecycleGeneration::INITIAL, |captured| {
                LifecycleGeneration::from_raw(captured.raw())
            });
        if generation == LifecycleGeneration::MAX {
            return Err(RecoveryInstallError::InvalidPreparedRecovery);
        }
        if prepared.captured_generation.is_some() != prepared.captured_at.is_some() {
            return Err(RecoveryInstallError::InvalidPreparedRecovery);
        }

        #[cfg(test)]
        fail_install_at(RecoveryInstallFailurePoint::FreshAuthority)?;
        if !network.is_fresh_for_recovery_install() {
            return Err(RecoveryInstallError::NonFreshAuthority);
        }

        #[cfg(test)]
        fail_install_at(RecoveryInstallFailurePoint::PendingEffects)?;
        if network.peer_effect_ledger.has_pending() || network.snapshot_effect_ledger.has_pending()
        {
            return Err(RecoveryInstallError::PendingLifecycleEffects);
        }

        #[cfg(test)]
        fail_install_at(RecoveryInstallFailurePoint::ProjectionBuild)?;
        let canonical_members = prepared
            .staged_mempool
            .entries()
            .iter()
            .map(|(txid, entry)| MempoolMemberIdentity {
                txid: *txid,
                wtxid: entry.wtxid,
            })
            .collect::<BTreeSet<_>>();
        let ordered_members = prepared
            .ordered_survivors
            .iter()
            .copied()
            .collect::<BTreeSet<_>>();
        if canonical_members != ordered_members
            || prepared.ordered_survivors.len() != canonical_members.len()
            || !prepared.unbroadcast_members.is_subset(&canonical_members)
        {
            return Err(RecoveryInstallError::InvalidPreparedRecovery);
        }

        let mut transactions_by_txid = BTreeMap::new();
        let mut transactions_by_wtxid = BTreeMap::new();
        let mut relay_serving = RelayServingCache::default();
        let mut relay_fanout = ManagedRelayFanoutState::default();
        for member in &prepared.ordered_survivors {
            let Some(entry) = prepared.staged_mempool.entry(&member.txid) else {
                return Err(RecoveryInstallError::InvalidPreparedRecovery);
            };
            if entry.wtxid != member.wtxid {
                return Err(RecoveryInstallError::InvalidPreparedRecovery);
            }
            let transaction = entry.transaction.clone();
            transactions_by_txid.insert(member.txid, transaction.clone());
            transactions_by_wtxid.insert(member.wtxid, transaction.clone());
            relay_serving.record_accepted_prevalidated(member.txid, member.wtxid, transaction);
            relay_fanout.seed_recovered_transaction(member.txid, member.wtxid);
        }
        let peer = network
            .peer_manager
            .prepare_transaction_lifecycle(PeerTransactionLifecycleInput::new(
                prepared
                    .ordered_survivors
                    .iter()
                    .map(|member| PeerTransactionIdentity::new(member.txid, member.wtxid))
                    .collect(),
                Vec::new(),
                Vec::new(),
            ))
            .map_err(RecoveryInstallError::PeerProjection)?;
        let admitted_members = u64::try_from(canonical_members.len()).unwrap_or(u64::MAX);
        let summary = ManagedMempoolRecoverySummary::from_records(prepared.recovery_records);

        Ok(Self {
            staged_mempool: prepared.staged_mempool,
            compact: CompactExtraTxnBuffer::with_defaults(),
            transactions_by_txid,
            transactions_by_wtxid,
            relay_serving,
            relay_fanout,
            peer,
            unbroadcast_members: prepared.unbroadcast_members,
            generation,
            maybe_captured_at: prepared.captured_at,
            lifecycle_evidence: LifecycleEvidenceSnapshot {
                committed_transitions: generation.raw(),
                admitted_members,
                ..LifecycleEvidenceSnapshot::default()
            },
            summary,
        })
    }
}

impl<S: ChainstateStore, V: CoinsView> ManagedPeerNetwork<S, V> {
    fn is_fresh_for_recovery_install(&self) -> bool {
        let peer = self.peer_manager.mempool_lifecycle_snapshot();
        self.mempool.mempool().entries().is_empty()
            && self.lifecycle_generation == LifecycleGeneration::INITIAL
            && self.dirty_generation.is_none()
            && self.lifecycle_evidence == LifecycleEvidenceSnapshot::default()
            && self.checkpoint_evidence.is_fresh_start()
            && self.latest_mempool_recovery.is_none()
            && self.reconcile_lifecycle_projection().is_clean()
            && peer.requests.candidate_count == 0
            && peer.requests.in_flight_count == 0
            && peer.requests.already_have_count == 0
            && peer.known_identities == 0
            && peer.orphan_transactions == 0
            && peer.candidate_cursors == 0
            && peer.accepted_packages == 0
            && peer.compact_download_peers == 0
    }
}

#[cfg(test)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(in crate::network) enum RecoveryInstallFailurePoint {
    AuthorityEpoch,
    Generation,
    FreshAuthority,
    PendingEffects,
    ProjectionBuild,
}

#[cfg(test)]
impl RecoveryInstallFailurePoint {
    pub(in crate::network) const ALL: [Self; 5] = [
        Self::AuthorityEpoch,
        Self::Generation,
        Self::FreshAuthority,
        Self::PendingEffects,
        Self::ProjectionBuild,
    ];
}

#[cfg(test)]
thread_local! {
    static INJECTED_INSTALL_FAILURE: Cell<Option<RecoveryInstallFailurePoint>> =
        const { Cell::new(None) };
}

#[cfg(test)]
pub(in crate::network) struct RecoveryInstallFailureGuard {
    previous: Option<RecoveryInstallFailurePoint>,
}

#[cfg(test)]
impl RecoveryInstallFailureGuard {
    pub(in crate::network) fn inject(point: RecoveryInstallFailurePoint) -> Self {
        let previous = INJECTED_INSTALL_FAILURE.replace(Some(point));
        Self { previous }
    }
}

#[cfg(test)]
impl Drop for RecoveryInstallFailureGuard {
    fn drop(&mut self) {
        INJECTED_INSTALL_FAILURE.set(self.previous);
    }
}

#[cfg(test)]
fn fail_install_at(point: RecoveryInstallFailurePoint) -> Result<(), RecoveryInstallError> {
    if INJECTED_INSTALL_FAILURE.get() == Some(point) {
        return Err(RecoveryInstallError::InjectedFailure(point));
    }
    Ok(())
}

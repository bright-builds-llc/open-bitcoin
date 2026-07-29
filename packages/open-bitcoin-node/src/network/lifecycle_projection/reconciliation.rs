// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! Read-only, bounded reconciliation for explicit audit and recovery entrypoints.

use std::collections::BTreeSet;

use open_bitcoin_core::consensus::{transaction_txid, transaction_wtxid};
use open_bitcoin_mempool::MempoolMemberIdentity;
use open_bitcoin_network::PeerTransactionIdentity;

use crate::{ChainstateStore, ManagedPeerNetwork};

/// Fixed low-cardinality reconciliation result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::network) struct LifecycleReconciliationReport {
    counts: [usize; Self::TARGET_COUNT],
}

impl LifecycleReconciliationReport {
    pub(in crate::network) const TARGET_COUNT: usize = 7;
    pub(in crate::network) const MAX_MISMATCH_COUNT: usize = 10_000;
    pub(in crate::network) const FIXED_TARGET_LABELS: [&'static str; Self::TARGET_COUNT] = [
        "serving",
        "fanout",
        "peer",
        "compact",
        "unbroadcast",
        "persistence",
        "evidence",
    ];

    pub(in crate::network) const fn labels(&self) -> [&'static str; Self::TARGET_COUNT] {
        Self::FIXED_TARGET_LABELS
    }

    pub(in crate::network) const fn counts(&self) -> [usize; Self::TARGET_COUNT] {
        self.counts
    }

    pub(in crate::network) fn is_clean(&self) -> bool {
        self.counts.iter().all(|count| *count == 0)
    }

    fn new(counts: [usize; Self::TARGET_COUNT]) -> Self {
        Self {
            counts: counts.map(|count| count.min(Self::MAX_MISMATCH_COUNT)),
        }
    }
}

#[cfg(test)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::network) struct ExactLifecycleProjectionMismatches {
    pub(in crate::network) serving: BTreeSet<MempoolMemberIdentity>,
    pub(in crate::network) fanout: BTreeSet<MempoolMemberIdentity>,
    pub(in crate::network) peer: BTreeSet<MempoolMemberIdentity>,
    pub(in crate::network) compact: BTreeSet<MempoolMemberIdentity>,
    pub(in crate::network) unbroadcast: BTreeSet<MempoolMemberIdentity>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    /// Explicit audit/startup oracle. It never mutates or repairs lifecycle state.
    pub(in crate::network) fn reconcile_lifecycle_projection(
        &self,
    ) -> LifecycleReconciliationReport {
        let canonical = self.canonical_lifecycle_members();
        LifecycleReconciliationReport::new([
            self.serving_mismatch_count(&canonical),
            self.relay_fanout.lifecycle_mismatch_count(&canonical),
            self.peer_mismatch_count(&canonical),
            self.compact_mismatch_count(&canonical),
            self.unbroadcast_mismatch_count(&canonical),
            self.persistence_mismatch_count(),
            self.evidence_mismatch_count(),
        ])
    }

    fn canonical_lifecycle_members(&self) -> BTreeSet<MempoolMemberIdentity> {
        self.mempool
            .mempool()
            .entries()
            .iter()
            .map(|(txid, entry)| MempoolMemberIdentity {
                txid: *txid,
                wtxid: entry.wtxid,
            })
            .collect()
    }

    fn serving_mismatch_count(&self, canonical: &BTreeSet<MempoolMemberIdentity>) -> usize {
        let canonical_txids = canonical
            .iter()
            .map(|member| member.txid)
            .collect::<BTreeSet<_>>();
        let canonical_wtxids = canonical
            .iter()
            .map(|member| member.wtxid)
            .collect::<BTreeSet<_>>();
        self.relay_serving
            .lifecycle_mismatch_count(canonical)
            .saturating_add(
                self.transactions_by_txid
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .symmetric_difference(&canonical_txids)
                    .count(),
            )
            .saturating_add(
                self.transactions_by_wtxid
                    .keys()
                    .copied()
                    .collect::<BTreeSet<_>>()
                    .symmetric_difference(&canonical_wtxids)
                    .count(),
            )
    }

    fn peer_mismatch_count(&self, canonical: &BTreeSet<MempoolMemberIdentity>) -> usize {
        let peer_canonical = canonical
            .iter()
            .map(|member| PeerTransactionIdentity::new(member.txid, member.wtxid))
            .collect();
        self.peer_manager
            .mempool_lifecycle_mismatch_count(&peer_canonical)
    }

    fn compact_mismatch_count(&self, canonical: &BTreeSet<MempoolMemberIdentity>) -> usize {
        self.compact_extra_txn
            .iter_available()
            .filter(|(stored_wtxid, transaction)| {
                let Ok(txid) = transaction_txid(transaction) else {
                    return true;
                };
                let Ok(actual_wtxid) = transaction_wtxid(transaction) else {
                    return true;
                };
                *stored_wtxid != actual_wtxid
                    || canonical.contains(&MempoolMemberIdentity {
                        txid,
                        wtxid: actual_wtxid,
                    })
            })
            .count()
    }

    fn unbroadcast_mismatch_count(&self, canonical: &BTreeSet<MempoolMemberIdentity>) -> usize {
        let expected = self.expected_unbroadcast_members(canonical);
        self.unbroadcast_members
            .symmetric_difference(&expected)
            .count()
    }

    fn expected_unbroadcast_members(
        &self,
        canonical: &BTreeSet<MempoolMemberIdentity>,
    ) -> BTreeSet<MempoolMemberIdentity> {
        canonical
            .iter()
            .filter(|member| {
                self.mempool
                    .mempool()
                    .entry(&member.txid)
                    .is_some_and(|entry| {
                        entry.wtxid == member.wtxid && entry.metadata.is_retry_eligible(true)
                    })
            })
            .copied()
            .collect()
    }

    fn persistence_mismatch_count(&self) -> usize {
        usize::from(
            self.dirty_generation
                .is_some_and(|dirty| dirty != self.lifecycle_generation),
        )
    }

    fn evidence_mismatch_count(&self) -> usize {
        usize::try_from(
            self.lifecycle_evidence
                .committed_transitions
                .abs_diff(self.lifecycle_generation.raw()),
        )
        .unwrap_or(usize::MAX)
    }

    #[cfg(test)]
    pub(in crate::network) fn reconcile_lifecycle_projection_exact_for_test(
        &self,
    ) -> ExactLifecycleProjectionMismatches {
        let canonical = self.canonical_lifecycle_members();
        let serving_members = self.relay_serving.lifecycle_members_for_test();
        let fanout_members = self.relay_fanout.lifecycle_members_for_test();
        ExactLifecycleProjectionMismatches {
            serving: serving_members
                .symmetric_difference(&canonical)
                .copied()
                .collect(),
            fanout: fanout_members
                .symmetric_difference(&canonical)
                .copied()
                .collect(),
            peer: canonical
                .iter()
                .filter(|member| {
                    !self
                        .peer_manager
                        .mempool_identity_known(PeerTransactionIdentity::new(
                            member.txid,
                            member.wtxid,
                        ))
                })
                .copied()
                .collect(),
            compact: self.compact_exact_mismatches(&canonical),
            unbroadcast: self
                .unbroadcast_members
                .symmetric_difference(&self.expected_unbroadcast_members(&canonical))
                .copied()
                .collect(),
        }
    }

    #[cfg(test)]
    fn compact_exact_mismatches(
        &self,
        canonical: &BTreeSet<MempoolMemberIdentity>,
    ) -> BTreeSet<MempoolMemberIdentity> {
        self.compact_extra_txn
            .iter_available()
            .filter_map(|(stored_wtxid, transaction)| {
                let txid = transaction_txid(transaction).ok()?;
                let actual_wtxid = transaction_wtxid(transaction).ok()?;
                let member = MempoolMemberIdentity {
                    txid,
                    wtxid: *stored_wtxid,
                };
                (*stored_wtxid != actual_wtxid
                    || canonical.contains(&MempoolMemberIdentity {
                        txid,
                        wtxid: actual_wtxid,
                    }))
                .then_some(member)
            })
            .collect()
    }
}

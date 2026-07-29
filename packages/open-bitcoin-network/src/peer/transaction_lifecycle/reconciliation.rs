// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.cpp

use std::collections::BTreeSet;

use crate::peer::{PeerManager, TxDownloadSnapshot};

use super::PeerTransactionIdentity;

/// Fixed aggregate view used by lifecycle audits without exposing transaction identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerMempoolLifecycleSnapshot {
    pub requests: TxDownloadSnapshot,
    pub known_identities: usize,
    pub orphan_transactions: usize,
    pub candidate_cursors: usize,
    pub accepted_packages: usize,
    pub compact_download_peers: usize,
}

impl PeerManager {
    /// Reports whether every peer-local accepted-identity index contains this exact member.
    pub fn mempool_identity_known(&self, identity: PeerTransactionIdentity) -> bool {
        self.known_txids.contains(&identity.txid)
            && self.known_wtxids.contains(&identity.wtxid)
            && self.known_wtxids_by_txid.get(&identity.txid) == Some(&identity.wtxid)
            && identity
                .relay_ids()
                .into_iter()
                .all(|relay_id| self.mempool_known.contains(&relay_id))
    }

    #[cfg(test)]
    pub(crate) fn debug_mempool_identity_known(&self, identity: PeerTransactionIdentity) -> bool {
        self.mempool_identity_known(identity)
    }

    /// Returns bounded, low-cardinality peer lifecycle state for reconciliation.
    pub fn mempool_lifecycle_snapshot(&self) -> PeerMempoolLifecycleSnapshot {
        PeerMempoolLifecycleSnapshot {
            requests: self.tx_download.snapshot(),
            known_identities: self.known_wtxids_by_txid.len(),
            orphan_transactions: self.orphanage.len(),
            candidate_cursors: self.orphanage.candidate_cursors().count(),
            accepted_packages: self.orphanage.accepted_package_fingerprints().count(),
            compact_download_peers: self.compact_download_states.len(),
        }
    }

    /// Compares canonical membership with all peer-local accepted identity indexes.
    pub fn mempool_lifecycle_mismatch_count(
        &self,
        canonical: &BTreeSet<PeerTransactionIdentity>,
    ) -> usize {
        let canonical_txids = canonical
            .iter()
            .map(|identity| identity.txid)
            .collect::<BTreeSet<_>>();
        let canonical_wtxids = canonical
            .iter()
            .map(|identity| identity.wtxid)
            .collect::<BTreeSet<_>>();
        let canonical_relay_ids = canonical
            .iter()
            .flat_map(|identity| identity.relay_ids())
            .collect::<BTreeSet<_>>();
        let known_identities = self
            .known_wtxids_by_txid
            .iter()
            .map(|(txid, wtxid)| PeerTransactionIdentity::new(*txid, *wtxid))
            .collect::<BTreeSet<_>>();
        let orphan_overlap = self
            .orphanage
            .orphan_identities()
            .filter(|(txid, wtxid)| {
                canonical_txids.contains(txid) || canonical_wtxids.contains(wtxid)
            })
            .count();
        let candidate_overlap = self
            .orphanage
            .candidate_cursors()
            .filter(|(key, parent_txid, child_identities, _)| {
                canonical_txids.contains(parent_txid)
                    || canonical_wtxids.contains(&key.0)
                    || child_identities.iter().any(|identity| {
                        canonical_txids.contains(&identity.txid())
                            || canonical_wtxids.contains(&identity.wtxid())
                    })
            })
            .count();
        let package_mismatch = self
            .orphanage
            .accepted_package_fingerprints()
            .filter(|(_, members)| !members.is_subset(&canonical_wtxids))
            .count();

        symmetric_difference_count(&known_identities, canonical)
            .saturating_add(symmetric_difference_count(
                &self.known_txids,
                &canonical_txids,
            ))
            .saturating_add(symmetric_difference_count(
                &self.known_wtxids,
                &canonical_wtxids,
            ))
            .saturating_add(symmetric_difference_count(
                &self.mempool_known,
                &canonical_relay_ids,
            ))
            .saturating_add(
                self.tx_download
                    .lifecycle_mismatch_count(&canonical_relay_ids),
            )
            .saturating_add(orphan_overlap)
            .saturating_add(candidate_overlap)
            .saturating_add(package_mismatch)
    }
}

fn symmetric_difference_count<T: Ord>(left: &BTreeSet<T>, right: &BTreeSet<T>) -> usize {
    left.symmetric_difference(right).count()
}

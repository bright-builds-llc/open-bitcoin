// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_opportunistic_1p1c.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use std::collections::BTreeSet;

use open_bitcoin_primitives::{Transaction, TransactionInput, TransactionOutput, Txid, Wtxid};

use crate::error::PeerId;

use super::super::{HardRejectEvidence, ReconsiderableEvidenceKey, ReconsiderableRejectEvidence};
use super::{
    OrphanEntry, OrphanReconsiderationCandidate, ReceivedTransactionProvenance, TxOrphanage,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SamePeerOneParentOneChildCandidate {
    pub(super) members: [Transaction; 2],
    pub(super) origins: [PeerId; 2],
    pub(super) provenances: [ReceivedTransactionProvenance; 2],
}

impl SamePeerOneParentOneChildCandidate {
    /// Consumes the proof into request-ordered bodies and qualifying origins.
    pub fn into_ordered_parts(self) -> ([Transaction; 2], [PeerId; 2]) {
        (self.members, self.origins)
    }

    /// Consumes the proof while preserving retained announcers for outcome feedback.
    pub fn into_ordered_parts_with_provenance(
        self,
    ) -> (
        [Transaction; 2],
        [PeerId; 2],
        [ReceivedTransactionProvenance; 2],
    ) {
        (self.members, self.origins, self.provenances)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct SamePeerCandidateCursor {
    pub(super) parent: Transaction,
    pub(super) parent_txid: Txid,
    pub(super) parent_peer: PeerId,
    pub(super) child_wtxids: Box<[Wtxid]>,
    pub(super) next_child: usize,
    pub(super) visited: usize,
    parent_body_bytes: usize,
}

impl SamePeerCandidateCursor {
    pub(super) fn retained_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(self.parent_body_bytes)
            .saturating_add(
                self.child_wtxids
                    .len()
                    .saturating_mul(std::mem::size_of::<Wtxid>()),
            )
    }
}

impl TxOrphanage {
    pub fn begin_same_peer_candidate(
        &mut self,
        parent: Transaction,
        parent_txid: Txid,
        parent_wtxid: Wtxid,
        parent_peer: PeerId,
        reconsiderable: &ReconsiderableRejectEvidence,
        hard_rejects: &HardRejectEvidence,
    ) -> Option<SamePeerOneParentOneChildCandidate> {
        let cursor_key = (parent_wtxid, parent_peer);
        self.candidate_cursors.remove(&cursor_key);
        if !reconsiderable.contains(ReconsiderableEvidenceKey::Transaction(parent_wtxid)) {
            return None;
        }

        let child_wtxids = self
            .children_by_parent
            .get(&parent_txid)?
            .iter()
            .filter_map(|(_, wtxid)| {
                let entry = self.orphans.get(wtxid)?;
                (entry.missing_parents.len() == 1 && entry.announcers.contains(parent_peer))
                    .then_some(*wtxid)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let parent_body_bytes = transaction_body_bytes(&parent);
        let cursor = SamePeerCandidateCursor {
            parent,
            parent_txid,
            parent_peer,
            child_wtxids,
            next_child: 0,
            visited: 0,
            parent_body_bytes,
        };
        if self
            .retained_bytes()
            .saturating_add(cursor.retained_bytes())
            > self.policy.max_retained_bytes
        {
            return None;
        }
        self.candidate_cursors.insert(cursor_key, cursor);
        self.advance_same_peer_candidate(parent_wtxid, parent_peer, hard_rejects)
    }

    pub fn advance_same_peer_candidate(
        &mut self,
        parent_wtxid: Wtxid,
        parent_peer: PeerId,
        hard_rejects: &HardRejectEvidence,
    ) -> Option<SamePeerOneParentOneChildCandidate> {
        let cursor_key = (parent_wtxid, parent_peer);
        let mut cursor = self.candidate_cursors.remove(&cursor_key)?;

        while cursor.visited < self.policy.max_reconsiderations_per_parent
            && cursor.next_child < cursor.child_wtxids.len()
        {
            let child_wtxid = cursor.child_wtxids[cursor.next_child];
            cursor.next_child += 1;
            cursor.visited += 1;
            if hard_rejects.contains(child_wtxid) {
                continue;
            }
            let entry = self.orphans.get(&child_wtxid)?;

            let candidate = SamePeerOneParentOneChildCandidate {
                members: [cursor.parent.clone(), entry.transaction.clone()],
                origins: [cursor.parent_peer; 2],
                provenances: [
                    super::ReceivedTransactionProvenance {
                        delivered_by: cursor.parent_peer,
                        announcers: vec![cursor.parent_peer],
                    },
                    entry.announcers.provenance(),
                ],
            };
            if cursor.visited < self.policy.max_reconsiderations_per_parent
                && cursor.next_child < cursor.child_wtxids.len()
            {
                self.candidate_cursors.insert(cursor_key, cursor);
            }
            return Some(candidate);
        }
        None
    }

    pub(crate) fn remove_orphan_without_candidate_scan(&mut self, wtxid: Wtxid) {
        self.pending_reconsideration.remove(&wtxid);
        let maybe_entry = self.orphans.remove(&wtxid);
        for entry in maybe_entry.iter() {
            for parent_txid in &entry.missing_parents {
                self.remove_child_index(*parent_txid, wtxid);
            }
            for peer_id in &entry.announcers.peers {
                self.decrement_peer_count(*peer_id);
            }
        }
    }

    pub(crate) fn orphan_identities(&self) -> impl Iterator<Item = (Txid, Wtxid)> + '_ {
        self.orphans.values().map(|entry| (entry.txid, entry.wtxid))
    }

    pub(crate) fn orphan_count_by_peer_values(&self) -> impl Iterator<Item = usize> + '_ {
        self.orphan_count_by_peer.values().copied()
    }

    pub(crate) fn candidate_cursors(
        &self,
    ) -> impl Iterator<Item = ((Wtxid, PeerId), Txid, &[Wtxid], usize)> + '_ {
        self.candidate_cursors.iter().map(|(key, cursor)| {
            (
                *key,
                cursor.parent_txid,
                cursor.child_wtxids.as_ref(),
                cursor.visited,
            )
        })
    }

    pub(crate) fn remove_candidate_cursor(&mut self, key: (Wtxid, PeerId)) {
        self.candidate_cursors.remove(&key);
    }

    pub(crate) fn accepted_package_fingerprints(
        &self,
    ) -> impl Iterator<Item = (&[u8; 32], &BTreeSet<Wtxid>)> {
        self.accepted_package_fingerprints.iter()
    }

    pub(crate) fn record_accepted_package_fingerprint(
        &mut self,
        fingerprint: [u8; 32],
        members: BTreeSet<Wtxid>,
    ) {
        self.accepted_package_fingerprints
            .insert(fingerprint, members);
    }

    pub(crate) fn retire_accepted_package_fingerprint(&mut self, fingerprint: [u8; 32]) {
        self.accepted_package_fingerprints.remove(&fingerprint);
    }

    #[cfg(test)]
    pub(crate) fn debug_accepted_package_fingerprint_contains(
        &self,
        fingerprint: [u8; 32],
    ) -> bool {
        self.accepted_package_fingerprints
            .contains_key(&fingerprint)
    }

    #[cfg(test)]
    pub(crate) fn debug_candidate_cursor_count(&self) -> usize {
        self.candidate_cursors.len()
    }
}

impl TxOrphanage {
    pub fn len(&self) -> usize {
        self.orphans.len()
    }

    pub fn peer_len(&self, peer_id: PeerId) -> usize {
        self.orphan_count_by_peer
            .get(&peer_id)
            .copied()
            .unwrap_or_default()
    }

    pub fn is_empty(&self) -> bool {
        self.orphans.is_empty()
    }

    pub(super) fn retained_bytes(&self) -> usize {
        let orphan_bytes = self.orphans.values().fold(0_usize, |total, entry| {
            total.saturating_add(entry.retained_bytes())
        });
        self.candidate_cursors
            .values()
            .fold(orphan_bytes, |total, cursor| {
                total.saturating_add(cursor.retained_bytes())
            })
    }

    #[cfg(test)]
    pub(crate) fn debug_retained_bytes(&self) -> usize {
        self.retained_bytes()
    }

    #[cfg(test)]
    pub(crate) fn debug_candidate_cursor_retention(&self) -> (usize, usize, usize) {
        (
            self.candidate_cursors.len(),
            self.candidate_cursors
                .values()
                .map(|cursor| cursor.child_wtxids.len())
                .sum(),
            self.candidate_cursors
                .values()
                .fold(0_usize, |total, cursor| {
                    total.saturating_add(cursor.retained_bytes())
                }),
        )
    }
}

impl OrphanEntry {
    fn retained_bytes(&self) -> usize {
        std::mem::size_of::<Self>()
            .saturating_add(self.body_bytes)
            .saturating_add(
                self.missing_parents
                    .len()
                    .saturating_mul(std::mem::size_of::<Txid>()),
            )
            .saturating_add(
                self.announcers
                    .peers
                    .len()
                    .saturating_mul(std::mem::size_of::<PeerId>()),
            )
    }

    pub(super) fn candidate(&self) -> OrphanReconsiderationCandidate {
        OrphanReconsiderationCandidate {
            peer_id: self.announcers.primary_peer(),
            provenance: ReceivedTransactionProvenance {
                delivered_by: self.announcers.primary_peer(),
                announcers: self.announcers.peers.iter().copied().collect(),
            },
            transaction: self.transaction.clone(),
            txid: self.txid,
            wtxid: self.wtxid,
            missing_parents: self.missing_parents.iter().copied().collect(),
        }
    }
}

pub(super) fn transaction_body_bytes(transaction: &Transaction) -> usize {
    let input_bytes = transaction.inputs.iter().fold(0_usize, |total, input| {
        let witness_bytes = input.witness.stack().iter().fold(
            input
                .witness
                .stack()
                .len()
                .saturating_mul(std::mem::size_of::<Vec<u8>>()),
            |witness_total, item| witness_total.saturating_add(item.len()),
        );
        total
            .saturating_add(std::mem::size_of::<TransactionInput>())
            .saturating_add(input.script_sig.as_bytes().len())
            .saturating_add(witness_bytes)
    });
    let output_bytes = transaction.outputs.iter().fold(0_usize, |total, output| {
        total
            .saturating_add(std::mem::size_of::<TransactionOutput>())
            .saturating_add(output.script_pubkey.as_bytes().len())
    });
    std::mem::size_of::<Transaction>()
        .saturating_add(input_bytes)
        .saturating_add(output_bytes)
}

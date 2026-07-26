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

use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

use crate::error::PeerId;

use super::super::{HardRejectEvidence, ReconsiderableEvidenceKey, ReconsiderableRejectEvidence};
use super::{ReceivedTransactionProvenance, TxOrphanage};

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
    pub(super) children: Vec<(Wtxid, Transaction, ReceivedTransactionProvenance)>,
    pub(super) next_child: usize,
    pub(super) visited: usize,
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

        let children =
            self.children_by_parent
                .get(&parent_txid)?
                .iter()
                .filter_map(|(_, wtxid)| {
                    let entry = self.orphans.get(wtxid)?;
                    (entry.missing_parents.len() == 1 && entry.announcers.contains(parent_peer))
                        .then(|| {
                            (
                                *wtxid,
                                entry.transaction.clone(),
                                entry.announcers.provenance(),
                            )
                        })
                })
                .collect();
        self.candidate_cursors.insert(
            cursor_key,
            SamePeerCandidateCursor {
                parent,
                parent_txid,
                parent_peer,
                children,
                next_child: 0,
                visited: 0,
            },
        );
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
            && cursor.next_child < cursor.children.len()
        {
            let (child_wtxid, child, child_provenance) = &cursor.children[cursor.next_child];
            cursor.next_child += 1;
            cursor.visited += 1;
            if hard_rejects.contains(*child_wtxid) {
                continue;
            }

            let candidate = SamePeerOneParentOneChildCandidate {
                members: [cursor.parent.clone(), child.clone()],
                origins: [cursor.parent_peer; 2],
                provenances: [
                    super::ReceivedTransactionProvenance {
                        delivered_by: cursor.parent_peer,
                        announcers: vec![cursor.parent_peer],
                    },
                    child_provenance.clone(),
                ],
            };
            if cursor.visited < self.policy.max_reconsiderations_per_parent
                && cursor.next_child < cursor.children.len()
            {
                self.candidate_cursors.insert(cursor_key, cursor);
            }
            return Some(candidate);
        }
        None
    }
}

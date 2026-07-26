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
use super::{SamePeerCandidateCursor, SamePeerOneParentOneChildCandidate, TxOrphanage};

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

        let children = self
            .children_by_parent
            .get(&parent_txid)?
            .iter()
            .filter_map(|(_, wtxid)| {
                let entry = self.orphans.get(wtxid)?;
                (entry.missing_parents.len() == 1 && entry.announcers.contains(parent_peer))
                    .then(|| (*wtxid, entry.transaction.clone()))
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
            let (child_wtxid, child) = &cursor.children[cursor.next_child];
            cursor.next_child += 1;
            cursor.visited += 1;
            if hard_rejects.contains(*child_wtxid) {
                continue;
            }

            let candidate = SamePeerOneParentOneChildCandidate {
                members: [cursor.parent.clone(), child.clone()],
                origins: [cursor.parent_peer; 2],
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

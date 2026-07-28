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

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

use crate::error::PeerId;

use super::{ReceivedTransactionProvenance, TxRelayId};

mod candidate;
pub use candidate::SamePeerOneParentOneChildCandidate;
use candidate::{SamePeerCandidateCursor, transaction_body_bytes};

pub const PHASE102_MAX_ORPHAN_TRANSACTIONS: usize = 100;
pub const PHASE102_MAX_ORPHANS_PER_PEER: usize = 25;
pub const PHASE102_ORPHAN_TTL_SECONDS: i64 = 20 * 60;
pub const PHASE102_MAX_RECONSIDERATIONS_PER_PARENT: usize = 32;
pub const PHASE133_MAX_ANNOUNCERS_PER_ORPHAN: usize = 8;
pub const PHASE133_MAX_ORPHAN_RETAINED_BYTES: usize = 40_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrphanPolicy {
    pub max_total_orphans: usize,
    pub max_orphans_per_peer: usize,
    pub max_announcers_per_orphan: usize,
    pub max_retained_bytes: usize,
    pub orphan_ttl_seconds: i64,
    pub max_reconsiderations_per_parent: usize,
}

impl Default for OrphanPolicy {
    fn default() -> Self {
        Self {
            max_total_orphans: PHASE102_MAX_ORPHAN_TRANSACTIONS,
            max_orphans_per_peer: PHASE102_MAX_ORPHANS_PER_PEER,
            max_announcers_per_orphan: PHASE133_MAX_ANNOUNCERS_PER_ORPHAN,
            max_retained_bytes: PHASE133_MAX_ORPHAN_RETAINED_BYTES,
            orphan_ttl_seconds: PHASE102_ORPHAN_TTL_SECONDS,
            max_reconsiderations_per_parent: PHASE102_MAX_RECONSIDERATIONS_PER_PARENT,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrphanEvidenceLabel {
    Orphaned,
    ParentRequested,
    OrphanEvicted,
    OrphanExpired,
    OrphanReconsidered,
}

impl OrphanEvidenceLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Orphaned => "orphaned",
            Self::ParentRequested => "parent_requested",
            Self::OrphanEvicted => "orphan_evicted",
            Self::OrphanExpired => "orphan_expired",
            Self::OrphanReconsidered => "orphan_reconsidered",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrphanReconsiderationStatus {
    Accepted,
    StillMissingParent,
    Rejected,
    Expired,
    Evicted,
}

impl OrphanReconsiderationStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted_child",
            Self::StillMissingParent => "still_missing_parent",
            Self::Rejected => "rejected_child",
            Self::Expired => "expired_child",
            Self::Evicted => "evicted_child",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanStageInput {
    pub transaction: Transaction,
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub missing_parents: Vec<Txid>,
    pub now_unix_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedOrphanAnnouncers {
    delivered_by: PeerId,
    peers: BTreeSet<PeerId>,
}

impl BoundedOrphanAnnouncers {
    fn from_provenance(provenance: ReceivedTransactionProvenance, max_announcers: usize) -> Self {
        let capacity = max_announcers.max(1);
        let mut peers = BTreeSet::from([provenance.delivered_by]);
        let ordered_announcers: BTreeSet<_> = provenance.announcers.into_iter().collect();
        for peer_id in ordered_announcers {
            if peers.len() == capacity {
                break;
            }
            peers.insert(peer_id);
        }
        Self {
            delivered_by: provenance.delivered_by,
            peers,
        }
    }

    fn contains(&self, peer_id: PeerId) -> bool {
        self.peers.contains(&peer_id)
    }

    fn primary_peer(&self) -> PeerId {
        self.contains(self.delivered_by)
            .then_some(self.delivered_by)
            .or_else(|| self.peers.first().copied())
            .unwrap_or(self.delivered_by)
    }

    fn provenance(&self) -> ReceivedTransactionProvenance {
        ReceivedTransactionProvenance {
            delivered_by: self.primary_peer(),
            announcers: self.peers.iter().copied().collect(),
        }
    }

    fn add(&mut self, peer_id: PeerId, max_announcers: usize) -> bool {
        if self.peers.len() >= max_announcers.max(1) {
            return false;
        }
        self.peers.insert(peer_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanReconsiderationCandidate {
    pub peer_id: PeerId,
    pub provenance: ReceivedTransactionProvenance,
    pub transaction: Transaction,
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub missing_parents: Vec<Txid>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrphanAction {
    RequestParent {
        peer_id: PeerId,
        relay_id: TxRelayId,
        label: OrphanEvidenceLabel,
    },
    Reconsider {
        candidate: OrphanReconsiderationCandidate,
        label: OrphanEvidenceLabel,
    },
    Evicted {
        peer_id: PeerId,
        txid: Txid,
        wtxid: Wtxid,
        label: OrphanEvidenceLabel,
    },
    Expired {
        peer_id: PeerId,
        txid: Txid,
        wtxid: Wtxid,
        label: OrphanEvidenceLabel,
    },
    PeerCleanup {
        peer_id: PeerId,
        removed: usize,
        label: OrphanEvidenceLabel,
    },
    Reconsidered {
        txid: Txid,
        wtxid: Wtxid,
        status: OrphanReconsiderationStatus,
        label: OrphanEvidenceLabel,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxOrphanage {
    policy: OrphanPolicy,
    orphans: BTreeMap<Wtxid, OrphanEntry>,
    pending_reconsideration: BTreeSet<Wtxid>,
    children_by_parent: BTreeMap<Txid, BTreeSet<(Reverse<u64>, Wtxid)>>,
    orphan_count_by_peer: BTreeMap<PeerId, usize>,
    candidate_cursors: BTreeMap<(Wtxid, PeerId), SamePeerCandidateCursor>,
    accepted_package_fingerprints: BTreeMap<[u8; 32], BTreeSet<Wtxid>>,
    insertion_sequence: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrphanEntry {
    announcers: BoundedOrphanAnnouncers,
    transaction: Transaction,
    txid: Txid,
    wtxid: Wtxid,
    missing_parents: BTreeSet<Txid>,
    expires_at_unix_seconds: i64,
    insertion_sequence: u64,
    body_bytes: usize,
}

impl TxOrphanage {
    pub fn new(policy: OrphanPolicy) -> Self {
        Self {
            policy,
            orphans: BTreeMap::new(),
            pending_reconsideration: BTreeSet::new(),
            children_by_parent: BTreeMap::new(),
            orphan_count_by_peer: BTreeMap::new(),
            candidate_cursors: BTreeMap::new(),
            accepted_package_fingerprints: BTreeMap::new(),
            insertion_sequence: 0,
        }
    }

    pub fn stage_missing_parent_with_provenance(
        &mut self,
        input: OrphanStageInput,
        provenance: ReceivedTransactionProvenance,
    ) -> Vec<OrphanAction> {
        let missing_parent_set: BTreeSet<_> = input.missing_parents.into_iter().collect();
        let missing_parents: Vec<_> = missing_parent_set.iter().copied().collect();
        let delivered_by = provenance.delivered_by;
        let announcers = BoundedOrphanAnnouncers::from_provenance(
            provenance,
            self.policy.max_announcers_per_orphan,
        );
        let expires_at_unix_seconds = input
            .now_unix_seconds
            .saturating_add(self.policy.orphan_ttl_seconds);
        let body_bytes = transaction_body_bytes(&input.transaction);

        if expires_at_unix_seconds <= input.now_unix_seconds {
            return vec![OrphanAction::Expired {
                peer_id: delivered_by,
                txid: input.txid,
                wtxid: input.wtxid,
                label: OrphanEvidenceLabel::OrphanExpired,
            }];
        }

        self.remove_orphan(input.wtxid);
        let insertion_sequence = self.insertion_sequence;
        self.insertion_sequence = self.insertion_sequence.saturating_add(1);
        for parent_txid in &missing_parent_set {
            self.children_by_parent
                .entry(*parent_txid)
                .or_default()
                .insert((Reverse(insertion_sequence), input.wtxid));
        }
        for peer_id in &announcers.peers {
            *self.orphan_count_by_peer.entry(*peer_id).or_default() += 1;
        }
        self.orphans.insert(
            input.wtxid,
            OrphanEntry {
                announcers,
                transaction: input.transaction,
                txid: input.txid,
                wtxid: input.wtxid,
                missing_parents: missing_parent_set,
                expires_at_unix_seconds,
                insertion_sequence,
                body_bytes,
            },
        );

        let mut actions = self.enforce_caps();
        if self.orphans.contains_key(&input.wtxid) {
            actions.extend(missing_parents.into_iter().map(|parent_txid| {
                OrphanAction::RequestParent {
                    peer_id: delivered_by,
                    relay_id: TxRelayId::Txid(parent_txid),
                    label: OrphanEvidenceLabel::ParentRequested,
                }
            }));
        }
        actions
    }

    pub fn add_announcer(&mut self, wtxid: Wtxid, peer_id: PeerId) -> bool {
        if self.peer_len(peer_id) >= self.policy.max_orphans_per_peer {
            return false;
        }
        if self
            .retained_bytes()
            .saturating_add(std::mem::size_of::<PeerId>())
            > self.policy.max_retained_bytes
        {
            return false;
        }
        let Some(entry) = self.orphans.get_mut(&wtxid) else {
            return false;
        };
        if !entry
            .announcers
            .add(peer_id, self.policy.max_announcers_per_orphan)
        {
            return false;
        }
        *self.orphan_count_by_peer.entry(peer_id).or_default() += 1;
        true
    }

    pub fn contains(&self, wtxid: Wtxid) -> bool {
        self.orphans.contains_key(&wtxid)
    }

    pub fn retained_wtxid(&self, relay_id: TxRelayId) -> Option<Wtxid> {
        match relay_id {
            TxRelayId::Wtxid(wtxid) => self.contains(wtxid).then_some(wtxid),
            TxRelayId::Txid(txid) => self
                .orphans
                .iter()
                .find_map(|(wtxid, entry)| (entry.txid == txid).then_some(*wtxid)),
        }
    }

    pub fn reconsider_after_parent(
        &mut self,
        accepted_parent: TxRelayId,
        now_unix_seconds: i64,
    ) -> Vec<OrphanAction> {
        let mut actions = self.expire(now_unix_seconds);
        let TxRelayId::Txid(parent_txid) = accepted_parent else {
            return actions;
        };
        self.candidate_cursors
            .retain(|_, cursor| cursor.parent_txid != parent_txid);

        let wtxids: Vec<_> = self.orphans.keys().copied().collect();
        for wtxid in wtxids {
            let is_ready = self.orphans.get_mut(&wtxid).is_some_and(|entry| {
                entry.missing_parents.remove(&parent_txid) && entry.missing_parents.is_empty()
            });
            self.remove_child_index(parent_txid, wtxid);
            if is_ready {
                self.pending_reconsideration.insert(wtxid);
            }
        }

        actions.extend(self.drain_reconsideration_batch());

        actions
    }

    pub fn drain_pending_reconsiderations(&mut self, now_unix_seconds: i64) -> Vec<OrphanAction> {
        let mut actions = self.expire(now_unix_seconds);
        actions.extend(self.drain_reconsideration_batch());
        actions
    }

    fn drain_reconsideration_batch(&mut self) -> Vec<OrphanAction> {
        let ready: Vec<_> = self
            .pending_reconsideration
            .iter()
            .copied()
            .take(self.policy.max_reconsiderations_per_parent)
            .collect();
        let mut actions = Vec::new();
        for wtxid in ready {
            self.pending_reconsideration.remove(&wtxid);
            if let Some(entry) = self.orphans.get(&wtxid) {
                actions.push(OrphanAction::Reconsider {
                    candidate: entry.candidate(),
                    label: OrphanEvidenceLabel::OrphanReconsidered,
                });
            }
        }

        actions
    }

    pub fn record_reconsideration_outcome(
        &mut self,
        wtxid: Wtxid,
        status: OrphanReconsiderationStatus,
    ) -> Vec<OrphanAction> {
        let Some((txid, wtxid)) = self
            .orphans
            .get(&wtxid)
            .map(|entry| (entry.txid, entry.wtxid))
        else {
            return Vec::new();
        };

        self.pending_reconsideration.remove(&wtxid);
        if status != OrphanReconsiderationStatus::StillMissingParent {
            self.remove_orphan(wtxid);
        }

        vec![OrphanAction::Reconsidered {
            txid,
            wtxid,
            status,
            label: OrphanEvidenceLabel::OrphanReconsidered,
        }]
    }

    pub fn expire(&mut self, now_unix_seconds: i64) -> Vec<OrphanAction> {
        let mut expired: Vec<_> = self
            .orphans
            .iter()
            .filter(|(_, entry)| entry.expires_at_unix_seconds <= now_unix_seconds)
            .map(|(wtxid, entry)| {
                (
                    entry.expires_at_unix_seconds,
                    entry.announcers.primary_peer(),
                    *wtxid,
                )
            })
            .collect();
        expired.sort();

        let mut actions = Vec::new();
        for (_, _, wtxid) in expired {
            actions.extend(
                self.remove_orphan(wtxid)
                    .map(|entry| OrphanAction::Expired {
                        peer_id: entry.announcers.primary_peer(),
                        txid: entry.txid,
                        wtxid: entry.wtxid,
                        label: OrphanEvidenceLabel::OrphanExpired,
                    }),
            );
        }
        actions
    }

    pub fn cleanup_peer(&mut self, peer_id: PeerId) -> Vec<OrphanAction> {
        let wtxids: Vec<_> = self
            .orphans
            .iter()
            .filter(|(_, entry)| entry.announcers.contains(peer_id))
            .map(|(wtxid, _)| *wtxid)
            .collect();
        let associations = wtxids.len();
        let mut removed = 0;
        for wtxid in wtxids {
            let should_remove = self.orphans.get_mut(&wtxid).is_some_and(|entry| {
                entry.announcers.peers.remove(&peer_id);
                entry.announcers.peers.is_empty()
            });
            self.decrement_peer_count(peer_id);
            if should_remove {
                removed += usize::from(self.remove_orphan(wtxid).is_some());
            }
        }
        self.candidate_cursors
            .retain(|(_, cursor_peer), _| *cursor_peer != peer_id);

        if associations == 0 {
            return Vec::new();
        }

        vec![OrphanAction::PeerCleanup {
            peer_id,
            removed,
            label: OrphanEvidenceLabel::OrphanEvicted,
        }]
    }

    fn enforce_caps(&mut self) -> Vec<OrphanAction> {
        let mut actions = Vec::new();

        while let Some(action) = (self.orphans.len() > self.policy.max_total_orphans)
            .then(|| self.evict_next(None))
            .flatten()
        {
            actions.push(action);
        }

        while let Some(action) = (self.retained_bytes() > self.policy.max_retained_bytes)
            .then(|| self.evict_next(None))
            .flatten()
        {
            actions.push(action);
        }

        while let Some(action) = self
            .orphan_count_by_peer
            .clone()
            .into_iter()
            .find(|(_, count)| *count > self.policy.max_orphans_per_peer)
            .map(|(peer_id, _)| peer_id)
            .and_then(|peer_id| self.evict_next(Some(peer_id)))
        {
            actions.push(action);
        }

        actions
    }

    fn evict_next(&mut self, maybe_peer_id: Option<PeerId>) -> Option<OrphanAction> {
        let maybe_wtxid = self
            .orphans
            .iter()
            .filter(|(_, entry)| {
                maybe_peer_id.is_none_or(|peer_id| entry.announcers.contains(peer_id))
            })
            .min_by_key(|(wtxid, entry)| {
                (
                    entry.expires_at_unix_seconds,
                    entry.announcers.primary_peer(),
                    **wtxid,
                )
            })
            .map(|(wtxid, _)| *wtxid);
        let entry = self.remove_orphan(maybe_wtxid?)?;

        Some(OrphanAction::Evicted {
            peer_id: maybe_peer_id.unwrap_or_else(|| entry.announcers.primary_peer()),
            txid: entry.txid,
            wtxid: entry.wtxid,
            label: OrphanEvidenceLabel::OrphanEvicted,
        })
    }

    fn remove_child_index(&mut self, parent_txid: Txid, wtxid: Wtxid) {
        let mut remove_parent = false;
        if let Some(children) = self.children_by_parent.get_mut(&parent_txid) {
            children.retain(|(_, child_wtxid)| *child_wtxid != wtxid);
            remove_parent = children.is_empty();
        }
        if remove_parent {
            self.children_by_parent.remove(&parent_txid);
        }
    }

    fn decrement_peer_count(&mut self, peer_id: PeerId) {
        let mut remove_peer = false;
        if let Some(count) = self.orphan_count_by_peer.get_mut(&peer_id) {
            *count = count.saturating_sub(1);
            remove_peer = *count == 0;
        }
        if remove_peer {
            self.orphan_count_by_peer.remove(&peer_id);
        }
    }

    fn remove_orphan(&mut self, wtxid: Wtxid) -> Option<OrphanEntry> {
        self.pending_reconsideration.remove(&wtxid);
        let entry = self.orphans.remove(&wtxid)?;
        for parent_txid in &entry.missing_parents {
            self.remove_child_index(*parent_txid, wtxid);
        }
        for peer_id in &entry.announcers.peers {
            self.decrement_peer_count(*peer_id);
        }
        for cursor in self.candidate_cursors.values_mut() {
            let removed_before_next = cursor.child_wtxids[..cursor.next_child]
                .iter()
                .filter(|child_wtxid| **child_wtxid == wtxid)
                .count();
            cursor.child_wtxids = cursor
                .child_wtxids
                .iter()
                .copied()
                .filter(|child_wtxid| *child_wtxid != wtxid)
                .collect::<Vec<_>>()
                .into_boxed_slice();
            cursor.next_child = cursor.next_child.saturating_sub(removed_before_next);
        }
        Some(entry)
    }

    #[cfg(test)]
    pub(crate) fn debug_indexes_match_oracle(&self) -> bool {
        let mut children_by_parent = BTreeMap::new();
        let mut orphan_count_by_peer = BTreeMap::new();
        for (wtxid, entry) in &self.orphans {
            for parent_txid in &entry.missing_parents {
                children_by_parent
                    .entry(*parent_txid)
                    .or_insert_with(BTreeSet::new)
                    .insert((Reverse(entry.insertion_sequence), *wtxid));
            }
            for peer_id in &entry.announcers.peers {
                *orphan_count_by_peer.entry(*peer_id).or_default() += 1;
            }
        }
        children_by_parent == self.children_by_parent
            && orphan_count_by_peer == self.orphan_count_by_peer
            && self
                .pending_reconsideration
                .iter()
                .all(|wtxid| self.orphans.contains_key(wtxid))
    }
}

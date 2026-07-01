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
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_primitives::{Transaction, Txid, Wtxid};

use crate::error::PeerId;

use super::TxRelayId;

pub const PHASE102_MAX_ORPHAN_TRANSACTIONS: usize = 100;
pub const PHASE102_MAX_ORPHANS_PER_PEER: usize = 25;
pub const PHASE102_ORPHAN_TTL_SECONDS: i64 = 20 * 60;
pub const PHASE102_MAX_RECONSIDERATIONS_PER_PARENT: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrphanPolicy {
    pub max_total_orphans: usize,
    pub max_orphans_per_peer: usize,
    pub orphan_ttl_seconds: i64,
    pub max_reconsiderations_per_parent: usize,
}

impl Default for OrphanPolicy {
    fn default() -> Self {
        Self {
            max_total_orphans: PHASE102_MAX_ORPHAN_TRANSACTIONS,
            max_orphans_per_peer: PHASE102_MAX_ORPHANS_PER_PEER,
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
    pub peer_id: PeerId,
    pub transaction: Transaction,
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub missing_parents: Vec<Txid>,
    pub now_unix_seconds: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrphanReconsiderationCandidate {
    pub peer_id: PeerId,
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
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct OrphanEntry {
    peer_id: PeerId,
    transaction: Transaction,
    txid: Txid,
    wtxid: Wtxid,
    missing_parents: BTreeSet<Txid>,
    expires_at_unix_seconds: i64,
}

impl TxOrphanage {
    pub fn new(policy: OrphanPolicy) -> Self {
        Self {
            policy,
            orphans: BTreeMap::new(),
            pending_reconsideration: BTreeSet::new(),
        }
    }

    pub fn stage_missing_parent(&mut self, input: OrphanStageInput) -> Vec<OrphanAction> {
        let missing_parent_set: BTreeSet<_> = input.missing_parents.into_iter().collect();
        let missing_parents: Vec<_> = missing_parent_set.iter().copied().collect();
        let expires_at_unix_seconds = input
            .now_unix_seconds
            .saturating_add(self.policy.orphan_ttl_seconds);

        if expires_at_unix_seconds <= input.now_unix_seconds {
            return vec![OrphanAction::Expired {
                peer_id: input.peer_id,
                txid: input.txid,
                wtxid: input.wtxid,
                label: OrphanEvidenceLabel::OrphanExpired,
            }];
        }

        self.remove_orphan(input.wtxid);
        self.orphans.insert(
            input.wtxid,
            OrphanEntry {
                peer_id: input.peer_id,
                transaction: input.transaction,
                txid: input.txid,
                wtxid: input.wtxid,
                missing_parents: missing_parent_set,
                expires_at_unix_seconds,
            },
        );

        let mut actions = self.enforce_caps();
        if self.orphans.contains_key(&input.wtxid) {
            actions.extend(missing_parents.into_iter().map(|parent_txid| {
                OrphanAction::RequestParent {
                    peer_id: input.peer_id,
                    relay_id: TxRelayId::Txid(parent_txid),
                    label: OrphanEvidenceLabel::ParentRequested,
                }
            }));
        }
        actions
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

        let wtxids: Vec<_> = self.orphans.keys().copied().collect();
        for wtxid in wtxids {
            let is_ready = self.orphans.get_mut(&wtxid).is_some_and(|entry| {
                entry.missing_parents.remove(&parent_txid) && entry.missing_parents.is_empty()
            });
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
            .map(|(wtxid, entry)| (entry.expires_at_unix_seconds, entry.peer_id, *wtxid))
            .collect();
        expired.sort();

        let mut actions = Vec::new();
        for (_, _, wtxid) in expired {
            actions.extend(
                self.remove_orphan(wtxid)
                    .map(|entry| OrphanAction::Expired {
                        peer_id: entry.peer_id,
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
            .filter(|(_, entry)| entry.peer_id == peer_id)
            .map(|(wtxid, _)| *wtxid)
            .collect();
        let removed = wtxids.len();

        for wtxid in wtxids {
            self.remove_orphan(wtxid);
        }

        if removed == 0 {
            return Vec::new();
        }

        vec![OrphanAction::PeerCleanup {
            peer_id,
            removed,
            label: OrphanEvidenceLabel::OrphanEvicted,
        }]
    }

    pub fn len(&self) -> usize {
        self.orphans.len()
    }

    pub fn peer_len(&self, peer_id: PeerId) -> usize {
        self.orphans
            .values()
            .filter(|entry| entry.peer_id == peer_id)
            .count()
    }

    pub fn is_empty(&self) -> bool {
        self.orphans.is_empty()
    }

    fn enforce_caps(&mut self) -> Vec<OrphanAction> {
        let mut actions = Vec::new();

        while let Some(action) = (self.orphans.len() > self.policy.max_total_orphans)
            .then(|| self.evict_next(None))
            .flatten()
        {
            actions.push(action);
        }

        while let Some(action) = self
            .peer_counts()
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
            .filter(|(_, entry)| maybe_peer_id.is_none_or(|peer_id| entry.peer_id == peer_id))
            .min_by_key(|(wtxid, entry)| (entry.expires_at_unix_seconds, entry.peer_id, **wtxid))
            .map(|(wtxid, _)| *wtxid);
        let entry = self.remove_orphan(maybe_wtxid?)?;

        Some(OrphanAction::Evicted {
            peer_id: entry.peer_id,
            txid: entry.txid,
            wtxid: entry.wtxid,
            label: OrphanEvidenceLabel::OrphanEvicted,
        })
    }

    fn peer_counts(&self) -> BTreeMap<PeerId, usize> {
        let mut counts = BTreeMap::new();
        for entry in self.orphans.values() {
            *counts.entry(entry.peer_id).or_insert(0) += 1;
        }
        counts
    }

    fn remove_orphan(&mut self, wtxid: Wtxid) -> Option<OrphanEntry> {
        self.pending_reconsideration.remove(&wtxid);
        self.orphans.remove(&wtxid)
    }
}

impl OrphanEntry {
    fn candidate(&self) -> OrphanReconsiderationCandidate {
        OrphanReconsiderationCandidate {
            peer_id: self.peer_id,
            transaction: self.transaction.clone(),
            txid: self.txid,
            wtxid: self.wtxid,
            missing_parents: self.missing_parents.iter().copied().collect(),
        }
    }
}

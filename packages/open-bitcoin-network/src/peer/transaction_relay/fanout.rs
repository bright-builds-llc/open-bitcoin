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

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use open_bitcoin_primitives::{Txid, Wtxid};

use crate::{RelayEligibilityDecision, error::PeerId};

use super::{TxRelayId, TxRelayPeerMode};

pub const PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER: usize = 1024;
pub const PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER: usize = 16;
pub const PHASE104_TX_FANOUT_MIN_INTERVAL_SECONDS: i64 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxFanoutPolicy {
    pub max_queue_per_peer: usize,
    pub max_drain_per_peer: usize,
    pub min_interval_seconds: i64,
}

impl Default for TxFanoutPolicy {
    fn default() -> Self {
        Self {
            max_queue_per_peer: PHASE104_MAX_TX_FANOUT_QUEUE_PER_PEER,
            max_drain_per_peer: PHASE104_MAX_TX_FANOUT_DRAIN_PER_PEER,
            min_interval_seconds: PHASE104_TX_FANOUT_MIN_INTERVAL_SECONDS,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxFanoutAdmissionOutcome {
    Accepted,
    Replaced,
}

impl TxFanoutAdmissionOutcome {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Replaced => "replaced",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TxFanoutAdmission {
    pub txid: Txid,
    pub wtxid: Wtxid,
    pub outcome: TxFanoutAdmissionOutcome,
}

impl TxFanoutAdmission {
    pub const fn relay_id_for_peer_mode(self, peer_mode: TxRelayPeerMode) -> TxRelayId {
        match peer_mode {
            TxRelayPeerMode::TxidOnly => TxRelayId::Txid(self.txid),
            TxRelayPeerMode::WtxidRelay => TxRelayId::Wtxid(self.wtxid),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxFanoutSuppressionReason {
    OriginPeer,
    AlreadyHave,
    RecentReject,
    InFlight,
    MempoolKnown,
    RelayDisabled,
    NotRelayEligible,
    QueueCapReached,
    RateLimited,
    IdentityUnavailable,
}

impl TxFanoutSuppressionReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::OriginPeer => "origin_peer",
            Self::AlreadyHave => "already_have",
            Self::RecentReject => "recent_reject",
            Self::InFlight => "in_flight",
            Self::MempoolKnown => "mempool_known",
            Self::RelayDisabled => "relay_disabled",
            Self::NotRelayEligible => "not_relay_eligible",
            Self::QueueCapReached => "queue_cap_reached",
            Self::RateLimited => "rate_limited",
            Self::IdentityUnavailable => "identity_unavailable",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxFanoutCleanupReason {
    Confirmed,
    Replaced,
    Evicted,
    Expired,
    PeerDisconnected,
}

impl TxFanoutCleanupReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Confirmed => "confirmed",
            Self::Replaced => "replaced",
            Self::Evicted => "evicted",
            Self::Expired => "expired",
            Self::PeerDisconnected => "peer_disconnected",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxFanoutAction {
    Announce {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    Suppress {
        peer_id: PeerId,
        relay_id: TxRelayId,
        reason: TxFanoutSuppressionReason,
    },
    QueueCap {
        peer_id: PeerId,
        relay_id: TxRelayId,
    },
    RateLimit {
        peer_id: PeerId,
        ready_at_unix_seconds: i64,
    },
    Cleanup {
        relay_id: TxRelayId,
        reason: TxFanoutCleanupReason,
    },
    RebroadcastDeferred {
        relay_id: TxRelayId,
    },
}

impl TxFanoutAction {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Announce { .. } => "announce",
            Self::Suppress { .. } => "suppress",
            Self::QueueCap { .. } => "queue_cap",
            Self::RateLimit { .. } => "rate_limit",
            Self::Cleanup { .. } => "cleanup",
            Self::RebroadcastDeferred { .. } => "rebroadcast_deferred",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxFanoutPeerInput {
    pub peer_id: PeerId,
    pub peer_mode: TxRelayPeerMode,
    pub relay_eligibility: RelayEligibilityDecision,
    pub origin_peer: bool,
    pub already_have: bool,
    pub recent_reject: bool,
    pub in_flight: bool,
    pub mempool_known: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TxFanoutSnapshot {
    pub peer_count: usize,
    pub queued_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TxFanoutQueue {
    policy: TxFanoutPolicy,
    peers: BTreeMap<PeerId, PeerFanoutQueue>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct PeerFanoutQueue {
    queued: VecDeque<TxRelayId>,
    queued_ids: BTreeSet<TxRelayId>,
    maybe_last_drained_at_unix_seconds: Option<i64>,
}

impl TxFanoutQueue {
    pub fn new(policy: TxFanoutPolicy) -> Self {
        Self {
            policy,
            peers: BTreeMap::new(),
        }
    }

    pub fn enqueue_admission(
        &mut self,
        admission: TxFanoutAdmission,
        peers: &[TxFanoutPeerInput],
    ) -> Vec<TxFanoutAction> {
        let mut actions = Vec::new();

        for peer in peers {
            let relay_id = admission.relay_id_for_peer_mode(peer.peer_mode);
            if let Some(reason) = suppression_reason(peer, relay_id) {
                actions.push(TxFanoutAction::Suppress {
                    peer_id: peer.peer_id,
                    relay_id,
                    reason,
                });
                continue;
            }

            if self.peer_queue(peer.peer_id).queued_ids.contains(&relay_id) {
                actions.push(TxFanoutAction::Suppress {
                    peer_id: peer.peer_id,
                    relay_id,
                    reason: TxFanoutSuppressionReason::AlreadyHave,
                });
                continue;
            }

            if self.peer_queue(peer.peer_id).queued.len() >= self.policy.max_queue_per_peer {
                actions.push(TxFanoutAction::QueueCap {
                    peer_id: peer.peer_id,
                    relay_id,
                });
                continue;
            }

            let peer_queue = self.peer_queue_mut(peer.peer_id);
            peer_queue.queued.push_back(relay_id);
            peer_queue.queued_ids.insert(relay_id);
        }

        actions
    }

    pub fn drain_peer(&mut self, peer_id: PeerId, now_unix_seconds: i64) -> Vec<TxFanoutAction> {
        let policy = self.policy;
        let Some(peer_queue) = self.peers.get_mut(&peer_id) else {
            return Vec::new();
        };

        if peer_queue.queued.is_empty() {
            return Vec::new();
        }

        if let Some(ready_at_unix_seconds) =
            ready_at_unix_seconds(policy, peer_queue, now_unix_seconds)
        {
            return vec![TxFanoutAction::RateLimit {
                peer_id,
                ready_at_unix_seconds,
            }];
        }

        let mut actions = Vec::new();
        for _ in 0..policy.max_drain_per_peer {
            let Some(relay_id) = peer_queue.queued.pop_front() else {
                break;
            };
            peer_queue.queued_ids.remove(&relay_id);
            actions.push(TxFanoutAction::Announce { peer_id, relay_id });
        }
        peer_queue.maybe_last_drained_at_unix_seconds = Some(now_unix_seconds);
        actions
    }

    pub fn cleanup_transaction(
        &mut self,
        txid: Txid,
        wtxid: Wtxid,
        reason: TxFanoutCleanupReason,
    ) -> Vec<TxFanoutAction> {
        let mut actions = Vec::new();
        actions.extend(self.cleanup_relay_id(TxRelayId::Txid(txid), reason));
        actions.extend(self.cleanup_relay_id(TxRelayId::Wtxid(wtxid), reason));
        actions
    }

    pub fn cleanup_relay_id(
        &mut self,
        relay_id: TxRelayId,
        reason: TxFanoutCleanupReason,
    ) -> Vec<TxFanoutAction> {
        let mut removed_any = false;
        for peer_queue in self.peers.values_mut() {
            if !peer_queue.queued_ids.remove(&relay_id) {
                continue;
            }
            peer_queue.queued.retain(|queued_id| *queued_id != relay_id);
            removed_any = true;
        }

        if !removed_any {
            return Vec::new();
        }

        vec![TxFanoutAction::Cleanup { relay_id, reason }]
    }

    pub fn cleanup_peer(
        &mut self,
        peer_id: PeerId,
        reason: TxFanoutCleanupReason,
    ) -> Vec<TxFanoutAction> {
        let Some(peer_queue) = self.peers.remove(&peer_id) else {
            return Vec::new();
        };

        peer_queue
            .queued_ids
            .into_iter()
            .map(|relay_id| TxFanoutAction::Cleanup { relay_id, reason })
            .collect()
    }

    pub fn snapshot(&self) -> TxFanoutSnapshot {
        TxFanoutSnapshot {
            peer_count: self.peers.len(),
            queued_count: self
                .peers
                .values()
                .map(|peer_queue| peer_queue.queued.len())
                .sum(),
        }
    }

    fn peer_queue(&self, peer_id: PeerId) -> PeerFanoutQueue {
        self.peers.get(&peer_id).cloned().unwrap_or_default()
    }

    fn peer_queue_mut(&mut self, peer_id: PeerId) -> &mut PeerFanoutQueue {
        self.peers.entry(peer_id).or_default()
    }
}

impl Default for TxFanoutQueue {
    fn default() -> Self {
        Self::new(TxFanoutPolicy::default())
    }
}

pub fn defer_local_rebroadcast(
    admission: TxFanoutAdmission,
    local_origin: bool,
    periodic_rebroadcast_requested: bool,
) -> Option<TxFanoutAction> {
    if !local_origin || !periodic_rebroadcast_requested {
        return None;
    }

    if admission.outcome != TxFanoutAdmissionOutcome::Accepted {
        return None;
    }

    Some(TxFanoutAction::RebroadcastDeferred {
        relay_id: TxRelayId::Txid(admission.txid),
    })
}

fn suppression_reason(
    peer: &TxFanoutPeerInput,
    relay_id: TxRelayId,
) -> Option<TxFanoutSuppressionReason> {
    if matches!(relay_id, TxRelayId::Txid(txid) if txid == Txid::from_byte_array([0; 32]))
        || matches!(relay_id, TxRelayId::Wtxid(wtxid) if wtxid == Wtxid::from_byte_array([0; 32]))
    {
        return Some(TxFanoutSuppressionReason::IdentityUnavailable);
    }

    if peer.origin_peer {
        return Some(TxFanoutSuppressionReason::OriginPeer);
    }

    if !peer.relay_eligibility.eligible {
        return Some(TxFanoutSuppressionReason::NotRelayEligible);
    }

    if !peer.relay_eligibility.version_message_relay {
        return Some(TxFanoutSuppressionReason::RelayDisabled);
    }

    if peer.already_have {
        return Some(TxFanoutSuppressionReason::AlreadyHave);
    }

    if peer.recent_reject {
        return Some(TxFanoutSuppressionReason::RecentReject);
    }

    if peer.in_flight {
        return Some(TxFanoutSuppressionReason::InFlight);
    }

    if peer.mempool_known {
        return Some(TxFanoutSuppressionReason::MempoolKnown);
    }

    None
}

fn ready_at_unix_seconds(
    policy: TxFanoutPolicy,
    peer_queue: &PeerFanoutQueue,
    now_unix_seconds: i64,
) -> Option<i64> {
    let last_drained_at_unix_seconds = peer_queue.maybe_last_drained_at_unix_seconds?;
    let ready_at_unix_seconds =
        last_drained_at_unix_seconds.saturating_add(policy.min_interval_seconds);
    if ready_at_unix_seconds > now_unix_seconds {
        return Some(ready_at_unix_seconds);
    }
    None
}

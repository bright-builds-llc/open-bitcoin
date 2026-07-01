// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_core::primitives::Txid;
use open_bitcoin_mempool::MempoolOutcome;
use open_bitcoin_network::{
    InventoryList, PeerId, TxFanoutAction, TxFanoutAdmission, TxFanoutAdmissionOutcome,
    TxFanoutCleanupReason, TxFanoutPeerInput, TxFanoutQueue, TxRelayId, TxServingRecordStatus,
    WireNetworkMessage,
};

use super::ManagedPeerNetwork;
use crate::ChainstateStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ManagedRelayFanoutInfo {
    pub known_transactions: usize,
    pub queued_peers: usize,
    pub queued_transactions: usize,
    pub latest_actions: Vec<ManagedRelayFanoutActionInfo>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedRelayFanoutActionInfo {
    pub label: &'static str,
    pub reason: Option<&'static str>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct ManagedRelayFanoutState {
    queue: TxFanoutQueue,
    wtxids_by_txid: BTreeMap<Txid, open_bitcoin_core::primitives::Wtxid>,
    recent_rejects_by_peer: BTreeMap<PeerId, BTreeSet<TxRelayId>>,
    latest_actions: Vec<ManagedRelayFanoutActionInfo>,
}

impl ManagedRelayFanoutState {
    pub(super) fn record_admission_outcome(
        &mut self,
        origin_peer: Option<PeerId>,
        outcome: &MempoolOutcome,
        peers: &[TxFanoutPeerInput],
    ) -> Vec<TxFanoutAction> {
        let actions = match outcome {
            MempoolOutcome::Accepted { txid, wtxid, .. } => {
                self.wtxids_by_txid.insert(*txid, *wtxid);
                self.queue.enqueue_admission(
                    TxFanoutAdmission {
                        txid: *txid,
                        wtxid: *wtxid,
                        outcome: TxFanoutAdmissionOutcome::Accepted,
                    },
                    peers,
                )
            }
            MempoolOutcome::Replaced { txid, wtxid, .. } => {
                self.wtxids_by_txid.insert(*txid, *wtxid);
                self.queue.enqueue_admission(
                    TxFanoutAdmission {
                        txid: *txid,
                        wtxid: *wtxid,
                        outcome: TxFanoutAdmissionOutcome::Replaced,
                    },
                    peers,
                )
            }
            MempoolOutcome::Rejected { txid, wtxid, .. } => {
                if let Some(peer_id) = origin_peer {
                    let rejects = self.recent_rejects_by_peer.entry(peer_id).or_default();
                    rejects.insert(TxRelayId::Txid(*txid));
                    rejects.insert(TxRelayId::Wtxid(*wtxid));
                }
                Vec::new()
            }
            MempoolOutcome::Evicted { txid, .. } => {
                self.cleanup_transactions(&[*txid], TxFanoutCleanupReason::Evicted)
            }
            MempoolOutcome::Expired { txid, .. } => {
                self.cleanup_transactions(&[*txid], TxFanoutCleanupReason::Expired)
            }
            MempoolOutcome::Duplicate { .. } | MempoolOutcome::Orphaned { .. } => Vec::new(),
        };
        if !actions.is_empty() {
            self.replace_latest_actions(&actions);
        }
        actions
    }

    pub(super) fn drain_peer_fanout(
        &mut self,
        peer_id: PeerId,
        now_unix_seconds: i64,
    ) -> Vec<TxFanoutAction> {
        let actions = self.queue.drain_peer(peer_id, now_unix_seconds);
        self.append_latest_actions(&actions);
        actions
    }

    pub(super) fn cleanup_transactions(
        &mut self,
        txids: &[Txid],
        reason: TxFanoutCleanupReason,
    ) -> Vec<TxFanoutAction> {
        let mut actions = Vec::new();
        for txid in txids {
            let Some(wtxid) = self.wtxids_by_txid.remove(txid) else {
                actions.extend(self.queue.cleanup_relay_id(TxRelayId::Txid(*txid), reason));
                self.remove_recent_reject(TxRelayId::Txid(*txid));
                continue;
            };

            let before = actions.len();
            actions.extend(self.queue.cleanup_transaction(*txid, wtxid, reason));
            if actions.len() == before {
                actions.push(TxFanoutAction::Cleanup {
                    relay_id: TxRelayId::Txid(*txid),
                    reason,
                });
            }
            self.remove_recent_reject(TxRelayId::Txid(*txid));
            self.remove_recent_reject(TxRelayId::Wtxid(wtxid));
        }
        if !actions.is_empty() {
            self.replace_latest_actions(&actions);
        }
        actions
    }

    pub(super) fn cleanup_peer(&mut self, peer_id: PeerId) -> Vec<TxFanoutAction> {
        self.recent_rejects_by_peer.remove(&peer_id);
        let actions = self
            .queue
            .cleanup_peer(peer_id, TxFanoutCleanupReason::PeerDisconnected);
        self.replace_latest_actions(&actions);
        actions
    }

    pub(super) fn info(&self) -> ManagedRelayFanoutInfo {
        let snapshot = self.queue.snapshot();
        ManagedRelayFanoutInfo {
            known_transactions: self.wtxids_by_txid.len(),
            queued_peers: snapshot.peer_count,
            queued_transactions: snapshot.queued_count,
            latest_actions: self.latest_actions.clone(),
        }
    }

    pub(super) fn is_recent_reject(&self, peer_id: PeerId, relay_id: TxRelayId) -> bool {
        self.recent_rejects_by_peer
            .get(&peer_id)
            .is_some_and(|rejects| rejects.contains(&relay_id))
    }

    fn remove_recent_reject(&mut self, relay_id: TxRelayId) {
        self.recent_rejects_by_peer.retain(|_peer_id, rejects| {
            rejects.remove(&relay_id);
            !rejects.is_empty()
        });
    }

    fn replace_latest_actions(&mut self, actions: &[TxFanoutAction]) {
        self.latest_actions.clear();
        self.append_latest_actions(actions);
    }

    fn append_latest_actions(&mut self, actions: &[TxFanoutAction]) {
        self.latest_actions
            .extend(actions.iter().map(ManagedRelayFanoutActionInfo::from));
    }
}

impl From<&TxFanoutAction> for ManagedRelayFanoutActionInfo {
    fn from(action: &TxFanoutAction) -> Self {
        Self {
            label: action.as_str(),
            reason: fanout_action_reason(action),
        }
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn relay_fanout_info(&self) -> ManagedRelayFanoutInfo {
        self.relay_fanout.info()
    }

    pub(super) fn record_relay_fanout_for_outcome(
        &mut self,
        origin_peer: Option<PeerId>,
        outcome: &MempoolOutcome,
        now_unix_seconds: i64,
    ) -> Vec<(PeerId, WireNetworkMessage)> {
        let maybe_admission = tx_fanout_admission_from_outcome(outcome);
        let peer_inputs = self.relay_fanout_peer_inputs(origin_peer, maybe_admission);
        let peer_ids = self.peer_manager.peer_ids();
        let mut actions =
            self.relay_fanout
                .record_admission_outcome(origin_peer, outcome, &peer_inputs);
        for peer_id in peer_ids {
            actions.extend(
                self.relay_fanout
                    .drain_peer_fanout(peer_id, now_unix_seconds),
            );
        }
        actions
            .into_iter()
            .filter_map(translate_fanout_action)
            .collect()
    }

    fn relay_fanout_peer_inputs(
        &self,
        origin_peer: Option<PeerId>,
        maybe_admission: Option<TxFanoutAdmission>,
    ) -> Vec<TxFanoutPeerInput> {
        self.peer_manager
            .peer_ids()
            .into_iter()
            .filter_map(|peer_id| {
                let _peer = self.peer_manager.peer_state(peer_id)?;
                let (peer_mode, relay_eligibility) = self.relay_serving_context_for_peer(peer_id);
                let relay_id =
                    maybe_admission.map(|admission| admission.relay_id_for_peer_mode(peer_mode));
                Some(TxFanoutPeerInput {
                    peer_id,
                    peer_mode,
                    relay_eligibility,
                    origin_peer: origin_peer == Some(peer_id),
                    already_have: false,
                    recent_reject: relay_id.is_some_and(|relay_id| {
                        self.relay_fanout.is_recent_reject(peer_id, relay_id)
                    }),
                    in_flight: false,
                    mempool_known: false,
                })
            })
            .collect()
    }
}

pub(super) fn cleanup_reason_for_serving_status(
    status: TxServingRecordStatus,
) -> Option<TxFanoutCleanupReason> {
    match status {
        TxServingRecordStatus::Confirmed => Some(TxFanoutCleanupReason::Confirmed),
        TxServingRecordStatus::Replaced => Some(TxFanoutCleanupReason::Replaced),
        TxServingRecordStatus::Evicted => Some(TxFanoutCleanupReason::Evicted),
        TxServingRecordStatus::Expired => Some(TxFanoutCleanupReason::Expired),
        TxServingRecordStatus::Accepted
        | TxServingRecordStatus::Stale
        | TxServingRecordStatus::Rejected => None,
    }
}

fn tx_fanout_admission_from_outcome(outcome: &MempoolOutcome) -> Option<TxFanoutAdmission> {
    match outcome {
        MempoolOutcome::Accepted { txid, wtxid, .. } => Some(TxFanoutAdmission {
            txid: *txid,
            wtxid: *wtxid,
            outcome: TxFanoutAdmissionOutcome::Accepted,
        }),
        MempoolOutcome::Replaced { txid, wtxid, .. } => Some(TxFanoutAdmission {
            txid: *txid,
            wtxid: *wtxid,
            outcome: TxFanoutAdmissionOutcome::Replaced,
        }),
        MempoolOutcome::Rejected { .. }
        | MempoolOutcome::Duplicate { .. }
        | MempoolOutcome::Orphaned { .. }
        | MempoolOutcome::Evicted { .. }
        | MempoolOutcome::Expired { .. } => None,
    }
}

fn translate_fanout_action(action: TxFanoutAction) -> Option<(PeerId, WireNetworkMessage)> {
    let TxFanoutAction::Announce { peer_id, relay_id } = action else {
        return None;
    };
    Some((
        peer_id,
        WireNetworkMessage::Inv(InventoryList::new(vec![relay_id.to_inventory_vector()])),
    ))
}

fn fanout_action_reason(action: &TxFanoutAction) -> Option<&'static str> {
    match action {
        TxFanoutAction::Suppress { reason, .. } => Some(reason.as_str()),
        TxFanoutAction::Cleanup { reason, .. } => Some(reason.as_str()),
        TxFanoutAction::QueueCap { .. } => Some("queue_cap_reached"),
        TxFanoutAction::RateLimit { .. } => Some("rate_limited"),
        TxFanoutAction::Announce { .. } | TxFanoutAction::RebroadcastDeferred { .. } => None,
    }
}

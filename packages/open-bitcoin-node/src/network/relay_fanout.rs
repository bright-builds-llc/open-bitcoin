// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use std::collections::{BTreeMap, BTreeSet};

use open_bitcoin_core::primitives::{Txid, Wtxid};
use open_bitcoin_mempool::{MempoolOutcome, RelayIntent};
use open_bitcoin_network::{
    InventoryList, PeerId, TxFanoutAction, TxFanoutAdmission, TxFanoutAdmissionOutcome,
    TxFanoutCleanupReason, TxFanoutPeerInput, TxFanoutQueue, TxFanoutSuppressionReason, TxRelayId,
    WireNetworkMessage, defer_local_rebroadcast,
};

use super::ManagedPeerNetwork;
use super::lifecycle_projection::PreparedFanoutProjection;
use super::relay_serving::ManagedRelayServingInfo;
use crate::ChainstateStore;
use crate::status::relay_evidence::{
    RELAY_RECOVERY_EVIDENCE_UNAVAILABLE_REASON, RelayActivationEvidence, RelayCapabilityEvidence,
    RelayDownloadEligibilityCounters, RelayEvidenceCapability, RelayEvidenceCounters,
    RelayEvidenceField, RelayEvidenceStatus, RelayRecoveryCounters,
};

mod action_info;
mod lifecycle;

pub(super) use lifecycle::cleanup_reason_for_serving_status;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalRelaySubmissionLabel {
    Accepted,
    Queued,
    Suppressed,
    NotEligible,
    RelayDisabled,
    Duplicate,
    Rejected,
    Orphaned,
    Evicted,
    Expired,
    RebroadcastDeferred,
}

impl LocalRelaySubmissionLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Queued => "queued",
            Self::Suppressed => "suppressed",
            Self::NotEligible => "not_eligible",
            Self::RelayDisabled => "relay_disabled",
            Self::Duplicate => "duplicate",
            Self::Rejected => "rejected",
            Self::Orphaned => "orphaned",
            Self::Evicted => "evicted",
            Self::Expired => "expired",
            Self::RebroadcastDeferred => "rebroadcast_deferred",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebroadcastEvidenceLabel {
    Deferred,
}

impl RebroadcastEvidenceLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Deferred => "rebroadcast_deferred",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalRelaySubmissionEvidence {
    pub labels: Vec<LocalRelaySubmissionLabel>,
    pub queued_count: usize,
    pub suppressed_count: usize,
    pub not_eligible_count: usize,
    pub relay_disabled_count: usize,
    pub maybe_rebroadcast: Option<RebroadcastEvidenceLabel>,
}

#[derive(Debug, Clone, Default)]
pub(super) struct ManagedRelayFanoutState {
    queue: TxFanoutQueue,
    wtxids_by_txid: BTreeMap<Txid, open_bitcoin_core::primitives::Wtxid>,
    recent_rejects_by_peer: BTreeMap<PeerId, BTreeSet<TxRelayId>>,
    latest_actions: Vec<ManagedRelayFanoutActionInfo>,
    latest_local_submission: Option<LocalRelaySubmissionEvidence>,
}

impl ManagedRelayFanoutState {
    pub(super) fn seed_recovered_transaction(&mut self, txid: Txid, wtxid: Wtxid) {
        self.wtxids_by_txid.insert(txid, wtxid);
    }

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

    pub(super) fn record_local_submission_outcome(
        &mut self,
        outcome: &MempoolOutcome,
        peers: &[TxFanoutPeerInput],
        periodic_rebroadcast_requested: bool,
    ) -> Vec<TxFanoutAction> {
        let queued_before = self.queue.snapshot().queued_count;
        let mut actions = match outcome {
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                self.record_admission_outcome(None, outcome, peers)
            }
            MempoolOutcome::Rejected { .. }
            | MempoolOutcome::Duplicate { .. }
            | MempoolOutcome::Orphaned { .. }
            | MempoolOutcome::Evicted { .. }
            | MempoolOutcome::Expired { .. } => Vec::new(),
        };
        if let Some(rebroadcast_action) =
            tx_fanout_admission_from_outcome(outcome).and_then(|admission| {
                defer_local_rebroadcast(admission, true, periodic_rebroadcast_requested)
            })
        {
            actions.push(rebroadcast_action);
        }
        let queued_after = self.queue.snapshot().queued_count;
        let queued_count = queued_after.saturating_sub(queued_before);
        self.latest_local_submission =
            Some(local_submission_evidence(outcome, queued_count, &actions));
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

    pub(super) fn latest_local_submission_evidence(&self) -> Option<LocalRelaySubmissionEvidence> {
        self.latest_local_submission.clone()
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

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    #[allow(dead_code)] // Plan 134-05 invokes the closed aggregate apply.
    pub(super) fn apply_prepared_fanout(&mut self, prepared: PreparedFanoutProjection) {
        self.relay_fanout = prepared.replacement;
    }

    pub fn relay_fanout_info(&self) -> ManagedRelayFanoutInfo {
        self.relay_fanout.info()
    }

    pub fn latest_local_submission_evidence(&self) -> Option<LocalRelaySubmissionEvidence> {
        self.relay_fanout.latest_local_submission_evidence()
    }

    pub fn relay_evidence_status(&self) -> RelayEvidenceStatus {
        let maybe_local_submission = self.latest_local_submission_evidence();
        let fanout_info = self.relay_fanout_info();
        let serving_info = self.relay_serving_info();
        let recovery_counters = if self.latest_mempool_recovery_storage_error.is_some() {
            RelayEvidenceField::unavailable(RELAY_RECOVERY_EVIDENCE_UNAVAILABLE_REASON)
        } else {
            RelayEvidenceField::implemented(
                self.latest_mempool_recovery
                    .as_ref()
                    .map(RelayRecoveryCounters::from)
                    .unwrap_or_default(),
            )
        };
        let activation = RelayActivationEvidence {
            enabled: self.relay_activation.enabled,
        };
        let download_eligibility = self.relay_download_eligibility_counters();
        relay_evidence_status_from_parts(
            activation,
            download_eligibility,
            recovery_counters,
            maybe_local_submission.as_ref(),
            &fanout_info,
            &serving_info,
        )
    }

    pub(super) fn record_local_submission_outcome(
        &mut self,
        outcome: &MempoolOutcome,
        relay_intent: RelayIntent,
    ) -> Vec<TxFanoutAction> {
        let maybe_admission = tx_fanout_admission_from_outcome(outcome);
        let peer_inputs = match relay_intent {
            RelayIntent::Requested => self.relay_fanout_peer_inputs(None, maybe_admission),
            RelayIntent::NotRequested => Vec::new(),
        };
        self.relay_fanout.record_local_submission_outcome(
            outcome,
            &peer_inputs,
            relay_intent == RelayIntent::Requested,
        )
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

fn relay_evidence_status_from_parts(
    activation: RelayActivationEvidence,
    download_eligibility: RelayDownloadEligibilityCounters,
    recovery: RelayEvidenceField<RelayRecoveryCounters>,
    maybe_local_submission: Option<&LocalRelaySubmissionEvidence>,
    fanout_info: &ManagedRelayFanoutInfo,
    serving_info: &ManagedRelayServingInfo,
) -> RelayEvidenceStatus {
    let mut counters = RelayEvidenceCounters::default();
    if let Some(local_submission) = maybe_local_submission {
        project_local_submission_counters(&mut counters, local_submission);
    }
    project_fanout_counters(&mut counters, fanout_info);
    project_serving_counters(&mut counters, serving_info);

    let mut status = RelayEvidenceStatus::with_activation_and_counters(
        activation,
        download_eligibility,
        counters,
    );
    status.recovery_counters = recovery;
    if maybe_local_submission.is_some() {
        status.mempool_admission =
            implemented_capability(RelayEvidenceCapability::MempoolAdmission);
        status.local_submission =
            implemented_capability(RelayEvidenceCapability::LocalSubmissionRelay);
    }
    status.fanout = implemented_capability(RelayEvidenceCapability::RelayFanout);
    status.serving = implemented_capability(RelayEvidenceCapability::RelayServing);
    if counters.rebroadcast_deferred_count > 0 {
        status.rebroadcast = implemented_capability(RelayEvidenceCapability::Rebroadcast);
    }
    status
}

fn implemented_capability(
    capability: RelayEvidenceCapability,
) -> RelayEvidenceField<RelayCapabilityEvidence> {
    RelayEvidenceField::implemented(RelayCapabilityEvidence::new(capability))
}

fn project_local_submission_counters(
    counters: &mut RelayEvidenceCounters,
    evidence: &LocalRelaySubmissionEvidence,
) {
    for label in &evidence.labels {
        match label {
            LocalRelaySubmissionLabel::Accepted => counters.accepted_count += 1,
            LocalRelaySubmissionLabel::Rejected => counters.rejected_count += 1,
            LocalRelaySubmissionLabel::Orphaned => counters.orphaned_count += 1,
            LocalRelaySubmissionLabel::Evicted => counters.evicted_count += 1,
            LocalRelaySubmissionLabel::Expired => counters.expired_count += 1,
            LocalRelaySubmissionLabel::Queued
            | LocalRelaySubmissionLabel::Suppressed
            | LocalRelaySubmissionLabel::NotEligible
            | LocalRelaySubmissionLabel::RelayDisabled
            | LocalRelaySubmissionLabel::Duplicate
            | LocalRelaySubmissionLabel::RebroadcastDeferred => {}
        }
    }
}

fn project_fanout_counters(
    counters: &mut RelayEvidenceCounters,
    fanout_info: &ManagedRelayFanoutInfo,
) {
    for action in &fanout_info.latest_actions {
        match action.label {
            "announce" => counters.announced_count += 1,
            "suppress" | "queue_cap" | "rate_limit" => counters.suppressed_count += 1,
            "cleanup" => match action.reason {
                Some("evicted") => counters.evicted_count += 1,
                Some("expired") => counters.expired_count += 1,
                Some(_) | None => {}
            },
            "rebroadcast_deferred" => counters.rebroadcast_deferred_count += 1,
            _ => {}
        }
    }
}

fn project_serving_counters(
    counters: &mut RelayEvidenceCounters,
    serving_info: &ManagedRelayServingInfo,
) {
    for outcome in &serving_info.latest_outcomes {
        counters.requested_count += 1;
        match outcome.label {
            "served" => counters.served_count += 1,
            "rejected" => counters.rejected_count += 1,
            "evicted" => counters.evicted_count += 1,
            "expired" => counters.expired_count += 1,
            _ => {}
        }
    }
}

fn local_submission_evidence(
    outcome: &MempoolOutcome,
    queued_count: usize,
    actions: &[TxFanoutAction],
) -> LocalRelaySubmissionEvidence {
    let mut labels = vec![local_submission_outcome_label(outcome)];
    if queued_count > 0 {
        labels.push(LocalRelaySubmissionLabel::Queued);
    }

    let mut suppressed_count = 0;
    let mut not_eligible_count = 0;
    let mut relay_disabled_count = 0;
    let mut maybe_rebroadcast = None;
    for action in actions {
        match action {
            TxFanoutAction::Suppress { reason, .. } => match reason {
                TxFanoutSuppressionReason::NotRelayEligible => {
                    suppressed_count += 1;
                    not_eligible_count += 1;
                }
                TxFanoutSuppressionReason::RelayDisabled => {
                    suppressed_count += 1;
                    relay_disabled_count += 1;
                }
                TxFanoutSuppressionReason::OriginPeer
                | TxFanoutSuppressionReason::AlreadyHave
                | TxFanoutSuppressionReason::RecentReject
                | TxFanoutSuppressionReason::InFlight
                | TxFanoutSuppressionReason::MempoolKnown
                | TxFanoutSuppressionReason::QueueCapReached
                | TxFanoutSuppressionReason::RateLimited
                | TxFanoutSuppressionReason::IdentityUnavailable => {
                    suppressed_count += 1;
                }
            },
            TxFanoutAction::QueueCap { .. } | TxFanoutAction::RateLimit { .. } => {
                suppressed_count += 1;
            }
            TxFanoutAction::RebroadcastDeferred { .. } => {
                maybe_rebroadcast = Some(RebroadcastEvidenceLabel::Deferred);
            }
            TxFanoutAction::Announce { .. } | TxFanoutAction::Cleanup { .. } => {}
        }
    }

    if suppressed_count > 0 {
        labels.push(LocalRelaySubmissionLabel::Suppressed);
    }
    if not_eligible_count > 0 {
        labels.push(LocalRelaySubmissionLabel::NotEligible);
    }
    if relay_disabled_count > 0 {
        labels.push(LocalRelaySubmissionLabel::RelayDisabled);
    }
    if maybe_rebroadcast.is_some() {
        labels.push(LocalRelaySubmissionLabel::RebroadcastDeferred);
    }

    LocalRelaySubmissionEvidence {
        labels,
        queued_count,
        suppressed_count,
        not_eligible_count,
        relay_disabled_count,
        maybe_rebroadcast,
    }
}

fn local_submission_outcome_label(outcome: &MempoolOutcome) -> LocalRelaySubmissionLabel {
    match outcome {
        MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
            LocalRelaySubmissionLabel::Accepted
        }
        MempoolOutcome::Duplicate { .. } => LocalRelaySubmissionLabel::Duplicate,
        MempoolOutcome::Rejected { .. } => LocalRelaySubmissionLabel::Rejected,
        MempoolOutcome::Orphaned { .. } => LocalRelaySubmissionLabel::Orphaned,
        MempoolOutcome::Evicted { .. } => LocalRelaySubmissionLabel::Evicted,
        MempoolOutcome::Expired { .. } => LocalRelaySubmissionLabel::Expired,
    }
}

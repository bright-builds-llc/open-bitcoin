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

use open_bitcoin_core::primitives::{Txid, Wtxid};
use open_bitcoin_mempool::{
    MempoolOrigin, MempoolOutcome, MempoolRemovalCause, PreparedLifecycleFacts, RelayIntent,
};
use open_bitcoin_network::{
    TxFanoutAction, TxFanoutAdmission, TxFanoutAdmissionOutcome, TxFanoutCleanupReason,
    TxFanoutPeerInput, TxRelayId, TxServingRecordStatus, defer_local_rebroadcast,
};

use super::{ManagedRelayFanoutState, local_submission_evidence};
use crate::network::lifecycle_projection::{AdmissionProjectionSource, PreparedFanoutProjection};
use crate::{ChainstateStore, ManagedPeerNetwork};

impl ManagedRelayFanoutState {
    fn record_prepared_admission(
        &mut self,
        admission: TxFanoutAdmission,
        peers: &[TxFanoutPeerInput],
    ) -> Vec<TxFanoutAction> {
        self.wtxids_by_txid.insert(admission.txid, admission.wtxid);
        let actions = self.queue.enqueue_admission(admission, peers);
        if !actions.is_empty() {
            self.replace_latest_actions(&actions);
        }
        actions
    }

    fn cleanup_prepared_identity(
        &mut self,
        txid: Txid,
        wtxid: Wtxid,
        reason: TxFanoutCleanupReason,
    ) {
        if self.wtxids_by_txid.get(&txid) == Some(&wtxid) {
            self.wtxids_by_txid.remove(&txid);
        }
        let mut actions = self.queue.cleanup_transaction(txid, wtxid, reason);
        if actions.is_empty() {
            actions.push(TxFanoutAction::Cleanup {
                relay_id: TxRelayId::Txid(txid),
                reason,
            });
        }
        self.remove_recent_reject(TxRelayId::Txid(txid));
        self.remove_recent_reject(TxRelayId::Wtxid(wtxid));
        self.replace_latest_actions(&actions);
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(in crate::network) fn prepare_fanout_projection(
        &self,
        facts: &PreparedLifecycleFacts,
        source: &AdmissionProjectionSource,
    ) -> PreparedFanoutProjection {
        let mut replacement = self.relay_fanout.clone();
        for member in facts.teardown_order() {
            let reason = facts
                .removed()
                .iter()
                .find(|removed| removed.removal.member == *member)
                .map(|removed| fanout_reason_for_lifecycle_removal(removed.removal.cause))
                .unwrap_or(TxFanoutCleanupReason::Confirmed);
            replacement.cleanup_prepared_identity(member.txid, member.wtxid, reason);
        }

        let admission_outcome = if facts
            .removed()
            .iter()
            .any(|removed| removed.removal.cause == MempoolRemovalCause::Replacement)
        {
            TxFanoutAdmissionOutcome::Replaced
        } else {
            TxFanoutAdmissionOutcome::Accepted
        };
        let queued_before = replacement.queue.snapshot().queued_count;
        let mut admission_actions = Vec::new();
        for member in facts.final_present() {
            let admission = TxFanoutAdmission {
                txid: member.member.txid,
                wtxid: member.member.wtxid,
                outcome: admission_outcome,
            };
            let origin_peer = source.maybe_origin_peer(member.member);
            let local_not_requested = matches!(source, AdmissionProjectionSource::Local)
                && member.metadata.origin == MempoolOrigin::Local
                && member.metadata.relay_intent == RelayIntent::NotRequested;
            let peer_inputs = if local_not_requested {
                Vec::new()
            } else {
                self.relay_fanout_peer_inputs(origin_peer, Some(admission))
            };
            admission_actions
                .extend(replacement.record_prepared_admission(admission, &peer_inputs));
            if matches!(source, AdmissionProjectionSource::Local)
                && member.metadata.relay_intent == RelayIntent::Requested
                && let Some(action) = defer_local_rebroadcast(admission, true, true)
            {
                admission_actions.push(action);
            }
        }
        if matches!(source, AdmissionProjectionSource::Local)
            && let Some(member) = facts.final_present().first()
        {
            let queued_after = replacement.queue.snapshot().queued_count;
            replacement.latest_local_submission = Some(local_submission_evidence(
                &MempoolOutcome::Accepted {
                    txid: member.member.txid,
                    wtxid: member.member.wtxid,
                    evicted: Vec::new(),
                },
                queued_after.saturating_sub(queued_before),
                &admission_actions,
            ));
            if !admission_actions.is_empty() {
                replacement.replace_latest_actions(&admission_actions);
            }
        }
        PreparedFanoutProjection { replacement }
    }
}

const fn fanout_reason_for_lifecycle_removal(cause: MempoolRemovalCause) -> TxFanoutCleanupReason {
    match cause {
        MempoolRemovalCause::Replacement => TxFanoutCleanupReason::Replaced,
        MempoolRemovalCause::Expiry => TxFanoutCleanupReason::Expired,
        MempoolRemovalCause::Pressure => TxFanoutCleanupReason::Evicted,
        MempoolRemovalCause::BlockConfirmation
        | MempoolRemovalCause::BlockConflict
        | MempoolRemovalCause::Reorg => TxFanoutCleanupReason::Confirmed,
    }
}

pub(in crate::network) const fn cleanup_reason_for_serving_status(
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

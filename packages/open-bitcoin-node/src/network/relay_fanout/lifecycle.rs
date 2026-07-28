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
use open_bitcoin_mempool::{MempoolRemovalCause, PreparedLifecycleFacts};
use open_bitcoin_network::{
    TxFanoutAction, TxFanoutAdmission, TxFanoutAdmissionOutcome, TxFanoutCleanupReason,
    TxFanoutPeerInput, TxRelayId, TxServingRecordStatus,
};

use super::ManagedRelayFanoutState;
use crate::network::lifecycle_projection::PreparedFanoutProjection;
use crate::{ChainstateStore, ManagedPeerNetwork};

impl ManagedRelayFanoutState {
    fn record_prepared_admission(
        &mut self,
        admission: TxFanoutAdmission,
        peers: &[TxFanoutPeerInput],
    ) {
        self.wtxids_by_txid.insert(admission.txid, admission.wtxid);
        let actions = self.queue.enqueue_admission(admission, peers);
        if !actions.is_empty() {
            self.replace_latest_actions(&actions);
        }
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
        for member in facts.final_present() {
            let admission = TxFanoutAdmission {
                txid: member.member.txid,
                wtxid: member.member.wtxid,
                outcome: admission_outcome,
            };
            let peer_inputs = self.relay_fanout_peer_inputs(None, Some(admission));
            replacement.record_prepared_admission(admission, &peer_inputs);
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

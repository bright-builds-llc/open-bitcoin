// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_wtxid},
    primitives::{Hash32, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{AdmissionResult, MempoolOutcome};
use open_bitcoin_network::{
    OrphanAction, OrphanReconsiderationCandidate, OrphanReconsiderationStatus, OrphanStageInput,
    PeerAction, PeerId, TxRelayId, WireNetworkMessage,
};

use super::action_translation::process_transaction_relay_action;
use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

pub(super) struct ManagedAdmissionBridgeResult {
    pub outcome: MempoolOutcome,
    pub targeted_outbound: Vec<(PeerId, WireNetworkMessage)>,
    pub reconsidered: Vec<MempoolOutcome>,
}

impl ManagedAdmissionBridgeResult {
    fn new(outcome: MempoolOutcome) -> Self {
        Self {
            outcome,
            targeted_outbound: Vec::new(),
            reconsidered: Vec::new(),
        }
    }
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn process_peer_transaction_admission(
        &mut self,
        peer_id: PeerId,
        transaction: Transaction,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedAdmissionBridgeResult, ManagedNetworkError> {
        let outcome = self.mempool.submit_transaction_outcome(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        let mut result = ManagedAdmissionBridgeResult::new(outcome.clone());

        match &outcome {
            MempoolOutcome::Accepted { txid, .. } | MempoolOutcome::Replaced { txid, .. } => {
                self.apply_admitted_outcome(&outcome, transaction)?;
                let child_result = self.reconsider_orphans_after_acceptance(
                    *txid,
                    timestamp,
                    verify_flags,
                    consensus_params,
                )?;
                result
                    .targeted_outbound
                    .extend(child_result.targeted_outbound);
                result.reconsidered.extend(child_result.reconsidered);
            }
            MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents,
            } => {
                let actions = self.orphanage.stage_missing_parent(OrphanStageInput {
                    peer_id,
                    transaction,
                    txid: *txid,
                    wtxid: *wtxid,
                    missing_parents: missing_parents.clone(),
                    now_unix_seconds: timestamp,
                });
                self.apply_orphan_actions(actions, timestamp, &mut result)?;
            }
            MempoolOutcome::Rejected { .. }
            | MempoolOutcome::Duplicate { .. }
            | MempoolOutcome::Evicted { .. } => {
                self.remove_evicted_outcome(&outcome)?;
                self.note_recent_reject_for_outcome(&outcome, Some(&transaction))?;
            }
            MempoolOutcome::Expired { .. } => {
                self.remove_evicted_outcome(&outcome)?;
            }
        }

        Ok(result)
    }

    pub(super) fn reconsider_orphans_after_acceptance(
        &mut self,
        accepted_parent: Txid,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedAdmissionBridgeResult, ManagedNetworkError> {
        let mut result = ManagedAdmissionBridgeResult::new(MempoolOutcome::Accepted {
            txid: accepted_parent,
            wtxid: Wtxid::from(Hash32::from(accepted_parent)),
            evicted: Vec::new(),
        });
        let actions = self
            .orphanage
            .reconsider_after_parent(TxRelayId::Txid(accepted_parent), timestamp);

        for action in actions {
            match action {
                OrphanAction::Reconsider { candidate, .. } => {
                    self.reconsider_child(
                        candidate,
                        timestamp,
                        verify_flags,
                        consensus_params,
                        &mut result,
                    )?;
                }
                other => self.apply_orphan_action(other, timestamp, &mut result)?,
            }
        }

        Ok(result)
    }

    pub fn expire_orphan_transactions(&mut self, now_unix_seconds: i64) -> Vec<MempoolOutcome> {
        self.orphanage
            .expire(now_unix_seconds)
            .into_iter()
            .filter_map(orphan_action_outcome)
            .collect()
    }

    pub fn orphan_count(&self) -> usize {
        self.orphanage.len()
    }

    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        let result = self.mempool.submit_transaction(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        self.remove_stored_transactions(&result.replaced)?;
        self.remove_stored_transactions(&result.evicted)?;
        self.store_transaction(transaction)?;
        Ok(result)
    }

    pub fn submit_local_transaction_outcome(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<MempoolOutcome, ManagedNetworkError> {
        let outcome = self.mempool.submit_transaction_outcome(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        match &outcome {
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                self.apply_admitted_outcome(&outcome, transaction)?;
            }
            MempoolOutcome::Evicted { .. } | MempoolOutcome::Expired { .. } => {
                self.remove_evicted_outcome(&outcome)?;
            }
            MempoolOutcome::Rejected { .. }
            | MempoolOutcome::Duplicate { .. }
            | MempoolOutcome::Orphaned { .. } => {}
        }
        Ok(outcome)
    }

    fn reconsider_child(
        &mut self,
        candidate: OrphanReconsiderationCandidate,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        result: &mut ManagedAdmissionBridgeResult,
    ) -> Result<(), ManagedNetworkError> {
        let outcome = self.mempool.submit_transaction_outcome(
            &self.chainstate,
            candidate.transaction.clone(),
            verify_flags,
            consensus_params,
        )?;
        let status = reconsideration_status(&outcome);

        match &outcome {
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                self.apply_admitted_outcome(&outcome, candidate.transaction.clone())?;
            }
            MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents,
            } => {
                self.orphanage
                    .record_reconsideration_outcome(candidate.wtxid, status);
                let actions = self.orphanage.stage_missing_parent(OrphanStageInput {
                    peer_id: candidate.peer_id,
                    transaction: candidate.transaction.clone(),
                    txid: *txid,
                    wtxid: *wtxid,
                    missing_parents: missing_parents.clone(),
                    now_unix_seconds: timestamp,
                });
                self.apply_orphan_actions(actions, timestamp, result)?;
            }
            MempoolOutcome::Rejected { .. }
            | MempoolOutcome::Duplicate { .. }
            | MempoolOutcome::Evicted { .. } => {
                self.remove_evicted_outcome(&outcome)?;
                self.note_recent_reject_for_outcome(&outcome, Some(&candidate.transaction))?;
            }
            MempoolOutcome::Expired { .. } => {
                self.remove_evicted_outcome(&outcome)?;
            }
        }

        if !matches!(outcome, MempoolOutcome::Orphaned { .. }) {
            let actions = self
                .orphanage
                .record_reconsideration_outcome(candidate.wtxid, status);
            self.apply_orphan_actions(actions, timestamp, result)?;
        }
        result.reconsidered.push(outcome);
        Ok(())
    }

    fn apply_admitted_outcome(
        &mut self,
        outcome: &MempoolOutcome,
        transaction: Transaction,
    ) -> Result<(), ManagedNetworkError> {
        let mut removed = outcome.replaced().to_vec();
        removed.extend_from_slice(outcome.evicted());
        self.remove_stored_transactions(&removed)?;
        self.store_transaction(transaction)?;
        Ok(())
    }

    fn remove_evicted_outcome(
        &mut self,
        outcome: &MempoolOutcome,
    ) -> Result<(), ManagedNetworkError> {
        match outcome {
            MempoolOutcome::Evicted { txid, .. } | MempoolOutcome::Expired { txid, .. } => {
                self.remove_stored_transactions(&[*txid])?;
            }
            MempoolOutcome::Accepted { .. }
            | MempoolOutcome::Rejected { .. }
            | MempoolOutcome::Duplicate { .. }
            | MempoolOutcome::Replaced { .. }
            | MempoolOutcome::Orphaned { .. } => {}
        }
        Ok(())
    }

    fn note_recent_reject_for_outcome(
        &mut self,
        outcome: &MempoolOutcome,
        maybe_transaction: Option<&Transaction>,
    ) -> Result<(), ManagedNetworkError> {
        self.peer_manager
            .note_recent_reject(TxRelayId::Txid(outcome.txid()));
        let maybe_wtxid = outcome.maybe_wtxid().or_else(|| {
            maybe_transaction.and_then(|transaction| transaction_wtxid(transaction).ok())
        });
        if let Some(wtxid) = maybe_wtxid {
            self.peer_manager
                .note_recent_reject(TxRelayId::Wtxid(wtxid));
        }
        Ok(())
    }

    fn apply_orphan_actions(
        &mut self,
        actions: Vec<OrphanAction>,
        timestamp: i64,
        result: &mut ManagedAdmissionBridgeResult,
    ) -> Result<(), ManagedNetworkError> {
        for action in actions {
            self.apply_orphan_action(action, timestamp, result)?;
        }
        Ok(())
    }

    fn apply_orphan_action(
        &mut self,
        action: OrphanAction,
        timestamp: i64,
        result: &mut ManagedAdmissionBridgeResult,
    ) -> Result<(), ManagedNetworkError> {
        match action {
            OrphanAction::RequestParent {
                peer_id,
                relay_id: TxRelayId::Txid(parent_txid),
                ..
            } => {
                let actions =
                    self.peer_manager
                        .request_orphan_parent(peer_id, parent_txid, timestamp)?;
                result
                    .targeted_outbound
                    .extend(transaction_relay_messages(actions));
            }
            OrphanAction::RequestParent { .. } => {}
            OrphanAction::Evicted { .. } | OrphanAction::Expired { .. } => {
                if let Some(outcome) = orphan_action_outcome(action) {
                    result.reconsidered.push(outcome);
                }
            }
            OrphanAction::PeerCleanup { .. } | OrphanAction::Reconsidered { .. } => {}
            OrphanAction::Reconsider { .. } => {}
        }
        Ok(())
    }
}

fn transaction_relay_messages(actions: Vec<PeerAction>) -> Vec<(PeerId, WireNetworkMessage)> {
    actions
        .into_iter()
        .filter_map(|action| match action {
            PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
            PeerAction::Send(_)
            | PeerAction::ServeInventory(_)
            | PeerAction::ReceivedTransaction(_)
            | PeerAction::ReceivedBlock(_)
            | PeerAction::Disconnect(_)
            | PeerAction::ResourceGovernanceDisconnect(_) => None,
        })
        .collect()
}

fn orphan_action_outcome(action: OrphanAction) -> Option<MempoolOutcome> {
    match action {
        OrphanAction::Evicted { txid, wtxid, .. } => Some(MempoolOutcome::Evicted { txid, wtxid }),
        OrphanAction::Expired { txid, wtxid, .. } => Some(MempoolOutcome::Expired { txid, wtxid }),
        OrphanAction::RequestParent { .. }
        | OrphanAction::Reconsider { .. }
        | OrphanAction::PeerCleanup { .. }
        | OrphanAction::Reconsidered { .. } => None,
    }
}

fn reconsideration_status(outcome: &MempoolOutcome) -> OrphanReconsiderationStatus {
    match outcome {
        MempoolOutcome::Accepted { .. }
        | MempoolOutcome::Duplicate { .. }
        | MempoolOutcome::Replaced { .. } => OrphanReconsiderationStatus::Accepted,
        MempoolOutcome::Orphaned { .. } => OrphanReconsiderationStatus::StillMissingParent,
        MempoolOutcome::Rejected { .. } => OrphanReconsiderationStatus::Rejected,
        MempoolOutcome::Evicted { .. } => OrphanReconsiderationStatus::Evicted,
        MempoolOutcome::Expired { .. } => OrphanReconsiderationStatus::Expired,
    }
}

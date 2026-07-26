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

use open_bitcoin_core::{
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid, transaction_wtxid},
    primitives::{Hash32, Transaction, Txid, Wtxid},
};
use open_bitcoin_mempool::{
    AdmissionContext, AdmissionResult, FinalMempoolMembership, MempoolError, MempoolLifecycleDelta,
    MempoolOutcome, MempoolRemovalCause, MempoolTransition, PolicyTime, RelayIntent,
};
use open_bitcoin_network::{
    OrphanAction, OrphanReconsiderationCandidate, OrphanReconsiderationStatus, OrphanStageInput,
    PeerAction, PeerId, TxRelayId, TxServingRecordStatus, WireNetworkMessage,
};

use super::action_translation::process_transaction_relay_action;
use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

pub(super) struct ManagedAdmissionBridgeResult {
    pub outcome: MempoolOutcome,
    pub delta: MempoolLifecycleDelta,
    pub targeted_outbound: Vec<(PeerId, WireNetworkMessage)>,
    pub reconsidered: Vec<MempoolOutcome>,
}

impl ManagedAdmissionBridgeResult {
    fn new(transition: MempoolTransition) -> Self {
        Self {
            outcome: transition.outcome,
            delta: transition.delta,
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
        let transition = self.mempool.submit_transaction_transition_with_context(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
            AdmissionContext::peer(PolicyTime::from_unix_seconds(timestamp)),
        )?;
        let mut result = ManagedAdmissionBridgeResult::new(transition.clone());

        match &transition.outcome {
            MempoolOutcome::Accepted { txid, .. } | MempoolOutcome::Replaced { txid, .. } => {
                self.apply_admitted_transition(&transition, transaction)?;
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
                self.compact_extra_txn.push(*wtxid, transaction.clone());
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
            MempoolOutcome::Rejected { wtxid, .. } => {
                let _ = self
                    .compact_extra_txn
                    .push_gated(*wtxid, transaction.clone());
                self.note_recent_reject_for_outcome(&transition.outcome, Some(&transaction))?;
            }
            MempoolOutcome::Duplicate { .. } | MempoolOutcome::Evicted { .. } => {
                self.note_recent_reject_for_outcome(&transition.outcome, Some(&transaction))?;
            }
            MempoolOutcome::Expired { .. } => {}
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
        let mut result = ManagedAdmissionBridgeResult::new(MempoolTransition {
            outcome: MempoolOutcome::Accepted {
                txid: accepted_parent,
                wtxid: Wtxid::from(Hash32::from(accepted_parent)),
                evicted: Vec::new(),
            },
            delta: MempoolLifecycleDelta::empty(),
        });
        let mut actions = self
            .orphanage
            .reconsider_after_parent(TxRelayId::Txid(accepted_parent), timestamp);

        while !actions.is_empty() {
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
            actions = self.orphanage.drain_pending_reconsiderations(timestamp);
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

    #[cfg(test)]
    pub(super) fn with_orphan_policy(&mut self, policy: open_bitcoin_network::OrphanPolicy) {
        self.orphanage = open_bitcoin_network::TxOrphanage::new(policy);
    }

    /// Fail-closed no-time admission retained for wallet and other AdmissionResult callers.
    #[deprecated(
        note = "prefer submit_local_transaction_outcome_at with shell-sampled time and typed relay intent"
    )]
    pub fn submit_local_transaction(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<AdmissionResult, ManagedNetworkError> {
        let transition = self.submit_local_transaction_transition_with_context(
            transaction.clone(),
            verify_flags,
            consensus_params,
            AdmissionContext::legacy_unknown(),
        )?;
        admission_result_from_transition(&transaction, transition)
    }

    pub fn submit_local_transaction_outcome_at(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        now_unix_seconds: i64,
        relay_intent: RelayIntent,
    ) -> Result<MempoolOutcome, ManagedNetworkError> {
        let transition = self.submit_local_transaction_transition_with_context(
            transaction,
            verify_flags,
            consensus_params,
            AdmissionContext::local(
                PolicyTime::from_unix_seconds(now_unix_seconds),
                relay_intent,
            ),
        )?;
        self.record_local_submission_outcome(&transition.outcome, relay_intent);
        Ok(transition.outcome)
    }

    fn submit_local_transaction_transition_with_context(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: AdmissionContext,
    ) -> Result<MempoolTransition, ManagedNetworkError> {
        let transition = self.mempool.submit_transaction_transition_with_context(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
            context,
        )?;
        self.apply_admitted_transition(&transition, transaction)?;
        Ok(transition)
    }

    fn reconsider_child(
        &mut self,
        candidate: OrphanReconsiderationCandidate,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        result: &mut ManagedAdmissionBridgeResult,
    ) -> Result<(), ManagedNetworkError> {
        let transition = self.mempool.submit_transaction_transition_with_context(
            &self.chainstate,
            candidate.transaction.clone(),
            verify_flags,
            consensus_params,
            AdmissionContext::peer(PolicyTime::from_unix_seconds(timestamp)),
        )?;
        let status = reconsideration_status(&transition.outcome);

        match &transition.outcome {
            MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                self.apply_admitted_transition(&transition, candidate.transaction.clone())?;
            }
            MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents,
            } => {
                self.compact_extra_txn
                    .push(*wtxid, candidate.transaction.clone());
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
            MempoolOutcome::Rejected { wtxid, .. } => {
                let _ = self
                    .compact_extra_txn
                    .push_gated(*wtxid, candidate.transaction.clone());
                self.note_recent_reject_for_outcome(
                    &transition.outcome,
                    Some(&candidate.transaction),
                )?;
            }
            MempoolOutcome::Duplicate { .. } | MempoolOutcome::Evicted { .. } => {
                self.note_recent_reject_for_outcome(
                    &transition.outcome,
                    Some(&candidate.transaction),
                )?;
            }
            MempoolOutcome::Expired { .. } => {}
        }

        if !matches!(transition.outcome, MempoolOutcome::Orphaned { .. }) {
            let actions = self
                .orphanage
                .record_reconsideration_outcome(candidate.wtxid, status);
            self.apply_orphan_actions(actions, timestamp, result)?;
        }
        result.reconsidered.push(transition.outcome);
        Ok(())
    }

    pub(super) fn apply_admitted_transition(
        &mut self,
        transition: &MempoolTransition,
        transaction: Transaction,
    ) -> Result<(), ManagedNetworkError> {
        let replaced_txids = transition
            .delta
            .removed
            .iter()
            .filter(|removal| removal.cause == MempoolRemovalCause::Replacement)
            .map(|removal| removal.member.txid)
            .collect::<Vec<_>>();
        self.feed_replaced_victims_to_compact_extra(&replaced_txids);

        for removal in &transition.delta.removed {
            self.peer_manager
                .on_mempool_transaction_removed(&removal.member.wtxid);
            self.remove_stored_transactions_with_status(
                &[removal.member.txid],
                serving_status_for_removal(removal.cause),
            )?;
        }

        if !replaced_txids.is_empty() {
            self.relay_serving
                .record_replaced(transaction.clone(), &replaced_txids)?;
        }

        let admitted_txid = transaction_txid(&transaction)?;
        let maybe_admitted = transition
            .delta
            .admitted
            .iter()
            .find(|member| member.txid == admitted_txid);
        let should_store = maybe_admitted.is_some_and(|admitted| {
            transition.delta.final_membership.iter().any(|state| {
                state.member == *admitted && state.membership == FinalMempoolMembership::Present
            })
        });
        if should_store {
            self.store_transaction(transaction)?;
        }

        Ok(())
    }

    /// Push replaced-victim bodies into the compact extra ring before demotion (D-05).
    ///
    /// Does not push the admitted Replaced wtxid — only prior victim bodies.
    fn feed_replaced_victims_to_compact_extra(&mut self, victim_txids: &[Txid]) {
        for victim_txid in victim_txids {
            let maybe_from_relay = self
                .relay_serving
                .maybe_accepted_wtxid_and_transaction(*victim_txid);
            let maybe_pair = maybe_from_relay.or_else(|| {
                let transaction = self.transactions_by_txid.get(victim_txid)?.clone();
                let wtxid = transaction_wtxid(&transaction).ok()?;
                Some((wtxid, transaction))
            });
            let Some((wtxid, transaction)) = maybe_pair else {
                continue;
            };
            self.compact_extra_txn.push(wtxid, transaction);
        }
    }

    fn note_recent_reject_for_outcome(
        &mut self,
        outcome: &MempoolOutcome,
        maybe_transaction: Option<&Transaction>,
    ) -> Result<(), ManagedNetworkError> {
        let maybe_wtxid = outcome.maybe_wtxid().or_else(|| {
            maybe_transaction.and_then(|transaction| transaction_wtxid(transaction).ok())
        });
        if let Some(wtxid) = maybe_wtxid {
            self.peer_manager.record_hard_reject(wtxid);
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

fn serving_status_for_removal(cause: MempoolRemovalCause) -> TxServingRecordStatus {
    match cause {
        MempoolRemovalCause::Replacement => TxServingRecordStatus::Replaced,
        MempoolRemovalCause::Expiry => TxServingRecordStatus::Expired,
        MempoolRemovalCause::Pressure => TxServingRecordStatus::Evicted,
        MempoolRemovalCause::BlockConfirmation | MempoolRemovalCause::BlockConflict => {
            TxServingRecordStatus::Confirmed
        }
        MempoolRemovalCause::Reorg => TxServingRecordStatus::Stale,
    }
}

fn admission_result_from_transition(
    transaction: &Transaction,
    transition: MempoolTransition,
) -> Result<AdmissionResult, ManagedNetworkError> {
    match transition.outcome {
        MempoolOutcome::Accepted { txid, evicted, .. } => Ok(AdmissionResult {
            accepted: txid,
            replaced: Vec::new(),
            evicted,
        }),
        MempoolOutcome::Replaced {
            txid,
            replaced,
            evicted,
            ..
        } => Ok(AdmissionResult {
            accepted: txid,
            replaced,
            evicted,
        }),
        MempoolOutcome::Duplicate { txid } => {
            Err(MempoolError::DuplicateTransaction { txid }.into())
        }
        MempoolOutcome::Orphaned {
            txid,
            missing_parents,
            ..
        } => {
            let maybe_missing_input = transaction
                .inputs
                .iter()
                .find(|input| missing_parents.contains(&input.previous_output.txid));
            let Some(missing_input) = maybe_missing_input else {
                return Err(MempoolError::InternalInvariant {
                    reason: format!("orphaned transaction {txid:?} has no missing input"),
                }
                .into());
            };
            Err(MempoolError::MissingInput {
                outpoint: missing_input.previous_output.clone(),
            }
            .into())
        }
        MempoolOutcome::Evicted { txid, .. } => Err(MempoolError::CandidateEvicted { txid }.into()),
        MempoolOutcome::Rejected { category, .. } => Err(MempoolError::Validation {
            reason: format!("transaction rejected during compatibility admission: {category:?}"),
        }
        .into()),
        MempoolOutcome::Expired { txid, .. } => Err(MempoolError::InternalInvariant {
            reason: format!("new local transaction {txid:?} returned an expired outcome"),
        }
        .into()),
    }
}

fn transaction_relay_messages(actions: Vec<PeerAction>) -> Vec<(PeerId, WireNetworkMessage)> {
    actions
        .into_iter()
        .filter_map(|action| match action {
            PeerAction::TransactionRelay(action) => process_transaction_relay_action(action),
            PeerAction::Send(_)
            | PeerAction::ServeInventory(_)
            | PeerAction::ServeCompactBlockTransactions(_)
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

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/validation.cpp

//! In-memory authority guards and consuming aggregate application.

use std::collections::BTreeSet;

use open_bitcoin_core::{chainstate::ChainPosition, primitives::Block};
use open_bitcoin_mempool::{
    MempoolLifecycleDelta, MempoolMemberIdentity, MempoolRemovalCause, PreparedLifecycleFacts,
    PreparedMempoolTransition,
};

use super::{
    AuthorityEpoch, LifecycleEvidenceSnapshot, LifecycleGeneration, LifecyclePreparationError,
    LifecycleProjectionError, LifecycleProjectionPlan, MAX_UNBROADCAST_MEMBERS,
    PreparedCompactProjection, PreparedFanoutProjection, PreparedLifecycleEvidence,
    PreparedPeerLifecycleProjection, PreparedPersistenceProjection, PreparedServingProjection,
    PreparedUnbroadcastProjection,
};
use crate::chainstate::PreparedChainstateConnect;
use crate::{ChainstateStore, ManagedPeerNetwork};

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(in crate::network) fn apply_prepared_peer_lifecycle(
        &mut self,
        prepared: PreparedPeerLifecycleProjection,
    ) {
        self.peer_manager
            .apply_prepared_transaction_lifecycle(prepared.prepared);
    }

    pub(in crate::network) const fn authority_epoch(&self) -> AuthorityEpoch {
        self.authority_epoch
    }

    pub(in crate::network) const fn lifecycle_generation(&self) -> LifecycleGeneration {
        self.lifecycle_generation
    }

    pub(in crate::network) const fn dirty_generation(&self) -> Option<LifecycleGeneration> {
        self.dirty_generation
    }

    pub(in crate::network) fn unbroadcast_members(&self) -> &BTreeSet<MempoolMemberIdentity> {
        &self.unbroadcast_members
    }

    pub(in crate::network) const fn lifecycle_evidence_snapshot(
        &self,
    ) -> LifecycleEvidenceSnapshot {
        self.lifecycle_evidence
    }

    pub(super) fn prepare_unbroadcast_projection(
        &self,
        facts: &PreparedLifecycleFacts,
    ) -> Result<PreparedUnbroadcastProjection, LifecyclePreparationError> {
        let mut replacement = self.unbroadcast_members.clone();
        for member in facts.final_present() {
            if member.metadata.is_retry_eligible(true) {
                replacement.insert(member.member);
            }
        }
        for member in facts.teardown_order() {
            replacement.remove(member);
        }
        for clear in &facts.delta().retry_clears {
            replacement.remove(&clear.member);
        }
        if replacement.len() > MAX_UNBROADCAST_MEMBERS {
            return Err(LifecyclePreparationError::UnbroadcastCapacity {
                attempted: replacement.len(),
                capacity: MAX_UNBROADCAST_MEMBERS,
            });
        }
        Ok(PreparedUnbroadcastProjection { replacement })
    }

    pub(super) fn prepare_persistence_projection(
        &self,
        facts: &PreparedLifecycleFacts,
    ) -> Result<PreparedPersistenceProjection, LifecyclePreparationError> {
        if facts.delta().is_empty() {
            return Ok(PreparedPersistenceProjection {
                lifecycle_generation: self.lifecycle_generation,
                dirty_generation: self.dirty_generation,
            });
        }
        let next = self.lifecycle_generation.checked_next()?;
        Ok(PreparedPersistenceProjection {
            lifecycle_generation: next,
            dirty_generation: Some(next),
        })
    }

    pub(super) fn prepare_lifecycle_evidence(
        &self,
        facts: &PreparedLifecycleFacts,
    ) -> PreparedLifecycleEvidence {
        let mut replacement = self.lifecycle_evidence;
        if facts.delta().is_empty() {
            return PreparedLifecycleEvidence { replacement };
        }
        replacement.committed_transitions = replacement.committed_transitions.saturating_add(1);
        replacement.admitted_members = replacement
            .admitted_members
            .saturating_add(facts.final_present().len() as u64);
        replacement.removed_members = replacement
            .removed_members
            .saturating_add(facts.removed().len() as u64);
        replacement.retry_clears = replacement
            .retry_clears
            .saturating_add(facts.delta().retry_clears.len() as u64);
        for removed in facts.removed() {
            let counter = match removed.removal.cause {
                MempoolRemovalCause::Replacement => &mut replacement.replacement_removals,
                MempoolRemovalCause::Expiry => &mut replacement.expiry_removals,
                MempoolRemovalCause::Pressure => &mut replacement.pressure_removals,
                MempoolRemovalCause::BlockConfirmation => {
                    &mut replacement.block_confirmation_removals
                }
                MempoolRemovalCause::BlockConflict => &mut replacement.block_conflict_removals,
                MempoolRemovalCause::Reorg => &mut replacement.reorg_removals,
            };
            *counter = counter.saturating_add(1);
        }
        PreparedLifecycleEvidence { replacement }
    }

    pub(super) fn apply_prepared_unbroadcast(&mut self, prepared: PreparedUnbroadcastProjection) {
        self.unbroadcast_members = prepared.replacement;
    }

    pub(super) fn apply_prepared_persistence(&mut self, prepared: PreparedPersistenceProjection) {
        self.lifecycle_generation = prepared.lifecycle_generation;
        self.dirty_generation = prepared.dirty_generation;
    }

    pub(super) fn apply_prepared_evidence(&mut self, prepared: PreparedLifecycleEvidence) {
        self.lifecycle_evidence = prepared.replacement;
    }
}

/// Non-forgeable proof that the authority guard passed for a complete projection.
pub(in crate::network) struct SealedLifecycleProjection {
    core: PreparedMempoolTransition,
    compact: PreparedCompactProjection,
    serving: PreparedServingProjection,
    fanout: PreparedFanoutProjection,
    peers: PreparedPeerLifecycleProjection,
    unbroadcast: PreparedUnbroadcastProjection,
    persistence: PreparedPersistenceProjection,
    evidence: PreparedLifecycleEvidence,
}

pub(in crate::network) struct PreparedDependentLifecycleProjection {
    compact: PreparedCompactProjection,
    serving: PreparedServingProjection,
    fanout: PreparedFanoutProjection,
    peers: PreparedPeerLifecycleProjection,
    unbroadcast: PreparedUnbroadcastProjection,
    persistence: PreparedPersistenceProjection,
    evidence: PreparedLifecycleEvidence,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(in crate::network) fn validate_prepared_lifecycle(
        &self,
        plan: LifecycleProjectionPlan,
    ) -> Result<SealedLifecycleProjection, LifecycleProjectionError> {
        if self.authority_epoch != plan.authority_epoch {
            return Err(LifecycleProjectionError::StaleAuthorityEpoch {
                expected: plan.authority_epoch,
                actual: self.authority_epoch,
            });
        }
        let LifecycleProjectionPlan {
            authority_epoch: _,
            core,
            compact,
            serving,
            fanout,
            peers,
            unbroadcast,
            persistence,
            evidence,
        } = plan;
        Ok(SealedLifecycleProjection {
            core,
            compact,
            serving,
            fanout,
            peers,
            unbroadcast,
            persistence,
            evidence,
        })
    }

    pub(in crate::network) fn commit_sealed_lifecycle(
        &mut self,
        sealed: SealedLifecycleProjection,
    ) -> Result<MempoolLifecycleDelta, LifecycleProjectionError> {
        let (core, prepared) = sealed.into_parts();
        let ((), committed_delta) = self
            .mempool
            .mempool_mut()
            .commit_prepared_mempool_transition_with(core, || ())
            .map_err(LifecycleProjectionError::Mempool)?;
        self.apply_prepared_lifecycle(prepared);
        Ok(committed_delta)
    }

    pub(in crate::network) fn commit_connected_block_lifecycle_transaction(
        &mut self,
        block: &Block,
        position: &ChainPosition,
        prepared_chainstate: PreparedChainstateConnect,
        sealed: SealedLifecycleProjection,
    ) -> Result<MempoolLifecycleDelta, LifecycleProjectionError> {
        let (core, dependent) = sealed.into_parts();
        let chainstate = &mut self.chainstate;
        let peer_manager = &mut self.peer_manager;
        let blocks_by_hash = &mut self.blocks_by_hash;
        let ((), delta) = self
            .mempool
            .mempool_mut()
            .commit_prepared_mempool_transition_with(core, || {
                chainstate.commit_prepared_connect(prepared_chainstate);
                peer_manager.on_active_tip_changed(
                    super::super::relay_serving::fresh_reject_evidence_tweak(),
                );
                blocks_by_hash.insert(position.block_hash, block.clone());
                peer_manager.note_local_position(position);
            })
            .map_err(LifecycleProjectionError::Mempool)?;
        self.apply_prepared_lifecycle(dependent);
        Ok(delta)
    }

    pub(in crate::network) fn apply_prepared_lifecycle(
        &mut self,
        prepared: PreparedDependentLifecycleProjection,
    ) {
        let PreparedDependentLifecycleProjection {
            compact,
            serving,
            fanout,
            peers,
            unbroadcast,
            persistence,
            evidence,
        } = prepared;
        self.apply_prepared_compact(compact);
        self.apply_prepared_serving(serving);
        self.apply_prepared_fanout(fanout);
        self.apply_prepared_peer_lifecycle(peers);
        self.apply_prepared_unbroadcast(unbroadcast);
        self.apply_prepared_persistence(persistence);
        self.apply_prepared_evidence(evidence);
    }
}

impl SealedLifecycleProjection {
    pub(in crate::network) fn into_parts(
        self,
    ) -> (
        PreparedMempoolTransition,
        PreparedDependentLifecycleProjection,
    ) {
        let Self {
            core,
            compact,
            serving,
            fanout,
            peers,
            unbroadcast,
            persistence,
            evidence,
        } = self;
        (
            core,
            PreparedDependentLifecycleProjection {
                compact,
                serving,
                fanout,
                peers,
                unbroadcast,
                persistence,
                evidence,
            },
        )
    }
}

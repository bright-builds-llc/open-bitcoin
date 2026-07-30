// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, transaction_txid, transaction_wtxid,
    },
    primitives::{Block, Transaction},
};
use open_bitcoin_mempool::{
    AdmissionContext, BlockLifecycleContext, MempoolError, MempoolLifecycleDelta, MempoolOutcome,
    MempoolRejectionCategory, MempoolTransition, PolicyTime, PreparedMempoolTransition,
    ReorgLifecycleContext,
};

use super::{BlockConnectDisposition, ManagedNetworkError, ManagedPeerNetwork, ManagedResult};
use crate::ChainstateStore;
use crate::network::lifecycle_projection::{
    LifecycleCommand, LifecycleProjectionPlan, SealedLifecycleProjection,
};
use crate::network::runtime_authority::{LifecycleCommandResult, apply_lifecycle_command};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedMempoolBlockLifecycle {
    pub context: BlockLifecycleContext,
    pub delta: MempoolLifecycleDelta,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedMempoolReorgLifecycle {
    pub connected: Vec<ManagedMempoolBlockLifecycle>,
    pub reconsidered: Vec<MempoolOutcome>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub fn connect_local_block(
        &mut self,
        block: &Block,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ChainPosition, ManagedNetworkError> {
        let prepared_chainstate = self.chainstate.prepare_connect_block(
            block,
            self.next_chain_work(),
            verify_flags,
            consensus_params,
        )?;
        let position = prepared_chainstate.position().clone();
        let context = block_lifecycle_context(i64::from(block.header.time), position.height);
        let prepared_lifecycle = self
            .mempool
            .mempool()
            .prepare_connected_block_transition(block, context)?;
        let sealed_lifecycle = self.prepare_maintenance_step(prepared_lifecycle)?;

        self.chainstate.commit_prepared_connect(prepared_chainstate);
        self.peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        self.blocks_by_hash
            .insert(position.block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        self.commit_maintenance_step(sealed_lifecycle);
        Ok(position)
    }

    pub fn connect_stored_block(
        &mut self,
        block: &Block,
        chain_work: u128,
        timestamp: i64,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<BlockConnectDisposition> {
        let block_hash = block_hash(&block.header);
        if self
            .chainstate
            .chainstate()
            .snapshot()
            .active_chain
            .iter()
            .any(|position| position.block_hash == block_hash)
        {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Duplicate(block_hash));
        }

        let maybe_tip = self.chainstate.chainstate().tip().cloned();
        let extends_tip = maybe_tip
            .as_ref()
            .is_none_or(|tip| tip.block_hash == block.header.previous_block_hash);
        let is_genesis = block.header.previous_block_hash.to_byte_array() == [0_u8; 32];
        if maybe_tip.is_some() && !extends_tip {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::NonExtending {
                block_hash,
                previous_block_hash: block.header.previous_block_hash,
            });
        }
        if maybe_tip.is_none() && !is_genesis {
            self.blocks_by_hash.insert(block_hash, block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
            return Ok(BlockConnectDisposition::Disconnected { block_hash });
        }

        let prepared_chainstate = self.chainstate.prepare_connect_block_with_current_time(
            block,
            chain_work,
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        let position = prepared_chainstate.position().clone();
        let context = block_lifecycle_context(timestamp, position.height);
        let prepared_lifecycle = self
            .mempool
            .mempool()
            .prepare_connected_block_transition(block, context)?;
        let sealed_lifecycle = self.prepare_maintenance_step(prepared_lifecycle)?;

        self.chainstate.commit_prepared_connect(prepared_chainstate);
        self.peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        self.blocks_by_hash.insert(block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        self.commit_maintenance_step(sealed_lifecycle);
        Ok(BlockConnectDisposition::Connected(position))
    }

    pub fn reorg_to_branch(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        context: ReorgLifecycleContext,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<ChainTransition>
    where
        S: Clone,
    {
        let prepared_chainstate = self.chainstate.prepare_reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        let transition = prepared_chainstate.transition().clone();
        let mut staged = self.clone();
        staged
            .chainstate
            .install_prepared_reorg_preview(&prepared_chainstate);
        staged
            .peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        for anchored_block in replacement_branch {
            let block_hash = block_hash(&anchored_block.block.header);
            staged
                .blocks_by_hash
                .insert(block_hash, anchored_block.block.clone());
            staged.peer_manager.note_local_block_hash(block_hash);
        }
        for position in &transition.connected {
            staged.peer_manager.note_local_position(position);
        }
        staged.apply_reorg_mempool_lifecycle(
            disconnect_blocks,
            replacement_branch,
            &transition.connected,
            context,
            verify_flags,
            consensus_params,
        )?;

        self.chainstate.commit_prepared_reorg(prepared_chainstate);
        std::mem::swap(&mut self.chainstate, &mut staged.chainstate);
        *self = staged;
        Ok(transition)
    }

    #[cfg(test)]
    pub(super) fn apply_connected_block_mempool_lifecycle(
        &mut self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<ManagedMempoolBlockLifecycle, ManagedNetworkError> {
        let prepared = self
            .mempool
            .mempool()
            .prepare_connected_block_transition(block, context)?;
        let delta =
            self.apply_prepared_maintenance_step(prepared, LifecycleCommand::ConnectedBlock)?;

        Ok(ManagedMempoolBlockLifecycle { context, delta })
    }

    pub(super) fn apply_reorg_connected_block_mempool_lifecycle(
        &mut self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<ManagedMempoolBlockLifecycle, ManagedNetworkError> {
        let prepared = self
            .mempool
            .mempool()
            .prepare_connected_block_transition(block, context)?;
        let delta = self.apply_prepared_maintenance_step(prepared, LifecycleCommand::ReorgStep)?;

        Ok(ManagedMempoolBlockLifecycle { context, delta })
    }

    /// Expires aged mempool entries using shell-injected `PolicyTime` (PRESS-04 / D-12).
    pub fn expire_mempool(
        &mut self,
        now: PolicyTime,
    ) -> Result<MempoolLifecycleDelta, ManagedNetworkError> {
        let prepared = self.mempool.mempool().prepare_expiry(now)?;
        self.apply_prepared_maintenance_step(prepared, LifecycleCommand::Expiry)
    }

    fn apply_prepared_maintenance_step(
        &mut self,
        prepared: PreparedMempoolTransition,
        command: fn(LifecycleProjectionPlan) -> LifecycleCommand,
    ) -> Result<MempoolLifecycleDelta, ManagedNetworkError> {
        let plan = LifecycleProjectionPlan::prepare(self, self.authority_epoch(), prepared)
            .map_err(maintenance_lifecycle_error)?;
        let LifecycleCommandResult::Lifecycle(delta) =
            apply_lifecycle_command(self, command(plan)).map_err(maintenance_lifecycle_error)?
        else {
            return Err(MempoolError::InternalInvariant {
                reason: "maintenance dispatcher returned a non-lifecycle result".to_string(),
            }
            .into());
        };
        Ok(delta)
    }

    fn prepare_maintenance_step(
        &self,
        prepared: PreparedMempoolTransition,
    ) -> Result<SealedLifecycleProjection, ManagedNetworkError> {
        let plan = LifecycleProjectionPlan::prepare(self, self.authority_epoch(), prepared)
            .map_err(maintenance_lifecycle_error)?;
        self.validate_prepared_lifecycle(plan)
            .map_err(maintenance_lifecycle_error)
    }

    fn commit_maintenance_step(
        &mut self,
        sealed: SealedLifecycleProjection,
    ) -> MempoolLifecycleDelta {
        self.commit_sealed_lifecycle(sealed)
    }

    pub(super) fn apply_reorg_mempool_lifecycle(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        connected_positions: &[ChainPosition],
        context: ReorgLifecycleContext,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedMempoolReorgLifecycle, ManagedNetworkError> {
        if replacement_branch.len() != connected_positions.len() {
            return Err(open_bitcoin_mempool::MempoolError::InternalInvariant {
                reason: "replacement branch and connected positions must have equal length"
                    .to_string(),
            }
            .into());
        }
        let mut connected = Vec::with_capacity(replacement_branch.len());
        for (anchored_block, position) in replacement_branch.iter().zip(connected_positions) {
            let block_context = block_lifecycle_context_from_reorg(context, position.height);
            connected.push(self.apply_reorg_connected_block_mempool_lifecycle(
                &anchored_block.block,
                block_context,
            )?);
        }

        let mut reconsidered = Vec::new();
        for block in disconnect_blocks.iter().rev() {
            for transaction in block.transactions.iter().skip(1) {
                let transition = self.apply_reorg_transaction_mempool_lifecycle(
                    transaction.clone(),
                    verify_flags,
                    consensus_params,
                    context,
                )?;
                reconsidered.push(transition.outcome);
            }
        }

        Ok(ManagedMempoolReorgLifecycle {
            connected,
            reconsidered,
        })
    }

    pub(super) fn apply_reorg_transaction_mempool_lifecycle(
        &mut self,
        transaction: Transaction,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
        context: ReorgLifecycleContext,
    ) -> Result<MempoolTransition, ManagedNetworkError> {
        let prepared = match self.mempool.prepare_transaction_with_context(
            &self.chainstate,
            transaction.clone(),
            verify_flags,
            consensus_params,
            AdmissionContext::reorg(context.occurred_at),
        ) {
            Ok(prepared) => prepared,
            Err(error) => return self.reorg_noop_transition(&transaction, &error),
        };
        let outcome = prepared_reorg_outcome(&prepared, &transaction)?;
        let delta = self.apply_prepared_maintenance_step(prepared, LifecycleCommand::ReorgStep)?;
        Ok(MempoolTransition { outcome, delta })
    }

    fn reorg_noop_transition(
        &self,
        transaction: &Transaction,
        error: &MempoolError,
    ) -> Result<MempoolTransition, ManagedNetworkError> {
        let txid = transaction_txid(transaction)?;
        let wtxid = transaction_wtxid(transaction)?;
        let outcome = match error {
            MempoolError::DuplicateTransaction { txid } => {
                MempoolOutcome::Duplicate { txid: *txid }
            }
            MempoolError::MissingInput { .. } => MempoolOutcome::Orphaned {
                txid,
                wtxid,
                missing_parents: transaction
                    .inputs
                    .iter()
                    .filter_map(|input| {
                        let parent_txid = input.previous_output.txid;
                        let parent_in_mempool = self
                            .mempool
                            .mempool()
                            .entry(&parent_txid)
                            .is_some_and(|entry| {
                                (input.previous_output.vout as usize)
                                    < entry.transaction.outputs.len()
                            });
                        let parent_in_chainstate = self
                            .chainstate
                            .chainstate()
                            .utxos()
                            .contains_key(&input.previous_output);
                        (!parent_in_mempool && !parent_in_chainstate).then_some(parent_txid)
                    })
                    .fold(Vec::new(), |mut parents, parent| {
                        if !parents.contains(&parent) {
                            parents.push(parent);
                        }
                        parents
                    }),
            },
            MempoolError::CandidateEvicted { txid } => {
                MempoolOutcome::Evicted { txid: *txid, wtxid }
            }
            _ => MempoolOutcome::Rejected {
                txid,
                wtxid,
                category: MempoolRejectionCategory::from_error(error)
                    .unwrap_or(MempoolRejectionCategory::InternalInvariant),
            },
        };
        Ok(MempoolTransition {
            outcome,
            delta: MempoolLifecycleDelta::empty(),
        })
    }
}

pub(super) fn block_lifecycle_context(
    connected_at_unix_seconds: i64,
    height: u32,
) -> BlockLifecycleContext {
    BlockLifecycleContext::new(
        PolicyTime::from_unix_seconds(connected_at_unix_seconds),
        height,
    )
}

pub(super) const fn block_lifecycle_context_from_reorg(
    context: ReorgLifecycleContext,
    height: u32,
) -> BlockLifecycleContext {
    BlockLifecycleContext::new(context.occurred_at, height)
}

fn maintenance_lifecycle_error(error: impl core::fmt::Display) -> ManagedNetworkError {
    MempoolError::InternalInvariant {
        reason: format!("maintenance lifecycle projection failed: {error}"),
    }
    .into()
}

fn prepared_reorg_outcome(
    prepared: &PreparedMempoolTransition,
    transaction: &Transaction,
) -> Result<MempoolOutcome, ManagedNetworkError> {
    let admission = prepared
        .facts()
        .maybe_admission_result()
        .cloned()
        .ok_or_else(|| MempoolError::InternalInvariant {
            reason: "reorg preparation omitted its admission result".to_string(),
        })?;
    let wtxid = transaction_wtxid(transaction)?;
    if admission.replaced.is_empty() {
        return Ok(MempoolOutcome::Accepted {
            txid: admission.accepted,
            wtxid,
            evicted: admission.evicted,
        });
    }
    Ok(MempoolOutcome::Replaced {
        txid: admission.accepted,
        wtxid,
        replaced: admission.replaced,
        evicted: admission.evicted,
    })
}

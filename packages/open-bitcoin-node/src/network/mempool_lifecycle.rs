// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp
// - packages/bitcoin-knots/src/blockencodings.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_core::{
    chainstate::{AnchoredBlock, ChainPosition, ChainTransition},
    consensus::{ConsensusParams, ScriptVerifyFlags, block_hash},
    primitives::Block,
};
use open_bitcoin_mempool::{
    AdmissionContext, BlockLifecycleContext, FinalMempoolMembership, MempoolLifecycleDelta,
    MempoolOutcome, MempoolRemovalCause, PolicyTime, ReorgLifecycleContext,
};
use open_bitcoin_network::TxServingRecordStatus;

use super::{BlockConnectDisposition, ManagedNetworkError, ManagedPeerNetwork, ManagedResult};
use crate::ChainstateStore;

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
        let position = self.chainstate.connect_block(
            block,
            self.next_chain_work(),
            verify_flags,
            consensus_params,
        )?;
        self.peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        self.blocks_by_hash
            .insert(position.block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        let context = block_lifecycle_context(i64::from(block.header.time), position.height);
        self.apply_connected_block_mempool_lifecycle(block, context)?;
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

        let position = self.chainstate.connect_block_with_current_time(
            block,
            chain_work,
            timestamp,
            verify_flags,
            consensus_params,
        )?;
        self.peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        self.blocks_by_hash.insert(block_hash, block.clone());
        self.peer_manager.note_local_position(&position);
        let context = block_lifecycle_context(timestamp, position.height);
        self.apply_connected_block_mempool_lifecycle(block, context)?;
        Ok(BlockConnectDisposition::Connected(position))
    }

    pub fn reorg_to_branch(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        context: ReorgLifecycleContext,
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> ManagedResult<ChainTransition> {
        let transition = self.chainstate.reorg(
            disconnect_blocks,
            replacement_branch,
            verify_flags,
            consensus_params,
        )?;
        self.peer_manager
            .on_active_tip_changed(super::relay_serving::fresh_reject_evidence_tweak());
        for anchored_block in replacement_branch {
            let block_hash = block_hash(&anchored_block.block.header);
            self.blocks_by_hash
                .insert(block_hash, anchored_block.block.clone());
            self.peer_manager.note_local_block_hash(block_hash);
        }
        for position in &transition.connected {
            self.peer_manager.note_local_position(position);
        }
        self.apply_reorg_mempool_lifecycle(
            disconnect_blocks,
            replacement_branch,
            &transition.connected,
            context,
            verify_flags,
            consensus_params,
        )?;
        Ok(transition)
    }

    pub(super) fn apply_connected_block_mempool_lifecycle(
        &mut self,
        block: &Block,
        context: BlockLifecycleContext,
    ) -> Result<ManagedMempoolBlockLifecycle, ManagedNetworkError> {
        let delta = self
            .mempool
            .mempool_mut()
            .remove_for_connected_block_transition(block, context)?;
        for removal in &delta.removed {
            let is_absent = delta.final_membership.iter().any(|state| {
                state.member == removal.member && state.membership == FinalMempoolMembership::Absent
            });
            if !is_absent {
                continue;
            }
            self.peer_manager
                .on_mempool_transaction_removed(&removal.member.wtxid);
            self.remove_stored_transactions_with_status(
                &[removal.member.txid],
                serving_status_for_removal(removal.cause),
            )?;
        }

        Ok(ManagedMempoolBlockLifecycle { context, delta })
    }

    /// Expires aged mempool entries using shell-injected `PolicyTime` (PRESS-04 / D-12).
    pub fn expire_mempool(
        &mut self,
        now: PolicyTime,
    ) -> Result<MempoolLifecycleDelta, ManagedNetworkError> {
        let delta = self.mempool.mempool_mut().expire(now)?;
        for removal in &delta.removed {
            let is_absent = delta.final_membership.iter().any(|state| {
                state.member == removal.member && state.membership == FinalMempoolMembership::Absent
            });
            if !is_absent {
                continue;
            }
            self.peer_manager
                .on_mempool_transaction_removed(&removal.member.wtxid);
            self.remove_stored_transactions_with_status(
                &[removal.member.txid],
                serving_status_for_removal(removal.cause),
            )?;
        }
        Ok(delta)
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
            connected.push(
                self.apply_connected_block_mempool_lifecycle(&anchored_block.block, block_context)?,
            );
        }

        let mut reconsidered = Vec::new();
        for block in disconnect_blocks.iter().rev() {
            for transaction in block.transactions.iter().skip(1) {
                let transition = self.mempool.submit_transaction_transition_with_context(
                    &self.chainstate,
                    transaction.clone(),
                    verify_flags,
                    consensus_params,
                    AdmissionContext::reorg(context.occurred_at),
                )?;
                self.apply_admitted_transition(&transition, transaction.clone())?;
                reconsidered.push(transition.outcome);
            }
        }

        Ok(ManagedMempoolReorgLifecycle {
            connected,
            reconsidered,
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

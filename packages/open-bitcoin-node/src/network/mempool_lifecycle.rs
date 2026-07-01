// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/txmempool.cpp
// - packages/bitcoin-knots/src/kernel/disconnected_transactions.cpp

use open_bitcoin_core::{
    chainstate::AnchoredBlock,
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::Block,
};
use open_bitcoin_mempool::{MempoolLifecycleSummary, MempoolOutcome};
use open_bitcoin_network::TxServingRecordStatus;

use super::{ManagedNetworkError, ManagedPeerNetwork};
use crate::ChainstateStore;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ManagedMempoolReorgLifecycle {
    pub connected: Vec<MempoolLifecycleSummary>,
    pub reconsidered: Vec<MempoolOutcome>,
}

impl<S: ChainstateStore> ManagedPeerNetwork<S> {
    pub(super) fn apply_connected_block_mempool_lifecycle(
        &mut self,
        block: &Block,
    ) -> Result<MempoolLifecycleSummary, ManagedNetworkError> {
        let summary = self
            .mempool
            .mempool_mut()
            .remove_for_connected_block(block)?;
        let removed_txids = summary
            .removed
            .iter()
            .map(|removal| removal.txid)
            .collect::<Vec<_>>();
        self.remove_stored_transactions_with_status(
            &removed_txids,
            TxServingRecordStatus::Confirmed,
        )?;

        Ok(summary)
    }

    pub(super) fn apply_reorg_mempool_lifecycle(
        &mut self,
        disconnect_blocks: &[Block],
        replacement_branch: &[AnchoredBlock],
        verify_flags: ScriptVerifyFlags,
        consensus_params: ConsensusParams,
    ) -> Result<ManagedMempoolReorgLifecycle, ManagedNetworkError> {
        let mut connected = Vec::with_capacity(replacement_branch.len());
        for anchored_block in replacement_branch {
            connected.push(self.apply_connected_block_mempool_lifecycle(&anchored_block.block)?);
        }

        let mut reconsidered = Vec::new();
        for block in disconnect_blocks.iter().rev() {
            for transaction in block.transactions.iter().skip(1) {
                let outcome = self.mempool.submit_transaction_outcome(
                    &self.chainstate,
                    transaction.clone(),
                    verify_flags,
                    consensus_params,
                )?;
                match &outcome {
                    MempoolOutcome::Accepted { .. } | MempoolOutcome::Replaced { .. } => {
                        self.apply_admitted_outcome(&outcome, transaction.clone())?;
                    }
                    MempoolOutcome::Evicted { txid, .. } | MempoolOutcome::Expired { txid, .. } => {
                        let status = match outcome {
                            MempoolOutcome::Evicted { .. } => TxServingRecordStatus::Evicted,
                            MempoolOutcome::Expired { .. } => TxServingRecordStatus::Expired,
                            _ => TxServingRecordStatus::Stale,
                        };
                        self.remove_stored_transactions_with_status(&[*txid], status)?;
                    }
                    MempoolOutcome::Rejected { .. }
                    | MempoolOutcome::Duplicate { .. }
                    | MempoolOutcome::Orphaned { .. } => {}
                }
                reconsidered.push(outcome);
            }
        }

        Ok(ManagedMempoolReorgLifecycle {
            connected,
            reconsidered,
        })
    }
}

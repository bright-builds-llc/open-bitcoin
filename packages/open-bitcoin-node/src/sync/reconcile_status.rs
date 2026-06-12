// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use crate::{
    FieldAvailability, RuntimeMetadata,
    status::{SyncReconcileProgressStatus, SyncReorgEvidence, SyncStatus},
};

use super::{DurableSyncRuntime, SyncRunSummary, tip, types::SyncReconcileProgress};

impl DurableSyncRuntime {
    pub(super) fn record_reconcile_progress(&mut self, progress: SyncReconcileProgress) {
        if matches!(progress, SyncReconcileProgress::NoChange)
            && self.maybe_reconcile_progress.is_some()
        {
            return;
        }
        self.maybe_reconcile_progress = Some(progress);
    }

    pub(super) fn project_reconcile_status(
        &self,
        sync: &mut SyncStatus,
        summary: &SyncRunSummary,
        metadata: &RuntimeMetadata,
    ) {
        let maybe_previous_reorg = previous_latest_reorg(metadata);
        let Some(progress) = summary.maybe_reconcile_progress.as_ref() else {
            if let Some(evidence) = maybe_previous_reorg {
                sync.latest_reorg = FieldAvailability::available(evidence);
            }
            return;
        };

        if let SyncReconcileProgress::ReorgPersisted(evidence) = progress {
            sync.latest_reorg = FieldAvailability::available(evidence.clone());
        } else if let Some(evidence) = maybe_previous_reorg {
            sync.latest_reorg = FieldAvailability::available(evidence);
        }
        if let Some(status) = self.reconcile_progress_status(progress) {
            sync.reconcile_progress = FieldAvailability::available(status);
        }
    }

    fn reconcile_progress_status(
        &self,
        progress: &SyncReconcileProgress,
    ) -> Option<SyncReconcileProgressStatus> {
        match progress {
            SyncReconcileProgress::NoChange => Some(SyncReconcileProgressStatus::NoChange),
            SyncReconcileProgress::ExtendedActiveChain { connected_count } => self
                .connected_block()
                .map(|tip| SyncReconcileProgressStatus::ExtendedActiveChain {
                    connected_count: *connected_count,
                    final_active_height: tip.height,
                    final_active_hash: tip::block_hash_hex(tip.block_hash),
                }),
            SyncReconcileProgress::BranchCompetitionAwaitingBodies { missing_count, .. } => {
                let (common_ancestor, branch_tip) = self.branch_reconcile_points()?;
                Some(
                    SyncReconcileProgressStatus::BranchCompetitionAwaitingBodies {
                        common_ancestor_height: u64::from(common_ancestor.height),
                        common_ancestor_hash: tip::block_hash_hex(common_ancestor.block_hash),
                        branch_tip_height: u64::from(branch_tip.height),
                        branch_tip_hash: tip::block_hash_hex(branch_tip.block_hash),
                        missing_block_count: *missing_count,
                    },
                )
            }
            SyncReconcileProgress::SideBranchPreserved => {
                let active_tip = self.network.chainstate_snapshot().active_chain.pop()?;
                let branch_tip = self.network.best_chain_entries().pop()?;
                Some(SyncReconcileProgressStatus::SideBranchPreserved {
                    branch_tip_height: u64::from(branch_tip.height),
                    branch_tip_hash: tip::block_hash_hex(branch_tip.block_hash),
                    active_tip_height: u64::from(active_tip.height),
                    active_tip_hash: tip::block_hash_hex(active_tip.block_hash),
                })
            }
            SyncReconcileProgress::ReorgPersisted(evidence) => {
                Some(SyncReconcileProgressStatus::ReorgPersisted {
                    evidence: evidence.clone(),
                })
            }
        }
    }

    fn branch_reconcile_points(
        &self,
    ) -> Option<(
        open_bitcoin_core::chainstate::ChainPosition,
        open_bitcoin_network::HeaderEntry,
    )> {
        let active_chain = self.network.chainstate_snapshot().active_chain;
        let best_chain = self.network.best_chain_entries();
        let common_prefix_len = active_chain
            .iter()
            .zip(&best_chain)
            .take_while(|(active, best)| active.block_hash == best.block_hash)
            .count();
        let common_ancestor = active_chain
            .get(common_prefix_len.saturating_sub(1))
            .or_else(|| active_chain.first())?;
        Some((common_ancestor.clone(), best_chain.last()?.clone()))
    }
}

fn previous_latest_reorg(metadata: &RuntimeMetadata) -> Option<SyncReorgEvidence> {
    metadata
        .maybe_sync_state
        .as_ref()
        .and_then(|state| match &state.sync.latest_reorg {
            FieldAvailability::Available(evidence) => Some(evidence.clone()),
            FieldAvailability::Unavailable { .. } => None,
        })
}

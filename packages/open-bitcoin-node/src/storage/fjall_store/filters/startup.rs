// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Exclusive startup recovery after coins H/B recovery and before prune resume.
//! Runtime construction has not exposed any concurrent coins/metadata writer yet.

use super::{FjallNodeStore, StorageError, index_corruption};
use open_bitcoin_core::{
    chainstate::{
        FilterRecoveryPlan, VerifiedChainstateFence, filter_index::lifecycle::IndexLifecycle,
    },
    primitives::BlockHash,
};

impl FjallNodeStore {
    /// Validate saved authority/protection before permitting startup deletion.
    pub(crate) fn recover_basic_filter_index_before_prune(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
    ) -> Result<(), StorageError> {
        self.recover_basic_filter_index(maybe_recovered_best_block)
            .map_err(|error| match error {
                StorageError::Corruption { detail, .. } => index_corruption(format!(
                    "fail_closed BASIC index startup stopped closed and did not reindex: {detail}"
                )),
                other => other,
            })
    }

    fn recover_basic_filter_index(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        if self.maybe_basic_filter_state()?.is_some() {
            self.validate_basic_filter_records()?;
        }
        let Some(owner) = self.maybe_basic_filter_owner_guarded(&control)? else {
            return Ok(());
        };
        let Some(checkpoint) = self.maybe_basic_filter_checkpoint()? else {
            return Err(index_corruption("missing BASIC checkpoint"));
        };
        if matches!(owner.lifecycle(), IndexLifecycle::Disabled { .. }) {
            if let Some(protection) = owner.maybe_effective_protection()
                && let Some(intent) = self.maybe_prune_intent()?
            {
                protection
                    .check_prune_intent(intent.height)
                    .map_err(index_corruption)?;
            }
            return Ok(());
        }
        let protection = owner
            .maybe_effective_protection()
            .ok_or_else(|| index_corruption("missing BASIC reserved protection"))?;
        let (positions, _) = self.load_chain_meta_for_open()?;
        let fence = VerifiedChainstateFence::new(maybe_recovered_best_block, Some(&positions))
            .map_err(index_corruption)?;
        let plan = self.scan_basic_filter_checkpoint(checkpoint, protection, &fence)?;
        let (checkpoint, protection, reconcile) = match plan {
            FilterRecoveryPlan::Keep {
                checkpoint,
                protection,
            } => (checkpoint, protection, false),
            FilterRecoveryPlan::Reconcile {
                checkpoint,
                protection,
            } => (checkpoint, protection, true),
            FilterRecoveryPlan::Refuse(error) => return Err(index_corruption(error)),
            FilterRecoveryPlan::LegacyAbsent => {
                return Err(index_corruption("unexpected saved BASIC absence"));
            }
        };
        // The ordinary buffered helper deliberately excludes heights 0/1. Check
        // the recovered required-input boundary directly before any publication.
        if let Some(intent) = self.maybe_prune_intent()? {
            protection
                .check_prune_intent(intent.height)
                .map_err(index_corruption)?;
        }
        if self.maybe_basic_filter_lifecycle()?.is_none() {
            self.materialize_basic_filter_owner_guarded(owner.lifecycle(), &mut control)?;
        }
        if reconcile {
            // The concrete publisher rereads B/metadata and SyncAll-publishes
            // checkpoint and stronger lock together; it never erases suffix rows.
            self.publish_basic_filter_checkpoint_guarded(
                &fence,
                checkpoint,
                protection,
                &[],
                &mut control,
            )?;
        }
        Ok(())
    }
}

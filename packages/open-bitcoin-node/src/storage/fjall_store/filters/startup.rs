// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Exclusive startup recovery after coins H/B recovery and before prune resume.
//! Runtime construction has not exposed any concurrent coins/metadata writer yet.

use super::{FjallNodeStore, StorageError, index_corruption};
use open_bitcoin_core::{
    chainstate::{
        BASIC_INDEX_PRUNE_LOCK, FilterRecoveryPlan, IndexInputProtection, VerifiedChainstateFence,
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
        let maybe_state = self.maybe_basic_filter_state()?;
        let locks = self.load_prune_locks()?;
        let maybe_lock = locks
            .iter()
            .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
        let maybe_protection = maybe_lock
            .map(IndexInputProtection::from_saved_lock)
            .transpose()
            .map_err(index_corruption)?;
        let Some(state) = maybe_state else {
            let (records, projections) = self.basic_filter_artifacts()?;
            return match FilterRecoveryPlan::for_absent_state(
                records,
                projections,
                maybe_protection,
            ) {
                FilterRecoveryPlan::LegacyAbsent => Ok(()),
                FilterRecoveryPlan::Refuse(error) => Err(index_corruption(error)),
                _ => Err(index_corruption(
                    "unexpected absent BASIC recovery decision",
                )),
            };
        };

        self.validate_basic_filter_records()?;
        let Some(checkpoint) = self.maybe_basic_filter_checkpoint()? else {
            return Err(index_corruption("missing BASIC checkpoint"));
        };
        let protection = match maybe_protection {
            Some(protection) => protection,
            None if state.protection == IndexInputProtection::HeightSpaceExhausted => {
                IndexInputProtection::HeightSpaceExhausted
            }
            None => return Err(index_corruption("missing BASIC reserved protection")),
        };
        if !protection.covers(state.protection) || !protection.covers(checkpoint.input_protection())
        {
            return Err(index_corruption("weak BASIC reserved protection"));
        }
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
        if reconcile {
            // The concrete publisher rereads B/metadata and SyncAll-publishes
            // checkpoint and stronger lock together; it never erases suffix rows.
            self.publish_basic_filter_checkpoint(&fence, checkpoint, protection, &[])?;
        }
        Ok(())
    }
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Exclusive startup recovery after coins H/B recovery and before prune resume.
//! Runtime construction has not exposed any concurrent coins/metadata writer yet.

use super::{FjallNodeStore, StorageError, index_corruption};
use crate::chainstate::BasicFilterStartupMode;
use open_bitcoin_core::{
    chainstate::{
        FilterCheckpoint, FilterRecoveryPlan, IndexInputProtection, VerifiedChainstateFence,
        filter_index::lifecycle::{EffectiveIndexOwnership, IndexLifecycle},
    },
    primitives::BlockHash,
};

/// Inspection grants no publication authority and does not change saved bytes.
pub(super) struct InspectedBasicFilterRecovery {
    pub checkpoint: FilterCheckpoint,
    pub protection: IndexInputProtection,
    pub reconcile: bool,
}

impl FjallNodeStore {
    /// Validate saved authority/protection before permitting startup deletion.
    pub(crate) fn recover_basic_filter_index_before_prune(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
    ) -> Result<(), StorageError> {
        self.configure_basic_filter_index_before_prune(
            maybe_recovered_best_block,
            BasicFilterStartupMode::PreserveSaved,
        )
    }

    /// Configured lifecycle effects are exclusive and precede resumed pruning.
    pub(crate) fn configure_basic_filter_index_before_prune(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
        mode: BasicFilterStartupMode,
    ) -> Result<(), StorageError> {
        self.filter_publication_guard()?.read_integrity = false;
        self.apply_basic_filter_startup(maybe_recovered_best_block, mode)
            .and_then(|()| {
                let mut control = self.filter_publication_guard()?;
                control.read_integrity = false;
                self.validate_basic_filter_records()?;
                control.read_integrity = true;
                Ok(())
            })
            .map_err(|error| match error {
                StorageError::Corruption { detail, .. } => index_corruption(format!(
                    "fail_closed BASIC index startup stopped closed and did not reindex: {detail}"
                )),
                other => other,
            })
    }

    fn apply_basic_filter_startup(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
        mode: BasicFilterStartupMode,
    ) -> Result<(), StorageError> {
        match mode {
            BasicFilterStartupMode::PreserveSaved => {
                self.recover_basic_filter_index(maybe_recovered_best_block)
            }
            BasicFilterStartupMode::Disabled => self.disable_basic_filter_index(),
            BasicFilterStartupMode::Enabled => {
                if maybe_recovered_best_block.is_none() {
                    return Err(index_corruption("validated genesis history required"));
                }
                let (positions, _) = self.load_chain_meta_for_open()?;
                let fence =
                    VerifiedChainstateFence::new(maybe_recovered_best_block, Some(&positions))
                        .map_err(index_corruption)?;
                self.enable_basic_filter_index(&fence)
            }
        }
    }

    fn recover_basic_filter_index(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        control.invalidate_append()?;
        if self.maybe_basic_filter_state()?.is_some() {
            self.validate_basic_filter_records()?;
        }
        let Some(owner) = self.maybe_basic_filter_owner_guarded(&control)? else {
            return Ok(());
        };
        if matches!(owner.lifecycle(), IndexLifecycle::Disabled { .. }) {
            self.maybe_basic_filter_checkpoint()?;
            if let Some(protection) = owner.maybe_effective_protection()
                && let Some(intent) = self.maybe_prune_intent()?
            {
                protection
                    .check_prune_intent(intent.height)
                    .map_err(index_corruption)?;
            }
            return Ok(());
        }
        let (positions, _) = self.load_chain_meta_for_open()?;
        let fence = VerifiedChainstateFence::new(maybe_recovered_best_block, Some(&positions))
            .map_err(index_corruption)?;
        let InspectedBasicFilterRecovery {
            checkpoint,
            protection,
            reconcile,
        } = self.inspect_basic_filter_recovery(owner, &fence)?;
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

    /// Full saved-prefix recovery is read-only until required inputs have passed.
    pub(super) fn inspect_basic_filter_recovery(
        &self,
        owner: EffectiveIndexOwnership,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<InspectedBasicFilterRecovery, StorageError> {
        let saved = self
            .maybe_basic_filter_checkpoint()?
            .ok_or_else(|| index_corruption("missing BASIC checkpoint"))?;
        let plan = self.scan_basic_filter_checkpoint(saved, owner.saved_protection(), fence)?;
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
        let protection = owner
            .maybe_effective_protection()
            .filter(|retained| retained.covers(protection))
            .unwrap_or(protection);
        Ok(InspectedBasicFilterRecovery {
            checkpoint,
            protection,
            reconcile,
        })
    }
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Exclusive startup recovery after coins H/B recovery and before prune resume.
//! Runtime construction has not exposed any concurrent coins/metadata writer yet.

use super::retention::{RetainedBasicFilterObservation, RetentionObservationLimits};
use super::{FjallNodeStore, PublicationControl, StorageError, index_corruption};
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
    /// Capture effective saved intent once after successful startup validation.
    /// This bounded ownership read neither validates the forest nor recovers it.
    pub(crate) fn resolved_basic_filter_indexing_after_startup(
        &self,
    ) -> Result<Option<bool>, StorageError> {
        let control = self.filter_publication_guard()?;
        Ok(self
            .maybe_basic_filter_owner_guarded(&control)?
            .map(|owner| matches!(owner.lifecycle(), IndexLifecycle::Active { .. })))
    }

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
        self.configure_basic_filter_index_before_prune_with_retention_limits(
            maybe_recovered_best_block,
            mode,
            super::retention::RetentionObservationLimits::default(),
        )
    }

    pub(super) fn configure_basic_filter_index_before_prune_with_retention_limits(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
        mode: BasicFilterStartupMode,
        limits: super::retention::RetentionObservationLimits,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        let maybe_previous_fault = control.maybe_read_fault;
        control.invalidate_read_integrity()?;
        let result = self
            .apply_basic_filter_startup_guarded(
                maybe_recovered_best_block,
                mode,
                limits,
                &mut control,
            )
            // Preserve startup's closing capture revocation without rescanning.
            .and_then(|()| control.advance_integrity_epoch());
        if result.is_err() {
            control.maybe_read_fault = maybe_previous_fault;
            control.maybe_read_fault_protection = None;
            control.read_integrity = false;
            control.install_retention(RetainedBasicFilterObservation::Failed);
        } else {
            let retained = control.retention;
            control.reset_verified_forest(retained);
        }
        result.map_err(|error| match error {
            StorageError::Corruption { detail, .. } => index_corruption(format!(
                "fail_closed BASIC index startup stopped closed and did not reindex: {detail}"
            )),
            other => other,
        })
    }

    fn apply_basic_filter_startup_guarded(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
        mode: BasicFilterStartupMode,
        limits: RetentionObservationLimits,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        match mode {
            BasicFilterStartupMode::PreserveSaved => {
                self.recover_basic_filter_index_guarded(maybe_recovered_best_block, limits, control)
            }
            BasicFilterStartupMode::Disabled => {
                self.disable_basic_filter_index_guarded(control, limits)
            }
            BasicFilterStartupMode::Enabled => {
                if maybe_recovered_best_block.is_none() {
                    return Err(index_corruption("validated genesis history required"));
                }
                let (positions, _) = self.load_chain_meta_for_open()?;
                let fence =
                    VerifiedChainstateFence::new(maybe_recovered_best_block, Some(&positions))
                        .map_err(index_corruption)?;
                self.enable_basic_filter_index_guarded(&fence, control, limits)
            }
        }
    }

    fn recover_basic_filter_index_guarded(
        &self,
        maybe_recovered_best_block: Option<BlockHash>,
        limits: RetentionObservationLimits,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        control.invalidate_append()?;
        let retained = if self.maybe_basic_filter_state()?.is_some() {
            self.measure_basic_filter_records(limits)?
        } else {
            // The bounded owner/artifact classification below must prove absence
            // before this candidate zero can be installed. Preserve its refusal
            // precedence for partial state rather than scanning orphan artifacts.
            RetainedBasicFilterObservation::empty()
        };
        let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {
            control.read_integrity = true;
            control.install_retention(retained);
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
            control.read_integrity = true;
            control.install_retention(retained);
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
            self.materialize_basic_filter_owner_guarded(owner.lifecycle(), control)?;
        }
        if reconcile {
            self.publish_basic_filter_checkpoint_guarded(
                &fence,
                checkpoint,
                protection,
                &[],
                control,
            )?;
        }
        control.read_integrity = true;
        control.install_retention(retained);
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

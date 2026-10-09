// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! One guarded durable ownership loader. Lock order is authority, automatic state,
//! publication, then payload; guarded helpers never reacquire publication.

use super::publication::PublicationControl;
use super::{FjallNodeStore, StorageError, StorageNamespace, codec, index_corruption};
use open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork;
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, CoinsView, FilterCheckpoint, FilterIndexError, IndexPrefix,
    VerifiedChainstateFence,
    filter_index::lifecycle::{
        EffectiveIndexOwnership, IndexCheckpointIdentity, IndexGeneration, IndexLifecycle,
        IndexLifecycleError, IndexWorkIdentity,
    },
};
use open_bitcoin_core::primitives::BlockHash;
use std::sync::{Arc, Mutex};

mod proofs;

/// Constant-size facts proved by complete recovery, then maintained by trusted writes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::storage::fjall_store) struct BasicFilterAppendIdentity {
    pub owner: EffectiveIndexOwnership,
    pub generation: IndexGeneration,
    pub branch_identity: BlockHash,
    pub processed: FilterCheckpoint,
    pub safe_checkpoint: FilterCheckpoint,
    pub durable_height: u32,
    pub durable_hash: BlockHash,
    pub revision: u64,
    pub projection_authority: BasicFilterProjectionAuthority,
    pub maybe_replacement: Option<BasicFilterReplacementIdentity>,
}

/// Proven prefix repair is distinct from acceptance of a live replacement.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::storage::fjall_store) enum BasicFilterProjectionAuthority {
    RecoveredPrefix,
    ValidatedReorg,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::storage::fjall_store) struct BasicFilterReplacementIdentity {
    pub maybe_accepted: Option<(u32, BlockHash)>,
    pub maybe_displaced_fence: Option<(u32, BlockHash)>,
}

/// Opaque same-store capability. Domain progress or caller-selected tips cannot mint it.
pub(crate) struct BasicFilterAppendProof {
    pub(super) identity: BasicFilterAppendIdentity,
    publication: Arc<Mutex<PublicationControl>>,
    preparation_work: TurnWork,
    maybe_maximum_work: Option<TurnWork>,
}

impl BasicFilterAppendProof {
    pub(super) fn bounded_reorg_copy(&self, maximum_work: TurnWork) -> Result<Self, StorageError> {
        if !self.preparation_work.fits(maximum_work) {
            return Err(index_corruption("BASIC reorg preparation work budget"));
        }
        Ok(Self {
            identity: self.identity,
            publication: Arc::clone(&self.publication),
            preparation_work: self.preparation_work,
            maybe_maximum_work: Some(maximum_work),
        })
    }
    pub(crate) fn preparation_work(&self) -> TurnWork {
        self.preparation_work
    }
    /// Tighten bookkeeping only; capability identity and captured work stay intact.
    pub(crate) fn with_remaining_budget(
        mut self,
        maximum_work: TurnWork,
    ) -> Result<Self, StorageError> {
        let Some(previous) = self.maybe_maximum_work else {
            return Err(index_corruption(
                "BASIC remaining budget requires budgeted acquisition",
            ));
        };
        if !maximum_work.fits(previous) || !self.preparation_work.fits(maximum_work) {
            return Err(index_corruption(
                "BASIC remaining budget cannot expand or discard captured work",
            ));
        }
        self.maybe_maximum_work = Some(maximum_work);
        Ok(self)
    }
    pub(super) fn maybe_maximum_work(&self) -> Option<TurnWork> {
        self.maybe_maximum_work
    }
    pub(crate) fn revision(&self) -> u64 {
        self.identity.revision
    }
    pub(crate) fn generation(&self) -> IndexGeneration {
        self.identity.generation
    }
    pub(crate) fn branch_identity(&self) -> BlockHash {
        self.identity.branch_identity
    }
    pub(crate) fn processed(&self) -> FilterCheckpoint {
        self.identity.processed
    }
    pub(crate) fn protection(&self) -> open_bitcoin_core::chainstate::IndexInputProtection {
        self.identity
            .owner
            .maybe_effective_protection()
            .unwrap_or(self.identity.owner.saved_protection())
    }
    pub(crate) fn safe_checkpoint(&self) -> FilterCheckpoint {
        self.identity.safe_checkpoint
    }
    pub(crate) fn durable_tip(&self) -> (u32, BlockHash) {
        (self.identity.durable_height, self.identity.durable_hash)
    }
}

/// Only the guarded same-store Active loader can construct live work authority.
pub(crate) struct BasicFilterWorkToken {
    identity: IndexWorkIdentity,
    publication: Arc<Mutex<PublicationControl>>,
}

impl FjallNodeStore {
    pub(crate) fn shares_basic_filter_store(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.filter_publication, &other.filter_publication)
    }

    pub(crate) fn invalidate_basic_filter_append(&self) -> Result<(), StorageError> {
        self.filter_publication_guard()?.invalidate_append()
    }

    pub(crate) fn discard_basic_filter_flush(
        &self,
        proof: &BasicFilterAppendProof,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        if !Arc::ptr_eq(&self.filter_publication, &proof.publication) {
            return Err(index_corruption("foreign BASIC flush cleanup"));
        }
        if control
            .maybe_pending_coins
            .is_some_and(|pending| pending.previous == proof.identity)
        {
            control.maybe_pending_coins = None;
        }
        if control
            .maybe_completed_metadata
            .is_some_and(|pending| pending.coins.previous == proof.identity)
        {
            control.maybe_completed_metadata = None;
        }
        Ok(())
    }

    pub(crate) fn confirm_basic_filter_flush(
        &self,
        completed: crate::chainstate::CompletedValidatedFlush,
    ) -> Result<(), StorageError> {
        #[cfg(test)]
        self.run_basic_filter_writer_interleave(super::BasicFilterWriterInterleave::BeforeConfirm)?;
        let mut control = self.filter_publication_guard()?;
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption(
                "BASIC flush confirmation suspended for reorg preview",
            ));
        }
        let result = (|| {
            let work = completed.initial_proof();
            if !completed.belongs_to(self)
                || !Arc::ptr_eq(&work.publication, &self.filter_publication)
            {
                return Err(index_corruption("foreign BASIC validated flush"));
            }
            let pending = control
                .maybe_completed_metadata
                .ok_or_else(|| index_corruption("absent BASIC completed metadata receipt"))?;
            if pending.coins.previous != work.identity
                || !pending.coins.completed
                || work.identity.revision.checked_add(1) != Some(pending.coins.revision)
                || work.identity.revision.checked_add(2) != Some(control.revision)
                || completed.metadata_revision() != control.revision
                || pending.identity.revision != control.revision
                || pending.identity.generation != work.identity.generation
                || pending.identity.branch_identity != work.identity.branch_identity
                || pending.identity.owner != work.identity.owner
                || pending.identity.processed != work.identity.processed
                || pending.identity.safe_checkpoint != work.identity.safe_checkpoint
                || completed.accepted_endpoint()
                    != (
                        pending.identity.durable_height,
                        pending.identity.durable_hash,
                    )
            {
                return Err(index_corruption("stale BASIC own-publication receipt"));
            }
            let owner = self
                .maybe_basic_filter_owner_guarded(&control)?
                .ok_or_else(|| index_corruption("absent BASIC flush owner"))?;
            let view = self.coins_view();
            if owner != pending.identity.owner
                || !view.head_blocks().map_err(index_corruption)?.is_empty()
                || view.best_block().map_err(index_corruption)?
                    != Some(pending.identity.durable_hash)
            {
                return Err(index_corruption("BASIC completed flush source identity"));
            }
            control.maybe_completed_metadata = None;
            let mut identity = pending.identity;
            if let Some(replacement) = &mut identity.maybe_replacement {
                replacement.maybe_displaced_fence = None;
            }
            control.maybe_append_identity = Some(identity);
            Ok(())
        })();
        let work = completed.initial_proof();
        if result.is_err() && Arc::ptr_eq(&self.filter_publication, &work.publication) {
            if control
                .maybe_completed_metadata
                .is_some_and(|pending| pending.coins.previous == work.identity)
            {
                control.maybe_completed_metadata = None;
            }
            if control
                .maybe_pending_coins
                .is_some_and(|pending| pending.previous == work.identity)
            {
                control.maybe_pending_coins = None;
            }
        }
        result
    }
    pub(crate) fn check_basic_filter_append_proof(
        &self,
        proof: &BasicFilterAppendProof,
    ) -> Result<TurnWork, StorageError> {
        let control = self.filter_publication_guard()?;
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption("BASIC append suspended for reorg preview"));
        }
        let mut work = proof.preparation_work();
        self.check_basic_filter_append_proof_guarded_counted(proof, &control, &mut work)?;
        Ok(work)
    }

    pub(super) fn check_basic_filter_append_proof_guarded_counted(
        &self,
        proof: &BasicFilterAppendProof,
        control: &PublicationControl,
        work: &mut TurnWork,
    ) -> Result<BasicFilterAppendIdentity, StorageError> {
        if !Arc::ptr_eq(&proof.publication, &self.filter_publication) {
            return Err(index_corruption("foreign BASIC append proof"));
        }
        if control.maybe_append_identity != Some(proof.identity)
            || control.revision != proof.identity.revision
        {
            return Err(index_corruption("stale or invalidated BASIC append proof"));
        }
        let owner = self
            .maybe_basic_filter_owner_guarded_counted(control, work, proof.maybe_maximum_work)?
            .ok_or_else(|| index_corruption("absent BASIC append owner"))?;
        if owner != proof.identity.owner
            || owner.lifecycle()
                != (IndexLifecycle::Active {
                    generation: proof.identity.generation,
                })
            || owner.checkpoint().checkpoint() != proof.identity.safe_checkpoint
        {
            return Err(index_corruption(
                "stale BASIC append generation or checkpoint",
            ));
        }
        if proof.identity.processed != proof.identity.safe_checkpoint {
            let IndexPrefix::Committed(processed) = proof.identity.processed.prefix() else {
                return Err(index_corruption(
                    "empty BASIC processed frontier ahead of safe",
                ));
            };
            if self.maybe_basic_filter_append_projection(
                processed.height(),
                work,
                proof.maybe_maximum_work,
            )? != Some(processed.block_hash())
                || self.read_basic_filter_local_identity(
                    processed.block_hash(),
                    work,
                    proof.maybe_maximum_work,
                )? != processed
            {
                return Err(index_corruption("BASIC processed frontier identity"));
            }
        }
        let view = self.coins_view();
        // Each trait call independently classifies both H and B.
        super::append::charge_append_work(
            work,
            TurnWork {
                checkpoint_operations: 6,
                cloned_bytes: 64,
                ..TurnWork::default()
            },
            proof.maybe_maximum_work,
        )?;
        if !view.head_blocks().map_err(index_corruption)?.is_empty()
            || view.best_block().map_err(index_corruption)? != Some(proof.identity.durable_hash)
        {
            return Err(index_corruption("BASIC append durable coins identity"));
        }
        Ok(proof.identity)
    }

    /// Exclusive caller has completed forest, projection, fence and suffix preflight.
    pub(super) fn install_recovered_basic_filter_append_guarded(
        &self,
        fence: &VerifiedChainstateFence<'_>,
        control: &mut PublicationControl,
    ) -> Result<(), StorageError> {
        let owner = self
            .maybe_basic_filter_owner_guarded(control)?
            .ok_or_else(|| index_corruption("absent recovered BASIC owner"))?;
        let IndexLifecycle::Active { generation } = owner.lifecycle() else {
            return Err(index_corruption("disabled recovered BASIC owner"));
        };
        let checkpoint = owner.checkpoint().checkpoint();
        control.maybe_append_identity = Some(BasicFilterAppendIdentity {
            owner,
            generation,
            branch_identity: fence.tip().block_hash,
            processed: checkpoint,
            safe_checkpoint: checkpoint,
            durable_height: fence.tip().height,
            durable_hash: fence.tip().block_hash,
            revision: control.revision,
            projection_authority: BasicFilterProjectionAuthority::RecoveredPrefix,
            maybe_replacement: None,
        });
        Ok(())
    }

    /// Reload bounded ownership facts while the shared publication guard is held.
    /// Startup/release separately prove complete forest and projection integrity.
    pub(in crate::storage::fjall_store) fn maybe_basic_filter_owner_guarded(
        &self,
        _control: &PublicationControl,
    ) -> Result<Option<EffectiveIndexOwnership>, StorageError> {
        self.maybe_basic_filter_owner_guarded_counted(_control, &mut TurnWork::default(), None)
    }

    pub(super) fn maybe_basic_filter_owner_guarded_counted(
        &self,
        _control: &PublicationControl,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Option<EffectiveIndexOwnership>, StorageError> {
        super::append::charge_append_work(
            work,
            TurnWork {
                checkpoint_operations: 4,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        #[cfg(test)]
        self.count_filter_integrity_read();
        let maybe_state = self
            .block_index
            .get(codec::STATE_KEY)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
            .map(|bytes| codec::decode_state(bytes.as_ref()))
            .transpose()?;
        #[cfg(test)]
        self.count_filter_integrity_read();
        let maybe_lifecycle = self
            .block_index
            .get(codec::ownership::OWNER_KEY)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
            .map(|bytes| codec::ownership::decode_owner(bytes.as_ref()))
            .transpose()?;
        let locks = self.load_basic_filter_append_locks(work, maybe_maximum_work)?;
        let maybe_lock = locks
            .iter()
            .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
        let maybe_checkpoint = maybe_state
            .map(|state| {
                self.bounded_basic_filter_checkpoint(state, work, maybe_maximum_work)
                    .map(|checkpoint| {
                        IndexCheckpointIdentity::new(
                            checkpoint,
                            state.fence_height,
                            state.fence_hash,
                        )
                    })
            })
            .transpose()?;
        let (records, projections) = if maybe_state.is_none() {
            self.basic_filter_artifacts()?
        } else {
            (false, false)
        };
        EffectiveIndexOwnership::maybe_from_artifacts(
            maybe_lifecycle,
            maybe_checkpoint,
            maybe_state.map(|state| state.protection),
            maybe_lock,
            records,
            projections,
        )
        .map_err(|error| match error {
            IndexLifecycleError::Integrity(FilterIndexError::WeakProtection)
                if maybe_lock.is_none() =>
            {
                index_corruption("missing BASIC reserved protection")
            }
            IndexLifecycleError::Integrity(FilterIndexError::WeakProtection) => {
                index_corruption("weak BASIC reserved protection")
            }
            other => index_corruption(other),
        })
    }

    pub(super) fn maybe_basic_filter_lifecycle(
        &self,
    ) -> Result<Option<IndexLifecycle>, StorageError> {
        self.get_bytes(StorageNamespace::BlockIndex, codec::ownership::OWNER_KEY)?
            .map(|bytes| codec::ownership::decode_owner(&bytes))
            .transpose()
    }

    fn bounded_basic_filter_checkpoint(
        &self,
        state: super::StoredFilterState,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<FilterCheckpoint, StorageError> {
        let Some((height, hash)) = state.maybe_endpoint else {
            return Ok(FilterCheckpoint::new(IndexPrefix::Empty));
        };
        if height > state.fence_height {
            return Err(index_corruption("BASIC checkpoint exceeds saved fence"));
        }
        if height == state.fence_height && hash != state.fence_hash {
            return Err(index_corruption(
                "BASIC checkpoint differs from same-height saved fence",
            ));
        }
        if self.maybe_basic_filter_append_projection(height, work, maybe_maximum_work)?
            != Some(hash)
        {
            return Err(index_corruption("BASIC checkpoint projection identity"));
        }
        let identity = self.read_basic_filter_local_identity(hash, work, maybe_maximum_work)?;
        if identity.height() != height {
            return Err(index_corruption("BASIC checkpoint projection identity"));
        }
        Ok(FilterCheckpoint::new(IndexPrefix::Committed(identity)))
    }

    /// Borrow the value and inspect its fixed header before any name allocation.
    pub(super) fn load_basic_filter_append_locks(
        &self,
        work: &mut TurnWork,
        maybe_maximum_work: Option<TurnWork>,
    ) -> Result<Vec<open_bitcoin_core::chainstate::PruneLockInfo>, StorageError> {
        use super::super::prune::{PRUNE_LOCKS_KEY, decode_prune_locks};
        // General recovery/own-flush and operator paths keep their codec errors
        // and complete map behavior. Only budgeted turn capabilities take admission.
        if maybe_maximum_work.is_none() {
            super::append::charge_append_work(
                work,
                TurnWork {
                    checkpoint_operations: 1,
                    ..TurnWork::default()
                },
                None,
            )?;
            return self.load_prune_locks();
        }
        super::append::charge_append_work(
            work,
            TurnWork {
                checkpoint_operations: 1,
                ..TurnWork::default()
            },
            maybe_maximum_work,
        )?;
        let Some(bytes) = self
            .block_index
            .get(PRUNE_LOCKS_KEY)
            .map_err(|error| super::super::backend_failure(StorageNamespace::BlockIndex, error))?
        else {
            return Ok(Vec::new());
        };
        let header: [u8; 4] = bytes
            .as_ref()
            .get(..4)
            .ok_or_else(|| index_corruption("BASIC prune lock-map header"))?
            .try_into()
            .map_err(index_corruption)?;
        let count = u64::from(u32::from_le_bytes(header));
        let encoded = bytes.len() as u64;
        if count > encoded.saturating_sub(4) / 10 {
            return Err(index_corruption(
                "BASIC prune lock-map count exceeds envelope",
            ));
        }
        // Reserve a deliberately conservative comparison ceiling, including UTF
        // comparisons and BTreeSet allocations. This is a bound, not a timing claim.
        let cost = basic_filter_lock_map_cost(count, encoded)?;
        super::append::charge_append_work(work, cost, maybe_maximum_work)?;
        decode_prune_locks(bytes.as_ref())
    }

    /// Mint only from validated Active facts and the current durable preparation fence.
    #[cfg_attr(not(test), allow(dead_code))] // Scheduled workers arrive in Phase 157.
    pub(crate) fn maybe_basic_filter_work(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<Option<BasicFilterWorkToken>, StorageError> {
        let mut control = self.filter_publication_guard()?;
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption("BASIC work suspended for reorg preview"));
        }
        let Some(owner) = self.maybe_basic_filter_owner_guarded(&control)? else {
            return Ok(None);
        };
        if !matches!(owner.lifecycle(), IndexLifecycle::Active { .. }) {
            return Ok(None);
        }
        self.verify_basic_filter_fence(fence)?;
        if self.maybe_basic_filter_lifecycle()?.is_none() {
            self.validate_basic_filter_records()?;
            self.maybe_basic_filter_checkpoint()?;
            self.materialize_basic_filter_owner_guarded(owner.lifecycle(), &mut control)?;
        }
        Ok(Some(BasicFilterWorkToken {
            identity: IndexWorkIdentity::new(owner, fence.tip().height, fence.tip().block_hash),
            publication: Arc::clone(&self.filter_publication),
        }))
    }

    pub(super) fn check_basic_filter_work_guarded(
        &self,
        work: &BasicFilterWorkToken,
        fence: &VerifiedChainstateFence<'_>,
        control: &PublicationControl,
    ) -> Result<(), StorageError> {
        if control.maybe_reorg_suspension.is_some() {
            return Err(index_corruption(
                "BASIC publication suspended for reorg preview",
            ));
        }
        if !Arc::ptr_eq(&work.publication, &self.filter_publication) {
            return Err(index_corruption("foreign BASIC index work"));
        }
        let Some(owner) = self.maybe_basic_filter_owner_guarded(control)? else {
            return Err(index_corruption("absent BASIC index work owner"));
        };
        owner
            .check_work(work.identity, fence.tip().height, fence.tip().block_hash)
            .map_err(index_corruption)?;
        self.verify_basic_filter_fence(fence)
    }
}

/// Conservative pre-decode/encode reservation, separate from point-read counts.
pub(super) fn basic_filter_lock_map_cost(
    count: u64,
    encoded: u64,
) -> Result<TurnWork, StorageError> {
    let comparisons = count
        .checked_mul(
            count
                .checked_add(1)
                .ok_or_else(|| index_corruption("BASIC lock-map cost overflow"))?,
        )
        .and_then(|cost| cost.checked_div(2))
        .ok_or_else(|| index_corruption("BASIC lock-map cost overflow"))?;
    // Every comparison can examine the codec's complete 1024-byte name bound.
    // Four slots per entry cover Vec's small-capacity/growth case; 1024 bytes
    // per entry conservatively cover reference-only BTreeSet node allocation.
    Ok(TurnWork {
        checkpoint_operations: comparisons
            .checked_mul(1024)
            .and_then(|n| n.checked_add(encoded))
            .and_then(|n| n.checked_add(count))
            .and_then(|n| n.checked_add(1))
            .ok_or_else(|| index_corruption("BASIC lock-map cost overflow"))?,
        cloned_bytes: count
            .checked_mul(
                4 * std::mem::size_of::<open_bitcoin_core::chainstate::PruneLockInfo>() as u64
                    + 1024,
            )
            .and_then(|n| n.checked_add(encoded))
            .ok_or_else(|| index_corruption("BASIC lock-map allocation overflow"))?,
        ..TurnWork::default()
    })
}

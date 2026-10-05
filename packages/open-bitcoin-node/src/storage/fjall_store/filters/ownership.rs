// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! One guarded durable ownership loader. Lock order is authority, automatic state,
//! publication, then payload; guarded helpers never reacquire publication.

use super::publication::PublicationControl;
use super::{FjallNodeStore, StorageError, StorageNamespace, codec, index_corruption};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, FilterCheckpoint, FilterIndexError, IndexPrefix,
    VerifiedChainstateFence,
    filter_index::lifecycle::{
        EffectiveIndexOwnership, IndexCheckpointIdentity, IndexLifecycle, IndexLifecycleError,
        IndexWorkIdentity,
    },
};
use std::sync::{Arc, Mutex};

/// Only the guarded same-store Active loader can construct live work authority.
pub(crate) struct BasicFilterWorkToken {
    identity: IndexWorkIdentity,
    publication: Arc<Mutex<PublicationControl>>,
}

impl FjallNodeStore {
    /// Reload bounded ownership facts while the shared publication guard is held.
    /// Startup/release separately prove complete forest and projection integrity.
    pub(in crate::storage::fjall_store) fn maybe_basic_filter_owner_guarded(
        &self,
        _control: &PublicationControl,
    ) -> Result<Option<EffectiveIndexOwnership>, StorageError> {
        let maybe_state = self.maybe_basic_filter_state()?;
        let maybe_lifecycle = self.maybe_basic_filter_lifecycle()?;
        let locks = self.load_prune_locks()?;
        let maybe_lock = locks
            .iter()
            .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK);
        let maybe_checkpoint = maybe_state
            .map(|state| {
                self.bounded_basic_filter_checkpoint(state)
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
        let key = codec::record_key(hash);
        let bytes = self
            .get_bytes(StorageNamespace::BlockIndex, &key)?
            .ok_or_else(|| index_corruption("missing BASIC checkpoint record"))?;
        let parsed = codec::parse_record(&key, &bytes)?;
        if parsed.height != height || self.basic_filter_projection(height)? != hash {
            return Err(index_corruption("BASIC checkpoint projection identity"));
        }
        let maybe_parent_bytes = if height == 0 {
            None
        } else {
            Some(
                self.get_bytes(
                    StorageNamespace::BlockIndex,
                    &codec::record_key(parsed.parent),
                )?
                .ok_or_else(|| index_corruption("missing BASIC predecessor record"))?,
            )
        };
        let maybe_parent = maybe_parent_bytes
            .as_ref()
            .map(|bytes| codec::parse_record_fields(&codec::record_key(parsed.parent), bytes))
            .transpose()?;
        let identity = parsed.checkpoint_identity(maybe_parent.as_ref())?;
        Ok(FilterCheckpoint::new(IndexPrefix::Committed(identity)))
    }

    /// Mint only from validated Active facts and the current durable preparation fence.
    #[cfg_attr(not(test), allow(dead_code))] // Scheduled workers arrive in Phase 157.
    pub(crate) fn maybe_basic_filter_work(
        &self,
        fence: &VerifiedChainstateFence<'_>,
    ) -> Result<Option<BasicFilterWorkToken>, StorageError> {
        let mut control = self.filter_publication_guard()?;
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

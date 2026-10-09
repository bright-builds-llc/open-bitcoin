// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Constant-size authenticated rewind; immutable rows are never erased.

use super::append::charge_append_work;
use super::ownership::{
    BasicFilterAppendIdentity, BasicFilterProjectionAuthority, BasicFilterReplacementIdentity,
};
use super::publication::{checkpoint_fault, protection_fault, records_fault};
use super::{BasicFilterAppendProof, FjallNodeStore, StorageError, codec, index_corruption};
use crate::chainstate::ValidatedBasicFilterReorg;
use crate::storage::fjall_store::prune::{PRUNE_LOCKS_KEY, encode_prune_locks};
use open_bitcoin_core::chainstate::{
    BASIC_INDEX_PRUNE_LOCK, FilterCheckpoint, IndexInputProtection, IndexPrefix,
    StagedChainstateReorg,
    filter_index::catch_up::TurnWork,
    filter_index::lifecycle::{
        EffectiveIndexOwnership, IndexCheckpointIdentity, IndexGeneration, IndexLifecycle,
    },
};
use open_bitcoin_core::primitives::BlockHash;

// Match existing singleton acquisition ceilings, including legal large rows.
const BASIC_INDEX_TURN_WORK_LIMIT: TurnWork = TurnWork {
    cloned_bytes: 1024 * 1024 * 1024,
    record_operations: 512,
    checkpoint_operations: 1_000_000,
    projection_operations: 256,
    blocks: 0,
    body_bytes: 0,
    undo_bytes: 0,
    script_items: 0,
    script_bytes: 0,
    encoded_bytes: 0,
};

pub(crate) struct PreparedBasicFilterReorg {
    store: FjallNodeStore,
    previous: BasicFilterAppendIdentity,
    maybe_old: Option<(u32, BlockHash)>,
    maybe_new: Option<(u32, BlockHash)>,
    maybe_ancestor: Option<(u32, BlockHash)>,
    next: BasicFilterAppendIdentity,
    work: TurnWork,
}

pub(crate) struct CompletedBasicFilterReorg {
    prepared: PreparedBasicFilterReorg,
}

/// Preview blocks every ordinary writer without changing accepted or durable identity.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) struct BasicFilterReorgSuspension {
    previous: BasicFilterAppendIdentity,
    next: BasicFilterAppendIdentity,
    maybe_old: Option<(u32, BlockHash)>,
    maybe_new: Option<(u32, BlockHash)>,
    maybe_ancestor: Option<(u32, BlockHash)>,
}

impl PreparedBasicFilterReorg {
    fn suspension(&self) -> BasicFilterReorgSuspension {
        BasicFilterReorgSuspension {
            previous: self.previous,
            next: self.next,
            maybe_old: self.maybe_old,
            maybe_new: self.maybe_new,
            maybe_ancestor: self.maybe_ancestor,
        }
    }
    pub(crate) fn maximum_work() -> TurnWork {
        BASIC_INDEX_TURN_WORK_LIMIT
    }
    pub(crate) fn belongs_to(&self, store: &FjallNodeStore) -> bool {
        self.store.shares_basic_filter_store(store)
    }
    pub(crate) fn maybe_old_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_old
    }
    pub(crate) fn maybe_new_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_new
    }
    pub(crate) fn maybe_ancestor_endpoint(&self) -> Option<(u32, BlockHash)> {
        self.maybe_ancestor
    }
    pub(crate) fn old_identity(&self) -> (IndexGeneration, BlockHash, u64) {
        (
            self.previous.generation,
            self.previous.branch_identity,
            self.previous.revision,
        )
    }
    pub(crate) fn achieved_identity(&self) -> (IndexGeneration, BlockHash, u64) {
        (
            self.next.generation,
            self.next.branch_identity,
            self.next.revision,
        )
    }
    pub(crate) fn processed(&self) -> FilterCheckpoint {
        self.next.processed
    }
    pub(crate) fn safe_checkpoint(&self) -> FilterCheckpoint {
        self.next.safe_checkpoint
    }
    pub(crate) fn protection(&self) -> IndexInputProtection {
        self.next.owner.saved_protection()
    }
    pub(crate) fn work(&self) -> TurnWork {
        self.work
    }
}

impl CompletedBasicFilterReorg {
    pub(crate) fn prepared(&self) -> &PreparedBasicFilterReorg {
        &self.prepared
    }
}

impl FjallNodeStore {
    pub(crate) fn suspend_basic_filter_reorg(
        &self,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        let mut control = self.filter_publication_guard()?;
        if !prepared.belongs_to(self)
            || control.revision != prepared.previous.revision
            || control.maybe_append_identity != Some(prepared.previous)
            || control
                .maybe_reorg_suspension
                .is_some_and(|s| s != prepared.suspension())
        {
            return Err(index_corruption("changed BASIC preview preparation"));
        }
        control.maybe_reorg_suspension = Some(prepared.suspension());
        Ok(())
    }

    /// This private transition check grants no append or flush authority.
    pub(crate) fn check_basic_filter_reorg_suspension(
        &self,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<(), StorageError> {
        let control = self.filter_publication_guard()?;
        if !prepared.belongs_to(self)
            || control
                .maybe_reorg_suspension
                .is_some_and(|s| s != prepared.suspension())
            || control.revision != prepared.previous.revision
            || control.maybe_append_identity != Some(prepared.previous)
        {
            return Err(index_corruption("changed BASIC suspended reorg"));
        }
        Ok(())
    }

    pub(crate) fn prepare_basic_filter_reorg(
        &self,
        proof: &BasicFilterAppendProof,
        staged: &StagedChainstateReorg,
    ) -> Result<PreparedBasicFilterReorg, StorageError> {
        let proof = proof.bounded_reorg_copy(BASIC_INDEX_TURN_WORK_LIMIT)?;
        let control = self.filter_publication_guard()?;
        let mut work = proof.preparation_work();
        let previous =
            self.check_basic_filter_append_proof_guarded_counted(&proof, &control, &mut work)?;
        let maybe_old = staged.maybe_old_tip().map(|p| (p.height, p.block_hash));
        let maybe_ancestor = staged
            .maybe_common_ancestor()
            .map(|p| (p.height, p.block_hash));
        let maybe_new = staged
            .transition()
            .connected
            .last()
            .or_else(|| staged.maybe_common_ancestor())
            .map(|p| (p.height, p.block_hash));
        let old = maybe_old.ok_or_else(|| index_corruption("absent BASIC displaced endpoint"))?;
        // An achieved earlier reorg can leave taller displaced coins until own flush.
        if (previous.durable_height > old.0 && !proof.durable_displaced())
            || matches!(previous.processed.prefix(), IndexPrefix::Committed(id) if id.height() > old.0)
        {
            return Err(index_corruption("BASIC displaced endpoint behind progress"));
        }
        let processed = self.shared_basic_filter_prefix(previous.processed, staged, &mut work)?;
        let safe_checkpoint =
            self.shared_basic_filter_prefix(previous.safe_checkpoint, staged, &mut work)?;
        let protection =
            stronger_protection(proof.protection(), safe_checkpoint.input_protection());
        let generation = IndexGeneration::new(
            previous
                .generation
                .value()
                .checked_add(1)
                .ok_or_else(|| index_corruption("BASIC reorg generation exhausted"))?,
        );
        let revision = previous
            .revision
            .checked_add(1)
            .ok_or_else(|| index_corruption("BASIC reorg revision exhausted"))?;
        let mut locks =
            self.load_basic_filter_append_locks(&mut work, Some(BASIC_INDEX_TURN_WORK_LIMIT))?;
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let owner = EffectiveIndexOwnership::maybe_from_artifacts(
            Some(IndexLifecycle::Active { generation }),
            Some(IndexCheckpointIdentity::new(
                safe_checkpoint,
                previous.durable_height,
                previous.durable_hash,
            )),
            Some(protection),
            locks
                .iter()
                .find(|lock| lock.name == BASIC_INDEX_PRUNE_LOCK),
            false,
            false,
        )
        .map_err(index_corruption)?
        .ok_or_else(|| index_corruption("absent BASIC replacement owner"))?;
        let next = BasicFilterAppendIdentity {
            owner,
            generation,
            branch_identity: maybe_new.map(|endpoint| endpoint.1).unwrap_or_default(),
            processed,
            safe_checkpoint,
            revision,
            projection_authority: BasicFilterProjectionAuthority::ValidatedReorg,
            maybe_replacement: Some(BasicFilterReplacementIdentity {
                maybe_accepted: maybe_new,
                maybe_displaced_fence: staged
                    .maybe_position_at_height(previous.durable_height)
                    .is_none_or(|p| p.block_hash != previous.durable_hash)
                    .then_some((previous.durable_height, previous.durable_hash)),
            }),
            ..previous
        };
        Ok(PreparedBasicFilterReorg {
            store: self.clone(),
            previous,
            maybe_old,
            maybe_new,
            maybe_ancestor,
            next,
            work,
        })
    }

    fn shared_basic_filter_prefix(
        &self,
        checkpoint: FilterCheckpoint,
        staged: &StagedChainstateReorg,
        work: &mut TurnWork,
    ) -> Result<FilterCheckpoint, StorageError> {
        let (IndexPrefix::Committed(endpoint), Some(ancestor)) =
            (checkpoint.prefix(), staged.maybe_common_ancestor())
        else {
            return Ok(FilterCheckpoint::new(IndexPrefix::Empty));
        };
        let height = endpoint.height().min(ancestor.height);
        let position = staged
            .maybe_position_at_height(height)
            .ok_or_else(|| index_corruption("missing BASIC staged common position"))?;
        if self.maybe_basic_filter_append_projection(
            height,
            work,
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )? != Some(position.block_hash)
        {
            return Err(index_corruption("BASIC common projection identity"));
        }
        let identity = self.read_basic_filter_local_identity(
            position.block_hash,
            work,
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        if identity.height() != height {
            return Err(index_corruption("BASIC common record height"));
        }
        Ok(FilterCheckpoint::new(IndexPrefix::Committed(identity)))
    }

    pub(crate) fn complete_basic_filter_reorg(
        &self,
        mut prepared: PreparedBasicFilterReorg,
        authorization: ValidatedBasicFilterReorg,
    ) -> Result<CompletedBasicFilterReorg, StorageError> {
        authorization.validate_for(self, &prepared)?;
        charge_append_work(
            &mut prepared.work,
            authorization.work(),
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        let mut control = self.filter_publication_guard()?;
        if !prepared.belongs_to(self)
            || control
                .maybe_reorg_suspension
                .is_some_and(|s| s != prepared.suspension())
            || control.revision != prepared.previous.revision
            || control.maybe_append_identity != Some(prepared.previous)
        {
            return Err(index_corruption("stale BASIC reorg preparation"));
        }
        let mut locks = self.load_basic_filter_append_locks(
            &mut prepared.work,
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        let current = self
            .maybe_basic_filter_owner_guarded_counted(
                &control,
                &mut prepared.work,
                Some(BASIC_INDEX_TURN_WORK_LIMIT),
            )?
            .ok_or_else(|| index_corruption("absent BASIC reorg owner"))?;
        if current != prepared.previous.owner {
            return Err(index_corruption("changed BASIC reorg owner"));
        }
        let view = self.coins_view();
        charge_append_work(
            &mut prepared.work,
            TurnWork {
                checkpoint_operations: 6,
                cloned_bytes: 64,
                ..TurnWork::default()
            },
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        if !open_bitcoin_core::chainstate::CoinsView::head_blocks(&view)
            .map_err(index_corruption)?
            .is_empty()
            || open_bitcoin_core::chainstate::CoinsView::best_block(&view)
                .map_err(index_corruption)?
                != Some(prepared.previous.durable_hash)
        {
            return Err(index_corruption("changed BASIC reorg durable fence"));
        }
        let protection = prepared.protection();
        locks.retain(|lock| lock.name != BASIC_INDEX_PRUNE_LOCK);
        if let Some(lock) = protection.maybe_prune_lock() {
            locks.push(lock);
        }
        let state = codec::StoredFilterState {
            maybe_endpoint: match prepared.next.safe_checkpoint.prefix() {
                IndexPrefix::Empty => None,
                IndexPrefix::Committed(id) => Some((id.height(), id.block_hash())),
            },
            fence_height: prepared.previous.durable_height,
            fence_hash: prepared.previous.durable_hash,
            protection,
        };
        let lock_size = locks.iter().try_fold(4_u64, |size, lock| {
            size.checked_add(lock.name.len() as u64 + 10)
                .ok_or_else(|| index_corruption("BASIC lock-map overflow"))
        })?;
        charge_append_work(
            &mut prepared.work,
            super::ownership::basic_filter_lock_map_cost(locks.len() as u64, lock_size)?,
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        charge_append_work(
            &mut prepared.work,
            TurnWork {
                checkpoint_operations: 4,
                cloned_bytes: lock_size + 256,
                ..TurnWork::default()
            },
            Some(BASIC_INDEX_TURN_WORK_LIMIT),
        )?;
        let state_bytes = codec::encode_state(state);
        let lock_bytes = encode_prune_locks(&locks)?;
        let owner_bytes = codec::ownership::encode_owner(prepared.next.owner.lifecycle());
        let completed = CompletedBasicFilterReorg { prepared };
        let mut batch = self
            .db
            .batch()
            .durability(Some(fjall::PersistMode::SyncAll));
        records_fault(&mut control)?;
        checkpoint_fault(&mut control)?;
        batch.insert(&self.block_index, codec::STATE_KEY, state_bytes);
        batch.insert(&self.block_index, codec::ownership::OWNER_KEY, owner_bytes);
        protection_fault(&mut control)?;
        batch.insert(&self.block_index, PRUNE_LOCKS_KEY, lock_bytes);
        self.finish_basic_filter_batch(batch, &mut control)?;
        control.maybe_append_identity = Some(completed.prepared.next);
        control.maybe_reorg_suspension = None;
        Ok(completed)
    }
}

fn stronger_protection(
    left: IndexInputProtection,
    right: IndexInputProtection,
) -> IndexInputProtection {
    if left.covers(right) { left } else { right }
}

impl BasicFilterAppendProof {
    pub(crate) fn maybe_replacement_target(&self) -> Option<(u32, BlockHash)> {
        self.identity
            .maybe_replacement
            .and_then(|replacement| replacement.maybe_accepted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chainstate::ReorgFixture;
    use open_bitcoin_core::chainstate::{AnchoredBlock, CoinsView};
    use open_bitcoin_core::consensus::{ConsensusParams, ScriptVerifyFlags};

    fn refuse_unbound_taller_fence(
        remove_binding: impl FnOnce(&mut BasicFilterReplacementIdentity),
    ) {
        // Arrange: earn genuine A2 -> B1 authority, then corrupt only its fence binding.
        let mut fixture = ReorgFixture::new("unbound-taller-fence", 3);
        let path = fixture.path().to_owned();
        let (staged, replacement) = fixture.stage(1);
        let proof = fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        let prepared = fixture
            .store
            .prepare_basic_filter_reorg(&proof, &staged)
            .expect("prepare genuine replacement");
        fixture
            .publish(staged, prepared)
            .expect("achieved shorter replacement");
        fixture
            .append_replacement(&replacement)
            .expect("genuine B1 catch-up");
        let current = fixture.manager.chainstate();
        let disconnect = current.active_chain()[1..]
            .iter()
            .rev()
            .map(|position| {
                fixture
                    .store
                    .load_block(position.block_hash)
                    .expect("read")
                    .expect("retained B body")
            })
            .collect::<Vec<_>>();
        let connected = fixture.records[1..]
            .iter()
            .map(|record| AnchoredBlock {
                block: fixture
                    .store
                    .load_block(record.identity().block_hash())
                    .expect("read")
                    .expect("retained A body"),
                chain_work: u128::from(record.identity().height()) + 100,
            })
            .collect::<Vec<_>>();
        let staged = current
            .stage_reorg(
                &disconnect,
                &connected,
                ScriptVerifyFlags::P2SH,
                ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                },
            )
            .expect("genuine return stage");
        let mut proof = fixture
            .store
            .maybe_basic_filter_append_proof()
            .expect("proof")
            .expect("live");
        let actual_fence = proof.durable_tip();
        assert_eq!(actual_fence.0, 2);
        assert_eq!(staged.maybe_old_tip().expect("B1").height, 1);
        remove_binding(
            proof
                .identity
                .maybe_replacement
                .as_mut()
                .expect("achieved replacement"),
        );
        assert!(!proof.durable_displaced());
        // Test-only corruption removes an earned binding; no new capability is minted.
        fixture
            .store
            .filter_publication_guard()
            .expect("guard")
            .maybe_append_identity = Some(proof.identity);
        fixture
            .store
            .check_basic_filter_append_proof(&proof)
            .expect("other authentication remains valid");

        // Act
        let refused = fixture.store.prepare_basic_filter_reorg(&proof, &staged);

        // Assert: a taller checkpoint requires its exact achieved displaced tuple.
        assert!(
            matches!(refused, Err(error) if error.to_string().contains("BASIC displaced endpoint behind progress"))
        );
        assert_eq!(
            fixture
                .store
                .coins_view()
                .best_block()
                .expect("actual coins unchanged"),
            Some(actual_fence.1)
        );
        assert_eq!(
            fixture
                .store
                .maybe_basic_filter_checkpoint()
                .expect("checkpoint unchanged"),
            Some(proof.safe_checkpoint())
        );
        drop(staged);
        drop(proof);
        drop(fixture);
        std::fs::remove_dir_all(path).expect("cleanup");
    }

    #[test]
    fn phase158_storage_reorg_taller_fence_without_displaced_marker_refuses() {
        refuse_unbound_taller_fence(|replacement| replacement.maybe_displaced_fence = None);
    }

    #[test]
    fn phase158_storage_reorg_taller_fence_with_wrong_displaced_height_refuses() {
        refuse_unbound_taller_fence(|replacement| {
            let (height, hash) = replacement.maybe_displaced_fence.expect("earned fence");
            replacement.maybe_displaced_fence = Some((height - 1, hash));
        });
    }

    #[test]
    fn phase158_storage_reorg_taller_fence_with_wrong_displaced_hash_refuses() {
        refuse_unbound_taller_fence(|replacement| {
            let (height, _) = replacement.maybe_displaced_fence.expect("earned fence");
            replacement.maybe_displaced_fence = Some((height, BlockHash::default()));
        });
    }
}

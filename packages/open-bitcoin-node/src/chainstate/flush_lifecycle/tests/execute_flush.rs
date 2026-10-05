// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_core::chainstate::{
    ChainPosition, Chainstate, ChainstateSnapshot, PruneLockInfo, PrunePlan,
};

use super::*;
use crate::chainstate::{ChainstateStore, FjallChainstateStore, ManagedChainstate};
use crate::storage::coins_codec::encode_best_block_value;
use crate::storage::fjall_store::PairedDeleteOutcome;

#[test]
fn execute_flush_none_does_not_write_coins() {
    // Arrange
    let (path, store) = open_temp_store("none-no-write");
    let tip = BlockHash::from_byte_array([7_u8; 32]);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    };
    let mut planted = FjallCoinsView::from_store(&store);
    planted
        .batch_write(dirty_unspent_batch(&[(outpoint, sample_coin())]), Some(tip))
        .expect("plant consistent tip");
    let (mut lifecycle, view, mut cache) = initialize_ready(&store);
    let before = view.best_block().expect("best before");

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::IfNeeded,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("none flush");

    // Assert
    assert!(matches!(execution.decision, FlushDecision::None(_)));
    assert!(!execution.wrote_coins);
    assert_eq!(view.best_block().expect("best after"), before);
    assert_eq!(before, Some(tip));
    remove_dir_if_exists(&path);
}

#[test]
fn execute_flush_aborts_coins_when_undo_save_fails() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();
    let undo_hash = BlockHash::from_byte_array([0x22; 32]);

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut UndoFailingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[(undo_hash, BlockUndo::default())],
            &[],
            &[],
            &[],
        ),
        "undo persist aborts",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::BackendFailure {
                ref message,
                ..
            } if message == "undo persist failed"
        ),
        "expected undo persist failure, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_always_uses_cache_flush_kind_from_decide_flush() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    lifecycle.set_next_write(policy_now());
    let mut cache = dirty_recording_cache();

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("always flush");

    // Assert
    assert!(
        matches!(execution.decision, FlushDecision::Flush(_)),
        "Always must map to cache Flush, got {:?}",
        execution.decision
    );
    assert!(execution.wrote_coins);
    assert_eq!(cache.parent().writes.get(), 1);
}

#[test]
fn execute_flush_before_ready_does_not_write() {
    // Arrange
    let mut lifecycle = FlushLifecycle::not_ready_for_test();
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        ),
        "flush before ready",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "flush before ready"
        ),
        "expected flush before ready, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_refuse_disk_space_does_not_write_coins() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            0,
            &[],
            &[],
            &[],
            &[],
        ),
        "refuse disk space",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "refuse disk space"
        ),
        "expected refuse disk space, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

struct OrderingSink {
    events: Vec<&'static str>,
    coins_writes_seen: Vec<usize>,
    coins_writes: Rc<Cell<usize>>,
}

impl OrderingSink {
    fn record(&mut self, event: &'static str) {
        self.coins_writes_seen.push(self.coins_writes.get());
        self.events.push(event);
    }
}

impl FlushPersistSink for OrderingSink {
    fn load_prune_protection(&self) -> Result<super::super::PruneProtectionSnapshot, StorageError> {
        Ok(super::super::PruneProtectionSnapshot::no_index(Vec::new()))
    }
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        self.record("persist_block");
        Ok(())
    }

    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        self.record("persist_undo");
        Ok(())
    }

    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        self.record("persist_header_entries");
        Ok(())
    }

    fn persist_chain_meta(&mut self, _active_chain: &[ChainPosition]) -> Result<(), StorageError> {
        self.record("persist_chain_meta");
        Ok(())
    }

    fn commit_paired_unlink(
        &mut self,
        _height: u32,
        _block_hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.record("commit_paired_unlink");
        Ok(PairedDeleteOutcome::DeletedLiveMate)
    }
}

#[test]
fn applying_plan_ordering_writes_coins_after_unlink() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let coins_writes = Rc::new(Cell::new(0));
    let mut cache = CoinsCache::from_parent(RecordingCoinsView {
        writes: Rc::clone(&coins_writes),
    });
    cache
        .add_coin(
            OutPoint {
                txid: Txid::from_byte_array([0x11; 32]),
                vout: 0,
            },
            sample_coin(),
            true,
        )
        .expect("dirty overlay");
    let eligible_header = header(BlockHash::from_byte_array([0_u8; 32]), 3);
    let eligible = ChainPosition::new(eligible_header.clone(), 1, 1, 1);
    let tip = ChainPosition::new(header(eligible.block_hash, 4), 400, 400, 1);
    let body = Block {
        header: eligible_header.clone(),
        transactions: Vec::new(),
    };
    let plan = PrunePlan { heights: vec![1] };
    let mut sink = OrderingSink {
        events: Vec::new(),
        coins_writes_seen: Vec::new(),
        coins_writes,
    };
    let mut deleted = Vec::new();

    // Act
    let execution = lifecycle
        .execute_flush_applying_plan(
            &mut sink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[(eligible.block_hash, BlockUndo::default())],
            &[header_entry(eligible_header, 1, 1)],
            &[body],
            &[eligible.clone(), tip],
            &plan,
            &[],
            &mut |hash| deleted.push(hash),
        )
        .expect("applying plan");

    // Assert
    assert_eq!(
        sink.events.as_slice(),
        [
            "persist_block",
            "persist_undo",
            "persist_header_entries",
            "commit_paired_unlink",
            "persist_chain_meta",
        ]
    );
    assert_eq!(sink.coins_writes_seen[3], 0);
    assert_eq!(sink.coins_writes_seen[4], 1);
    assert!(execution.wrote_coins);
    assert_eq!(deleted, vec![eligible.block_hash]);
    assert_eq!(cache.parent().writes.get(), 1);
}

type FjallManaged = ManagedChainstate<FjallChainstateStore>;

fn plant_position(store: &FjallNodeStore, height: u32, nonce: u32) -> ChainPosition {
    let block_header = header(BlockHash::from_byte_array([0_u8; 32]), nonce);
    let position = ChainPosition::new(
        block_header.clone(),
        height,
        u128::from(height),
        1_700_000_000,
    );
    let body = Block {
        header: block_header,
        transactions: Vec::new(),
    };
    let saved = store
        .save_block(&body, PersistMode::Sync)
        .expect("save payload");
    assert_eq!(saved, position.block_hash);
    store
        .save_undo(
            position.block_hash,
            &BlockUndo::default(),
            PersistMode::Sync,
        )
        .expect("save undo");
    position
}

fn plant_coins_best(store: &FjallNodeStore) -> BlockHash {
    let best = BlockHash::from_byte_array([0x44; 32]);
    let value = encode_best_block_value(best);
    FjallCoinsView::from_store(store)
        .write_raw_bytes(&encode_best_block_key(), value)
        .expect("plant coins best");
    best
}

fn install_positions(managed: &mut FjallManaged, positions: Vec<ChainPosition>) {
    let mut undo_by_block = HashMap::new();
    for position in &positions {
        undo_by_block.insert(position.block_hash, BlockUndo::default());
    }
    managed.install_chainstate_for_test(Chainstate::from_snapshot(ChainstateSnapshot::new(
        positions,
        HashMap::new(),
        undo_by_block,
    )));
}

fn managed_from_positions(
    test_name: &str,
    specs: &[(u32, u32)],
) -> (
    std::path::PathBuf,
    FjallManaged,
    Vec<ChainPosition>,
    BlockHash,
) {
    let (path, store) = open_temp_store(test_name);
    let positions: Vec<_> = specs
        .iter()
        .copied()
        .map(|(height, nonce)| plant_position(&store, height, nonce))
        .collect();
    let best = plant_coins_best(&store);
    let mut managed = ManagedChainstate::from_store(FjallChainstateStore::from_store(store));
    install_positions(&mut managed, positions.clone());
    (path, managed, positions, best)
}

fn forbid_height(height: u32) -> PruneLockInfo {
    PruneLockInfo {
        name: "rescan".to_string(),
        height_first: height,
        height_last: height,
    }
}

#[test]
fn empty_plan_on_none_mode_does_not_unlink() {
    // Arrange
    let (path, mut managed, positions, best) = managed_from_positions("empty-plan", &[(1, 11)]);
    let hash = positions[0].block_hash;

    // Act
    let execution = managed
        .flush_with_mode(FlushMode::None, policy_now(), u64::MAX)
        .expect("empty plan");

    // Assert
    assert!(!execution.wrote_coins);
    assert!(execution.deleted_block_hashes.is_empty());
    assert!(managed.store().inner().has_block(hash).expect("payload"));
    assert!(managed.store().inner().has_undo(hash).expect("undo"));
    assert!(!managed.store().inner().load_have_pruned().expect("flag"));
    assert_eq!(managed.store().best_block().expect("best"), Some(best));
    remove_dir_if_exists(&path);
}

#[test]
fn lock_forbidden_height_keeps_both_mates() {
    // Arrange
    let (path, mut managed, positions, best) =
        managed_from_positions("lock-skip", &[(50, 12), (1, 11), (400, 13)]);
    let locked = positions[0].block_hash;
    let eligible = positions[1].block_hash;
    let plan = PrunePlan {
        heights: vec![50, 1],
    };
    let locks = [forbid_height(50)];

    // Act
    let execution = managed
        .flush_applying_plan(FlushMode::Always, policy_now(), u64::MAX, &plan, &locks)
        .expect("partial plan");

    // Assert
    assert!(
        managed
            .store()
            .inner()
            .has_block(locked)
            .expect("locked payload")
    );
    assert!(
        managed
            .store()
            .inner()
            .has_undo(locked)
            .expect("locked undo")
    );
    assert!(
        !managed
            .store()
            .inner()
            .has_block(eligible)
            .expect("eligible payload")
    );
    assert!(
        !managed
            .store()
            .inner()
            .has_undo(eligible)
            .expect("eligible undo")
    );
    assert!(managed.store().inner().load_have_pruned().expect("flag"));
    assert_eq!(execution.deleted_block_hashes, vec![eligible]);
    assert_eq!(managed.store().best_block().expect("best"), Some(best));
    remove_dir_if_exists(&path);
}

#[test]
fn keep_window_tip_deletes_nothing() {
    // Arrange
    let (path, mut managed, positions, _best) = managed_from_positions("keep-window", &[(15, 15)]);
    let hash = positions[0].block_hash;
    let plan = PrunePlan { heights: vec![15] };

    // Act
    let execution = managed
        .flush_applying_plan(FlushMode::None, policy_now(), u64::MAX, &plan, &[])
        .expect("tip plan");

    // Assert
    assert!(!execution.wrote_coins);
    assert!(execution.deleted_block_hashes.is_empty());
    assert!(managed.store().inner().has_block(hash).expect("payload"));
    assert!(managed.store().inner().has_undo(hash).expect("undo"));
    assert!(!managed.store().inner().load_have_pruned().expect("flag"));
    remove_dir_if_exists(&path);
}

#[test]
fn second_flush_does_not_restore_deleted_undo() {
    // Arrange
    let (path, mut managed, positions, best) =
        managed_from_positions("second-flush", &[(1, 21), (400, 22)]);
    let eligible = positions[0].block_hash;
    let plan = PrunePlan { heights: vec![1] };
    managed
        .flush_applying_plan(FlushMode::None, policy_now(), u64::MAX, &plan, &[])
        .expect("first unlink");
    assert!(!managed.chainstate().undo_by_block().contains_key(&eligible));

    // Act
    managed
        .flush_with_mode(FlushMode::Always, policy_now(), u64::MAX)
        .expect("second flush");

    // Assert
    assert!(!managed.store().inner().has_undo(eligible).expect("undo"));
    assert_eq!(managed.store().best_block().expect("best"), Some(best));
    remove_dir_if_exists(&path);
}

#[test]
fn none_mode_nonempty_plan_unlinks_without_writing_coins() {
    // Arrange
    let (path, mut managed, positions, best) =
        managed_from_positions("none-unlink", &[(1, 31), (400, 32)]);
    let eligible = positions[0].block_hash;
    let plan = PrunePlan { heights: vec![1] };

    // Act
    let execution = managed
        .flush_applying_plan(FlushMode::None, policy_now(), u64::MAX, &plan, &[])
        .expect("none unlink");

    // Assert
    assert!(!execution.wrote_coins);
    assert_eq!(execution.deleted_block_hashes, vec![eligible]);
    assert!(
        !managed
            .store()
            .inner()
            .has_block(eligible)
            .expect("payload")
    );
    assert!(!managed.store().inner().has_undo(eligible).expect("undo"));
    assert!(managed.store().inner().load_have_pruned().expect("flag"));
    assert_eq!(managed.store().best_block().expect("best"), Some(best));
    remove_dir_if_exists(&path);
}

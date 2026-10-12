// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Focused actual accepted-history faults, separate from the long retention proof.

use super::*;

type RawRows = Vec<(Vec<u8>, Vec<u8>)>;

pub(super) struct Cleanup {
    pub path: PathBuf,
}

impl Cleanup {
    pub fn new() -> Self {
        Self {
            path: reserved_filter_path("phase162-focused-fault"),
        }
    }
}

impl Drop for Cleanup {
    fn drop(&mut self) {
        // Declared before every owner: unwinding closes all handles first.
        drop(FjallNodeStore::open(&self.path).expect("cleanup all-handle-close reopen"));
        fs::remove_dir_all(&self.path).expect("focused fault datadir removal");
        assert!(!self.path.exists());
    }
}

pub(super) fn flush_only(runtime: &DurableSyncRuntime) {
    let receipt = runtime
        .network_handle()
        .flush_coins(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(500_000),
            u64::MAX,
        )
        .expect("genuine coins/undo/metadata settlement without index turn");
    assert!(receipt.wrote_coins);
}

pub(super) fn extend(fixture: &mut AcceptedRecipe, count: u32) {
    let recipe = recipe::Recipe::new(fixture.recipe.params, fixture.recipe.flags, count);
    assert_eq!(
        fixture.recipe.blocks,
        recipe.blocks[..fixture.receipts.len()]
    );
    for block in &recipe.blocks[fixture.receipts.len()..] {
        fixture
            .runtime
            .store()
            .save_block(block, PersistMode::Sync)
            .expect("ordinary body");
        let position = fixture
            .runtime
            .network_handle()
            .connect_local_block(block, recipe.flags, recipe.params)
            .expect("genuine normal-maturity suffix receipt");
        assert_eq!(position.height as usize, fixture.receipts.len());
        assert_eq!(position.block_hash, block_hash(&block.header));
        fixture.receipts.push(position);
    }
    fixture.recipe = recipe;
}

pub(super) fn history(cleanup: &Cleanup, saved: bool) -> AcceptedRecipe {
    let mut fixture = AcceptedRecipe::at(
        cleanup.path.clone(),
        if saved { 102 } else { 105 },
        if saved {
            BasicFilterStartupMode::Enabled
        } else {
            BasicFilterStartupMode::PreserveSaved
        },
    );
    if saved {
        fixture.finish();
        fixture.flush();
        extend(&mut fixture, 105);
    }
    flush_only(&fixture.runtime);
    assert_authority(fixture.runtime.store(), &fixture.receipts, 104);
    assert_eq!(fixture.recipe.params.coinbase_maturity, 100);
    assert!(fixture.recipe.blocks[101].transactions.len() > 1);
    let undo = fixture
        .runtime
        .store()
        .load_undo(fixture.receipts[102].block_hash)
        .expect("genuine historical undo")
        .expect("durably retained undo");
    let inputs = BasicFilterInputs::from_historical(
        &fixture.recipe.blocks[102],
        &fixture.receipts[102],
        Some(HistoricalBlockUndo {
            block_hash: fixture.receipts[102].block_hash,
            undo: &undo,
        }),
    )
    .expect("actual undo/same-block inputs");
    assert_eq!(
        inputs
            .spent_scripts()
            .map(<[u8]>::to_vec)
            .collect::<Vec<_>>(),
        fixture.recipe.spent_scripts[102]
    );
    assert!(inputs.spent_scripts().count() > 1);
    fixture
}

pub(super) fn assert_authority(store: &FjallNodeStore, positions: &[ChainPosition], height: usize) {
    assert_eq!(
        store.coins_view().best_block().expect("actual coins B"),
        Some(positions[height].block_hash)
    );
    assert_eq!(
        store.load_chain_meta_for_open().expect("actual metadata").0,
        positions[..=height]
    );
}

pub(super) fn snapshot(path: &Path) -> Vec<RawRows> {
    ["block_index", "chainstate", "coins"]
        .into_iter()
        .map(|namespace| {
            let mut rows = Vec::new();
            raw_namespace(path, namespace, |space| {
                for guard in space.iter() {
                    let (key, value) = guard.into_inner().expect("actual namespace row");
                    rows.push((key.to_vec(), value.to_vec()));
                }
            });
            rows
        })
        .collect()
}

pub(super) fn remove_mate(path: &Path, hash: BlockHash, body: bool) {
    let namespace = if body { "block_index" } else { "chainstate" };
    let prefix = if body { "block:" } else { "undo:" };
    let key = codec::record_key(hash).replacen(codec::RECORD_PREFIX, prefix, 1);
    raw_namespace(path, namespace, |space| {
        assert!(space.get(&key).expect("real pre-fault mate").is_some());
        space
            .remove(key)
            .expect("remove exactly one real physical mate");
    });
}

pub(super) fn open(path: &Path) -> Result<DurableSyncRuntime, SyncRuntimeError> {
    DurableSyncRuntime::open_configured(
        FjallNodeStore::open(path).expect("all-handle-close configured reopen"),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
}

pub(super) fn rows(store: &FjallNodeStore, positions: &[ChainPosition]) -> Vec<StoredFilterRecord> {
    positions
        .iter()
        .filter_map(|position| {
            store
                .load_basic_filter_record(position.block_hash)
                .expect("immutable hash lookup")
        })
        .collect()
}

pub(super) fn assert_refusals(path: &Path, category: &str) {
    let before = snapshot(path);
    for _ in 0..2 {
        assert_filter_refusal(open(path), category);
        assert_eq!(
            snapshot(path),
            before,
            "every source/coins/meta/immutable/owner/checkpoint/lock/intent byte survives closed reopen"
        );
    }
}

fn missing_history(saved: bool, body: bool) {
    // Arrange
    let cleanup = Cleanup::new();
    let fixture = history(&cleanup, saved);
    let store = fixture.runtime.store();
    let hash = fixture.receipts[102].block_hash;
    let old_rows = rows(store, &fixture.receipts);
    let old_checkpoint = store
        .maybe_basic_filter_checkpoint()
        .expect("pre-fault checkpoint");
    let old_locks = store.load_prune_locks().expect("pre-fault protection");
    let old_owner = store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("pre-fault owner");
    let retained_body = store
        .load_block(hash)
        .expect("pre-fault body")
        .expect("actual body");
    let retained_undo = store
        .load_undo(hash)
        .expect("pre-fault undo")
        .expect("actual undo");
    assert_eq!(old_rows.len(), if saved { 102 } else { 0 });
    assert_eq!(old_checkpoint, saved.then(|| checkpoint(&old_rows[101])));
    assert_eq!(old_owner.is_some(), saved);
    assert_eq!(
        old_locks,
        if saved {
            vec![
                IndexInputProtection::FromHeight(102)
                    .maybe_prune_lock()
                    .expect("saved conservative range"),
            ]
        } else {
            Vec::new()
        }
    );
    let positions = fixture.receipts.clone();
    drop(fixture);
    remove_mate(&cleanup.path, hash, body);

    // Act
    assert_refusals(
        &cleanup.path,
        &format!(
            "missing BASIC activation {} at height 102",
            if body { "body" } else { "undo" }
        ),
    );

    // Assert
    let store = FjallNodeStore::open(&cleanup.path).expect("post-refusal inspection reopen");
    assert_authority(&store, &positions, 104);
    assert_eq!(
        store.load_block(hash).expect("body"),
        (!body).then_some(retained_body)
    );
    assert_eq!(
        store.load_undo(hash).expect("undo"),
        body.then_some(retained_undo)
    );
    assert_eq!(rows(&store, &positions), old_rows);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("safe checkpoint"),
        old_checkpoint
    );
    assert_eq!(
        store.load_prune_locks().expect("conservative protection"),
        old_locks
    );
    assert_eq!(
        store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner"),
        old_owner
    );
    assert_eq!(
        store.maybe_prune_intent().expect("no invented intent"),
        None
    );
    assert!(
        !store
            .load_have_pruned()
            .expect("missing mate is not pruning")
    );
    drop(store);
    eprintln!(
        "phase162 missing-history: saved={saved} body={body} required=102 coins_height=104 refusals=2 preserved=true cleanup=guarded"
    );
}

#[test]
fn phase162_missing_history_fresh_body_refuses_without_owner() {
    missing_history(false, true);
}

#[test]
fn phase162_missing_history_fresh_undo_refuses_without_owner() {
    missing_history(false, false);
}

#[test]
fn phase162_missing_history_saved_lagged_body_preserves_prefix() {
    missing_history(true, true);
}

#[test]
fn phase162_missing_history_saved_lagged_undo_preserves_prefix() {
    missing_history(true, false);
}

#[test]
fn phase162_missing_history_irrelevant_unflushed_future_mates_do_not_poison() {
    for body in [true, false] {
        // Arrange: accepted future history is deliberately not coins/meta authority.
        let cleanup = Cleanup::new();
        let mut fixture =
            AcceptedRecipe::at(cleanup.path.clone(), 102, BasicFilterStartupMode::Enabled);
        fixture.finish();
        fixture.flush();
        extend(&mut fixture, 105);
        let hash = fixture.receipts[104].block_hash;
        let actual_undo = fixture
            .runtime
            .network_handle()
            .chainstate_snapshot()
            .expect("actual accepted snapshot")
            .undo_by_block[&hash]
            .clone();
        fixture
            .runtime
            .store()
            .save_undo(hash, &actual_undo, PersistMode::Sync)
            .expect("ordinary persistence of genuine future accepted undo only");
        let positions = fixture.receipts.clone();
        let old_rows = rows(fixture.runtime.store(), &positions);
        let old_checkpoint = fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe101");
        let old_locks = fixture
            .runtime
            .store()
            .load_prune_locks()
            .expect("protected102");
        drop(fixture);
        remove_mate(&cleanup.path, hash, body);
        let before = snapshot(&cleanup.path);

        // Act
        for _ in 0..2 {
            let runtime =
                open(&cleanup.path).expect("irrelevant future absence cannot poison prefix");
            // Assert
            assert_authority(runtime.store(), &positions, 101);
            assert_eq!(rows(runtime.store(), &positions), old_rows);
            assert_eq!(
                runtime
                    .store()
                    .maybe_basic_filter_checkpoint()
                    .expect("safe"),
                old_checkpoint
            );
            assert_eq!(
                runtime.store().load_prune_locks().expect("protection"),
                old_locks
            );
            assert_eq!(runtime.store().has_block(hash).expect("body mate"), !body);
            assert_eq!(runtime.store().has_undo(hash).expect("undo mate"), body);
            drop(runtime);
            assert_eq!(snapshot(&cleanup.path), before);
        }
    }
}

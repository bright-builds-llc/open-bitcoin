// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Sparse payloads with dense metadata exercise real manual owners and reopen.

use super::*;
use crate::chainstate::{FjallChainstateStore, FlushPersistSink};
use open_bitcoin_core::chainstate::{FlushMode, FlushPolicyTime, PrunePlan};

fn manual_fixture(name: &str) -> FilterStartupFixture {
    let fixture = FilterStartupFixture::new(name, 1, None);
    raw_index(&fixture.path, |index| {
        index.remove("prune_intent").expect("no pending work");
    });
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    for height in [0, 1, 20] {
        let position = &fixture.positions[height];
        store
            .save_block(
                &fixture_block(position.previous_block_hash(), height as u32),
                PersistMode::Sync,
            )
            .expect("candidate body");
        if height != 0 {
            store
                .save_undo(
                    position.block_hash,
                    &BlockUndo::default(),
                    PersistMode::Sync,
                )
                .expect("candidate undo");
        }
    }
    drop(store);
    fixture
}

#[test]
fn filter_index_prune_snapshot_reloads_validated_owner_in_both_concrete_sinks() {
    // Arrange
    let fixture = manual_fixture("snapshot-concrete");
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    let chainstate = FjallChainstateStore::from_store(store.clone());
    assert!(
        store
            .load_prune_protection()
            .expect("absent")
            .maybe_owner()
            .is_none()
    );
    store
        .initialize_basic_filter_state(&fence(&fixture.positions))
        .expect("active");

    // Act
    let direct = store
        .load_prune_protection()
        .expect("fresh direct snapshot");
    let forwarding = chainstate
        .load_prune_protection()
        .expect("fresh chainstate snapshot");

    // Assert
    assert_eq!(direct, forwarding);
    assert!(direct.protects_height(0));
    assert!(direct.protects_height(1));
    assert_eq!(
        direct
            .maybe_owner()
            .expect("validated owner")
            .maybe_effective_protection(),
        Some(IndexInputProtection::FromHeight(0))
    );
    assert_eq!(
        direct.locks(),
        store.load_prune_locks().expect("durable locks")
    );
    drop(chainstate);
    drop(store);
    fixture.cleanup();
}

fn generated_prefix(fixture: &FilterStartupFixture, endpoint: usize) -> Vec<StoredFilterRecord> {
    let mut records: Vec<StoredFilterRecord> = Vec::new();
    for position in &fixture.positions[..=endpoint] {
        let body = fixture_block(position.previous_block_hash(), position.height);
        let undo = BlockUndo::default();
        let maybe_history = (position.height != 0).then_some(HistoricalBlockUndo {
            block_hash: position.block_hash,
            undo: &undo,
        });
        let inputs = BasicFilterInputs::from_historical(&body, position, maybe_history)
            .expect("actual historical inputs");
        let maybe_previous = records.last().map(StoredFilterRecord::identity);
        records.push(
            StoredFilterRecord::generate(&inputs, position, maybe_previous.as_ref())
                .expect("immutable record"),
        );
    }
    records
}

fn assert_pairs(
    store: &FjallNodeStore,
    fixture: &FilterStartupFixture,
    heights: &[usize],
    present: bool,
) {
    for height in heights {
        let position = &fixture.positions[*height];
        assert_eq!(
            store.load_block(position.block_hash).expect("exact body"),
            present.then(|| fixture_block(position.previous_block_hash(), *height as u32))
        );
        assert_eq!(
            store.load_undo(position.block_hash).expect("exact undo"),
            (present && *height != 0).then(BlockUndo::default)
        );
    }
}

fn manual_apply(
    runtime: &DurableSyncRuntime,
    plan: &PrunePlan,
) -> Result<crate::chainstate::FlushExecution, crate::ManagedNetworkAuthorityError> {
    runtime.network_handle().flush_applying_prune_plan(
        FlushMode::IfNeeded,
        FlushPolicyTime::new(1),
        u64::MAX,
        plan,
        &[],
    )
}

#[test]
fn filter_index_preplanned_manual_owner_retains_inputs_until_genuine_fenced_release() {
    // Arrange
    let fixture = manual_fixture("manual-preplanned");
    let plan = PrunePlan {
        heights: vec![0, 1, 20],
    };
    let runtime = fixture
        .open_runtime()
        .expect("production absent-index reopen");
    runtime
        .store()
        .initialize_basic_filter_state(&fence(&fixture.positions))
        .expect("protection after planning");
    let records = generated_prefix(&fixture, 16);
    append_current(runtime.store(), &records).expect("record-only work");
    let usage = runtime
        .store()
        .retained_payload_usage(&fixture.positions)
        .expect("actual usage");

    // Act
    let stalled = manual_apply(&runtime, &plan).expect("protected candidates skip");

    // Assert
    assert!(stalled.deleted_block_hashes.is_empty());
    assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);
    assert_eq!(
        runtime
            .store()
            .retained_payload_usage(&fixture.positions)
            .expect("retained bytes")
            .current_usage_bytes,
        usage.current_usage_bytes
    );
    assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
    assert!(!runtime.store().load_have_pruned().expect("pruned"));
    assert_eq!(
        runtime
            .store()
            .load_operator_support_counts()
            .expect("support")
            .pruned_height_count,
        0
    );
    drop(runtime);
    let runtime = fixture
        .open_runtime()
        .expect("real protected production reopen");
    assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);

    // Arrange: genuine saved proof, not ahead rows, permits the prefix.
    let cp = checkpoint(&records[16]);
    publish_current(
        runtime.store(),
        &fence(&fixture.positions),
        cp,
        cp.input_protection(),
        &[],
    )
    .expect("fenced durable release");

    // Act
    let released = manual_apply(&runtime, &plan).expect("safe prefix applies");

    // Assert
    assert_eq!(
        released.deleted_block_hashes,
        vec![
            fixture.positions[0].block_hash,
            fixture.positions[1].block_hash
        ]
    );
    assert_pairs(runtime.store(), &fixture, &[0, 1], false);
    assert_pairs(runtime.store(), &fixture, &[20], true);
    assert!(
        runtime
            .store()
            .retained_payload_usage(&fixture.positions)
            .expect("actual remaining bytes")
            .current_usage_bytes
            < usage.current_usage_bytes
    );
    assert_eq!(
        runtime
            .store()
            .maybe_prune_intent()
            .expect("completed intent"),
        None
    );
    assert!(runtime.store().load_have_pruned().expect("earned pruned"));
    let counts = runtime
        .store()
        .load_operator_support_counts()
        .expect("earned support");
    assert_eq!(counts.successful_batch_count, 1);
    assert_eq!(counts.pruned_height_count, 2);
    drop(runtime);
    let runtime = fixture
        .open_runtime()
        .expect("real production reopen after release/deletion");
    assert_pairs(runtime.store(), &fixture, &[0, 1], false);
    assert_pairs(runtime.store(), &fixture, &[20], true);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("saved proof"),
        Some(cp)
    );
    for record in records {
        assert_eq!(
            runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable survives payload deletion"),
            Some(record)
        );
    }
    drop(runtime);
    fixture.cleanup();
}

#[test]
fn filter_index_manual_corrupt_owner_refuses_without_earning_receipts() {
    // Arrange
    let fixture = manual_fixture("manual-corrupt-owner");
    let runtime = fixture.open_runtime().expect("production reopen");
    runtime
        .store()
        .initialize_basic_filter_state(&fence(&fixture.positions))
        .expect("active");
    runtime
        .store()
        .write_raw_for_test(
            StorageNamespace::BlockIndex,
            codec::ownership::OWNER_KEY,
            vec![1],
        )
        .expect("malformed owner fixture");
    let usage = runtime
        .store()
        .retained_payload_usage(&fixture.positions)
        .expect("usage");

    // Act
    let result = manual_apply(
        &runtime,
        &PrunePlan {
            heights: vec![0, 1, 20],
        },
    );

    // Assert
    assert!(result.is_err());
    assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);
    assert_eq!(
        runtime
            .store()
            .retained_payload_usage(&fixture.positions)
            .expect("usage after refusal")
            .current_usage_bytes,
        usage.current_usage_bytes
    );
    assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
    assert!(!runtime.store().load_have_pruned().expect("pruned"));
    assert_eq!(
        runtime
            .store()
            .load_operator_support_counts()
            .expect("support")
            .pruned_height_count,
        0
    );
    drop(runtime);
    assert_filter_refusal(fixture.open_runtime(), "owner envelope length");
    let store = FjallNodeStore::open(&fixture.path).expect("inspect after real refusal");
    assert_pairs(&store, &fixture, &[0, 1, 20], true);
    drop(store);
    fixture.cleanup();
}

#[test]
fn filter_index_manual_fresh_snapshot_refuses_a_live_older_durable_fence() {
    // Arrange
    let fixture = manual_fixture("manual-stale-fence");
    let runtime = fixture.open_runtime().expect("runtime");
    runtime
        .store()
        .initialize_basic_filter_state(&fence(&fixture.positions))
        .expect("active");
    let records = generated_prefix(&fixture, 16);
    let cp = checkpoint(&records[16]);
    publish_current(
        runtime.store(),
        &fence(&fixture.positions),
        cp,
        cp.input_protection(),
        &records,
    )
    .expect("release");
    seed_authority(runtime.store(), &fixture.positions[..=10]);

    // Act
    let result = manual_apply(&runtime, &PrunePlan { heights: vec![1] });

    // Assert
    assert!(result.is_err());
    assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);
    assert_eq!(runtime.store().maybe_prune_intent().expect("intent"), None);
    assert_eq!(
        runtime
            .store()
            .load_operator_support_counts()
            .expect("support")
            .pruned_height_count,
        0
    );
    drop(runtime);
    fixture.cleanup();
}

struct SnapshotRaceSink {
    store: FjallNodeStore,
    snapshot_ready: std::sync::mpsc::Sender<()>,
    authority_changed: std::sync::mpsc::Receiver<()>,
}

impl FlushPersistSink for SnapshotRaceSink {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        FlushPersistSink::persist_block(&mut self.store, block)
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        FlushPersistSink::persist_undo(&mut self.store, hash, undo)
    }
    fn persist_header_entries(&mut self, entries: &[HeaderEntry]) -> Result<(), StorageError> {
        FlushPersistSink::persist_header_entries(&mut self.store, entries)
    }
    fn persist_chain_meta(&mut self, positions: &[ChainPosition]) -> Result<(), StorageError> {
        FlushPersistSink::persist_chain_meta(&mut self.store, positions)
    }
    fn load_prune_protection(
        &self,
    ) -> Result<crate::chainstate::PruneProtectionSnapshot, StorageError> {
        let snapshot = self.store.load_prune_protection()?;
        self.snapshot_ready.send(()).expect("actual snapshot ready");
        self.authority_changed
            .recv()
            .expect("clone published newer authority");
        Ok(snapshot)
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        hash: BlockHash,
    ) -> Result<crate::storage::fjall_store::PairedDeleteOutcome, StorageError> {
        self.store.commit_paired_delete(height, hash)
    }
}

#[test]
fn filter_index_clone_change_after_application_snapshot_reaches_final_concrete_refusal() {
    use crate::chainstate::{FlushLifecycle, default_coins_cache_byte_limit};
    use open_bitcoin_core::chainstate::{CoinsCache, MemoryCoinsView};
    use std::sync::mpsc;
    // Arrange
    let fixture = manual_fixture("manual-snapshot-clone-race");
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    store
        .initialize_basic_filter_state(&fence(&fixture.positions))
        .expect("active");
    let records = generated_prefix(&fixture, 16);
    let cp = checkpoint(&records[16]);
    publish_current(
        &store,
        &fence(&fixture.positions),
        cp,
        cp.input_protection(),
        &records,
    )
    .expect("release");
    let (ready_tx, ready_rx) = mpsc::channel();
    let (changed_tx, changed_rx) = mpsc::channel();
    let mut sink = SnapshotRaceSink {
        store: store.clone(),
        snapshot_ready: ready_tx,
        authority_changed: changed_rx,
    };
    let positions = fixture.positions.clone();
    let mut lifecycle = FlushLifecycle::ready_for_test(
        default_coins_cache_byte_limit(),
        0,
        FlushPolicyTime::new(1000),
        false,
    );
    let worker = std::thread::spawn(move || {
        let mut cache = CoinsCache::from_parent(MemoryCoinsView::default());
        let mut receipts = Vec::new();
        let result = lifecycle.execute_flush_applying_plan(
            &mut sink,
            &mut cache,
            FlushMode::IfNeeded,
            FlushPolicyTime::new(1),
            u64::MAX,
            &[],
            &[],
            &[],
            &positions,
            &PrunePlan { heights: vec![1] },
            &[],
            &mut |hash| receipts.push(hash),
        );
        (result, receipts)
    });
    ready_rx.recv().expect("fresh snapshot captured");
    seed_authority(&store, &fixture.positions[..=10]);
    let usage = store
        .retained_payload_usage(&fixture.positions)
        .expect("before final gate");

    // Act
    changed_tx.send(()).expect("apply previously safe snapshot");
    let (result, receipts) = worker.join().expect("application thread");

    // Assert
    assert!(matches!(
        result,
        Err(StorageError::Corruption {
            action: StorageRecoveryAction::Repair,
            ..
        })
    ));
    assert!(receipts.is_empty());
    assert_pairs(&store, &fixture, &[0, 1, 20], true);
    assert_eq!(
        store
            .payload_usage_revision()
            .expect("no accounting effect"),
        usage.revision
    );
    assert_eq!(store.maybe_prune_intent().expect("no intent"), None);
    assert!(!store.load_have_pruned().expect("no receipt"));
    drop(store);
    let runtime = fixture
        .open_runtime()
        .expect("production reopen genuinely reconciles saved cursor");
    assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("reconciled proof"),
        Some(checkpoint(&records[10]))
    );
    drop(runtime);
    fixture.cleanup();
}

#[test]
fn filter_index_runtime_direct_and_resume_required_genesis_and_height_one_refuse() {
    for height in [0_usize, 1] {
        // Arrange: dense metadata, sparse codec-valid payload deletion fixture.
        let fixture = manual_fixture("runtime-low-input-crossings");
        let runtime = fixture.open_runtime().expect("actual runtime");
        runtime
            .store()
            .initialize_basic_filter_state(&fence(&fixture.positions))
            .expect("Active Empty protection");
        let hash = fixture.positions[height].block_hash;
        let usage = runtime
            .store()
            .retained_payload_usage(&fixture.positions)
            .expect("actual bytes");
        assert_eq!(
            runtime
                .store()
                .load_undo(hash)
                .expect("genesis/no-undo vs height-one/undo"),
            (height == 1).then(BlockUndo::default)
        );

        // Act: current active hash, legal keep-window height and no caller locks.
        let error = runtime
            .store()
            .commit_paired_delete(height as u32, hash)
            .expect_err("required direct input");
        assert!(
            error
                .to_string()
                .contains("prune intent targets required BASIC input")
        );
        let intent = PruneIntent {
            height: height as u32,
            block_hash: hash,
        };
        seed_raw_prune_intent(runtime.store(), intent);
        let error = crate::storage::fjall_store::resume_prune_intent(runtime.store(), &[])
            .expect_err("required recovered intent");

        // Assert: direct protection, not ordinary lock buffering or hash mismatch.
        assert!(
            error
                .to_string()
                .contains("prune intent targets required BASIC input")
        );
        assert_pairs(runtime.store(), &fixture, &[0, 1, 20], true);
        assert_eq!(
            runtime
                .store()
                .retained_payload_usage(&fixture.positions)
                .expect("unchanged bytes")
                .current_usage_bytes,
            usage.current_usage_bytes
        );
        assert_eq!(
            runtime
                .store()
                .maybe_prune_intent()
                .expect("retained intent"),
            Some(intent)
        );
        assert!(
            !runtime
                .store()
                .load_have_pruned()
                .expect("no earned receipt")
        );
        assert_eq!(
            runtime
                .store()
                .load_operator_support_counts()
                .expect("unearned support")
                .pruned_height_count,
            0
        );
        drop(runtime);
        let before = snapshot_index(&fixture.path);
        assert_filter_refusal(
            fixture.open_runtime(),
            "prune intent targets required BASIC input",
        );
        assert_eq!(snapshot_index(&fixture.path), before);
        let store = FjallNodeStore::open(&fixture.path).expect("inspect actual refusal");
        assert_pairs(&store, &fixture, &[0, 1, 20], true);
        assert_eq!(
            store.maybe_prune_intent().expect("intent survives reopen"),
            Some(intent)
        );
        drop(store);
        fixture.cleanup();
    }
}

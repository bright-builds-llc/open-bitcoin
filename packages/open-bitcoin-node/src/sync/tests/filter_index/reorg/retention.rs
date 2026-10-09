// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase158_validated_reorg_displaced_exact_records_survive_real_reopen() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let old = fixture
        .history
        .records
        .iter()
        .map(|record| {
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("snapshot actual old row")
                .expect("indexed displaced identity")
        })
        .collect::<Vec<_>>();
    assert_eq!(old, fixture.history.records);
    let operator = open_bitcoin_core::chainstate::PruneLockInfo {
        name: "phase158-operator".to_string(),
        height_first: 1,
        height_last: 2,
    };
    fixture
        .runtime
        .network
        .replace_prune_lock(operator.clone())
        .expect("ordinary operator lock");
    let branch = fixture.fork(10, 16, 0x66);
    // Act
    fixture.apply(&branch);
    fixture.finish();
    fixture.flush();
    let fixture = fixture.reopen();
    // Assert: complete record equality includes bytes/hash/header/parent.
    branches::assert_active(&fixture, &branch, 10);
    for record in old {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("internal retained hash lookup"),
            Some(record)
        );
    }
    let locks = fixture
        .runtime
        .network
        .list_prune_locks()
        .expect("durable lock map");
    assert!(locks.contains(&operator));
    assert!(
        locks.contains(
            &IndexInputProtection::FromHeight(27)
                .maybe_prune_lock()
                .expect("reserved lock")
        )
    );
    assert!(
        fixture
            .runtime
            .network
            .clear_prune_lock(BASIC_INDEX_PRUNE_LOCK)
            .is_err()
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .list_prune_locks()
            .expect("reserved ownership remains"),
        locks
    );
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_pruned_shared_source_survives_repeated_indexed_branches() {
    // Arrange: enough real validated history to earn the 288-block keep window.
    let fixture = ForkFixture::compact(400, 1, 401);
    let shared = fixture.history.records[20].clone();
    let pruned = fixture
        .runtime
        .network
        .flush_applying_prune_plan(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(50_000),
            u64::MAX,
            &open_bitcoin_core::chainstate::PrunePlan { heights: vec![20] },
            &[],
        )
        .expect("ordinary serialized paired prune");
    assert_eq!(
        pruned.deleted_block_hashes,
        vec![shared.identity().block_hash()]
    );
    assert!(
        fixture
            .runtime
            .store()
            .load_have_pruned()
            .expect("earned prune")
    );
    let branch = fixture.fork(390, 10, 0x67);
    // Act: pruned shared height is never a disconnected consensus input.
    fixture.apply(&branch);
    fixture.finish();
    fixture.flush();
    let original = ForkBranch {
        disconnect: branch
            .connected
            .iter()
            .rev()
            .map(|a| a.block.clone())
            .collect(),
        connected: fixture.history.blocks[391..]
            .iter()
            .enumerate()
            .map(|(offset, block)| AnchoredBlock {
                block: block.clone(),
                chain_work: offset as u128 + 2_000,
            })
            .collect(),
        records: fixture.history.records[391..].to_vec(),
    };
    fixture.apply(&original);
    let reuse = fixture
        .runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary immutable reuse");
    assert!(reuse.reused_records > 0);
    assert_eq!(reuse.generations, 0);
    fixture.finish();
    fixture.flush();
    let fixture = fixture.reopen();
    // Assert: shared rows need no blanket regeneration after actual source prune.
    branches::assert_active(&fixture, &original, 390);
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_block(shared.identity().block_hash())
            .expect("paired body"),
        None
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_undo(shared.identity().block_hash())
            .expect("paired undo"),
        None
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_basic_filter_record(shared.identity().block_hash())
            .expect("immutable shared commitment"),
        Some(shared)
    );
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("displaced B"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

fn required_consensus_loss(undo: bool) {
    // Arrange: indexed rows cannot replace an actual disconnect body or undo.
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 13, 0x68);
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("old state");
    let progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("old owner");
    let locks = fixture
        .runtime
        .store()
        .load_prune_locks()
        .expect("old locks");
    fixture
        .runtime
        .store()
        .delete_basic_input_for_test(before.active_chain[23].block_hash, undo)
        .expect("actual missing required source");
    // Act
    let result = fixture.runtime.network.reorg_to_branch(
        &branch.disconnect,
        &branch.connected,
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
        fixture.runtime.verify_flags,
        fixture.runtime.consensus_params,
    );
    // Assert
    let detail = result
        .expect_err("required consensus source cannot be replaced with filter rows")
        .to_string();
    assert!(
        detail.contains(if undo {
            "required undo"
        } else {
            "required body"
        }),
        "{detail}"
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("unchanged state"),
        before
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("unchanged owner"),
        progress
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_prune_locks()
            .expect("unchanged locks"),
        locks
    );
    for record in &fixture.history.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("unchanged records"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_missing_consensus_body_refuses_despite_indexed_record() {
    required_consensus_loss(false);
}

#[test]
fn phase158_validated_reorg_missing_consensus_undo_refuses_despite_indexed_record() {
    required_consensus_loss(true);
}

#[test]
fn phase158_validated_reorg_continuous_startup_protection_precedes_interrupted_prune() {
    // Arrange: height 20 is eligible by the keep window, but required by BASIC.
    let fixture = ForkFixture::compact(400, 1, 16);
    let intent = PruneIntent {
        height: 20,
        block_hash: fixture.history.records[20].identity().block_hash(),
    };
    assert!(!open_bitcoin_core::chainstate::height_inside_keep_window(
        400, 20
    ));
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe prefix"),
        Some(checkpoint(&fixture.history.records[15]))
    );
    seed_raw_prune_intent(fixture.runtime.store(), intent);
    let ForkFixture {
        history, runtime, ..
    } = fixture;
    drop(runtime);
    let before = snapshot_index(&history.path);
    // Act: actual configured recovery must validate protection before deletion.
    let result = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("actual reopen"),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    );
    // Assert
    assert_filter_refusal(result, "prune intent targets required BASIC input");
    assert_eq!(snapshot_index(&history.path), before);
    let store = FjallNodeStore::open(&history.path).expect("inspect refused real reopen");
    assert_eq!(
        store.load_block(intent.block_hash).expect("preserved body"),
        Some(history.blocks[20].clone())
    );
    assert_eq!(
        store.load_undo(intent.block_hash).expect("preserved undo"),
        Some(history.full.undo_by_block[&intent.block_hash].clone())
    );
    assert_eq!(
        store.maybe_prune_intent().expect("live intent"),
        Some(intent)
    );
    assert_eq!(
        store.load_prune_locks().expect("reserved protection"),
        vec![
            IndexInputProtection::FromHeight(16)
                .maybe_prune_lock()
                .expect("required lock")
        ]
    );
    assert!(!store.load_have_pruned().expect("no fabricated deletion"));
    drop(store);
    history.cleanup();
}

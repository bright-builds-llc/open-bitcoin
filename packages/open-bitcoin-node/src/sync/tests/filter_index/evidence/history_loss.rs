// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Continuous accepted active history and achieved legal paired historical loss.

use super::*;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

#[test]
fn phase157_store_history_fresh_legal_paired_genesis_and_spending_loss_refuse_unchanged() {
    for height in [0, 20] {
        // Arrange: actual stage/commit at every height, not sparse metadata.
        let fixture = TurnHistory::new(16, 384, 1);
        let store = FjallNodeStore::open(&fixture.history.path).expect("fresh");
        fixture.history.persist_payloads(&store);
        store
            .seed_coins_from_snapshot(&fixture.history.full)
            .expect("accepted full authority");
        let runtime =
            DurableSyncRuntime::open(store, sync_config()).expect("ordinary disabled runtime");
        assert_pair(runtime.store(), &fixture.history, height, true);
        assert!(400 - 1 - height >= 288);
        // Act: shipped full checkpoint, keep-window check and paired-delete owner.
        let deletion = manual(&runtime, &[height as u32]);
        // Assert
        assert_eq!(
            deletion.deleted_block_hashes,
            vec![fixture.history.full.active_chain[height].block_hash]
        );
        assert_pair(runtime.store(), &fixture.history, height, false);
        assert!(runtime.store().load_have_pruned().expect("earned deletion"));
        drop(runtime);
        assert_refusal_unchanged(
            &fixture.history.path,
            &format!("missing BASIC activation body at height {height}"),
        );
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_store_history_disabled_prefix_required_legal_paired_suffix_loss_refuses_unchanged() {
    // Arrange
    let fixture = TurnHistory::new(16, 384, 1);
    let store = fixture.seed(16);
    store
        .disable_basic_filter_index()
        .expect("trusted disable releases reserved ownership");
    store
        .sync_prune_locks(&[open_bitcoin_core::chainstate::PruneLockInfo {
            name: "wallet-retained-range".to_owned(),
            height_first: 350,
            height_last: 360,
        }])
        .expect("surviving ordinary lock is part of exact refusal snapshot");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("saved Disabled runtime");
    let old = runtime
        .store()
        .maybe_basic_filter_state()
        .expect("saved prefix");
    // Act
    let deletion = manual(&runtime, &[20]);
    // Assert
    assert_eq!(
        deletion.deleted_block_hashes,
        vec![fixture.history.full.active_chain[20].block_hash]
    );
    assert_pair(runtime.store(), &fixture.history, 20, false);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_state()
            .expect("no erasure"),
        old
    );
    drop(runtime);
    assert_refusal_unchanged(
        &fixture.history.path,
        "missing BASIC activation body at height 20",
    );
    fixture.history.cleanup();
}

#[test]
fn phase157_store_history_independent_body_and_undo_missing_mates_preserve_live_intent() {
    for body in [true, false] {
        // Arrange: faults remove exactly one mate; paired deletion is proven separately.
        let fixture = TurnHistory::new(16, 384, 1);
        let store = fixture.seed(16);
        let intent = PruneIntent {
            height: 1,
            block_hash: fixture.history.full.active_chain[1].block_hash,
        };
        seed_raw_prune_intent(&store, intent);
        drop(store);
        let hash = fixture.history.full.active_chain[20].block_hash;
        remove_one_mate(&fixture.history.path, hash, body);
        // Act / Assert: history diagnosis precedes any effect on the legal indexed intent.
        assert_refusal_unchanged(
            &fixture.history.path,
            &format!(
                "missing BASIC activation {} at height 20",
                if body { "body" } else { "undo" }
            ),
        );
        let store = FjallNodeStore::open(&fixture.history.path).expect("inspection reopen");
        assert_eq!(
            store.maybe_prune_intent().expect("intent preserved"),
            Some(intent)
        );
        assert_eq!(store.has_block(hash).expect("body mate"), !body);
        assert_eq!(store.has_undo(hash).expect("undo mate"), body);
        assert_pair(&store, &fixture.history, 1, true);
        drop(store);
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_store_history_saved_active_ahead_rows_do_not_skip_required_suffix() {
    // Arrange: ahead rows exist but checkpoint remains at height 15.
    let fixture = TurnHistory::new(16, 384, 1);
    let store = fixture.seed(16);
    append_current(&store, &fixture.history.records[16..24]).expect("immutable ahead rows");
    drop(store);
    remove_one_mate(
        &fixture.history.path,
        fixture.history.full.active_chain[20].block_hash,
        false,
    );
    // Act / Assert
    assert_refusal_unchanged(
        &fixture.history.path,
        "missing BASIC activation undo at height 20",
    );
    fixture.history.cleanup();
}

#[test]
fn phase157_store_history_reconciled_old_branch_refuses_before_saved_authority_publication() {
    // Arrange: a deliberate durable replacement seed, not runtime-reorg evidence.
    let fixture = TurnHistory::new(16, 384, 1);
    let store = fixture.seed(16);
    let replacement = ValidatedHistory::new("evidence-replacement-seed", true);
    replacement.persist_payloads(&store);
    store
        .seed_coins_from_snapshot(&replacement.full)
        .expect("validated replacement B/metadata seed");
    drop(store);
    remove_one_mate(
        &fixture.history.path,
        replacement.full.active_chain[2].block_hash,
        true,
    );
    // Act / Assert: recovery would rewind old height 15 to common height 1.
    assert_refusal_unchanged(
        &fixture.history.path,
        "missing BASIC activation body at height 2",
    );
    let store = FjallNodeStore::open(&fixture.history.path).expect("inspect old authority");
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("old saved checkpoint"),
        Some(checkpoint(&fixture.history.records[15]))
    );
    drop(store);
    fixture.history.cleanup();
}

#[test]
fn phase157_store_history_indexed_legal_paired_loss_and_interrupted_disable_resume_suffix_only() {
    // Arrange
    let fixture = TurnHistory::new(16, 384, 1);
    let store = fixture.seed(16);
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("saved Active runtime");
    let deletion = manual(&runtime, &[1]);
    assert_eq!(
        deletion.deleted_block_hashes,
        vec![fixture.history.full.active_chain[1].block_hash]
    );
    assert_pair(runtime.store(), &fixture.history, 1, false);
    // Honest software fault seed: interruption after Disabled publication leaves a stronger lock.
    seed_raw_basic_protection(runtime.store(), IndexInputProtection::FromHeight(0));
    runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::AfterDisable);
    assert!(runtime.network.disable_basic_filter_index().is_err());
    drop(runtime);
    let before = snapshot_store(&fixture.history.path);
    // Act
    let runtime = configured(&fixture.history.path).expect("indexed source never required again");
    // Assert
    assert_pair(runtime.store(), &fixture.history, 1, false);
    assert_eq!(
        runtime
            .store()
            .load_prune_locks()
            .expect("strong covering protection"),
        vec![
            IndexInputProtection::FromHeight(0)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_lifecycle_for_test()
            .expect("generation"),
        Some(IndexLifecycle::Active {
            generation: IndexGeneration::new(2)
        })
    );
    finish(&runtime);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("earned safe suffix"),
        Some(checkpoint(fixture.history.records.last().expect("tip")))
    );
    assert_pair(runtime.store(), &fixture.history, 1, false);
    drop(runtime);
    let after = snapshot_store(&fixture.history.path);
    assert_eq!(
        before[1..],
        after[1..],
        "surviving undo/source metadata/coins bytes unchanged by indexing"
    );
    let runtime = configured(&fixture.history.path).expect("genuine completed reopen");
    assert_pair(runtime.store(), &fixture.history, 1, false);
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_store_history_genesis_no_undo_special_case_can_complete() {
    // Arrange
    let history = ValidatedHistory::new("evidence-genesis-no-undo", false);
    let store = FjallNodeStore::open(&history.path).expect("fresh");
    history.persist_payloads(&store);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("validated authority");
    drop(store);
    remove_one_mate(
        &history.path,
        history.full.active_chain[0].block_hash,
        false,
    );
    // Act
    let runtime = configured(&history.path).expect("genesis needs body only");
    // Assert
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe"),
        Some(checkpoint(&history.records[2]))
    );
    for record in &history.records {
        assert_eq!(
            runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("exact filter"),
            Some(record.clone())
        );
    }
    drop(runtime);
    history.cleanup();
}

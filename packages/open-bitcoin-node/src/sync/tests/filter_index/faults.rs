// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::{recovery::ValidatedHistory, *};
use crate::{chainstate::initialize, storage::fjall_store::filters::FilterPublicationFault};
use open_bitcoin_core::chainstate::{CoinsView, FlushMode, FlushPolicyTime};

#[test]
fn filter_index_production_after_record_commit_error_retains_ahead_rows_without_cursor() {
    // Arrange
    let history = ValidatedHistory::new("filter-validated-record-commit-error", false);
    let store = history.seed(1);
    store.set_basic_filter_fault(FilterPublicationFault::AfterCommit);
    // Act
    let result = store.persist_basic_filter_records(&history.records[2..]);
    assert!(
        result.is_err(),
        "a post-commit error is not proof of rollback"
    );
    drop(store);
    let runtime = history
        .reopen()
        .expect("production runtime after actual record commit error");
    // Assert
    let store = runtime.store();
    history.assert_rows_and_payloads(store);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("unchanged cursor"),
        Some(checkpoint(&history.records[1]))
    );
    assert_eq!(
        store.coins_view().best_block().expect("unchanged coins B"),
        Some(history.old.active_chain[1].block_hash)
    );
    assert_eq!(
        store
            .maybe_active_basic_filter_record(2)
            .expect("no phantom visibility"),
        None
    );
    assert_eq!(
        store.load_prune_locks().expect("unchanged protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    drop(runtime);
    history.cleanup();
}

#[test]
fn filter_index_production_validated_partial_coins_replay_precedes_guard() {
    for compatible_metadata in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("filter-validated-H-B", false);
        let store = history.seed(1);
        store
            .persist_basic_filter_records(&history.records[2..])
            .expect("ahead immutable rows");
        let mut view = store.coins_view();
        view.set_simulate_crash_after_partial(true);
        let new = history.full.active_chain[2].block_hash;
        let result = view.batch_write_with_limit(history.spend_batch(), Some(new), 1);
        assert!(
            result.is_err(),
            "real coins batch must stop after actual partial commit"
        );
        assert_eq!(view.partial_buffered_commits(), 1);
        assert_eq!(
            view.head_blocks().expect("actual H"),
            vec![new, history.old.active_chain[1].block_hash]
        );
        if compatible_metadata {
            store
                .save_chain_meta(&history.full.active_chain, PersistMode::Sync)
                .expect("compatible durable metadata");
        }
        let intent = PruneIntent {
            height: 2,
            block_hash: new,
        };
        if !compatible_metadata {
            store.sync_prune_intent(intent).expect("unsafe live intent");
        }
        drop(view);
        drop(store);
        let before = snapshot_index(&history.path);
        // Act
        let result = history.reopen();
        // Assert
        if compatible_metadata {
            let runtime = result.expect("replayed compatible authority");
            let store = runtime.store();
            assert!(
                store
                    .coins_view()
                    .head_blocks()
                    .expect("consumed H")
                    .is_empty()
            );
            assert_eq!(
                store.coins_view().best_block().expect("replayed B"),
                Some(new)
            );
            assert_eq!(
                store
                    .coins_view()
                    .collect_unspent_hint()
                    .expect("replayed actual coins"),
                history.full.utxos
            );
            assert_eq!(
                store.maybe_basic_filter_checkpoint().expect("old cursor"),
                Some(checkpoint(&history.records[1]))
            );
            assert_eq!(
                store
                    .maybe_active_basic_filter_record(2)
                    .expect("no phantom projection"),
                None
            );
            history.assert_rows_and_payloads(store);
            drop(runtime);
        } else {
            assert_filter_refusal(result, "coins best block differs from durable metadata");
            assert_eq!(snapshot_index(&history.path), before);
            let store = FjallNodeStore::open(&history.path).expect("inspect refused startup");
            assert!(
                store
                    .coins_view()
                    .head_blocks()
                    .expect("H already consumed")
                    .is_empty()
            );
            assert_eq!(
                store.coins_view().best_block().expect("advanced B"),
                Some(new)
            );
            assert_eq!(
                store.maybe_prune_intent().expect("retained intent"),
                Some(intent)
            );
            history.assert_rows_and_payloads(&store);
            drop(store);
        }
        history.cleanup();
    }
}

#[test]
fn filter_index_production_failed_metadata_flush_keeps_rows_and_refuses_cursor() {
    // Arrange
    let history = ValidatedHistory::new("filter-validated-meta-fault", false);
    let mut store = history.seed(1);
    store
        .persist_basic_filter_records(&history.records[2..])
        .expect("ahead immutable rows");
    let now = FlushPolicyTime::from_unix_seconds(1);
    let (mut lifecycle, view, mut cache) =
        initialize(&store, now, now, 0, false, u64::MAX).expect("ready old checkpoint");
    let intent = PruneIntent {
        height: 2,
        block_hash: history.full.active_chain[2].block_hash,
    };
    store
        .sync_prune_intent(intent)
        .expect("unsafe live intent before flush");
    cache.absorb_batch_write(
        history.spend_batch(),
        Some(history.full.active_chain[2].block_hash),
    );
    store.set_basic_filter_fault(FilterPublicationFault::BeforeChainMeta);
    // Act
    let result = lifecycle.execute_flush(
        &mut store,
        &mut cache,
        FlushMode::Always,
        now,
        u64::MAX,
        &[],
        &[],
        &[],
        &history.full.active_chain,
    );
    // Assert
    assert!(
        result
            .expect_err("actual metadata adapter failure")
            .to_string()
            .contains("injected BASIC publication failure")
    );
    assert_eq!(
        store.coins_view().best_block().expect("successful coins B"),
        Some(history.full.active_chain[2].block_hash)
    );
    assert_eq!(
        store.load_chain_meta_for_open().expect("old metadata").0,
        history.old.active_chain
    );
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("no phantom cursor"),
        Some(checkpoint(&history.records[1]))
    );
    drop(cache);
    drop(view);
    drop(store);
    let before = snapshot_index(&history.path);
    let result = history.reopen();
    assert_filter_refusal(result, "coins best block differs from durable metadata");
    assert_eq!(snapshot_index(&history.path), before);
    let store = FjallNodeStore::open(&history.path).expect("inspect real reopen");
    history.assert_rows_and_payloads(&store);
    assert_eq!(
        store.maybe_prune_intent().expect("retained intent"),
        Some(intent)
    );
    assert_eq!(
        store.load_prune_locks().expect("unchanged protection"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    drop(store);
    history.cleanup();
}

#[test]
fn filter_index_production_reopens_every_record_and_checkpoint_fault_boundary() {
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let history = ValidatedHistory::new("filter-validated-publication-fault", false);
        let store = history.seed(1);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("current validated authority");
        store.set_basic_filter_fault(fault);
        let cp = checkpoint(&history.records[2]);
        // Act
        let result = if fault == FilterPublicationFault::BeforeRecords {
            store.persist_basic_filter_records(&history.records[2..])
        } else {
            store.publish_basic_filter_checkpoint(
                &fence(&history.full.active_chain),
                cp,
                cp.input_protection(),
                &history.records[2..],
            )
        };
        assert!(result.is_err());
        drop(store);
        let runtime = history
            .reopen()
            .expect("production runtime after actual publication fault");
        // Assert
        let endpoint = if fault == FilterPublicationFault::AfterCommit {
            2
        } else {
            1
        };
        let store = runtime.store();
        assert_eq!(
            store
                .maybe_basic_filter_checkpoint()
                .expect("atomic cursor"),
            Some(checkpoint(&history.records[endpoint]))
        );
        assert_eq!(
            store
                .maybe_active_basic_filter_record(2)
                .expect("atomic visibility")
                .is_some(),
            endpoint == 2
        );
        assert_eq!(
            store
                .load_basic_filter_record(history.records[2].identity().block_hash())
                .expect("atomic immutable row")
                .is_some(),
            endpoint == 2
        );
        assert_eq!(
            store.load_prune_locks().expect("atomic protection"),
            vec![
                IndexInputProtection::FromHeight(endpoint as u32 + 1)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        assert_eq!(store.maybe_prune_intent().expect("intent"), None);
        assert_eq!(
            store
                .load_undo(history.full.active_chain[2].block_hash)
                .expect("undo"),
            Some(history.full.undo_by_block[&history.full.active_chain[2].block_hash].clone())
        );
        assert_eq!(
            store
                .load_block(history.full.active_chain[2].block_hash)
                .expect("body"),
            Some(history.blocks[2].clone())
        );
        drop(runtime);
        history.cleanup();
    }
}

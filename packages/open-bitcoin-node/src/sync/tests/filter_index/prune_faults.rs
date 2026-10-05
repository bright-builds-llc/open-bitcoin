// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Runtime consumers over a three-block engine-accepted spend history (maturity 1).
//! Faults inject software commit/reply boundaries, not hardware power loss.

use super::{recovery::ValidatedHistory, *};
use crate::storage::fjall_store::filters::FilterPublicationFault;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

fn history(name: &str) -> ValidatedHistory {
    let mut history = ValidatedHistory::new(name, false);
    history.path = reserved_filter_path(name);
    history
}

fn owner(store: &FjallNodeStore) -> IndexLifecycle {
    store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner read")
        .expect("durable owner")
}

pub(super) fn assert_unearned(store: &FjallNodeStore) {
    assert!(!store.load_have_pruned().expect("have-pruned"));
    assert_eq!(store.maybe_prune_intent().expect("intent"), None);
    let counts = store.load_operator_support_counts().expect("counts");
    assert_eq!(counts.successful_batch_count, 0);
    assert_eq!(counts.pruned_height_count, 0);
    assert_eq!(counts.maybe_last_prune_height, None);
}

#[test]
fn filter_index_runtime_constructor_materializes_legacy_nonempty_prefix_durably() {
    // Arrange
    let history = history("runtime-legacy-prefix");
    let store = history.seed(1);
    let saved = store.maybe_basic_filter_state().expect("state");
    drop(store);
    raw_index(&history.path, |index| {
        index
            .remove(codec::ownership::OWNER_KEY)
            .expect("legacy owner absence");
    });

    // Act: actual constructor consumes initialize's startup guard.
    let runtime = DurableSyncRuntime::open(
        FjallNodeStore::open(&history.path).expect("closed-store reopen"),
        sync_config(),
    )
    .expect("compatible legacy runtime");

    // Assert
    assert_eq!(
        owner(runtime.store()),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_state()
            .expect("unchanged prefix/fence"),
        saved
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("nonempty prefix"),
        Some(checkpoint(&history.records[1]))
    );
    for record in &history.records[..=1] {
        assert_eq!(
            runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable row"),
            Some(record.clone())
        );
    }
    assert_unearned(runtime.store());
    drop(runtime);
    let rows = snapshot_index(&history.path);
    let runtime = history.reopen().expect("second actual constructor reopen");
    assert_eq!(
        owner(runtime.store()),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_state()
            .expect("saved prefix"),
        saved
    );
    assert_unearned(runtime.store());
    drop(runtime);
    assert_eq!(snapshot_index(&history.path), rows);
    history.cleanup();
}

#[test]
fn filter_index_runtime_enable_faults_reopen_only_committed_ownership() {
    for fault in [
        FilterPublicationFault::BeforeEnable,
        FilterPublicationFault::AfterEnable,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let history = history("runtime-enable-fault");
        let store = history.seed(1);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("current engine-accepted authority");
        append_current(&store, &history.records[2..]).expect("retained immutable suffix");
        drop(store);
        let runtime = history.reopen().expect("runtime");
        runtime
            .network_handle()
            .disable_basic_filter_index()
            .expect("disabled");
        let usage = runtime
            .store()
            .retained_payload_usage(&history.full.active_chain)
            .expect("actual bytes");
        let saved = runtime
            .store()
            .maybe_basic_filter_state()
            .expect("saved prefix");
        runtime.store().set_basic_filter_fault(fault);

        // Act
        assert!(
            runtime
                .network_handle()
                .enable_basic_filter_index()
                .is_err()
        );

        // Assert: a poisoned live instance cannot issue work or destructively apply.
        assert!(
            runtime
                .store()
                .maybe_basic_filter_work(&fence(&history.full.active_chain))
                .is_err()
        );
        assert!(
            runtime
                .store()
                .commit_paired_delete(1, history.full.active_chain[1].block_hash)
                .is_err()
        );
        assert_eq!(
            runtime
                .store()
                .retained_payload_usage(&history.full.active_chain)
                .expect("retained bytes")
                .current_usage_bytes,
            usage.current_usage_bytes
        );
        assert_unearned(runtime.store());
        drop(runtime);
        let runtime = history.reopen().expect("truthful production reopen");
        let active = fault != FilterPublicationFault::BeforeEnable;
        assert_eq!(
            owner(runtime.store()),
            if active {
                IndexLifecycle::Active {
                    generation: IndexGeneration::new(2),
                }
            } else {
                IndexLifecycle::Disabled {
                    generation: IndexGeneration::new(1),
                }
            }
        );
        let expected_state = saved.map(|mut state| {
            if active {
                state.fence_height = 2;
                state.fence_hash = history.full.active_chain[2].block_hash;
            }
            state
        });
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_state()
                .expect("checkpoint/fence"),
            expected_state
        );
        assert_eq!(
            runtime
                .store()
                .load_prune_locks()
                .expect("committed covering protection"),
            if active {
                vec![
                    IndexInputProtection::FromHeight(2)
                        .maybe_prune_lock()
                        .expect("lock"),
                ]
            } else {
                vec![]
            }
        );
        history.assert_rows_and_payloads(runtime.store());
        assert_unearned(runtime.store());
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn filter_index_runtime_generation_exhaustion_refuses_without_durable_mutation() {
    for disabled in [false, true] {
        // Arrange
        let history = history("runtime-generation-max");
        let store = history.seed(1);
        if disabled {
            store
                .disable_basic_filter_index()
                .expect("disabled fixture");
        }
        drop(store);
        let lifecycle = if disabled {
            IndexLifecycle::Disabled {
                generation: IndexGeneration::new(u64::MAX),
            }
        } else {
            IndexLifecycle::Active {
                generation: IndexGeneration::new(u64::MAX),
            }
        };
        raw_index(&history.path, |index| {
            index
                .insert(
                    codec::ownership::OWNER_KEY,
                    codec::ownership::encode_owner(lifecycle),
                )
                .expect("max generation recovery fixture");
        });
        let before = snapshot_index(&history.path);
        let runtime = history.reopen().expect("max generation can reopen");

        // Act
        let result = if disabled {
            runtime.network_handle().enable_basic_filter_index()
        } else {
            runtime.network_handle().disable_basic_filter_index()
        };

        // Assert
        assert!(
            result
                .expect_err("checked transition")
                .to_string()
                .contains("generation exhausted")
        );
        assert_eq!(owner(runtime.store()), lifecycle);
        assert_unearned(runtime.store());
        assert_eq!(
            runtime
                .store()
                .load_block(history.full.active_chain[2].block_hash)
                .expect("body"),
            Some(history.blocks[2].clone())
        );
        assert_eq!(
            runtime
                .store()
                .load_undo(history.full.active_chain[2].block_hash)
                .expect("undo"),
            Some(history.full.undo_by_block[&history.full.active_chain[2].block_hash].clone())
        );
        drop(runtime);
        assert_eq!(snapshot_index(&history.path), before);
        let runtime = history.reopen().expect("unchanged max generation reopens");
        assert_eq!(owner(runtime.store()), lifecycle);
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn filter_index_runtime_same_generation_stale_frontier_rejects_both_writers() {
    // Arrange
    let history = history("runtime-stale-frontier");
    let store = history.seed(1);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("full coins fence");
    drop(store);
    let runtime = history.reopen().expect("runtime");
    let work = runtime
        .store()
        .maybe_basic_filter_work(&fence(&history.full.active_chain))
        .expect("work")
        .expect("active");
    let cp = checkpoint(&history.records[2]);
    publish_current(
        runtime.store(),
        &fence(&history.full.active_chain),
        cp,
        cp.input_protection(),
        &history.records[2..],
    )
    .expect("current frontier");
    let saved = runtime
        .store()
        .maybe_basic_filter_state()
        .expect("new saved frontier");

    // Act
    assert!(
        runtime
            .store()
            .persist_basic_filter_records(
                &work,
                &fence(&history.full.active_chain),
                &history.records[2..]
            )
            .is_err()
    );
    assert!(
        runtime
            .store()
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&history.full.active_chain),
                checkpoint(&history.records[1]),
                IndexInputProtection::FromHeight(2),
                &[]
            )
            .is_err()
    );

    // Assert
    assert_eq!(
        owner(runtime.store()),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_state()
            .expect("no rewind"),
        saved
    );
    history.assert_rows_and_payloads(runtime.store());
    assert_unearned(runtime.store());
    drop(work);
    drop(runtime);
    let before = snapshot_index(&history.path);
    let runtime = history.reopen().expect("current committed frontier");
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_state()
            .expect("durable frontier"),
        saved
    );
    history.assert_rows_and_payloads(runtime.store());
    drop(runtime);
    assert_eq!(snapshot_index(&history.path), before);
    history.cleanup();
}

#[test]
fn filter_index_runtime_foreign_reopen_incarnation_refuses_both_writers() {
    // Arrange
    let history = history("runtime-foreign-token");
    let store = history.seed(1);
    let work = store
        .maybe_basic_filter_work(&fence(&history.old.active_chain))
        .expect("work")
        .expect("Active");
    drop(store); // Token retains only publication identity, not the database.
    let before = snapshot_index(&history.path);
    let runtime = history.reopen().expect("new incarnation");

    // Act
    assert!(
        runtime
            .store()
            .persist_basic_filter_records(
                &work,
                &fence(&history.old.active_chain),
                &history.records[2..]
            )
            .is_err()
    );
    assert!(
        runtime
            .store()
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&history.old.active_chain),
                checkpoint(&history.records[1]),
                IndexInputProtection::FromHeight(2),
                &[]
            )
            .is_err()
    );

    // Assert
    assert_unearned(runtime.store());
    assert_eq!(
        runtime
            .store()
            .load_basic_filter_record(history.records[2].identity().block_hash())
            .expect("no foreign append"),
        None
    );
    drop(work);
    drop(runtime);
    assert_eq!(snapshot_index(&history.path), before);
    history.cleanup();
}

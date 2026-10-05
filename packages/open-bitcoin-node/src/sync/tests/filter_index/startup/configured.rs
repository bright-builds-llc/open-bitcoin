// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::super::recovery::ValidatedHistory;
use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::storage::fjall_store::filters::FilterPublicationFault;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

mod faults;

fn remove_historical_payload(path: &Path, hash: BlockHash, body: bool) {
    let hex: String = hash
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let namespace = if body { "block_index" } else { "chainstate" };
    let key = format!("{}:{hex}", if body { "block" } else { "undo" });
    raw_namespace(path, namespace, |space| {
        space.remove(key).expect("missing historical input fixture");
    });
}

fn open_configured(
    path: &Path,
    mode: BasicFilterStartupMode,
) -> Result<DurableSyncRuntime, SyncRuntimeError> {
    DurableSyncRuntime::open_configured(
        FjallNodeStore::open(path).expect("configured real reopen"),
        sync_config(),
        mode,
    )
}

fn snapshot_history(path: &Path) -> Vec<(Vec<u8>, Vec<u8>)> {
    let mut rows = Vec::new();
    raw_namespace(path, "chainstate", |space| {
        for guard in space.iter() {
            let (key, value) = guard.into_inner().expect("history row");
            rows.push((key.to_vec(), value.to_vec()));
        }
    });
    rows
}

/// Codec-valid height-400 history makes the intent legal under the keep window.
/// Genuine consensus spend/undo provenance is tested separately above.
fn retain_required_suffix(fixture: &FilterStartupFixture) {
    let store = FjallNodeStore::open(&fixture.path).expect("retain sparse suffix");
    for position in &fixture.positions {
        let block = fixture_block(position.previous_block_hash(), position.height);
        store
            .save_block(&block, PersistMode::Buffered)
            .expect("retained body");
        if position.height != 0 {
            store
                .save_undo(
                    position.block_hash,
                    &BlockUndo::default(),
                    PersistMode::Buffered,
                )
                .expect("retained coinbase undo");
        }
    }
    store
        .save_chain_meta(&fixture.positions, PersistMode::Sync)
        .expect("durable suffix");
}

#[test]
fn phase157_activation_configured_missing_suffix_preserves_each_saved_mode_and_intent() {
    for saved_mode in ["fresh", "active", "disabled"] {
        for body in [true, false] {
            // Arrange
            let history = ValidatedHistory::new("phase157-configured-missing", false);
            let store = if saved_mode == "fresh" {
                let store = FjallNodeStore::open(&history.path).expect("fresh");
                history.persist_payloads(&store);
                store
            } else {
                history.seed(1)
            };
            store
                .seed_coins_from_snapshot(&history.full)
                .expect("full fence");
            if saved_mode == "disabled" {
                store.disable_basic_filter_index().expect("disable");
            }
            let intent = PruneIntent {
                height: 1,
                block_hash: history.old.active_chain[1].block_hash,
            };
            seed_raw_prune_intent(&store, intent);
            drop(store);
            remove_historical_payload(&history.path, history.full.active_chain[2].block_hash, body);
            let before = snapshot_index(&history.path);
            let history_before = snapshot_history(&history.path);

            // Act
            let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

            // Assert
            assert_filter_refusal(
                result,
                if body {
                    "missing BASIC activation body at height 2"
                } else {
                    "missing BASIC activation undo at height 2"
                },
            );
            assert_eq!(
                snapshot_index(&history.path),
                before,
                "{saved_mode}: all index/body bytes"
            );
            assert_eq!(
                snapshot_history(&history.path),
                history_before,
                "{saved_mode}: surviving undo/metadata"
            );
            let store = FjallNodeStore::open(&history.path).expect("inspect unchanged refusal");
            assert_eq!(store.maybe_prune_intent().expect("intent"), Some(intent));
            assert_eq!(
                store
                    .load_block(intent.block_hash)
                    .expect("surviving intent body"),
                Some(history.blocks[1].clone())
            );
            assert_eq!(
                store
                    .load_undo(intent.block_hash)
                    .expect("surviving intent undo"),
                Some(history.full.undo_by_block[&intent.block_hash].clone())
            );
            drop(store);
            history.cleanup();
        }
    }
}

#[test]
fn phase157_activation_configured_empty_store_refuses_validated_genesis_requirement() {
    // Arrange
    let path = reserved_filter_path("phase157-empty-genesis");
    drop(FjallNodeStore::open(&path).expect("empty store"));
    let before = snapshot_index(&path);
    let history_before = snapshot_history(&path);

    // Act
    let result = open_configured(&path, BasicFilterStartupMode::Enabled);

    // Assert
    assert_filter_refusal(result, "validated genesis history required");
    assert_eq!(snapshot_index(&path), before);
    assert_eq!(snapshot_history(&path), history_before);
    fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase157_activation_configured_rewind_missing_body_refuses_before_reconciliation() {
    // Arrange
    let history = ValidatedHistory::new("phase157-rewind-missing", false);
    let replacement = ValidatedHistory::new("phase157-rewind-alternate", true);
    let store = history.seed(1);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("original full fence");
    publish_current(
        &store,
        &fence(&history.full.active_chain),
        checkpoint(&history.records[2]),
        checkpoint(&history.records[2]).input_protection(),
        &history.records[2..],
    )
    .expect("saved endpoint");
    replacement.persist_payloads(&store);
    store
        .seed_coins_from_snapshot(&replacement.full)
        .expect("recovered replacement fence");
    drop(store);
    remove_historical_payload(
        &history.path,
        replacement.full.active_chain[2].block_hash,
        true,
    );
    let before = snapshot_index(&history.path);
    let history_before = snapshot_history(&history.path);

    // Act
    let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

    // Assert
    assert_filter_refusal(result, "missing BASIC activation body at height 2");
    assert_eq!(snapshot_index(&history.path), before);
    assert_eq!(snapshot_history(&history.path), history_before);
    history.cleanup();
}

#[test]
fn phase157_activation_configured_safe_indexed_intent_resumes_after_preflight() {
    // Arrange
    let fixture = FilterStartupFixture::new("phase157-safe-intent", 1, Some(16));
    retain_required_suffix(&fixture);

    // Act
    let runtime =
        open_configured(&fixture.path, BasicFilterStartupMode::Enabled).expect("safe intent");

    // Assert
    assert_eq!(
        runtime.store.maybe_prune_intent().expect("resumed intent"),
        None
    );
    assert_payload(&runtime.store, &fixture, false, false);
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("saved prefix"),
        Some(checkpoint(&fixture.records[16]))
    );
    assert_eq!(
        runtime
            .store
            .load_block(fixture.positions[17].block_hash)
            .expect("required body"),
        Some(fixture_block(
            fixture.positions[17].previous_block_hash(),
            17
        ))
    );
    drop(runtime);
    let resumed = open_configured(&fixture.path, BasicFilterStartupMode::Enabled)
        .expect("safe resume does not reread indexed pruned inputs");
    assert_payload(&resumed.store, &fixture, false, false);
    drop(resumed);
    fixture.cleanup();
}

#[test]
fn phase157_activation_configured_unsafe_intent_refuses_before_acquisition_or_deletion() {
    for fresh in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("phase157-unsafe-intent", false);
        let store = if fresh {
            let store = FjallNodeStore::open(&history.path).expect("fresh");
            history.persist_payloads(&store);
            store
        } else {
            history.seed(0)
        };
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("full fence");
        seed_raw_prune_intent(
            &store,
            PruneIntent {
                height: 1,
                block_hash: history.old.active_chain[1].block_hash,
            },
        );
        drop(store);
        let before = snapshot_index(&history.path);
        let history_before = snapshot_history(&history.path);

        // Act
        let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

        // Assert
        assert_filter_refusal(result, "prune intent targets required BASIC input");
        assert_eq!(snapshot_index(&history.path), before);
        assert_eq!(snapshot_history(&history.path), history_before);
        history.cleanup();
    }
}

#[test]
fn phase157_activation_configured_disable_invalidates_owner_then_resumes_intent() {
    // Arrange
    let fixture = FilterStartupFixture::new("phase157-configured-disable", 20, Some(1));
    let store = FjallNodeStore::open(&fixture.path).expect("saved Active");
    let saved = store.maybe_basic_filter_checkpoint().expect("checkpoint");
    drop(store);

    // Act
    let runtime = open_configured(&fixture.path, BasicFilterStartupMode::Disabled)
        .expect("ordered disable startup");

    // Assert
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_lifecycle_for_test()
            .expect("owner"),
        Some(IndexLifecycle::Disabled {
            generation: IndexGeneration::new(1)
        })
    );
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("unchanged prefix"),
        saved
    );
    assert_eq!(
        runtime
            .store
            .load_basic_filter_record(fixture.records[0].identity().block_hash())
            .expect("immutable genesis"),
        Some(fixture.records[0].clone())
    );
    assert!(
        runtime
            .store
            .load_prune_locks()
            .expect("released lock")
            .is_empty()
    );
    assert_eq!(
        runtime.store.maybe_prune_intent().expect("finished intent"),
        None
    );
    assert_payload(&runtime.store, &fixture, false, false);
    drop(runtime);
    fixture.cleanup();
}

#[test]
fn phase157_activation_saved_active_preflights_missing_body_and_undo() {
    for body in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("phase157-active-missing", false);
        let store = history.seed(1);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("full fence");
        drop(store);
        remove_historical_payload(&history.path, history.full.active_chain[2].block_hash, body);
        let before = snapshot_index(&history.path);
        let runtime = history.reopen().expect("compatibility open");

        // Act
        let result = runtime.network.enable_basic_filter_index();

        // Assert
        let error = result.expect_err("saved Active must preflight required suffix");
        assert!(
            error.to_string().contains(if body {
                "missing BASIC activation body at height 2"
            } else {
                "missing BASIC activation undo at height 2"
            }),
            "{error}"
        );
        drop(runtime);
        assert_eq!(snapshot_index(&history.path), before);
        history.cleanup();
    }
}

#[test]
fn phase157_activation_ahead_rows_do_not_skip_required_body() {
    // Arrange
    let history = ValidatedHistory::new("phase157-ahead-missing", false);
    let store = history.seed(1);
    append_current(&store, &history.records[2..]).expect("ahead immutable row");
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("full fence");
    drop(store);
    remove_historical_payload(&history.path, history.full.active_chain[2].block_hash, true);
    let before = snapshot_index(&history.path);
    let runtime = history.reopen().expect("compatibility open");

    // Act
    let result = runtime.network.enable_basic_filter_index();

    // Assert
    assert!(
        result
            .expect_err("ahead row is not authority")
            .to_string()
            .contains("missing BASIC activation body at height 2")
    );
    drop(runtime);
    assert_eq!(snapshot_index(&history.path), before);
    history.cleanup();
}

#[test]
fn phase157_activation_stronger_lock_does_not_reread_indexed_pruned_inputs() {
    // Arrange
    let history = ValidatedHistory::new("phase157-stronger-indexed", false);
    let store = history.seed(1);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("full fence");
    store
        .commit_paired_delete(1, history.old.active_chain[1].block_hash)
        .expect("actual indexed paired deletion");
    seed_raw_basic_protection(&store, IndexInputProtection::FromHeight(0));
    store.set_basic_filter_fault(FilterPublicationFault::AfterDisable);
    assert!(store.disable_basic_filter_index().is_err());
    drop(store);
    // Act
    let runtime = open_configured(&history.path, BasicFilterStartupMode::Enabled)
        .expect("required suffix remains retained");

    // Assert
    assert_eq!(
        runtime
            .store
            .load_prune_locks()
            .expect("earned exact-tip lock"),
        vec![
            IndexInputProtection::FromHeight(3)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert!(
        !runtime
            .store
            .has_block(history.old.active_chain[1].block_hash)
            .expect("pruned")
    );
    assert!(
        !runtime
            .store
            .has_undo(history.old.active_chain[1].block_hash)
            .expect("indexed old undo stays absent")
    );
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("actual earned checkpoint"),
        Some(checkpoint(&history.records[2]))
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_activation_configured_actual_paired_loss_refuses_fresh_and_disabled() {
    for fresh in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("phase157-paired-loss", false);
        let store = if fresh {
            let store = FjallNodeStore::open(&history.path).expect("fresh store");
            history.persist_payloads(&store);
            store
        } else {
            let store = history.seed(1);
            store
                .disable_basic_filter_index()
                .expect("release for actual historical pruning");
            store
        };
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("full fence");
        let hash = history.full.active_chain[2].block_hash;
        store
            .commit_paired_delete(2, hash)
            .expect("concrete paired payload deletion");
        assert!(!store.has_block(hash).expect("body lost"));
        assert!(!store.has_undo(hash).expect("undo lost"));
        drop(store);
        let before = snapshot_index(&history.path);
        let history_before = snapshot_history(&history.path);

        // Act
        let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

        // Assert
        assert_filter_refusal(result, "missing BASIC activation body at height 2");
        assert_eq!(snapshot_index(&history.path), before);
        assert_eq!(snapshot_history(&history.path), history_before);
        history.cleanup();
    }
}

#[test]
fn phase157_activation_configured_genesis_body_required_but_undo_not_required() {
    for retained_body in [false, true] {
        // Arrange
        let history = ValidatedHistory::new("phase157-genesis-required", false);
        let store = FjallNodeStore::open(&history.path).expect("fresh store");
        history.persist_payloads(&store);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("full fence");
        drop(store);
        remove_historical_payload(
            &history.path,
            history.full.active_chain[0].block_hash,
            false,
        );
        if !retained_body {
            remove_historical_payload(&history.path, history.full.active_chain[0].block_hash, true);
        }
        let before = snapshot_index(&history.path);
        let history_before = snapshot_history(&history.path);

        // Act
        let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

        // Assert
        if retained_body {
            let runtime = result.expect("genesis needs no undo");
            assert!(
                !runtime
                    .store
                    .has_undo(history.full.active_chain[0].block_hash)
                    .expect("genesis undo remains absent")
            );
            for record in &history.records {
                assert_eq!(
                    runtime
                        .store
                        .load_basic_filter_record(record.identity().block_hash())
                        .expect("first turn generated exact historical filter"),
                    Some(record.clone())
                );
            }
            assert_eq!(
                runtime
                    .store
                    .maybe_basic_filter_checkpoint()
                    .expect("first turn reaches retained durable tip"),
                Some(checkpoint(&history.records[2]))
            );
            assert_eq!(
                runtime
                    .store
                    .load_prune_locks()
                    .expect("earned exact-tip protection"),
                vec![
                    IndexInputProtection::FromHeight(3)
                        .maybe_prune_lock()
                        .expect("lock")
                ]
            );
            drop(runtime);
        } else {
            assert_filter_refusal(result, "missing BASIC activation body at height 0");
            assert_eq!(snapshot_index(&history.path), before);
            assert_eq!(snapshot_history(&history.path), history_before);
        }
        history.cleanup();
    }
}

#[test]
fn phase157_activation_configured_rejects_undo_not_bound_to_spending_body() {
    // Arrange
    let history = ValidatedHistory::new("phase157-undo-bound", false);
    let store = history.seed(1);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("full fence");
    store
        .save_undo(
            history.full.active_chain[2].block_hash,
            &BlockUndo::default(),
            PersistMode::Sync,
        )
        .expect("coinbase-only undo cannot describe spending block");
    drop(store);
    let before = snapshot_index(&history.path);
    let history_before = snapshot_history(&history.path);

    // Act
    let result = open_configured(&history.path, BasicFilterStartupMode::Enabled);

    // Assert
    assert!(matches!(
        result,
        Err(SyncRuntimeError::Storage(StorageError::Corruption { .. }))
    ));
    assert_eq!(snapshot_index(&history.path), before);
    assert_eq!(snapshot_history(&history.path), history_before);
    history.cleanup();
}

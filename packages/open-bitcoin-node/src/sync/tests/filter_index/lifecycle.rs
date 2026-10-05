// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::recovery::ValidatedHistory;
use super::*;
use crate::storage::fjall_store::filters::FilterPublicationFault;
use open_bitcoin_core::chainstate::filter_index::lifecycle::{IndexGeneration, IndexLifecycle};

fn owner(store: &FjallNodeStore) -> IndexLifecycle {
    store
        .maybe_basic_filter_lifecycle_for_test()
        .expect("owner read")
        .expect("owner")
}

fn state_bytes(store: &FjallNodeStore) -> Vec<u8> {
    codec::encode_state(
        store
            .maybe_basic_filter_state()
            .expect("state read")
            .expect("state"),
    )
}

#[test]
fn filter_index_production_host_disable_enable_rejects_old_work_and_preserves_history() {
    // Arrange
    let history = ValidatedHistory::new("host-lifecycle", false);
    let store = history.seed(0);
    store
        .seed_coins_from_snapshot(&history.full)
        .expect("full durable authority");
    drop(store);
    let runtime = history.reopen().expect("production open");
    let work = runtime
        .store
        .maybe_basic_filter_work(&fence(&history.full.active_chain))
        .expect("work")
        .expect("active");
    let state = state_bytes(&runtime.store);

    // Act
    runtime
        .network
        .disable_basic_filter_index()
        .expect("host disable");

    // Assert
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(1)
        }
    );
    assert!(
        runtime
            .store
            .maybe_basic_filter_work(&fence(&history.full.active_chain))
            .expect("disabled")
            .is_none()
    );
    assert!(
        runtime
            .store
            .persist_basic_filter_records(
                &work,
                &fence(&history.full.active_chain),
                &history.records[1..]
            )
            .is_err()
    );
    assert!(
        runtime
            .store
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&history.full.active_chain),
                checkpoint(&history.records[2]),
                checkpoint(&history.records[2]).input_protection(),
                &history.records[1..]
            )
            .is_err()
    );
    assert_eq!(state_bytes(&runtime.store), state);
    runtime
        .network
        .enable_basic_filter_index()
        .expect("protected re-enable");
    runtime
        .network
        .enable_basic_filter_index()
        .expect("idempotent active");
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(2)
        }
    );
    assert!(
        runtime
            .store
            .persist_basic_filter_records(
                &work,
                &fence(&history.full.active_chain),
                &history.records[1..]
            )
            .is_err()
    );
    assert!(
        runtime
            .store
            .publish_basic_filter_checkpoint(
                &work,
                &fence(&history.full.active_chain),
                checkpoint(&history.records[2]),
                checkpoint(&history.records[2]).input_protection(),
                &history.records[1..]
            )
            .is_err()
    );
    assert_eq!(
        runtime
            .network
            .list_prune_locks()
            .expect("protected before deletion"),
        vec![
            IndexInputProtection::FromHeight(1)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    let reenabled_state = state_bytes(&runtime.store);
    runtime
        .network
        .disable_basic_filter_index()
        .expect("disable again");
    drop(runtime);
    let runtime = history.reopen().expect("Disabled production reopen");
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(3)
        }
    );
    assert_eq!(state_bytes(&runtime.store), reenabled_state);
    assert!(
        runtime
            .network
            .list_prune_locks()
            .expect("released")
            .is_empty()
    );
    runtime
        .network
        .enable_basic_filter_index()
        .expect("enable after reopen");
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(4)
        }
    );
    publish_current(
        &runtime.store,
        &fence(&history.full.active_chain),
        checkpoint(&history.records[2]),
        checkpoint(&history.records[2]).input_protection(),
        &history.records[1..],
    )
    .expect("fresh work publication");
    history.assert_rows_and_payloads(&runtime.store);
    drop(runtime);
    drop(work);
    let runtime = history.reopen().expect("Active production reopen");
    history.assert_rows_and_payloads(&runtime.store);
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("prefix"),
        Some(checkpoint(&history.records[2]))
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn filter_index_production_host_disable_faults_recover_valid_retained_lifecycle() {
    for (fault, disabled, locked) in [
        (FilterPublicationFault::BeforeDisable, false, true),
        (FilterPublicationFault::AfterDisable, true, true),
        (FilterPublicationFault::BeforeRelease, true, true),
        (FilterPublicationFault::AfterRelease, true, false),
    ] {
        // Arrange
        let history = ValidatedHistory::new("host-disable-fault", false);
        let store = history.seed(1);
        let state = state_bytes(&store);
        drop(store);
        let runtime = history.reopen().expect("runtime");
        runtime.store.set_basic_filter_fault(fault);

        // Act
        assert!(runtime.network.disable_basic_filter_index().is_err());
        assert!(
            runtime
                .store
                .commit_paired_delete(1, history.old.active_chain[1].block_hash)
                .is_err()
        );
        assert!(
            runtime
                .store
                .maybe_basic_filter_work(&fence(&history.old.active_chain))
                .is_err()
        );
        super::prune_faults::assert_unearned(&runtime.store);
        drop(runtime);
        let runtime = history.reopen().expect("production fault reopen");

        // Assert
        assert_eq!(
            matches!(owner(&runtime.store), IndexLifecycle::Disabled { .. }),
            disabled
        );
        assert_eq!(
            !runtime
                .network
                .list_prune_locks()
                .expect("locks")
                .is_empty(),
            locked
        );
        assert_eq!(state_bytes(&runtime.store), state);
        assert_eq!(
            owner(&runtime.store).generation(),
            IndexGeneration::new(u64::from(disabled))
        );
        super::prune_faults::assert_unearned(&runtime.store);
        assert_eq!(
            runtime
                .store
                .load_basic_filter_record(history.records[1].identity().block_hash())
                .expect("row"),
            Some(history.records[1].clone())
        );
        assert_eq!(
            runtime
                .store
                .load_block(history.full.active_chain[2].block_hash)
                .expect("body"),
            Some(history.blocks[2].clone())
        );
        assert_eq!(
            runtime
                .store
                .load_undo(history.full.active_chain[2].block_hash)
                .expect("undo"),
            Some(history.full.undo_by_block[&history.full.active_chain[2].block_hash].clone())
        );
        runtime
            .network
            .disable_basic_filter_index()
            .expect("finish release");
        runtime.network.disable_basic_filter_index().expect("retry");
        assert_eq!(
            owner(&runtime.store),
            IndexLifecycle::Disabled {
                generation: IndexGeneration::new(1)
            }
        );
        drop(runtime);
        history.cleanup();
    }
}

fn remove_historical_payload(path: &Path, hash: BlockHash, body: bool) {
    let hex: String = hash
        .as_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    let namespace = if body { "block_index" } else { "chainstate" };
    let key = format!("{}:{hex}", if body { "block" } else { "undo" });
    raw_namespace(path, namespace, |space| {
        space.remove(key).expect("missing input fixture");
    });
}

#[test]
fn filter_index_production_reenable_missing_body_or_undo_keeps_disabled_prefix() {
    for body in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("host-missing-history", false);
        let store = history.seed(1);
        store
            .seed_coins_from_snapshot(&history.full)
            .expect("full fence");
        drop(store);
        let runtime = history.reopen().expect("runtime");
        runtime
            .network
            .disable_basic_filter_index()
            .expect("disable");
        let state = state_bytes(&runtime.store);
        drop(runtime);
        let hash = history.full.active_chain[2].block_hash;
        remove_historical_payload(&history.path, hash, body);
        let before = snapshot_index(&history.path);
        let runtime = history
            .reopen()
            .expect("disabled tolerates released history");

        // Act
        let error = runtime
            .network
            .enable_basic_filter_index()
            .expect_err("missing input");

        // Assert
        assert!(error.to_string().contains(if body {
            "missing BASIC activation body"
        } else {
            "missing BASIC activation undo"
        }));
        assert_eq!(
            owner(&runtime.store),
            IndexLifecycle::Disabled {
                generation: IndexGeneration::new(1)
            }
        );
        assert_eq!(state_bytes(&runtime.store), state);
        assert!(
            runtime
                .network
                .list_prune_locks()
                .expect("no acquisition")
                .is_empty()
        );
        assert!(
            runtime
                .store
                .maybe_basic_filter_work(&fence(&history.full.active_chain))
                .expect("no work")
                .is_none()
        );
        drop(runtime);
        assert_eq!(snapshot_index(&history.path), before);
        let runtime = history.reopen().expect("retry open");
        if body {
            runtime
                .store
                .save_block(&history.blocks[2], PersistMode::Sync)
                .expect("restore fixture body");
        } else {
            runtime
                .store
                .save_undo(hash, &history.full.undo_by_block[&hash], PersistMode::Sync)
                .expect("restore fixture undo");
        }
        runtime
            .network
            .enable_basic_filter_index()
            .expect("retained input retry");
        assert_eq!(
            runtime.network.list_prune_locks().expect("protection"),
            vec![
                IndexInputProtection::FromHeight(2)
                    .maybe_prune_lock()
                    .expect("lock")
            ]
        );
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn filter_index_production_disabled_live_intent_finishes_with_saved_prefix_intact() {
    // Arrange: sparse deletion-order fixture, separate from validated spend history.
    let fixture = FilterStartupFixture::new("disabled-safe-live-intent", 20, Some(1));
    let store = FjallNodeStore::open(&fixture.path).expect("store");
    let state = state_bytes(&store);
    store
        .disable_basic_filter_index()
        .expect("trusted disable before startup");
    drop(store);

    // Act
    let runtime = fixture
        .open_runtime()
        .expect("production Disabled reopen resumes intent");

    // Assert
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(1)
        }
    );
    assert_eq!(state_bytes(&runtime.store), state);
    assert_payload(&runtime.store, &fixture, false, false);
    assert!(
        runtime
            .store
            .maybe_prune_intent()
            .expect("finished intent")
            .is_none()
    );
    assert!(
        runtime
            .network
            .list_prune_locks()
            .expect("released")
            .is_empty()
    );
    assert_eq!(
        runtime
            .store
            .load_basic_filter_record(fixture.records[1].identity().block_hash())
            .expect("retained record"),
        Some(fixture.records[1].clone())
    );
    drop(runtime);
    fixture.cleanup();
}

#[test]
fn filter_index_production_malformed_disabled_owner_refuses_before_payload_mutation() {
    // Arrange
    let history = ValidatedHistory::new("malformed-disabled", false);
    let store = history.seed(1);
    store.disable_basic_filter_index().expect("disable");
    drop(store);
    raw_index(&history.path, |index| {
        index
            .insert(
                codec::ownership::OWNER_KEY,
                [1, 0, 9, 1, 0, 0, 0, 0, 0, 0, 0],
            )
            .expect("invalid mode fixture");
    });
    let before = snapshot_index(&history.path);

    // Act
    let result = history.reopen();

    // Assert
    assert_filter_refusal(result, "owner mode");
    assert_eq!(snapshot_index(&history.path), before);
    let store = FjallNodeStore::open(&history.path).expect("inspect");
    assert_eq!(
        store
            .load_block(history.full.active_chain[2].block_hash)
            .expect("body"),
        Some(history.blocks[2].clone())
    );
    assert_eq!(
        store
            .load_undo(history.full.active_chain[2].block_hash)
            .expect("undo"),
        Some(history.full.undo_by_block[&history.full.active_chain[2].block_hash].clone())
    );
    drop(store);
    history.cleanup();
}

#[test]
fn filter_index_production_absent_host_activation_and_genesis_needs_no_undo() {
    // Arrange
    let history = ValidatedHistory::new("host-absent-activation", false);
    let store = FjallNodeStore::open(&history.path).expect("store");
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
    let runtime = history.reopen().expect("legacy absent runtime");

    // Act
    runtime
        .network
        .enable_basic_filter_index()
        .expect("trusted first activation");

    // Assert
    assert_eq!(
        owner(&runtime.store),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(
        runtime
            .store
            .maybe_basic_filter_checkpoint()
            .expect("empty"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    assert_eq!(
        runtime
            .network
            .list_prune_locks()
            .expect("all inputs protected"),
        vec![
            IndexInputProtection::FromHeight(0)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    assert!(
        runtime
            .store
            .load_undo(history.full.active_chain[0].block_hash)
            .expect("genesis no undo")
            .is_none()
    );
    drop(runtime);
    let runtime = history.reopen().expect("first activation durable reopen");
    assert!(
        runtime
            .store
            .maybe_basic_filter_work(&fence(&history.full.active_chain))
            .expect("protected work")
            .is_some()
    );
    drop(runtime);
    history.cleanup();
}

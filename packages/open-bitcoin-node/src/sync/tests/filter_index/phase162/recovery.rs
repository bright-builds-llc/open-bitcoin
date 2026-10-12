// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Interrupted intent and immutable-ahead evidence against genuine durable coins.

use super::{missing_history::*, *};

fn unaffected_rows(rows: &[(Vec<u8>, Vec<u8>)], keys: &[&[u8]]) -> Vec<(Vec<u8>, Vec<u8>)> {
    rows.iter()
        .filter(|(key, _)| !keys.contains(&key.as_slice()))
        .cloned()
        .collect()
}

fn assert_resume_counts(store: &FjallNodeStore, count: u64) {
    let summary = store
        .load_operator_support_counts()
        .expect("actual durable prune summary");
    assert_eq!(summary.successful_batch_count, count);
    assert_eq!(summary.pruned_height_count, count);
    assert_eq!(summary.maybe_last_prune_height, (count == 1).then_some(20));
}

fn conflicting_intent(saved: bool) {
    // Arrange: focused interrupted-intent seed below the real prune-after threshold.
    let cleanup = Cleanup::new();
    let fixture = history(&cleanup, saved);
    let intent = PruneIntent {
        height: 102,
        block_hash: fixture.receipts[102].block_hash,
    };
    let positions = fixture.receipts.clone();
    let old_body = fixture
        .runtime
        .store()
        .load_block(intent.block_hash)
        .expect("actual body");
    let old_undo = fixture
        .runtime
        .store()
        .load_undo(intent.block_hash)
        .expect("actual undo");
    assert!(old_body.is_some() && old_undo.is_some());
    seed_raw_prune_intent(fixture.runtime.store(), intent);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_prune_intent()
            .expect("private persisted identity"),
        Some(intent)
    );
    drop(fixture);

    // Act
    assert_refusals(&cleanup.path, "prune intent targets required BASIC input");

    // Assert: private intent identity and both required mates survive before resume.
    let store =
        FjallNodeStore::open(&cleanup.path).expect("private inspection after second refusal");
    assert_authority(&store, &positions, 104);
    assert_eq!(
        store.maybe_prune_intent().expect("unresumed same intent"),
        Some(intent)
    );
    assert_eq!(
        store.load_block(intent.block_hash).expect("required body"),
        old_body
    );
    assert_eq!(
        store.load_undo(intent.block_hash).expect("required undo"),
        old_undo
    );
    assert_eq!(rows(&store, &positions).len(), if saved { 102 } else { 0 });
    assert!(!store.load_have_pruned().expect("resume must not unlink"));
    drop(store);
}

#[test]
fn phase162_recovery_fresh_required_intent_refuses_before_resume() {
    conflicting_intent(false);
}

#[test]
fn phase162_recovery_saved_lagged_intent_refuses_before_resume() {
    conflicting_intent(true);
}

#[test]
fn phase162_recovery_completed_prefix_intent_settles_existing_owner_only() {
    // Arrange: actual mate identity; raw intent models interruption, not legal planning.
    let cleanup = Cleanup::new();
    let mut fixture = history(&cleanup, true);
    // Resume still enforces the real 288-block keep window on forced intents.
    extend(&mut fixture, 310);
    flush_only(&fixture.runtime);
    fixture.finish();
    let positions = fixture.receipts.clone();
    assert_authority(fixture.runtime.store(), &positions, 309);
    assert_resume_counts(fixture.runtime.store(), 0);
    let intent = PruneIntent {
        height: 20,
        block_hash: positions[20].block_hash,
    };
    let old_rows = rows(fixture.runtime.store(), &positions);
    let old_checkpoint = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("completed safe");
    let old_locks = fixture
        .runtime
        .store()
        .load_prune_locks()
        .expect("completed protection");
    let old_owner = fixture
        .runtime
        .store()
        .maybe_basic_filter_lifecycle_for_test()
        .expect("existing owner");
    assert!(
        fixture
            .runtime
            .store()
            .has_block(intent.block_hash)
            .expect("real body")
    );
    assert!(
        fixture
            .runtime
            .store()
            .has_undo(intent.block_hash)
            .expect("real undo")
    );
    seed_raw_prune_intent(fixture.runtime.store(), intent);
    drop(fixture);
    let before = snapshot(&cleanup.path);

    // Act
    let runtime =
        open(&cleanup.path).expect("serialized initialize resumes eligible interrupted intent");

    // Assert
    assert_authority(runtime.store(), &positions, 309);
    assert_resume_counts(runtime.store(), 1);
    assert_eq!(
        runtime
            .store()
            .maybe_prune_intent()
            .expect("settled intent"),
        None
    );
    assert!(
        !runtime
            .store()
            .has_block(intent.block_hash)
            .expect("paired body deletion")
    );
    assert!(
        !runtime
            .store()
            .has_undo(intent.block_hash)
            .expect("paired undo deletion")
    );
    assert!(
        runtime
            .store()
            .load_have_pruned()
            .expect("earned resumed deletion")
    );
    assert_eq!(rows(runtime.store(), &positions), old_rows);
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe unchanged"),
        old_checkpoint
    );
    assert_eq!(
        runtime
            .store()
            .load_prune_locks()
            .expect("protection unchanged"),
        old_locks
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_lifecycle_for_test()
            .expect("same owner"),
        old_owner
    );
    drop(runtime);
    let settled = snapshot(&cleanup.path);
    let body_key = codec::record_key(intent.block_hash).replacen(codec::RECORD_PREFIX, "block:", 1);
    let undo_key = codec::record_key(intent.block_hash).replacen(codec::RECORD_PREFIX, "undo:", 1);
    let changed_index_keys = [
        body_key.as_bytes(),
        b"prune_intent",
        b"have_pruned",
        b"prune_summary",
    ];
    assert!(
        unaffected_rows(&before[0], &changed_index_keys)
            == unaffected_rows(&settled[0], &changed_index_keys),
        "resume changes only the exact body, intent, have-pruned and decoded summary keys"
    );
    assert!(
        unaffected_rows(&before[1], &[undo_key.as_bytes()])
            == unaffected_rows(&settled[1], &[undo_key.as_bytes()]),
        "only the exact undo key changes; metadata and other mates remain byte-identical"
    );
    assert_eq!(
        before[2], settled[2],
        "all durable coins bytes unaffected by resume"
    );
    let runtime = open(&cleanup.path).expect("second cold reopen after paired unlink");
    assert_authority(runtime.store(), &positions, 309);
    assert_resume_counts(runtime.store(), 1);
    assert_eq!(rows(runtime.store(), &positions), old_rows);
    assert_eq!(
        runtime
            .store()
            .maybe_prune_intent()
            .expect("no repeated intent"),
        None
    );
    assert!(
        !runtime
            .store()
            .has_block(intent.block_hash)
            .expect("body remains absent")
    );
    assert!(
        !runtime
            .store()
            .has_undo(intent.block_hash)
            .expect("undo remains absent")
    );
    drop(runtime);
    assert_eq!(snapshot(&cleanup.path), settled);
}

#[test]
fn phase162_recovery_unflushed_ahead_rows_reopen_at_real_coins_prefix() {
    // Arrange: ordinary accepted/indexed work extends genuine durable h101 to h104.
    let cleanup = Cleanup::new();
    let mut fixture =
        AcceptedRecipe::at(cleanup.path.clone(), 102, BasicFilterStartupMode::Enabled);
    fixture.finish();
    fixture.flush();
    let old_checkpoint = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("real safe101");
    let old_locks = fixture
        .runtime
        .store()
        .load_prune_locks()
        .expect("required102");
    extend(&mut fixture, 105);
    fixture.finish();
    let positions = fixture.receipts.clone();
    let old_rows = rows(fixture.runtime.store(), &positions);
    assert_eq!(old_rows.len(), 105);
    let progress = fixture
        .runtime
        .network_handle()
        .maybe_basic_index_progress()
        .expect("owner")
        .expect("enabled");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("accepted work")
            .height(),
        104
    );
    assert_eq!(
        progress
            .maybe_safe_durable_endpoint()
            .expect("actual durable fence")
            .height(),
        101
    );
    assert_authority(fixture.runtime.store(), &positions, 101);
    drop(fixture);
    let before = snapshot(&cleanup.path);

    // Act
    for _ in 0..2 {
        let runtime = open(&cleanup.path).expect("recoverable old durable prefix");
        // Assert: extra immutable hashes survive but cannot authorize active progress.
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
            runtime.store().load_prune_locks().expect("conservative"),
            old_locks
        );
        for position in &positions[102..] {
            assert_eq!(
                runtime
                    .store()
                    .maybe_active_basic_filter_record(position.height)
                    .expect("active projection"),
                None
            );
            assert!(
                runtime
                    .store()
                    .load_basic_filter_record(position.block_hash)
                    .expect("immutable retained hash")
                    .is_some()
            );
        }
        let progress = runtime
            .network_handle()
            .maybe_basic_index_progress()
            .expect("owner")
            .expect("enabled");
        assert_eq!(
            progress
                .maybe_processed_endpoint()
                .expect("recovered processed")
                .height(),
            101
        );
        assert_eq!(
            progress
                .maybe_safe_durable_endpoint()
                .expect("recovered safe")
                .height(),
            101
        );
        assert_eq!(progress.current_lag(), 0);
        assert_eq!(
            runtime
                .store()
                .maybe_prune_intent()
                .expect("no invented intent"),
            None
        );
        drop(runtime);
        assert_eq!(snapshot(&cleanup.path), before);
    }
}

#[test]
fn phase162_recovery_ahead_rows_with_coins_metadata_conflict_refuse_without_repair() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;

    // Arrange: unflushed accepted work and immutable rows ahead of real safe h101.
    let cleanup = Cleanup::new();
    let mut fixture =
        AcceptedRecipe::at(cleanup.path.clone(), 102, BasicFilterStartupMode::Enabled);
    fixture.finish();
    fixture.flush();
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
    extend(&mut fixture, 105);
    fixture.finish();
    let positions = fixture.receipts.clone();
    let old_rows = rows(fixture.runtime.store(), &positions);
    assert_eq!(old_rows.len(), 105);
    assert_authority(fixture.runtime.store(), &positions, 101);
    fixture
        .runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::BeforeChainMeta);

    // Act: actual coins publication completes before the real metadata effect fails.
    let result = fixture.runtime.network_handle().flush_coins(
        FlushMode::Always,
        FlushPolicyTime::from_unix_seconds(500_000),
        u64::MAX,
    );

    // Assert: the failure cannot authorize a saved checkpoint or repair metadata.
    assert!(
        result.is_err(),
        "metadata effect fault must not return a successful flush receipt"
    );
    assert!(
        fixture
            .runtime
            .network_handle()
            .drive_basic_filter_index_turn()
            .is_err()
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("completed real coins B"),
        Some(positions[104].block_hash)
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_chain_meta_for_open()
            .expect("unchanged metadata")
            .0,
        positions[..=101]
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe"),
        old_checkpoint
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_prune_locks()
            .expect("protection"),
        old_locks
    );
    assert_eq!(rows(fixture.runtime.store(), &positions), old_rows);
    for position in &positions[1..] {
        assert!(
            fixture
                .runtime
                .store()
                .has_block(position.block_hash)
                .expect("real body")
        );
        assert!(
            fixture
                .runtime
                .store()
                .has_undo(position.block_hash)
                .expect("real flushed undo")
        );
    }
    drop(fixture);
    assert_refusals(
        &cleanup.path,
        "coins best block differs from durable metadata",
    );
    let store = FjallNodeStore::open(&cleanup.path).expect("second closed-refusal inspection");
    assert_eq!(
        store.coins_view().best_block().expect("unaltered B104"),
        Some(positions[104].block_hash)
    );
    assert_eq!(
        store
            .load_chain_meta_for_open()
            .expect("unaltered metadata101")
            .0,
        positions[..=101]
    );
    assert_eq!(rows(&store, &positions), old_rows);
    assert_eq!(
        store
            .maybe_basic_filter_checkpoint()
            .expect("no fabricated progress"),
        old_checkpoint
    );
    assert_eq!(
        store.load_prune_locks().expect("retained protection"),
        old_locks
    );
    assert_eq!(
        store.maybe_prune_intent().expect("no invented resume"),
        None
    );
    for position in &positions[1..] {
        assert!(
            store
                .has_block(position.block_hash)
                .expect("preserved body")
        );
        assert!(store.has_undo(position.block_hash).expect("preserved undo"));
    }
    drop(store);
}

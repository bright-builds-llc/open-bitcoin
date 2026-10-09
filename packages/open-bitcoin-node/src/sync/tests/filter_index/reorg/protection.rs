// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::{PruneLockInfo, PruneMode, PrunePlan};

#[test]
fn phase158_reorg_protection_interrupted_intent_refuses_after_reorg_before_deletion() {
    // Arrange: replacement is accepted/indexed ahead of the old coins fence.
    let fixture = ForkFixture::compact(64, 1, 16);
    let branch = fixture.fork(30, 34, 0x81);
    fixture.apply(&branch);
    fixture.finish();
    let intent = PruneIntent {
        height: 20,
        block_hash: fixture.history.records[20].identity().block_hash(),
    };
    seed_raw_prune_intent(fixture.runtime.store(), intent);
    let ForkFixture {
        history, runtime, ..
    } = fixture;
    drop(runtime);
    let before = snapshot_index(&history.path);
    // Act
    let result = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("actual closed reopen"),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    );
    // Assert: protection is restored before the interrupted paired-delete owner.
    assert_filter_refusal(result, "prune intent targets required BASIC input");
    assert_eq!(snapshot_index(&history.path), before);
    let store = FjallNodeStore::open(&history.path).expect("inspect refused reopen");
    assert!(
        store
            .has_block(intent.block_hash)
            .expect("retained required body")
    );
    assert!(
        store
            .has_undo(intent.block_hash)
            .expect("retained required undo")
    );
    assert_eq!(
        store.maybe_prune_intent().expect("preserved intent"),
        Some(intent)
    );
    assert!(
        !store
            .load_have_pruned()
            .expect("no fabricated deletion receipt")
    );
    for record in &branch.records {
        assert_eq!(
            store
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable replacement after refusal"),
            Some(record.clone())
        );
    }
    drop(store);
    history.cleanup();
}

#[test]
fn phase158_reorg_protection_reserved_crud_and_ifneeded_preserve_unflushed_coins() {
    for count in [13, 16] {
        // Arrange
        let fixture = ForkFixture::compact(16, 8, 24);
        let branch = fixture.fork(10, count, 0x79);
        let maybe_coins = fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("before coins");
        let metadata = fixture
            .runtime
            .store()
            .load_chain_meta_for_open()
            .expect("before meta");
        fixture.apply(&branch);
        let locks = fixture
            .runtime
            .network
            .list_prune_locks()
            .expect("replacement locks");
        // Act
        assert!(
            fixture
                .runtime
                .network
                .clear_prune_lock(BASIC_INDEX_PRUNE_LOCK)
                .is_err()
        );
        assert!(
            fixture
                .runtime
                .network
                .replace_prune_lock(PruneLockInfo {
                    name: BASIC_INDEX_PRUNE_LOCK.to_owned(),
                    height_first: u32::MAX,
                    height_last: u32::MAX
                })
                .is_err()
        );
        fixture.finish();
        let skipped = fixture
            .runtime
            .network
            .flush_coins(
                FlushMode::IfNeeded,
                FlushPolicyTime::from_unix_seconds(50_000),
                u64::MAX,
            )
            .expect("ordinary no pressure");
        // Assert: index processing never manufactures a coins receipt.
        assert!(!skipped.wrote_coins);
        assert_eq!(
            fixture
                .runtime
                .store()
                .coins_view()
                .best_block()
                .expect("after coins"),
            maybe_coins
        );
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_chain_meta_for_open()
                .expect("after meta"),
            metadata
        );
        assert_eq!(
            fixture
                .runtime
                .network
                .list_prune_locks()
                .expect("unchanged locks"),
            locks
        );
        let progress = fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("owner")
            .expect("accepted owner");
        assert_eq!(progress.current_lag(), 0);
        assert_eq!(
            progress.maybe_safe_durable_endpoint(),
            Some(fixture.history.records[10].identity())
        );
        fixture.flush();
        branches::assert_active(&fixture, &branch, 10);
        fixture.cleanup();
    }
}

#[test]
fn phase158_reorg_protection_preplanned_manual_required_source_skips_after_generation_change() {
    // Arrange: a bounded shared backlog keeps preflight inside existing caps.
    let fixture = ForkFixture::compact(64, 1, 16);
    let branch = fixture.fork(30, 34, 0x7a);
    let old = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("old")
        .expect("owner");
    let plan = PrunePlan { heights: vec![20] };
    let stale_locks = Vec::new();
    let hash = fixture.history.records[20].identity().block_hash();
    fixture.apply(&branch);
    let next = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("new")
        .expect("owner");
    assert!(next.generation().value() > old.generation().value());
    assert!(
        fixture
            .runtime
            .store()
            .commit_paired_delete(20, hash)
            .expect_err("fresh index protection wins before deletion")
            .to_string()
            .contains("required BASIC input")
    );
    // Act: old caller facts never replace fresh owned protection.
    let outcome = fixture
        .runtime
        .network
        .flush_applying_prune_plan(
            FlushMode::IfNeeded,
            FlushPolicyTime::from_unix_seconds(50_000),
            u64::MAX,
            &plan,
            &stale_locks,
        )
        .expect("fresh protection recheck");
    // Assert
    assert!(outcome.deleted_block_hashes.is_empty());
    assert!(
        fixture
            .runtime
            .store()
            .has_block(hash)
            .expect("retained body")
    );
    assert!(
        fixture
            .runtime
            .store()
            .has_undo(hash)
            .expect("retained undo")
    );
    assert!(
        !fixture
            .runtime
            .store()
            .load_have_pruned()
            .expect("no receipt")
    );
    fixture.finish();
    fixture.flush();
    branches::assert_active(&fixture, &branch, 30);
    fixture.cleanup();
}

#[test]
fn phase158_reorg_protection_automatic_prune_remeasures_new_generation_and_earns_own_flush() {
    // Arrange: actual configured regtest threshold 1000; continuous history.
    let mut fixture = ForkFixture::compact(1024, 1, 1008);
    fixture
        .runtime
        .network
        .set_prune_network(SyncNetwork::Regtest)
        .expect("actual threshold");
    fixture
        .runtime
        .network
        .set_prune_mode(PruneMode::Automatic { target_mib: 0 })
        .expect("test pressure");
    let branch = fixture.fork(1016, 16, 0x7b);
    let old = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("old")
        .expect("owner");
    let hash = branch.records[0].identity().block_hash();
    // Warm the old generation cache; the longer branch exposes new legal
    // shared-prefix candidates while replacement history remains protected.
    fixture
        .runtime
        .network
        .flush_coins(
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(50_000),
            u64::MAX,
        )
        .expect("old generation measured");
    fixture.apply(&branch);
    let next = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("new")
        .expect("owner");
    assert!(next.generation().value() > old.generation().value());
    // Act: same second must reload generation/protection, retaining required backlog.
    let outcome = fixture
        .runtime
        .network
        .flush_coins(
            FlushMode::Periodic,
            FlushPolicyTime::from_unix_seconds(50_000),
            u64::MAX,
        )
        .expect("new generation measured");
    // Assert: genuine automatic deletion forces its ordinary own checkpoint.
    assert!(outcome.wrote_coins);
    assert!(!outcome.deleted_block_hashes.is_empty());
    assert!(
        fixture
            .runtime
            .store()
            .has_block(hash)
            .expect("required body")
    );
    assert!(
        fixture
            .runtime
            .store()
            .has_undo(hash)
            .expect("required undo")
    );
    assert!(!outcome.deleted_block_hashes.contains(&hash));
    fixture.finish();
    fixture.flush();
    let tip = branch.records.last().expect("actual replacement tip");
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("own safe checkpoint"),
        Some(checkpoint(tip))
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("own coins fence"),
        Some(tip.identity().block_hash())
    );
    assert!(
        fixture
            .runtime
            .network
            .list_prune_locks()
            .expect("exact-tip protection")
            .contains(
                &checkpoint(tip)
                    .input_protection()
                    .maybe_prune_lock()
                    .expect("reserved ownership")
            )
    );
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("active replacement"),
            Some(record.clone())
        );
    }
    let pruned = &fixture.history.records[20];
    assert!(
        !fixture
            .runtime
            .store()
            .has_block(pruned.identity().block_hash())
            .expect("actual safe shared prune")
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .load_basic_filter_record(pruned.identity().block_hash())
            .expect("retained pruned shared commitment"),
        Some(pruned.clone())
    );
    fixture.cleanup();
}

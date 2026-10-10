// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Ordinary configured startup and bounded maintenance evidence.

use super::recovery::ValidatedHistory;
use super::*;

mod failures;
pub(super) mod fixtures;
mod measurements;
#[path = "../../../network/runtime_authority/filter_index/query/tests.rs"]
mod phase159_query_tests;
#[path = "../../../network/runtime_authority/filter_index/readiness/tests.rs"]
mod phase159_readiness_tests;
mod rpc_faults;
use fixtures::TurnHistory;

#[test]
fn phase157_turn_remaining_budget_cannot_inflate_or_discard_acquisition() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let cap = injected_budget().normal();
    let acquire = || {
        runtime
            .store()
            .maybe_basic_filter_append_proof_with_budget(cap)
            .expect("budgeted acquisition")
            .expect("actual proof")
    };
    let mut inflated = cap;
    inflated.record_operations += 1;
    let mut discarded = cap;
    discarded.checkpoint_operations = 0;
    // Act / Assert
    assert!(acquire().with_remaining_budget(inflated).is_err());
    assert!(acquire().with_remaining_budget(discarded).is_err());
    assert!(
        runtime
            .store()
            .maybe_basic_filter_append_proof()
            .expect("bare recovery proof")
            .expect("actual proof")
            .with_remaining_budget(cap)
            .is_err()
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_output_reservation_exact_and_one_under_yield_before_generation() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget;
    for one_under in [false, true] {
        // Arrange
        let fixture = TurnHistory::new(16, 24, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(16),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        let caps = injected_budget();
        let mut normal = caps.normal();
        let mut absolute = caps.absolute_singleton();
        normal.script_items = 1_000_000;
        absolute.script_items = 1_000_000;
        let wire = open_bitcoin_core::codec::encode_block(&fixture.history.blocks[24])
            .expect("wire")
            .len() as u64;
        normal.encoded_bytes = (wire / 9 + 2) * 4 + 179 - u64::from(one_under);
        let budget = BasicIndexTurnBudget::new(normal, absolute).expect("output budget");
        // Act
        let turn = runtime
            .network
            .drive_basic_filter_index_turn_with_budget(budget)
            .expect("pre-generation output admission");
        // Assert
        assert_eq!(turn.work.blocks, 1);
        assert_eq!(turn.generations, 1);
        assert_eq!(turn.body_decodes, 1);
        assert_eq!(turn.oversized_singleton, one_under);
        if !one_under {
            assert!(turn.work.fits(normal));
        }
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_turn_stronger_saved_lock_is_mirrored_until_actual_safe_advance() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let store = fixture.seed(16);
    seed_raw_basic_protection(&store, IndexInputProtection::FromHeight(0));
    // Act
    let runtime = DurableSyncRuntime::open_configured(
        store,
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let first = runtime.network.last_basic_filter_turn_for_test();
    let second = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("same safe");
    // Assert
    assert_eq!(
        first.maybe_progress.expect("owner").protection(),
        IndexInputProtection::FromHeight(0)
    );
    assert_eq!(
        second.maybe_progress.expect("owner").protection(),
        IndexInputProtection::FromHeight(0)
    );
    assert_eq!(
        runtime.store().load_prune_locks().expect("actual lock"),
        vec![
            IndexInputProtection::FromHeight(0)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
    let final_turn = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("exact tip");
    assert_eq!(
        final_turn.maybe_progress.expect("owner").protection(),
        IndexInputProtection::FromHeight(40)
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_prefix_independent_complete_operation_counts() {
    for prefix in [16, 128, 512] {
        // Arrange
        let fixture = TurnHistory::new(prefix, 24, 8_192);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(prefix),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        // Act
        let turn = runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("fixed workload");
        let reads = runtime.store().basic_filter_point_reads_for_test();
        runtime
            .store()
            .load_basic_filter_record(
                turn.maybe_progress
                    .expect("owner")
                    .maybe_processed_endpoint()
                    .expect("endpoint")
                    .block_hash(),
            )
            .expect("general ancestry control")
            .expect("record");
        let full_ancestry_reads = runtime.store().basic_filter_point_reads_for_test() - reads;
        // Assert
        assert_eq!(turn.body_reads, 8);
        assert_eq!(turn.body_decodes, 8);
        assert_eq!(turn.undo_borrows, 8);
        assert_eq!(turn.generations, 8);
        assert_eq!(
            turn.work.record_operations, 98,
            "turn work: {:?}; indexed point reads: {}; full ancestry reads: {full_ancestry_reads}",
            turn.work, turn.indexed_point_reads
        );
        assert_eq!(turn.work.projection_operations, 44);
        assert_eq!(turn.work.checkpoint_operations, 7_621);
        assert_eq!(turn.indexed_point_reads, 56);
        assert!(turn.work.body_bytes <= 68_000);
        assert!(turn.work.cloned_bytes < 5_000_000);
        assert_eq!(turn.persistence_batches, 1);
        assert!(
            full_ancestry_reads > turn.indexed_point_reads,
            "the real full ancestry helper would fail the turn bound"
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_turn_production_legal_large_singleton_makes_progress() {
    // Arrange
    let fixture = TurnHistory::legal_large_singleton();
    // Act
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("legal oversized startup");
    let turn = runtime.network.last_basic_filter_turn_for_test();
    // Assert
    assert!(turn.oversized_singleton);
    assert_eq!(turn.work.blocks, 1);
    assert!(turn.work.body_bytes < 1_000_000);
    assert!(turn.work.cloned_bytes > 16 * 1024 * 1024);
    assert_eq!(turn.maybe_progress.expect("owner").current_lag(), 0);
    drop(runtime);
    fixture.history.cleanup();
}

fn injected_budget() -> open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget
{
    use open_bitcoin_core::chainstate::filter_index::catch_up::{BasicIndexTurnBudget, TurnWork};
    let work = TurnWork {
        blocks: 8,
        body_bytes: 1024 * 1024,
        undo_bytes: 1024 * 1024,
        cloned_bytes: 16 * 1024 * 1024,
        script_items: 40 * 257,
        script_bytes: 1024 * 1024,
        encoded_bytes: 1024 * 1024,
        record_operations: 512,
        checkpoint_operations: 1_000_000,
        projection_operations: 256,
    };
    BasicIndexTurnBudget::new(work, TurnWork { blocks: 1, ..work }).expect("injected budget")
}

#[test]
fn phase157_turn_absolute_one_over_refuses_without_progress() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget;
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let before = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    let caps = injected_budget();
    let mut absolute = caps.absolute_singleton();
    absolute.body_bytes = 1;
    let budget =
        BasicIndexTurnBudget::new(caps.normal(), absolute).expect("absolute refusal budget");
    // Act
    let result = runtime
        .network
        .drive_basic_filter_index_turn_with_budget(budget);
    // Assert
    assert!(result.is_err());
    let after = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(
        after.maybe_processed_endpoint(),
        before.maybe_processed_endpoint()
    );
    assert_eq!(
        after.maybe_safe_durable_endpoint(),
        before.maybe_safe_durable_endpoint()
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_body_budget_exact_and_one_under_admit_before_decode() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget;
    for one_under in [false, true] {
        // Arrange
        let fixture = TurnHistory::new(16, 24, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(16),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        let caps = injected_budget();
        let mut normal = caps.normal();
        normal.body_bytes = open_bitcoin_core::codec::encode_block(&fixture.history.blocks[24])
            .expect("wire")
            .len() as u64
            - u64::from(one_under);
        let budget =
            BasicIndexTurnBudget::new(normal, caps.absolute_singleton()).expect("byte budget");
        // Act
        let turn = runtime
            .network
            .drive_basic_filter_index_turn_with_budget(budget)
            .expect("bounded byte turn");
        // Assert
        assert_eq!(turn.work.blocks, 1);
        assert_eq!(turn.body_decodes, 1);
        assert_eq!(turn.generations, 1);
        assert_eq!(turn.oversized_singleton, one_under);
        if !one_under {
            assert!(turn.work.fits(normal));
        }
        drop(runtime);
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_turn_accepted_unflushed_facts_process_ahead_then_genuine_flush_releases_without_record()
{
    // Arrange
    let history = ValidatedHistory::new("turn-accepted-error", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let result = runtime.network.connect_local_block(
        &history.blocks[2],
        open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
        open_bitcoin_core::consensus::ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    );
    result.expect("ordinary accepted connect with existing IfNeeded cadence");
    assert!(
        runtime
            .network
            .accepted_basic_facts_for_test()
            .expect("read")
            .is_some()
    );
    // Act
    let ahead = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("complete accepted facts retained");
    runtime
        .network
        .flush_coins(
            open_bitcoin_core::chainstate::FlushMode::Always,
            open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
            u64::MAX,
        )
        .expect("actual later own flush");
    let release = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("no-record safe release");
    // Assert
    assert_eq!(ahead.work.blocks, 1);
    assert_eq!(ahead.body_reads, 0);
    assert_eq!(
        ahead
            .maybe_progress
            .expect("owner")
            .maybe_processed_endpoint(),
        Some(history.records[2].identity())
    );
    assert_eq!(
        ahead
            .maybe_progress
            .expect("owner")
            .maybe_safe_durable_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(release.work.blocks, 0);
    assert_eq!(release.persistence_batches, 1);
    assert_eq!(
        release
            .maybe_progress
            .expect("owner")
            .maybe_safe_durable_endpoint(),
        Some(history.records[2].identity())
    );
    assert!(
        runtime
            .network
            .accepted_basic_facts_for_test()
            .expect("read")
            .is_none()
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_turn_item_budget_yields_before_next_generation() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexTurnBudget;
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let caps = injected_budget();
    let mut normal = caps.normal();
    normal.script_items = 32 * 257;
    let budget = BasicIndexTurnBudget::new(normal, caps.absolute_singleton()).expect("item budget");
    // Act
    let turn = runtime
        .network
        .drive_basic_filter_index_turn_with_budget(budget)
        .expect("bounded yield");
    // Assert
    assert_eq!(turn.work.blocks, 1);
    assert_eq!(turn.generations, 1);
    assert_eq!(turn.body_decodes, 1);
    assert_eq!(turn.body_reads, 2);
    assert_eq!(turn.examined_script_items, 5);
    assert!(turn.work.script_items <= 32 * 257);
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_startup_stale_disabled_publication_preserves_saved_checkpoint() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let before = runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("safe");
    runtime
        .store()
        .disable_basic_filter_index()
        .expect("interleaved lifecycle invalidation");
    // Act
    let result = runtime.network.drive_basic_filter_index_turn();
    // Assert
    assert!(result.is_err());
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe"),
        before
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_ordered_multiple_idle_turns_keep_old_safe_lock_until_exact_tip() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let first = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(
        first
            .maybe_processed_endpoint()
            .expect("processed")
            .height(),
        23
    );
    assert!(!first.initially_synchronized());
    assert_eq!(
        first.maybe_safe_durable_endpoint().expect("safe").height(),
        15
    );
    // Act
    let second = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("second idle");
    let third = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("third idle");
    let idle = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("caught up idle");
    // Assert
    assert_eq!(second.work.blocks, 8);
    assert_eq!(
        second
            .maybe_progress
            .expect("progress")
            .maybe_processed_endpoint()
            .expect("next")
            .height(),
        31
    );
    assert_eq!(
        second
            .maybe_progress
            .expect("progress")
            .maybe_safe_durable_endpoint()
            .expect("safe")
            .height(),
        15
    );
    assert_eq!(third.work.blocks, 8);
    let complete = third.maybe_progress.expect("progress");
    assert!(complete.initially_synchronized());
    assert_eq!(complete.current_lag(), 0);
    assert_eq!(
        complete
            .maybe_safe_durable_endpoint()
            .expect("safe")
            .height(),
        39
    );
    assert_eq!(idle.work.blocks, 0);
    drop(runtime);
    let reopened = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&fixture.history.path).expect("real reopen"),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("safe reopen");
    assert_eq!(
        reopened
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner")
            .current_lag(),
        0
    );
    drop(reopened);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_startup_consumes_next_height() {
    // Arrange
    let history = ValidatedHistory::new("turn-startup", false);
    let store = history.seed(0);
    // Act
    let runtime = DurableSyncRuntime::open_configured(
        store,
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("configured runtime");
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    // Assert
    assert_eq!(
        progress.maybe_processed_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(
        runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("idle turn")
            .work
            .blocks,
        0
    );
    drop(runtime);
    history.cleanup();
}

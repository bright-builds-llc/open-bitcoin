// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_core::chainstate::filter_index::catch_up::{BasicIndexTurnBudget, TurnWork};
use std::time::Instant;

fn assert_components(components: [TurnWork; 4]) {
    let budget = crate::chainstate::basic_filter_turn_budget().expect("existing budget");
    let caps = [
        crate::storage::fjall_store::filters::PreparedBasicFilterReorg::maximum_work(),
        TurnWork {
            blocks: budget.normal().blocks,
            ..budget.absolute_singleton()
        },
        TurnWork {
            body_bytes: crate::chainstate::filter_index::MAX_ACCEPTED_FACT_BYTES,
            undo_bytes: crate::chainstate::filter_index::MAX_ACCEPTED_FACT_BYTES,
            cloned_bytes: crate::chainstate::filter_index::MAX_ACCEPTED_FACT_BYTES,
            script_items: crate::chainstate::filter_index::MAX_ACCEPTED_FACT_ITEMS,
            script_bytes: crate::chainstate::filter_index::MAX_ACCEPTED_FACT_BYTES,
            ..Default::default()
        },
    ];
    for (work, cap) in components[..3].iter().zip(caps) {
        assert!(work.fits(cap));
    }
    let total = components[0]
        .checked_add(components[1])
        .and_then(|work| work.checked_add(components[2]))
        .expect("checked observed sum");
    let maximum = caps[0]
        .checked_add(caps[1])
        .and_then(|work| work.checked_add(caps[2]))
        .expect("checked sum of existing independent caps");
    assert_eq!(total, components[3]);
    assert!(total.fits(maximum));
}

fn preparation(fixture: &ForkFixture, branch: &ForkBranch) -> (TurnWork, std::time::Duration) {
    let (components, elapsed) = fixture
        .runtime
        .network
        .measure_reorg_preparation_for_test(
            &branch.disconnect,
            &branch.connected,
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect("genuine observed preparation");
    assert_components(components);
    (components[3], elapsed)
}

fn print_turn(
    prefix: usize,
    depth: usize,
    count: usize,
    indexed: usize,
    turn: usize,
    outcome: crate::BasicFilterTurnOutcome,
) {
    let progress = outcome.maybe_progress.expect("owner");
    eprintln!(
        "phase158 TURN prefix={prefix} depth={depth} count={count} seeded={indexed} turn={turn} hold_us={} storage_us={} point_reads={} batches={} batch_bytes={} bodies={} decoded={} undo={} gen={} reused={} processed={:?} safe={:?} work={:?}",
        outcome.authority_hold.as_micros(),
        outcome.storage_elapsed.as_micros(),
        outcome.indexed_point_reads,
        outcome.persistence_batches,
        outcome.batch_bytes,
        outcome.body_reads,
        outcome.body_decodes,
        outcome.undo_borrows,
        outcome.generations,
        outcome.reused_records,
        progress.maybe_processed_endpoint().map(|id| id.height()),
        progress.maybe_safe_durable_endpoint().map(|id| id.height()),
        outcome.work
    );
}

#[test]
#[ignore = "explicit concrete accounting and timings; no wall-time pass/fail ceiling"]
fn phase158_reorg_measurements() {
    for prefix in [16_usize, 128, 512] {
        for depth in [1, 8, 32] {
            for extra in [0, 3] {
                let total = prefix + 32;
                let ancestor = total - depth - 1;
                // Actual post-startup progress is observed. Fixed shared gaps do not
                // accidentally turn a prefix-scaling test into whole backlog work.
                for indexed in [ancestor.saturating_sub(16), ancestor + 1, total] {
                    // Arrange
                    let fixture = ForkFixture::compact(prefix, 32, indexed);
                    let branch = fixture.fork(ancestor, depth + extra, 0x7c);
                    let before = fixture
                        .runtime
                        .network
                        .maybe_basic_index_progress()
                        .expect("old")
                        .expect("owner");
                    let store = fixture.runtime.store();
                    let reads_before = store.basic_filter_point_reads_for_test();
                    let (components, elapsed) = fixture
                        .runtime
                        .network
                        .measure_reorg_preparation_for_test(
                            &branch.disconnect,
                            &branch.connected,
                            fixture.runtime.verify_flags,
                            fixture.runtime.consensus_params,
                        )
                        .expect("genuine measured preparation");
                    let prepare_reads = store.basic_filter_point_reads_for_test() - reads_before;
                    let work = components[3];
                    assert_components(components);
                    let caps =
                        crate::chainstate::basic_filter_turn_budget().expect("existing ceilings");
                    // Act: total production time includes preexisting core stage,
                    // preview/history, mempool and index completion. No delta claim.
                    let start = Instant::now();
                    let reads_before = store.basic_filter_point_reads_for_test();
                    fixture.apply(&branch);
                    let accept_us = start.elapsed().as_micros();
                    let acceptance_reads = store.basic_filter_point_reads_for_test() - reads_before;
                    eprintln!(
                        "phase158 PREP prefix={prefix} depth={depth} count={} seeded={indexed} old_processed={:?} old_safe={:?} prepare_us={} accept_us={accept_us} prepare_reads={prepare_reads} acceptance_reads={acceptance_reads} storage={:?} source={:?} facts={:?} work={work:?}",
                        depth + extra,
                        before.maybe_processed_endpoint().map(|id| id.height()),
                        before.maybe_safe_durable_endpoint().map(|id| id.height()),
                        elapsed.as_micros(),
                        components[0],
                        components[1],
                        components[2]
                    );
                    for turn in 0..256 {
                        let outcome = fixture
                            .runtime
                            .network
                            .drive_basic_filter_index_turn()
                            .expect("ordinary measured turn");
                        assert!(outcome.work.fits(if outcome.oversized_singleton {
                            caps.absolute_singleton()
                        } else {
                            caps.normal()
                        }));
                        print_turn(prefix, depth, depth + extra, indexed, turn, outcome);
                        if outcome.maybe_accepted_lag == Some(0) {
                            break;
                        }
                        assert!(turn < 255, "finite fixture driver");
                    }
                    fixture.cleanup();
                }
            }
        }
    }
}

#[test]
fn phase158_reorg_measurement_fixed_depth_complete_preparation_and_turns_do_not_scan_prefix() {
    let mut maybe_previous = None;
    for prefix in [16, 128, 512] {
        // Arrange: exactly eight disconnected/replacement records, fully indexed.
        let fixture = ForkFixture::compact(prefix, 8, prefix + 8);
        let branch = fixture.fork(prefix - 1, 8, 0x7d);
        // Act
        let (prepared, _) = preparation(&fixture, &branch);
        eprintln!("phase158 FIXED prefix={prefix} preparation={prepared:?}");
        fixture.apply(&branch);
        let outcome = fixture
            .runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("complete replacement turn");
        // Assert: bounded native source decoding is in preparation, not hidden.
        assert!(prepared.body_bytes > 0 && prepared.undo_bytes > 0);
        assert!(prepared.cloned_bytes > prepared.body_bytes);
        assert_eq!(outcome.work.blocks, 8);
        assert_eq!(outcome.persistence_batches, 1);
        let counters = (
            prepared.record_operations,
            prepared.projection_operations,
            outcome.work.record_operations,
            outcome.work.projection_operations,
            outcome.indexed_point_reads,
        );
        if let Some(previous) = maybe_previous {
            assert_eq!(counters, previous);
        }
        maybe_previous = Some(counters);
        fixture.cleanup();
    }
}

#[test]
fn phase158_reorg_measurement_exact_and_one_under_record_acquisition_preserve_effects() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(15, 8, 0x7f);
    fixture.apply(&branch);
    let store = fixture.runtime.store();
    let normal = crate::chainstate::basic_filter_turn_budget()
        .expect("existing budget")
        .normal();
    let measured = store
        .maybe_basic_filter_append_proof_with_budget(normal)
        .expect("measured acquisition")
        .expect("real proof")
        .preparation_work();
    assert!(measured.record_operations > 1);
    eprintln!(
        "phase158 BOUND acquisition_record_operations={}",
        measured.record_operations
    );
    let maybe_before = store.maybe_basic_filter_state().expect("state");
    let locks = store.load_prune_locks().expect("locks");
    // Act / Assert: exact measured physical operation budget, then one below.
    for one_under in [false, true] {
        let limit = TurnWork {
            record_operations: measured.record_operations - u64::from(one_under),
            ..normal
        };
        let result = store.maybe_basic_filter_append_proof_with_budget(limit);
        assert_eq!(result.is_ok(), !one_under);
        assert_eq!(
            store.maybe_basic_filter_state().expect("unchanged state"),
            maybe_before
        );
        assert_eq!(store.load_prune_locks().expect("unchanged locks"), locks);
        for record in &branch.records {
            assert_eq!(
                store
                    .load_basic_filter_record(record.identity().block_hash())
                    .expect("no row effect"),
                None
            );
        }
    }
    fixture.cleanup();
}

#[test]
fn phase158_reorg_measurement_retained_over_budget_gap_refuses_before_preview() {
    // Arrange: all inputs genuinely exist; only the finite source envelope refuses.
    let fixture = ForkFixture::compact(400, 1, 16);
    let branch = fixture.fork(390, 10, 0x80);
    for anchored in &branch.connected {
        fixture
            .runtime
            .store()
            .save_block(&anchored.block, PersistMode::Sync)
            .expect("retained replacement body");
    }
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    let maybe_progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before");
    let pool = fixture.runtime.network.mempool_info().expect("before");
    let maybe_state = fixture
        .runtime
        .store()
        .maybe_basic_filter_state()
        .expect("before");
    let locks = fixture.runtime.store().load_prune_locks().expect("before");
    // Act
    let error = fixture
        .runtime
        .network
        .reorg_to_branch(
            &branch.disconnect,
            &branch.connected,
            ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
            fixture.runtime.verify_flags,
            fixture.runtime.consensus_params,
        )
        .expect_err("existing source budget")
        .to_string();
    // Assert
    assert!(error.contains("BASIC append work budget"), "{error}");
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("after"),
        before
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("after"),
        maybe_progress
    );
    assert_eq!(fixture.runtime.network.mempool_info().expect("after"), pool);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_state()
            .expect("after"),
        maybe_state
    );
    assert_eq!(
        fixture.runtime.store().load_prune_locks().expect("after"),
        locks
    );
    assert!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_append_proof()
            .expect("not frozen")
            .is_some()
    );
    fixture.cleanup();
}

#[test]
fn phase158_reorg_measurement_absolute_acquisition_exhaustion_refuses_before_effects() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(15, 8, 0x7e);
    fixture.apply(&branch);
    let before = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before")
        .expect("owner");
    let maybe_checkpoint = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("before");
    let locks = fixture.runtime.store().load_prune_locks().expect("before");
    let production = crate::chainstate::basic_filter_turn_budget().expect("budget");
    let mut normal = production.normal();
    let mut absolute = production.absolute_singleton();
    normal.checkpoint_operations = 1;
    absolute.checkpoint_operations = 1;
    let budget = BasicIndexTurnBudget::new(normal, absolute).expect("finite admitted test limits");
    // Act
    assert!(
        fixture
            .runtime
            .network
            .drive_basic_filter_index_turn_with_budget(budget)
            .is_err()
    );
    // Assert
    let after = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("after")
        .expect("owner");
    assert_eq!(
        after.maybe_processed_endpoint(),
        before.maybe_processed_endpoint()
    );
    assert_eq!(
        after.maybe_safe_durable_endpoint(),
        before.maybe_safe_durable_endpoint()
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("after"),
        maybe_checkpoint
    );
    assert_eq!(
        fixture.runtime.store().load_prune_locks().expect("after"),
        locks
    );
    for record in &branch.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("no generated rows"),
            None
        );
    }
    fixture.cleanup();
}

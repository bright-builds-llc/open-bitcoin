// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Actual registered whole-turn measurements, including acquisition/completion.

use super::*;
use std::time::Instant;

#[test]
#[ignore = "explicit timing evidence; no wall-time assertion"]
fn phase157_measure_turns() {
    for prefix in [16, 128, 512] {
        // Arrange
        let fixture = TurnHistory::new(prefix, 32, 8_192);
        let store = fixture.seed(prefix);
        drop(store);
        let start = Instant::now();
        let runtime = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&fixture.history.path).expect("cold real reopen"),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        eprintln!(
            "phase157 startup prefix={prefix} elapsed_us={} (full recovery/preflight plus first turn)",
            start.elapsed().as_micros()
        );
        print_turn(
            prefix,
            0,
            runtime.network.last_basic_filter_turn_for_test(),
            None,
        );
        // Act
        for turn in 1..=4 {
            let start = Instant::now();
            let outcome = runtime
                .network
                .drive_basic_filter_index_turn()
                .expect("measured turn");
            print_turn(prefix, turn, outcome, Some(start.elapsed().as_micros()));
        }
        drop(runtime);
        let start = Instant::now();
        let reopened = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&fixture.history.path).expect("warm real reopen"),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        eprintln!(
            "phase157 warm prefix={prefix} elapsed_us={}",
            start.elapsed().as_micros()
        );
        // Assert
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
    let fixture = TurnHistory::legal_large_singleton();
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("genuine legal singleton");
    eprintln!("phase157 legal singleton: consensus-validated 99 additional 9985-byte outputs");
    print_turn(
        16,
        99,
        runtime.network.last_basic_filter_turn_for_test(),
        None,
    );
    drop(runtime);
    fixture.history.cleanup();
}

fn print_turn(
    prefix: usize,
    turn: usize,
    outcome: crate::BasicFilterTurnOutcome,
    maybe_elapsed: Option<u128>,
) {
    let elapsed = maybe_elapsed.map_or_else(
        || "startup_total_reported_separately".to_owned(),
        |elapsed| elapsed.to_string(),
    );
    eprintln!(
        "phase157 prefix={prefix} turn={turn} elapsed_us={elapsed} hold_us={} storage_us={} blocks={} body_read={} decoded={} undo_borrow={} gen={} batch={} batches={} indexed_reads={} examined_items={} examined_bytes={} hashed={} sorted={} work={:?}",
        outcome.authority_hold.as_micros(),
        outcome.storage_elapsed.as_micros(),
        outcome.work.blocks,
        outcome.body_reads,
        outcome.body_decodes,
        outcome.undo_borrows,
        outcome.generations,
        outcome.batch_bytes,
        outcome.persistence_batches,
        outcome.indexed_point_reads,
        outcome.examined_script_items,
        outcome.examined_script_bytes,
        outcome.hashed_items,
        outcome.sorted_items,
        outcome.work
    );
}

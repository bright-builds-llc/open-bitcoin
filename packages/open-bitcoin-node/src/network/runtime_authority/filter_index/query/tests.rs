// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp

//! Included from the existing continuous-history fixture owner; these tests do
//! not expose a second generator or fake acceptance/provenance constructor.

use super::*;
use crate::{BasicFilterQuery, ManagedNetworkHandle, ManagedPeerNetwork, MemoryChainstateStore};

#[test]
fn phase159_basic_query_authority_memory_is_disabled() {
    // Arrange
    let handle = ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        open_bitcoin_network::LocalPeerConfig::default(),
        open_bitcoin_mempool::PolicyConfig::default(),
    ));
    // Act / Assert
    assert_eq!(
        handle
            .basic_filter_query(BlockHash::default())
            .expect("query"),
        BasicFilterQuery::Disabled
    );
    assert_eq!(handle.maybe_basic_index_summary().expect("summary"), None);
}

#[test]
fn phase159_basic_query_authority_disabled_lifecycle_wins_before_unknown_block() {
    // Arrange
    let history = ValidatedHistory::new("query-disabled", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    runtime
        .network
        .disable_basic_filter_index()
        .expect("disable same store");
    // Act / Assert
    assert_eq!(
        runtime
            .network
            .basic_filter_query(BlockHash::from_byte_array([91; 32]))
            .expect("query"),
        BasicFilterQuery::Disabled
    );
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_summary()
            .expect("summary"),
        None
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_query_authority_corruption_is_typed_and_unknown_resolution_precedes_rows() {
    // Arrange
    let history = ValidatedHistory::new("query-corruption", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let hash = history.records[0].identity().block_hash();
    runtime
        .store()
        .clone()
        .remove_validation_filter_for_test(hash)
        .expect("raw clone mutation");
    // Act
    let known = runtime.network.basic_filter_query(hash);
    let unknown = runtime
        .network
        .basic_filter_query(BlockHash::from_byte_array([91; 32]));
    // Assert
    assert!(matches!(
        known,
        Err(crate::BasicFilterQueryError::Storage(
            crate::StorageError::Corruption { .. }
        ))
    ));
    assert_eq!(
        unknown.expect("resolve before row access"),
        BasicFilterQuery::UnknownBlock
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_query_authority_found_during_initial_catch_up_and_legacy_missing() {
    // Arrange
    let fixture = TurnHistory::new(16, 400, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let hash = fixture.history.records[0].identity().block_hash();
    let missing_hash = fixture.history.records[415].identity().block_hash();
    // Act
    let found = runtime.network.basic_filter_query(hash).expect("query");
    let missing = runtime
        .network
        .basic_filter_query(missing_hash)
        .expect("missing");
    let summary = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    // Assert
    assert!(matches!(found, BasicFilterQuery::Found(_)));
    assert!(matches!(
        missing,
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::UnknownLegacy,
            initially_synchronized: false
        }
    ));
    assert!(!summary.synced);
    assert_eq!(
        summary.best_block_height,
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner")
            .maybe_processed_endpoint()
            .expect("processed")
            .height()
    );
    assert_eq!(
        runtime
            .network
            .basic_filter_query(BlockHash::from_byte_array([91; 32]))
            .expect("unknown"),
        BasicFilterQuery::UnknownBlock
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase159_basic_query_authority_captures_unflushed_target_and_summary_processed_latch() {
    // Arrange
    let history = ValidatedHistory::new("query-unflushed", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let before = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    assert!(before.synced);
    runtime
        .network
        .connect_local_block(
            &history.blocks[2],
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("accepted");
    // Act
    let query = runtime
        .network
        .basic_filter_query(history.records[2].identity().block_hash())
        .expect("query");
    let lag = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    // Assert
    let BasicFilterQuery::Pending(frontier) = query else {
        panic!("expected opaque captured target: {query:?}");
    };
    assert_eq!(frontier.accepted_height(), 2);
    assert_eq!(
        frontier.accepted_hash(),
        history.records[2].identity().block_hash()
    );
    assert_eq!(lag, before);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("process accepted facts");
    let ahead = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    assert!(ahead.synced);
    assert_eq!(ahead.best_block_height, 2);
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("progress")
            .expect("owner")
            .maybe_safe_durable_endpoint()
            .expect("safe")
            .height(),
        1
    );
    assert!(matches!(
        runtime
            .network
            .basic_filter_query(frontier.accepted_hash())
            .expect("found"),
        BasicFilterQuery::Found(_)
    ));
    assert_eq!(frontier.accepted_height(), 2);
    drop(runtime);
    let reopened = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&history.path).expect("cold open"),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("recovery");
    let conservative = reopened
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    assert_eq!(conservative.best_block_height, 1);
    assert!(conservative.synced);
    let (stale, hold, elapsed) = reopened
        .network
        .measure_basic_filter_query_for_test(frontier.accepted_hash())
        .expect("retained stale");
    let BasicFilterQuery::Found(stale) = stale else {
        panic!("retained stale");
    };
    assert_eq!(stale.work().record_reads, 2);
    eprintln!(
        "phase159 QUERY stale height=2 elapsed_us={} hold_us={} work={:?}",
        elapsed.as_micros(),
        hold.as_micros(),
        stale.work()
    );
    drop(reopened);
    history.cleanup();
}

#[test]
fn phase159_basic_query_authority_pending_identity_survives_before_history_commit() {
    // Arrange
    let history = ValidatedHistory::new("query-pending-history", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    runtime.store().set_validation_history_fault(
        crate::storage::fjall_store::validation_history::HistoryPublicationFault::BeforeCommit,
    );
    let hash = history.records[2].identity().block_hash();
    // Act
    assert!(
        runtime
            .network
            .connect_local_block(
                &history.blocks[2],
                open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                open_bitcoin_core::consensus::ConsensusParams {
                    coinbase_maturity: 1,
                    ..Default::default()
                }
            )
            .is_err()
    );
    let query = runtime
        .network
        .basic_filter_query(hash)
        .expect("live identity and provenance");
    // Assert
    assert!(
        runtime
            .store()
            .maybe_validation_history_record(hash)
            .expect("durable row")
            .is_none()
    );
    assert!(
        matches!(query, BasicFilterQuery::Pending(frontier) if frontier.accepted_hash() == hash)
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_query_authority_empty_processed_summary_uses_zero_and_initial_missing_is_prompt()
{
    use open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexProgress;
    // Arrange
    let history = ValidatedHistory::new("query-zero", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("progress")
        .expect("owner");
    let empty = BasicIndexProgress::new(
        progress.generation(),
        progress.branch_identity(),
        progress.accepted_target(),
        None,
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("initial indexing snapshot");
    runtime
        .network
        .install_basic_index_owner_for_test(empty)
        .expect("test snapshot");
    runtime
        .network
        .connect_local_block(
            &history.blocks[2],
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine accepted live identity");
    // Act
    let summary = runtime
        .network
        .maybe_basic_index_summary()
        .expect("summary")
        .expect("enabled");
    let query = runtime
        .network
        .basic_filter_query(history.records[2].identity().block_hash())
        .expect("query");
    // Assert
    assert_eq!(
        summary,
        crate::BasicIndexSummary {
            synced: false,
            best_block_height: 0
        }
    );
    assert!(matches!(
        query,
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::ScriptsValid,
            initially_synchronized: false
        }
    ));
    assert!(matches!(
        runtime
            .network
            .basic_filter_query(history.records[0].identity().block_hash())
            .expect("stored genesis"),
        BasicFilterQuery::Found(_)
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_query_measure_continuous_heights_and_legal_singleton() {
    // Arrange
    for (label, fixture, prefix, heights) in [
        (
            "continuous",
            TurnHistory::new(16, 385, 1),
            401,
            vec![0, 20, 400],
        ),
        (
            "legal-singleton",
            TurnHistory::legal_large_singleton(),
            17,
            vec![16],
        ),
    ] {
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(prefix),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("runtime");
        for height in heights {
            let hash = fixture.history.records[height].identity().block_hash();
            let before = runtime.store().basic_filter_point_reads_for_test();
            // Act
            let (query, hold, elapsed) = runtime
                .network
                .measure_basic_filter_query_for_test(hash)
                .expect("query");
            // Assert
            let BasicFilterQuery::Found(view) = query else {
                panic!("found");
            };
            assert_eq!(
                view.encoded_bytes(),
                fixture.history.records[height].encoded_bytes()
            );
            assert_eq!(view.work().record_reads, if height == 0 { 1 } else { 2 });
            assert_eq!(
                runtime.store().basic_filter_point_reads_for_test() - before,
                view.work().record_reads + 1
            );
            assert_eq!(
                view.work().response_hex_bytes,
                2 * view.encoded_bytes().len() + 64
            );
            eprintln!(
                "phase159 QUERY label={label} height={height} elapsed_us={} hold_us={} metadata_reads=1 provenance_reads=0 body_reads=0 undo_reads=0 generations=0 work={:?}",
                elapsed.as_micros(),
                hold.as_micros(),
                view.work()
            );
        }
        let before = runtime.store().basic_filter_point_reads_for_test();
        runtime
            .network
            .maybe_basic_index_summary()
            .expect("summary");
        assert_eq!(
            runtime.store().basic_filter_point_reads_for_test() - before,
            1
        );
        drop(runtime);
        fixture.history.cleanup();
    }
}

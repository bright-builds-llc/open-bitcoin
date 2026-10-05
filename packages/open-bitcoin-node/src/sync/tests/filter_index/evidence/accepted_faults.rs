// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn assert_accepted(runtime: &DurableSyncRuntime, history: &ValidatedHistory) {
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("owner read")
        .expect("ordinary owner");
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("tip")
            .expect("accepted")
            .height,
        2
    );
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(
        runtime
            .network
            .accepted_basic_facts_for_test()
            .expect("facts")
            .expect("complete accepted facts")
            .spent_scripts,
        vec![vec![0x51], vec![0x52]]
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe saved"),
        Some(checkpoint(&history.records[1]))
    );
    assert_eq!(
        runtime.store().load_prune_locks().expect("covering lock"),
        vec![
            IndexInputProtection::FromHeight(2)
                .maybe_prune_lock()
                .expect("lock")
        ]
    );
}

#[test]
fn phase157_store_fault_ordinary_accepted_later_undo_coins_metadata_errors_reopen_truthfully() {
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange: the ordinary direct consumer retains the default IfNeeded cadence.
        let history = ValidatedHistory::new("evidence-ordinary-checkpoint-fault", false);
        let runtime = DurableSyncRuntime::open_configured(
            history.seed(1),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("genuine owner");
        runtime
            .network
            .connect_local_block(&history.blocks[2], ScriptVerifyFlags::P2SH, params())
            .expect("accepted unflushed block");
        assert_accepted(&runtime, &history);
        assert_eq!(
            runtime
                .store()
                .coins_view()
                .best_block()
                .expect("no forced connect flush"),
            Some(history.old.active_chain[1].block_hash)
        );
        runtime.store().set_basic_filter_fault(fault);
        // Act: a real normal Always request reaches the concrete effect writer.
        let result =
            runtime
                .network
                .flush_coins(FlushMode::Always, FlushPolicyTime::new(0), u64::MAX);
        // Assert: acceptance is visible independently of the failing later disposition.
        assert!(result.is_err(), "real {fault:?} writer must fail");
        assert_accepted(&runtime, &history);
        assert!(
            runtime.network.drive_basic_filter_index_turn().is_err(),
            "poison cannot publish later backlog first"
        );
        let changed_coins = fault == FilterPublicationFault::BeforeChainMeta;
        assert_eq!(
            runtime
                .store()
                .coins_view()
                .best_block()
                .expect("actual B boundary"),
            Some(history.full.active_chain[usize::from(changed_coins) + 1].block_hash)
        );
        assert_eq!(
            runtime
                .store()
                .load_chain_meta_for_open()
                .expect("old metadata")
                .0,
            history.old.active_chain
        );
        drop(runtime);
        if changed_coins {
            assert_refusal_unchanged(
                &history.path,
                "coins best block differs from durable metadata",
            );
        } else {
            let runtime = configured(&history.path).expect("valid old prefix recovers");
            assert_eq!(
                runtime
                    .network
                    .maybe_chain_tip()
                    .expect("tip")
                    .expect("old durable tip")
                    .height,
                1
            );
            assert_eq!(
                runtime
                    .store()
                    .maybe_basic_filter_checkpoint()
                    .expect("no phantom"),
                Some(checkpoint(&history.records[1]))
            );
            assert_eq!(
                runtime
                    .store()
                    .load_basic_filter_record(history.records[2].identity().block_hash())
                    .expect("no suffix"),
                None
            );
            drop(runtime);
        }
        history.cleanup();
    }
}

#[test]
fn phase157_store_fault_requested_body_error_keeps_accepted_target_and_real_old_reopen() {
    // Arrange: remove future body; ordinary requested peer path must attempt its save.
    let history = ValidatedHistory::new("evidence-requested-body-fault", false);
    drop(history.seed(1));
    remove_one_mate(&history.path, block_hash(&history.blocks[2].header), true);
    let mut runtime = configured(&history.path).expect("only old history required");
    runtime.consensus_params = params();
    runtime
        .store()
        .set_basic_filter_fault(FilterPublicationFault::BeforeBody);
    let block = &history.blocks[2];
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..Default::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![block.header.clone()],
        }),
        WireNetworkMessage::Block(block.clone()),
    ]]);
    // Act
    let report = runtime
        .sync_once(&mut transport, i64::from(block.header.time))
        .expect("ordinary peer failure report");
    // Assert
    assert!(getdata_block_hashes(&transport.sent_messages()).contains(&block_hash(&block.header)));
    assert_eq!(report.failed_peers, 1);
    assert_eq!(report.blocks_received, 0);
    assert!(report.peer_outcomes[0].maybe_error.is_some());
    assert_accepted(&runtime, &history);
    assert!(
        !runtime
            .store()
            .has_block(block_hash(&block.header))
            .expect("save actually failed")
    );
    assert!(runtime.network.drive_basic_filter_index_turn().is_err());
    drop(runtime);
    let runtime = configured(&history.path)
        .expect("old durable prefix can reopen without failed future body");
    assert_eq!(
        runtime
            .network
            .maybe_chain_tip()
            .expect("tip")
            .expect("old")
            .height,
        1
    );
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("safe"),
        Some(checkpoint(&history.records[1]))
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_store_fault_append_before_and_after_commit_reopen_identical_retry_without_duplicate_rows()
 {
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let fixture = TurnHistory::new(16, 24, 1);
        let runtime = DurableSyncRuntime::open_configured(
            fixture.seed(16),
            sync_config(),
            BasicFilterStartupMode::Enabled,
        )
        .expect("first turn");
        let before = runtime
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner");
        runtime.store().set_basic_filter_fault(fault);
        // Act
        let result = runtime.network.drive_basic_filter_index_turn();
        // Assert
        assert!(result.is_err(), "{fault:?}");
        let after = runtime
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner");
        assert_eq!(after.accepted_target(), before.accepted_target());
        assert_eq!(
            after.maybe_processed_endpoint(),
            before.maybe_processed_endpoint()
        );
        assert_eq!(
            after.maybe_safe_durable_endpoint(),
            before.maybe_safe_durable_endpoint()
        );
        drop(runtime);
        let saved = snapshot_index(&fixture.history.path)
            .into_iter()
            .filter(|(key, _)| key.starts_with(codec::RECORD_PREFIX.as_bytes()))
            .collect::<RawRows>();
        let expected = if fault == FilterPublicationFault::AfterCommit {
            32
        } else {
            24
        };
        assert_eq!(saved.len(), expected, "actual commit boundary");
        let runtime = configured(&fixture.history.path).expect("genuine safe-prefix replay");
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("conservative checkpoint"),
            Some(checkpoint(&fixture.history.records[15]))
        );
        finish(&runtime);
        assert_eq!(
            runtime
                .store()
                .maybe_basic_filter_checkpoint()
                .expect("achieved exact tip"),
            Some(checkpoint(&fixture.history.records[39]))
        );
        drop(runtime);
        let complete = snapshot_index(&fixture.history.path)
            .into_iter()
            .filter(|(key, _)| key.starts_with(codec::RECORD_PREFIX.as_bytes()))
            .collect::<RawRows>();
        assert_eq!(complete.len(), 40);
        for row in &saved {
            assert_eq!(
                complete.iter().find(|(key, _)| key == &row.0),
                Some(row),
                "identical retry preserves each immutable byte"
            );
        }
        let runtime = configured(&fixture.history.path).expect("identical repeat reopen");
        assert_eq!(
            runtime
                .network
                .drive_basic_filter_index_turn()
                .expect("idle identical retry")
                .work
                .blocks,
            0
        );
        drop(runtime);
        assert_eq!(
            snapshot_index(&fixture.history.path)
                .into_iter()
                .filter(|(key, _)| key.starts_with(codec::RECORD_PREFIX.as_bytes()))
                .collect::<RawRows>(),
            complete
        );
        fixture.history.cleanup();
    }
}

#[test]
fn phase157_store_fault_disabled_prepared_completion_cannot_publish_or_release() {
    // Arrange
    let fixture = TurnHistory::new(16, 24, 1);
    let runtime = DurableSyncRuntime::open_configured(
        fixture.seed(16),
        sync_config(),
        BasicFilterStartupMode::Enabled,
    )
    .expect("first turn");
    let budget = open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
        blocks: 8,
        body_bytes: 1024 * 1024,
        undo_bytes: 4 * 1024 * 1024,
        cloned_bytes: 16 * 1024 * 1024,
        script_items: 4 * 1024 * 1024,
        script_bytes: 32 * 1024 * 1024,
        encoded_bytes: 1024 * 1024,
        record_operations: 512,
        checkpoint_operations: 1_000_000,
        projection_operations: 256,
    };
    let proof = runtime
        .store()
        .maybe_basic_filter_append_proof_with_budget(budget)
        .expect("proof")
        .expect("sealed store authority");
    let prepared = runtime
        .store()
        .prepare_basic_filter_append(proof, &fixture.history.records[24..32])
        .expect("prepared suffix");
    runtime
        .network
        .disable_basic_filter_index()
        .expect("trusted disable");
    let checkpoint_before = runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("old safe");
    // Act
    assert!(
        runtime
            .store()
            .complete_basic_filter_append(prepared)
            .is_err()
    );
    assert!(runtime.network.drive_basic_filter_index_turn().is_err());
    // Assert
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("unchanged"),
        checkpoint_before
    );
    drop(runtime);
    let before = snapshot_store(&fixture.history.path);
    let runtime = DurableSyncRuntime::open_configured(
        FjallNodeStore::open(&fixture.history.path).expect("closed reopen"),
        sync_config(),
        BasicFilterStartupMode::Disabled,
    )
    .expect("disabled recovery");
    assert_eq!(
        runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("retained prefix"),
        checkpoint_before
    );
    drop(runtime);
    assert_eq!(snapshot_store(&fixture.history.path), before);
    fixture.history.cleanup();
}

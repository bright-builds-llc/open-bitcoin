// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Actual bounded driver failure, retained facts and real reopen controls.

use super::*;

#[test]
fn phase157_turn_mutated_body_pauses_and_retry_preserves_accepted_target() {
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
    let block = &fixture.history.blocks[24];
    // Canonical stored keys use forward raw bytes; reuse the existing key codec.
    let key =
        codec::record_key(block_hash(&block.header)).replacen(codec::RECORD_PREFIX, "block:", 1);
    runtime
        .store()
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, vec![0])
        .expect("body corruption");
    // Act
    let result = runtime.network.drive_basic_filter_index_turn();
    // Assert
    assert!(result.is_err());
    let paused = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(paused.accepted_target(), before.accepted_target());
    assert_eq!(
        paused.maybe_processed_endpoint(),
        before.maybe_processed_endpoint()
    );
    assert_eq!(
        paused.maybe_safe_durable_endpoint(),
        before.maybe_safe_durable_endpoint()
    );
    runtime
        .store()
        .save_block(block, PersistMode::Sync)
        .expect("restore same accepted body");
    let resumed = runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("bounded same-identity retry");
    assert_eq!(resumed.work.blocks, 8);
    assert_eq!(
        resumed.maybe_progress.expect("owner").accepted_target(),
        before.accepted_target()
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase157_turn_accepted_writer_error_refuses_poison_and_retains_complete_facts() {
    // Arrange
    let history = ValidatedHistory::new("turn-accepted-poison", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    runtime
        .network
        .force_basic_index_flush_for_test()
        .expect("test pressure");
    runtime.store().set_basic_filter_fault(
        crate::storage::fjall_store::filters::FilterPublicationFault::BeforeUndo,
    );
    let result = runtime.network.connect_local_block(
        &history.blocks[2],
        open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
        open_bitcoin_core::consensus::ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        },
    );
    assert!(result.is_err());
    // Act
    let turn = runtime.network.drive_basic_filter_index_turn();
    // Assert
    assert!(turn.is_err());
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(
        progress.maybe_processed_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(history.records[1].identity())
    );
    assert_eq!(
        runtime
            .network
            .accepted_basic_facts_for_test()
            .expect("facts")
            .expect("retained")
            .spent_scripts,
        vec![vec![0x51], vec![0x52]]
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase157_turn_real_append_faults_preserve_owner_and_reopen_from_safe_prefix() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
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
        let reopened = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&fixture.history.path).expect("real fault reopen"),
            sync_config(),
            crate::chainstate::BasicFilterStartupMode::Enabled,
        )
        .expect("fenced replay");
        assert_eq!(
            reopened
                .network
                .maybe_basic_index_progress()
                .expect("read")
                .expect("owner")
                .maybe_safe_durable_endpoint()
                .expect("safe")
                .height(),
            15
        );
        drop(reopened);
        fixture.history.cleanup();
    }
}

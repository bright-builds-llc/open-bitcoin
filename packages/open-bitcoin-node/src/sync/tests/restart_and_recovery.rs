// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_once_continues_header_batches_when_peer_advertises_more_work() {
    // Arrange
    let path = temp_store_path("header-batches");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let child = header(block_hash(&genesis), 2);
    let grandchild = header(block_hash(&child), 3);
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 2,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis],
        }),
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![child, grandchild],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_166)
        .expect("sync");

    // Assert
    assert_eq!(summary.best_header_height, 2);
    assert_eq!(summary.best_block_height, 0);
    let getheaders_requests = transport
        .sent_messages()
        .into_iter()
        .filter(|message| matches!(message, WireNetworkMessage::GetHeaders { .. }))
        .count();
    assert!(getheaders_requests >= 2);

    remove_dir_if_exists(&path);
}

#[test]
fn same_datadir_reopen_seeds_headers_from_durable_store() {
    // Arrange
    let path = temp_store_path("resume");
    remove_dir_if_exists(&path);
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 11);
    let child = header(block_hash(&genesis), 12);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        store
            .save_header_entries(
                &[
                    open_bitcoin_network::HeaderEntry {
                        block_hash: block_hash(&genesis),
                        header: genesis.clone(),
                        height: 0,
                        chain_work: 1,
                    },
                    open_bitcoin_network::HeaderEntry {
                        block_hash: block_hash(&child),
                        header: child,
                        height: 1,
                        chain_work: 2,
                    },
                ],
                PersistMode::Sync,
            )
            .expect("save headers");
    }

    // Act
    let store = FjallNodeStore::open(&path).expect("reopen store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime.snapshot_summary();
    let status = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_176)
        .expect("durable status after restart");

    // Assert
    assert_eq!(summary.best_header_height, 1);
    assert!(matches!(
        status.sync.sync_progress,
        FieldAvailability::Available(SyncProgress {
            header_height: 1,
            ..
        })
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_projects_storage_first_recovery_category() {
    // Arrange
    let path = temp_store_path("storage-first-recovery-category");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let metadata = RuntimeMetadata {
        maybe_last_recovery_action: Some(StorageRecoveryAction::Repair),
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary =
        summary_with_peer_failure(PeerFailureReason::Stall, "peer stalled waiting for headers");

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_182)
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption)
    );
    assert_eq!(
        state.sync.recovery_action,
        FieldAvailability::available(StorageRecoveryAction::Repair.operator_message().to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_storage_metadata_beats_peer_network_last_error_detail() {
    // Arrange
    let path = temp_store_path("storage-metadata-beats-peer-error");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let metadata = RuntimeMetadata {
        maybe_last_recovery_action: Some(StorageRecoveryAction::Repair),
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&metadata, PersistMode::Sync)
        .expect("save runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("peer stalled waiting for headers".to_string()),
            1_777_225_183,
        )
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_projects_storage_lock_category_from_last_error() {
    // Arrange
    let path = temp_store_path("storage-lock-last-error");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("database lock contention".to_string()),
            1_777_225_184,
        )
        .expect("durable status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StorageLockContention)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn durable_sync_state_distinguishes_clean_and_unclean_shutdown_category() {
    // Arrange
    let path = temp_store_path("shutdown-recovery-category");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let clean_metadata = RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    };
    store
        .save_runtime_metadata(&clean_metadata, PersistMode::Sync)
        .expect("save clean runtime metadata");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = SyncRunSummary::empty(0, 0, 1);

    // Act
    let clean_state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Stopped, None, 1_777_225_185)
        .expect("clean durable status");
    let unclean_metadata = RuntimeMetadata {
        last_clean_shutdown: false,
        ..RuntimeMetadata::default()
    };
    runtime
        .store()
        .save_runtime_metadata(&unclean_metadata, PersistMode::Sync)
        .expect("save unclean runtime metadata");
    let unclean_state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Recovering,
            None,
            1_777_225_186,
        )
        .expect("unclean durable status");

    // Assert
    assert_eq!(
        clean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::CleanShutdown)
    );
    assert_eq!(
        unclean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::UncleanShutdown)
    );

    remove_dir_if_exists(&path);
}

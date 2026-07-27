// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase71_same_datadir_resume_matrix_covers_clean_unclean_mid_download_mid_connect_and_stale_inflight()
 {
    // Arrange
    let clean_shutdown = "clean_shutdown";
    let clean_path = temp_store_path(clean_shutdown);
    remove_dir_if_exists(&clean_path);
    let clean_store = FjallNodeStore::open(&clean_path).expect("clean store");
    clean_store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: true,
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save clean shutdown metadata");
    let clean_runtime =
        DurableSyncRuntime::open(clean_store, sync_config()).expect("clean runtime");

    let unclean_shutdown = "unclean_shutdown";
    let unclean_path = temp_store_path(unclean_shutdown);
    remove_dir_if_exists(&unclean_path);
    let unclean_store = FjallNodeStore::open(&unclean_path).expect("unclean store");
    unclean_store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: false,
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save unclean shutdown metadata");
    let unclean_runtime =
        DurableSyncRuntime::open(unclean_store, sync_config()).expect("unclean runtime");

    let mid_download_interruption = "mid_download_interruption";
    let mid_download_path = temp_store_path(mid_download_interruption);
    remove_dir_if_exists(&mid_download_path);
    let mid_download_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let mid_download_child_one = build_block(block_hash(&mid_download_genesis.header), 1);
    let mid_download_child_two = build_block(block_hash(&mid_download_child_one.header), 2);
    save_chain_headers_snapshot_and_blocks(
        &mid_download_path,
        &[
            (&mid_download_genesis, 0),
            (&mid_download_child_one, 1),
            (&mid_download_child_two, 2),
        ],
        &[(&mid_download_genesis, 0)],
        &[(&mid_download_genesis, 0), (&mid_download_child_one, 1)],
    );

    let mid_connect_interruption = "mid_connect_interruption";
    let mid_connect_path = temp_store_path(mid_connect_interruption);
    remove_dir_if_exists(&mid_connect_path);
    let mid_connect_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let mid_connect_child = build_block(block_hash(&mid_connect_genesis.header), 1);
    let mid_connect_missing = build_block(block_hash(&mid_connect_child.header), 2);
    let mid_connect_child_hash = block_hash(&mid_connect_child.header);
    let mid_connect_missing_hash = block_hash(&mid_connect_missing.header);
    save_chain_headers_snapshot_and_blocks(
        &mid_connect_path,
        &[
            (&mid_connect_genesis, 0),
            (&mid_connect_child, 1),
            (&mid_connect_missing, 2),
        ],
        &[(&mid_connect_genesis, 0), (&mid_connect_child, 1)],
        &[(&mid_connect_genesis, 0), (&mid_connect_child, 1)],
    );

    let stale_inflight_after_reopen = "stale_inflight_after_reopen";
    let stale_inflight_path = temp_store_path(stale_inflight_after_reopen);
    remove_dir_if_exists(&stale_inflight_path);
    let stale_genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let stale_child = build_block(block_hash(&stale_genesis.header), 1);
    let stale_missing = build_block(block_hash(&stale_child.header), 2);
    let stale_missing_hash = block_hash(&stale_missing.header);
    save_chain_headers_snapshot_and_blocks(
        &stale_inflight_path,
        &[(&stale_genesis, 0), (&stale_child, 1), (&stale_missing, 2)],
        &[(&stale_genesis, 0), (&stale_child, 1)],
        &[(&stale_genesis, 0), (&stale_child, 1)],
    );

    // Act
    let clean_state = clean_runtime
        .durable_sync_state_for_summary(
            &SyncRunSummary::empty(0, 0, 1),
            SyncLifecycleState::Stopped,
            None,
            1_777_225_220,
        )
        .expect("clean shutdown state");
    let unclean_state = unclean_runtime
        .durable_sync_state_for_summary(
            &SyncRunSummary::empty(0, 0, 1),
            SyncLifecycleState::Recovering,
            None,
            1_777_225_221,
        )
        .expect("unclean shutdown state");

    let mid_download_store = FjallNodeStore::open(&mid_download_path).expect("mid-download store");
    let mid_download_runtime =
        DurableSyncRuntime::open(mid_download_store, sync_config()).expect("mid-download runtime");
    let mid_download_summary = mid_download_runtime.snapshot_summary();
    let mid_download_state = mid_download_runtime
        .durable_sync_state_for_summary(
            &mid_download_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(mid_download_child_two.header.time),
        )
        .expect("mid-download state");

    let mid_connect_store = FjallNodeStore::open(&mid_connect_path).expect("mid-connect store");
    let mut mid_connect_runtime =
        DurableSyncRuntime::open(mid_connect_store, sync_config()).expect("mid-connect runtime");
    let mid_connect_summary_before = mid_connect_runtime.snapshot_summary();
    let mut mid_connect_transport = ScriptedTransport::new(vec![version_verack_script(2)]);
    let mid_connect_summary_after = mid_connect_runtime
        .sync_once(
            &mut mid_connect_transport,
            i64::from(mid_connect_missing.header.time),
        )
        .expect("mid-connect resume sync");
    let mid_connect_requested = getdata_block_hashes(&mid_connect_transport.sent_messages());

    let stale_store = FjallNodeStore::open(&stale_inflight_path).expect("stale in-flight store");
    let mut stale_runtime =
        DurableSyncRuntime::open(stale_store, sync_config()).expect("stale in-flight runtime");
    stale_runtime.inflight_blocks.insert(stale_missing_hash);
    let stale_summary = SyncRunSummary::empty(1, 1, 1);
    let stale_state = stale_runtime
        .durable_sync_state_for_summary(
            &stale_summary,
            SyncLifecycleState::Active,
            None,
            i64::from(stale_missing.header.time),
        )
        .expect("stale in-flight state");
    drop(stale_runtime);
    let reopened_stale_store =
        FjallNodeStore::open(&stale_inflight_path).expect("reopened stale store");
    let reopened_stale_runtime = DurableSyncRuntime::open(reopened_stale_store, sync_config())
        .expect("reopened stale runtime");
    let reopened_stale_summary = reopened_stale_runtime.snapshot_summary();

    // Assert
    assert_eq!(
        clean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::CleanShutdown),
        "{clean_shutdown}"
    );
    assert_eq!(
        unclean_state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::UncleanShutdown),
        "{unclean_shutdown}"
    );

    assert_eq!(mid_download_summary.best_header_height, 2);
    assert_eq!(mid_download_summary.downloaded_block_height, 1);
    assert_eq!(mid_download_summary.best_block_height, 0);
    assert_eq!(
        mid_download_summary.maybe_downloaded_block_hash,
        Some(block_hash_hex(block_hash(&mid_download_child_one.header)))
    );
    let FieldAvailability::Available(mid_download_pressure) =
        mid_download_state.sync.resource_pressure
    else {
        panic!("missing {mid_download_interruption} resource pressure");
    };
    assert_eq!(mid_download_pressure.blocks_in_flight, 0);

    assert_eq!(mid_connect_summary_before.best_header_height, 2);
    assert_eq!(mid_connect_summary_before.best_block_height, 1);
    assert_eq!(
        mid_connect_summary_before.maybe_connected_block_hash,
        Some(block_hash_hex(mid_connect_child_hash))
    );
    assert_eq!(
        mid_connect_summary_before.maybe_validated_active_chain_work,
        Some("2".to_string())
    );
    assert_eq!(mid_connect_summary_after.best_block_height, 1);
    assert!(!mid_connect_requested.contains(&mid_connect_child_hash));
    assert!(mid_connect_requested.contains(&mid_connect_missing_hash));

    assert_no_progress_status(
        &stale_state,
        NoProgressDiagnosis::StaleInflightCleanup,
        "Wait for stale in-flight block cleanup and reassignment.",
    );
    let FieldAvailability::Available(stale_pressure) = stale_state.sync.resource_pressure else {
        panic!("missing {stale_inflight_after_reopen} resource pressure");
    };
    assert_eq!(stale_pressure.blocks_in_flight, 1);
    assert!(reopened_stale_runtime.inflight_blocks.is_empty());
    assert_eq!(reopened_stale_summary.downloaded_block_height, 1);
    assert_eq!(reopened_stale_summary.best_block_height, 1);
    assert_eq!(
        reopened_stale_summary.maybe_connected_block_hash,
        Some(block_hash_hex(block_hash(&stale_child.header)))
    );

    drop(clean_runtime);
    drop(unclean_runtime);
    drop(mid_download_runtime);
    drop(mid_connect_runtime);
    drop(reopened_stale_runtime);
    remove_dir_if_exists(&clean_path);
    remove_dir_if_exists(&unclean_path);
    remove_dir_if_exists(&mid_download_path);
    remove_dir_if_exists(&mid_connect_path);
    remove_dir_if_exists(&stale_inflight_path);
}

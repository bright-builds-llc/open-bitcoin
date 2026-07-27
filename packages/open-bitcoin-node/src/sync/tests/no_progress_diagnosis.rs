// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase70_no_progress_status_projects_at_tip() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-at-tip");
    remove_dir_if_exists(&path);
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    let child = build_block(block_hash(&genesis.header), 1);
    save_best_chain_with_active_blocks(
        &path,
        &[(&genesis, 0), (&child, 1)],
        &[(&genesis, 0), (&child, 1)],
    );
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            None,
            i64::from(child.header.time),
        )
        .expect("durable at-tip status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::CurrentAtBestKnownTip,
        "Confirm current-at-tip evidence; no sync action is required.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_branch_competition_awaiting_bodies() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-branch-competition");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(3, 2, 1);
    summary.maybe_reconcile_progress =
        Some(SyncReconcileProgress::BranchCompetitionAwaitingBodies {
            missing_count: 2,
            first_missing_height: 2,
            first_missing_hash: "11".repeat(32),
        });

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_158)
        .expect("durable branch competition status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::BranchCompetitionAwaitingBodies,
        "Wait for replacement branch block bodies before reorg.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_peer_backoff() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-peer-backoff");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Waiting,
        2,
        Some(PeerFailureReason::RetryBackoff),
        Some("peer waiting for retry backoff".to_string()),
    ));

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_159)
        .expect("durable peer backoff status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::PeerBackoff,
        "Wait for retry backoff or try another configured peer.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_stale_inflight_cleanup() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-stale-inflight");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime
        .inflight_blocks
        .insert(BlockHash::from_byte_array([17_u8; 32]));
    let summary = SyncRunSummary::empty(1, 1, 1);

    // Act
    let state = runtime
        .durable_sync_state_for_summary(&summary, SyncLifecycleState::Active, None, 1_777_225_160)
        .expect("durable stale in-flight status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::StaleInflightCleanup,
        "Wait for stale in-flight block cleanup and reassignment.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_no_progress_status_projects_storage_or_resource_blocker() {
    // Arrange
    let path = temp_store_path("phase70-no-progress-storage-blocker");
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
            1_777_225_161,
        )
        .expect("durable storage blocker status");

    // Assert
    assert_no_progress_status(
        &state,
        NoProgressDiagnosis::StorageOrResourceBlocked,
        "Inspect storage health, free disk space for the selected datadir, or increase bounded resource limits.",
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_storage_resource_pressure_outranks_peer_retry_advice() {
    // Arrange
    let path = temp_store_path("phase78-storage-outranks-peer-retry");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let mut summary = SyncRunSummary::empty(0, 0, 1);
    summary.peer_outcomes.push(peer_outcome(
        SyncPeerAddress::manual("127.0.0.1", 18_444),
        PeerSyncState::Waiting,
        2,
        Some(PeerFailureReason::RetryBackoff),
        Some("peer waiting for retry backoff".to_string()),
    ));

    // Act
    let state = runtime
        .durable_sync_state_for_summary(
            &summary,
            SyncLifecycleState::Active,
            Some("resource limit: storage cache exhausted".to_string()),
            1_777_225_162,
        )
        .expect("durable storage-precedence status");

    // Assert
    assert_eq!(
        state.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::ResourceExhaustion)
    );
    assert_eq!(
        state.sync.no_progress_diagnosis,
        FieldAvailability::available(NoProgressDiagnosis::StorageOrResourceBlocked)
    );
    let stall = available_stall_diagnosis(&state);
    assert_eq!(
        serialized_label(stall.stalled_subsystem),
        "storage_or_resource_pressure"
    );
    assert_eq!(
        stall.stalled_subsystem,
        StalledSubsystem::StorageOrResourcePressure
    );
    assert_eq!(
        stall.maybe_recovery_category,
        Some(SyncRecoveryCategory::ResourceExhaustion)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase78_operator_stop_and_shutdown_classify_local_subsystems() {
    // Arrange
    let path = temp_store_path("phase78-local-stop-classification");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let cases = [
        (
            SyncStopReason::OperatorPaused,
            "operator_stop",
            StalledSubsystem::OperatorStop,
        ),
        (
            SyncStopReason::ShutdownRequested,
            "local_shutdown",
            StalledSubsystem::LocalShutdown,
        ),
    ];

    // Act
    let states = cases
        .iter()
        .enumerate()
        .map(|(index, (stop_reason, _, _))| {
            let mut summary = SyncRunSummary::empty(0, 0, 1);
            summary.maybe_stop_reason = Some(*stop_reason);
            runtime
                .durable_sync_state_for_summary(
                    &summary,
                    SyncLifecycleState::Active,
                    None,
                    1_777_225_163 + i64::try_from(index).expect("index fits i64"),
                )
                .expect("durable local-stop status")
        })
        .collect::<Vec<_>>();

    // Assert
    for (state, (_, expected_label, expected_subsystem)) in states.iter().zip(cases) {
        let stall = available_stall_diagnosis(state);
        assert_eq!(serialized_label(stall.stalled_subsystem), expected_label);
        assert_eq!(stall.stalled_subsystem, expected_subsystem);
    }

    remove_dir_if_exists(&path);
}

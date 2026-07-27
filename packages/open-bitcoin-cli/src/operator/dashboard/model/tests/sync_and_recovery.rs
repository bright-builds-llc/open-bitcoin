use super::*;

#[test]
fn dashboard_sections_surface_sync_progress_and_peer_counts() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let sync_labels = sync_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        sync_labels,
        vec![
            "State",
            "Phase",
            "Configured targets",
            "Attempt counters",
            "Signal",
            "Best-known tip",
            "Stay-current",
            "Stay-current action",
            "No-progress diagnosis",
            "No-progress action",
            "Progress credit",
            "Expected progress window",
            "No-progress threshold",
            "Last useful work",
            "Last peer contribution",
            "Stalled subsystem",
            "Last progress",
            "Latest stop reason",
            "Last error",
            "Recovery category",
            "Recovery",
            "Recovery evidence",
            "Pressure",
            "Resource bounds",
            "Latest reorg",
            "Reconcile",
            "Peers",
            "Progress",
        ]
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Configured targets")
            .expect("configured targets row")
            .value,
        "outbound_peers=4 target_header_height=840200"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Attempt counters")
            .expect("attempt counters row")
            .value,
        "attempted_peers=3 connected_peers=2 failed_peers=1 max_sync_rounds=8"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Latest stop reason")
            .expect("latest stop reason row")
            .value,
        "target_header_reached"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Progress")
            .expect("progress row")
            .value,
        "99.99% headers=840100 downloaded_blocks=840006 connected_blocks=840004 validated_active_chain_height=840004 validated_active_chain_hash=1111111111111111111111111111111111111111111111111111111111111111 validated_active_chain_work=840005"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Signal")
            .expect("signal row")
            .value,
        "awaiting_blocks"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Peers")
            .expect("peers row")
            .value,
        "inbound=0 outbound=2"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Recovery category")
            .expect("recovery category row")
            .value,
        "invalid_peer_data"
    );
    assert!(
        sync_rows
            .iter()
            .find(|row| row.label == "Pressure")
            .expect("pressure row")
            .value
            .contains("peers 2/4")
    );
}

#[test]
fn dashboard_sections_surface_sync_progress_and_peer_counts_unavailable_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.configured_targets =
        FieldAvailability::unavailable("operator target unavailable");
    snapshot.sync.attempt_counters = FieldAvailability::unavailable("attempt counters unavailable");
    snapshot.sync.latest_stop_reason = FieldAvailability::unavailable("stop reason unavailable");

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Configured targets")
            .expect("configured targets row")
            .value,
        "Unavailable: operator target unavailable"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Attempt counters")
            .expect("attempt counters row")
            .value,
        "Unavailable: attempt counters unavailable"
    );
    assert_eq!(
        sync_rows
            .iter()
            .find(|row| row.label == "Latest stop reason")
            .expect("latest stop reason row")
            .value,
        "Unavailable: stop reason unavailable"
    );
}

#[test]
fn dashboard_sections_surface_phase78_progress_guarantee_fields() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    apply_phase78_available_sync_fields(&mut snapshot.sync);

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    for (label, expected) in [
        (
            "Progress credit",
            "kind=validated_durable_active_chain height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000020 rejected_activity_count=1",
        ),
        (
            "Expected progress window",
            "expected_progress_window_seconds=300 retry_backoff_seconds=30 max_sync_rounds=8 tip_freshness_threshold_seconds=600",
        ),
        (
            "No-progress threshold",
            "state=within_window threshold_seconds=300 elapsed_since_last_useful_work_seconds=12 evaluated_at_unix_seconds=1717000032",
        ),
        (
            "Last useful work",
            "kind=current_at_best_known_tip height=840004 hash=1111111111111111111111111111111111111111111111111111111111111111 work=840005 source_unix_seconds=1717000025 rejected_activity_count=0",
        ),
        (
            "Last peer contribution",
            "peer=peer-1 endpoint=203.0.113.10:8333 kind=headers_and_blocks messages=7 headers=3 blocks=1 last_activity_unix_seconds=1717000028 failure=Unavailable: no peer failure recorded",
        ),
        (
            "Stalled subsystem",
            "stalled_subsystem=at_tip_waiting confidence=high basis=stay_current,current_tip next_action=No operator action required. no_progress_diagnosis=current_at_best_known_tip recovery_category=Unavailable: no recovery category latest_stop_reason=best_known_tip_reached",
        ),
    ] {
        assert_eq!(
            sync_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase78 row")
                .value,
            expected
        );
    }
}

#[test]
fn dashboard_sections_surface_phase78_unavailable_reasons() {
    // Arrange
    let snapshot = shared_sync_truth_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    for (label, expected) in [
        (
            "Progress credit",
            "Unavailable: progress credit evidence unavailable",
        ),
        (
            "Expected progress window",
            "Unavailable: expected progress window unavailable",
        ),
        (
            "No-progress threshold",
            "Unavailable: no-progress threshold evidence unavailable",
        ),
        (
            "Last useful work",
            "Unavailable: last useful work unavailable",
        ),
        (
            "Last peer contribution",
            "Unavailable: last peer contribution unavailable",
        ),
        (
            "Stalled subsystem",
            "Unavailable: stall diagnosis unavailable",
        ),
    ] {
        assert_eq!(
            sync_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase78 row")
                .value,
            expected
        );
    }
}

#[test]
fn phase72_dashboard_projects_full_sync_truth_contract() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.sync.best_known_tip = FieldAvailability::available(BestKnownTipStatus {
        source: BestKnownTipSource::HeaderStore,
        height: 840_004,
        block_hash: "11".repeat(32),
        work: "840005".to_string(),
        block_time_unix_seconds: 1_717_000_010,
        observed_at_unix_seconds: 1_717_000_020,
        freshness: TipFreshnessStatus::Fresh,
        peer_agreement: vec![PeerTipAgreement {
            peer: "peer-1".to_string(),
            maybe_resolved_endpoint: Some("203.0.113.10:8333".to_string()),
            status: PeerTipAgreementStatus::Agrees,
            maybe_height: Some(840_004),
            maybe_hash: Some("11".repeat(32)),
            maybe_work: Some("840005".to_string()),
            maybe_last_activity_unix_seconds: Some(1_717_000_020),
        }],
    });
    snapshot.sync.stay_current =
        FieldAvailability::available(StayCurrentStatus::CurrentAtBestKnownTip);
    snapshot.sync.stay_current_next_action =
        FieldAvailability::available("Continue monitoring best-known tip freshness.".to_string());
    snapshot.sync.no_progress_diagnosis =
        FieldAvailability::available(NoProgressDiagnosis::CurrentAtBestKnownTip);
    snapshot.sync.no_progress_next_action =
        FieldAvailability::available("No operator action required.".to_string());
    snapshot.sync.latest_reorg = FieldAvailability::available(SyncReorgEvidence {
        common_ancestor_height: 840_000,
        common_ancestor_hash: "00".repeat(32),
        disconnected_count: 2,
        connected_count: 4,
        final_active_height: 840_004,
        final_active_hash: "11".repeat(32),
        fully_persisted: true,
    });
    snapshot.sync.reconcile_progress =
        FieldAvailability::available(SyncReconcileProgressStatus::ExtendedActiveChain {
            connected_count: 4,
            final_active_height: 840_004,
            final_active_hash: "11".repeat(32),
        });

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let labels = sync_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    for label in [
        "Best-known tip",
        "Stay-current",
        "Stay-current action",
        "No-progress diagnosis",
        "No-progress action",
        "Latest reorg",
        "Reconcile",
        "Pressure",
        "Progress",
    ] {
        assert!(labels.contains(&label), "missing row {label}");
    }
    let progress = sync_rows
        .iter()
        .find(|row| row.label == "Progress")
        .expect("progress row");
    for expected in [
        "validated_active_chain_height=840004",
        "validated_active_chain_hash=1111111111111111111111111111111111111111111111111111111111111111",
        "validated_active_chain_work=840005",
    ] {
        assert!(progress.value.contains(expected), "missing {expected}");
    }

    // Arrange
    snapshot.sync.best_known_tip =
        FieldAvailability::unavailable("best-known tip evidence unavailable");
    snapshot.sync.stay_current = FieldAvailability::unavailable("stay-current state unavailable");
    snapshot.sync.latest_reorg = FieldAvailability::unavailable("no reorg evidence recorded");
    snapshot.sync.reconcile_progress =
        FieldAvailability::unavailable("reconcile progress unavailable");

    // Act
    let unavailable_state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let unavailable_rows = &unavailable_state.sections[1].rows;
    for (label, expected) in [
        (
            "Best-known tip",
            "Unavailable: best-known tip evidence unavailable",
        ),
        (
            "Stay-current",
            "Unavailable: stay-current state unavailable",
        ),
        ("Latest reorg", "Unavailable: no reorg evidence recorded"),
        ("Reconcile", "Unavailable: reconcile progress unavailable"),
    ] {
        assert_eq!(
            unavailable_rows
                .iter()
                .find(|row| row.label == label)
                .expect("phase72 row")
                .value,
            expected
        );
    }
}

#[test]
fn dashboard_service_restart_resume_rows_surface_phase64_evidence() {
    // Arrange
    let snapshot = test_snapshot();

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let service_rows = &state.sections[3].rows;
    let labels = service_rows
        .iter()
        .map(|row| row.label.as_str())
        .collect::<Vec<_>>();
    assert_eq!(
        labels,
        vec![
            "Lifecycle",
            "Manager",
            "Installed",
            "Enabled",
            "Running",
            "Service file",
            "Logs",
            "Diagnostics",
            "Restart/resume",
            "Prior shutdown",
            "Resume progress",
            "Stale in-flight",
            "Resume action",
        ]
    );
    assert_eq!(service_rows[0].value, "running");
    assert_eq!(service_rows[1].value, "launchd");
    assert_eq!(service_rows[5].value, "/tmp/open-bitcoin-node.service");
    assert_eq!(service_rows[6].value, "/tmp/logs/open-bitcoin.log");
    assert_eq!(
        service_rows[7].value,
        "Unavailable: service diagnostics unavailable"
    );
    assert_eq!(
        service_rows[8].value,
        "datadir=/tmp/open-bitcoin same_datadir=true recovery_category=clean_shutdown"
    );
    assert_eq!(service_rows[9].value, "clean");
    assert_eq!(service_rows[10].value, "downloaded=840006 connected=840004");
    assert_eq!(service_rows[11].value, "cleared");
    assert_eq!(
        service_rows[12].value,
        "Resume service sync review from preserved durable progress."
    );

    let mut unavailable = test_snapshot();
    unavailable.service = ServiceStatus {
        manager: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        lifecycle: FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager),
        installed: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        enabled: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        running: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        service_file_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        log_path: FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: manager unavailable",
        ),
        diagnostics: FieldAvailability::available(
            "unsupported platform: manager unavailable".to_string(),
        ),
        restart_resume: FieldAvailability::unavailable(
            "service restart/resume evidence unavailable",
        ),
    };

    let unavailable_state = DashboardState::from_snapshot(&unavailable);
    let unavailable_rows = &unavailable_state.sections[3].rows;
    assert_eq!(unavailable_rows[0].value, "unavailable-manager");
    assert_eq!(
        unavailable_rows[1].value,
        "Unavailable: service manager unavailable: unsupported platform: manager unavailable"
    );
    assert_eq!(
        unavailable_rows[7].value,
        "unsupported platform: manager unavailable"
    );
    assert_eq!(
        unavailable_rows[8].value,
        "Unavailable: service restart/resume evidence unavailable"
    );
}

#[test]
fn dashboard_recovery_evidence_available_row_uses_top_level_status_evidence() {
    // Arrange
    let mut snapshot = shared_sync_truth_snapshot();
    snapshot.recovery_evidence = FieldAvailability::available(phase77_recovery_evidence());
    snapshot.sync.recovery_action =
        FieldAvailability::available("legacy action cause=legacy action_class=legacy".to_string());

    // Act
    let state = DashboardState::from_snapshot(&snapshot);

    // Assert
    let sync_rows = &state.sections[1].rows;
    let recovery_evidence = sync_rows
        .iter()
        .find(|row| row.label == "Recovery evidence")
        .expect("recovery evidence row");
    assert_eq!(
        recovery_evidence.value,
        "category=storage_lock_contention cause=stale_lock_evidence action_class=read_only_inspection next_action=Inspect the datadir read-only and avoid deleting lock artifacts automatically."
    );
}

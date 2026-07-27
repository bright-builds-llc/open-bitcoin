// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn build_provenance_from_inputs_marks_present_fields_available() {
    // Arrange
    let inputs = BuildProvenanceInputs {
        version: "0.1.0",
        maybe_commit: Some("abc123"),
        maybe_build_time: Some("2026-04-28T12:43:00Z"),
        maybe_target: Some("aarch64-apple-darwin"),
        maybe_profile: Some("debug"),
    };

    // Act
    let provenance = build_provenance_from_inputs(inputs);

    // Assert
    assert_eq!(provenance.version, "0.1.0");
    assert_eq!(
        provenance.commit,
        FieldAvailability::available("abc123".to_string())
    );
    assert_eq!(
        provenance.build_time,
        FieldAvailability::available("2026-04-28T12:43:00Z".to_string())
    );
    assert_eq!(
        provenance.target,
        FieldAvailability::available("aarch64-apple-darwin".to_string())
    );
    assert_eq!(
        provenance.profile,
        FieldAvailability::available("debug".to_string())
    );
}

#[test]
fn status_wallet_rpc_access_stays_root_without_store_inspection() {
    // Arrange
    let path = temp_path("wallet-access-probe-only");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

    // Act
    let access = resolve_status_wallet_rpc_access(Some(&path));

    // Assert
    assert_eq!(access, StatusWalletRpcAccess::Root);
    assert!(!path.exists());
}

#[test]
fn human_and_json_renderers_surface_wallet_freshness_and_scan_reasons() {
    // Arrange
    let snapshot = OpenBitcoinStatusSnapshot {
        node: NodeStatus {
            state: NodeRuntimeState::Running,
            version: "0.1.0".to_string(),
        },
        config: ConfigStatus {
            datadir: FieldAvailability::available("/tmp/open-bitcoin".to_string()),
            config_paths: vec!["/tmp/open-bitcoin/open-bitcoin.jsonc".to_string()],
        },
        service: ServiceStatus {
            manager: FieldAvailability::available("launchd".to_string()),
            lifecycle: FieldAvailability::available(ServiceLifecycleStatus::Running),
            installed: FieldAvailability::available(true),
            enabled: FieldAvailability::available(true),
            running: FieldAvailability::available(true),
            service_file_path: FieldAvailability::available(
                "/tmp/open-bitcoin-node.service".to_string(),
            ),
            log_path: FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string()),
            diagnostics: FieldAvailability::unavailable("service diagnostics unavailable"),
            restart_resume: FieldAvailability::unavailable(
                "service restart/resume evidence unavailable",
            ),
        },
        sync: SyncStatus {
            network: FieldAvailability::available("regtest".to_string()),
            chain_tip: FieldAvailability::unavailable("tip unavailable"),
            sync_progress: FieldAvailability::unavailable("sync unavailable"),
            lifecycle: FieldAvailability::unavailable("sync lifecycle unavailable"),
            phase: FieldAvailability::unavailable("sync phase unavailable"),
            configured_targets: FieldAvailability::<SyncConfiguredTargets>::unavailable(
                "sync configured targets unavailable",
            ),
            attempt_counters: FieldAvailability::<SyncAttemptCounters>::unavailable(
                "sync attempt counters unavailable",
            ),
            progress_signal: FieldAvailability::available(SyncProgressSignal::Steady),
            lag: FieldAvailability::unavailable("sync lag unavailable"),
            last_successful_progress_unix_seconds: FieldAvailability::unavailable(
                "sync last progress unavailable",
            ),
            progress_credit: FieldAvailability::unavailable("progress credit evidence unavailable"),
            expected_progress_window: FieldAvailability::unavailable(
                "expected progress window unavailable",
            ),
            no_progress_threshold: FieldAvailability::unavailable(
                "no-progress threshold evidence unavailable",
            ),
            last_useful_work: FieldAvailability::unavailable("last useful work unavailable"),
            last_peer_contribution: FieldAvailability::unavailable(
                "last peer contribution unavailable",
            ),
            stall_diagnosis: FieldAvailability::unavailable("stall diagnosis unavailable"),
            latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
                "sync latest stop reason unavailable",
            ),
            last_error: FieldAvailability::unavailable("sync error unavailable"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("sync recovery unavailable"),
            resource_pressure: FieldAvailability::unavailable("sync pressure unavailable"),
            best_known_tip: FieldAvailability::<BestKnownTipStatus>::unavailable(
                "best-known tip evidence unavailable",
            ),
            stay_current: FieldAvailability::<StayCurrentStatus>::unavailable(
                "stay-current state unavailable",
            ),
            stay_current_next_action: FieldAvailability::unavailable(
                "stay-current next action unavailable",
            ),
            no_progress_diagnosis: FieldAvailability::unavailable(
                "no-progress diagnosis unavailable",
            ),
            no_progress_next_action: FieldAvailability::unavailable(
                "no-progress next action unavailable",
            ),
            latest_reorg: FieldAvailability::unavailable("no reorg evidence recorded"),
            reconcile_progress: FieldAvailability::unavailable("reconcile progress unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 1,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
            inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                INBOUND_STATUS_UNAVAILABLE_REASON,
            ),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(3),
            relay: RelayEvidenceStatus::default(),
        },
        block_relay: BlockRelayEvidenceStatus::default_unavailable(),
        wallet: WalletStatus {
            trusted_balance_sats: FieldAvailability::available(25_000),
            freshness: FieldAvailability::available(WalletFreshness::Scanning),
            scan_progress: FieldAvailability::available(WalletScanProgress {
                scanned_through_height: 30,
                target_tip_height: 60,
            }),
        },
        logs: open_bitcoin_node::LogStatus::default(),
        metrics: open_bitcoin_node::MetricsStatus::default(),
        recovery_evidence: FieldAvailability::default(),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        health_signals: Vec::new(),
        build: BuildProvenance::unavailable(),
    };

    // Act
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("json status");

    // Assert
    assert!(human.contains("Wallet freshness: scanning"));
    assert!(human.contains("Wallet scan: height 30/60 (50.00%)"));
    assert!(json.contains("\"freshness\""));
    assert!(json.contains("\"scan_progress\""));
}

#[test]
fn human_status_contains_required_labels_and_detection_uncertainty() {
    // Arrange
    let input = status_input(vec![detected_installation()]);
    let snapshot = collect_status_snapshot(&input, None);

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    for label in [
        "Daemon:",
        "Version:",
        "Build:",
        "Datadir:",
        "Config:",
        "Network:",
        "Chain:",
        "Sync:",
        "Sync signal:",
        "Sync last progress:",
        "Peers:",
        "Mempool:",
        "Wallet:",
        "Wallet freshness:",
        "Wallet scan:",
        "Service:",
        "Logs:",
        "Metrics:",
        "Health:",
    ] {
        assert!(rendered.contains(label), "missing {label}");
    }
    assert!(rendered.contains("/tmp/core/.bitcoin/bitcoin.conf"));
    assert!(rendered.contains("uncertain"));
    assert!(rendered.contains("Unavailable: node stopped"));
}

#[test]
fn human_status_surfaces_warning_health_signals_before_daemon_line() {
    // Arrange
    let input = status_input(Vec::new());
    let mut snapshot = collect_status_snapshot(&input, None);
    snapshot.health_signals.insert(
        0,
        open_bitcoin_node::status::HealthSignal {
            level: open_bitcoin_node::status::HealthSignalLevel::Warn,
            source: "live_rpc_bootstrap".to_string(),
            message:
                "live RPC was not attempted because no rediscoverable RPC credentials were found."
                    .to_string(),
        },
    );

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let lines = rendered.lines().collect::<Vec<_>>();

    // Assert
    assert!(
        lines
            .first()
            .expect("warning line")
            .starts_with("Warnings: ")
    );
    assert!(lines[0].contains("live_rpc_bootstrap"));
    assert!(lines[0].contains("live RPC was not attempted"));
    assert!(
        lines
            .iter()
            .position(|line| line.starts_with("Warnings: "))
            .expect("warning line")
            < lines
                .iter()
                .position(|line| line.starts_with("Daemon: "))
                .expect("daemon line")
    );
    assert!(rendered.contains("Health: warn:live_rpc_bootstrap:"));
}

#[test]
fn status_rendering_redacts_credentials_and_cookie_contents() {
    // Arrange
    let input = status_input(vec![detected_installation()]);
    let snapshot = collect_status_snapshot(&input, Some(&FakeStatusRpcClient::running()));

    // Act
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let combined = format!("{json}\n{human}");

    // Assert
    assert!(!combined.contains("secret"));
    assert!(!combined.contains("Authorization"));
    assert!(!combined.contains("Basic "));
    assert!(!combined.contains("rpcpassword"));
    assert!(!combined.contains("__cookie__:fixture"));
}

// --- Service manager injection tests ---

#[test]
fn phase63_service_lifecycle_projection_maps_snapshot_states() {
    // Arrange
    let cases = [
        (
            service_snapshot(
                ServiceLifecycleState::Unmanaged,
                None,
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::Unmanaged,
        ),
        (
            service_snapshot(
                ServiceLifecycleState::Running,
                Some(false),
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::Running,
        ),
        (
            service_snapshot(
                ServiceLifecycleState::Failed,
                Some(true),
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::Failed,
        ),
        (
            service_snapshot(
                ServiceLifecycleState::Installed,
                Some(false),
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::Disabled,
        ),
        (
            service_snapshot(
                ServiceLifecycleState::Stopped,
                Some(true),
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::InstalledStopped,
        ),
        (
            service_snapshot(
                ServiceLifecycleState::Enabled,
                Some(true),
                Path::new("/tmp/open-bitcoin"),
            ),
            ServiceLifecycleStatus::InstalledStopped,
        ),
    ];

    // Act
    let actual = cases
        .iter()
        .map(|(snapshot, _)| service_lifecycle_from_snapshot(snapshot))
        .collect::<Vec<_>>();

    // Assert
    assert_eq!(
        actual,
        cases
            .iter()
            .map(|(_, expected)| *expected)
            .collect::<Vec<_>>()
    );
}

#[test]
fn phase63_service_lifecycle_projection_collects_manager_evidence() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: Some("launchctl reports running".to_string()),
        maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
        maybe_log_path_unavailable_reason: None,
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = status_input_with_manager(Box::new(fake), config_resolution());

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.lifecycle,
        FieldAvailability::available(ServiceLifecycleStatus::Running)
    );
    assert_eq!(
        snapshot.service.enabled,
        FieldAvailability::available(false),
        "manager enablement evidence should not be inferred away"
    );
    assert_eq!(
        snapshot.service.service_file_path,
        FieldAvailability::available("/tmp/open-bitcoin-node.service".to_string())
    );
    assert_eq!(
        snapshot.service.log_path,
        FieldAvailability::available("/tmp/logs/open-bitcoin.log".to_string())
    );
    assert_eq!(
        snapshot.service.diagnostics,
        FieldAvailability::available("launchctl reports running".to_string())
    );

    let missing_enablement_fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Stopped,
        maybe_enabled: None,
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: Some("   ".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("manager did not report log path".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let missing_enablement_input =
        status_input_with_manager(Box::new(missing_enablement_fake), config_resolution());
    let missing_enablement = collect_status_snapshot(&missing_enablement_input, None);
    assert_eq!(
        missing_enablement.service.lifecycle,
        FieldAvailability::available(ServiceLifecycleStatus::InstalledStopped)
    );
    assert_eq!(
        missing_enablement.service.enabled,
        FieldAvailability::unavailable("service manager did not report enablement")
    );
    assert_eq!(
        missing_enablement.service.log_path,
        FieldAvailability::unavailable("manager did not report log path")
    );
    assert_eq!(
        missing_enablement.service.diagnostics,
        FieldAvailability::unavailable("service diagnostics unavailable")
    );
}

#[test]
fn collect_status_snapshot_with_no_service_manager_preserves_unavailable_service_fields() {
    // Arrange — no service manager, no detected service candidates
    let input = status_input(Vec::new());

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert — all service fields remain unavailable (existing fallback preserved)
    assert!(
        matches!(
            &snapshot.service.manager,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.manager should be unavailable when no manager injected"
    );
    assert!(
        matches!(
            &snapshot.service.installed,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.installed should be unavailable when no manager injected"
    );
    assert!(
        matches!(
            &snapshot.service.enabled,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.enabled should be unavailable when no manager injected"
    );
    assert!(
        matches!(
            &snapshot.service.running,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.running should be unavailable when no manager injected"
    );
}

#[test]
fn collect_status_snapshot_without_manager_uses_detected_service_candidates() {
    // Arrange
    let input =
        status_input_with_service_candidates(Vec::new(), vec![detected_service_candidate()]);

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.manager,
        open_bitcoin_node::status::FieldAvailability::available("systemd".to_string())
    );
    assert_eq!(
        snapshot.service.installed,
        open_bitcoin_node::status::FieldAvailability::available(true)
    );
    assert!(
        matches!(
            &snapshot.service.enabled,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.enabled should stay unavailable when only detection evidence exists"
    );
    assert!(
        matches!(
            &snapshot.service.running,
            open_bitcoin_node::status::FieldAvailability::Unavailable { .. }
        ),
        "service.running should stay unavailable when only detection evidence exists"
    );
}

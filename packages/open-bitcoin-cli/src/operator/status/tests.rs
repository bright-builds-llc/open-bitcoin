// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs, io,
    path::{Path, PathBuf},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use super::{
    BuildProvenanceInputs, StatusCollectorInput, StatusDetectionEvidence,
    StatusLiveRpcAdapterInput, StatusRenderMode, StatusRequest, StatusRpcAuthSource,
    StatusRpcClient, StatusRpcError, StatusWalletRpcAccess, build_provenance_from_inputs,
    collect_status_snapshot, render_status, resolve_status_wallet_rpc_access,
    service_status::service_lifecycle_from_snapshot,
};
use crate::operator::{
    NetworkSelection,
    config::{
        OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
        OperatorConfigSource,
    },
    detect::{
        DetectedInstallation, DetectionConfidence, DetectionSourcePath, DetectionSourcePathKind,
        DetectionUncertainty, ProductFamily, ServiceCandidate, ServiceManager, WalletCandidate,
        WalletCandidateKind,
    },
    service::{
        ServiceError, ServiceLifecycleState, ServiceStateSnapshot, fake::FakeServiceManager,
    },
};
use open_bitcoin_node::status::{
    BuildProvenance, ConfigStatus, FieldAvailability, MempoolStatus, NodeRuntimeState, NodeStatus,
    OpenBitcoinStatusSnapshot, PeerCounts, PeerStatus, ServiceLifecycleStatus,
    ServicePriorShutdownStatus, ServiceResumeProgressStatus, ServiceStaleInflightStatus,
    ServiceStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncLagStatus, SyncLifecycleState,
    SyncProgress, SyncProgressSignal, SyncRecoveryCategory, SyncResourcePressure, SyncStatus,
    SyncStopReasonStatus, WalletFreshness, WalletScanProgress, WalletStatus,
};
use open_bitcoin_node::{
    DurableSyncState, FjallNodeStore, PersistMode, RuntimeMetadata, WalletRegistry,
    core::wallet::{AddressNetwork, Wallet},
};
use open_bitcoin_rpc::{
    RpcErrorCode, RpcErrorDetail,
    method::{
        GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse,
        GetNetworkInfoResponse, GetWalletInfoResponse, WalletBalanceDetails,
    },
};

#[test]
fn status_request_defines_render_mode() {
    // Act
    let request = StatusRequest {
        render_mode: StatusRenderMode::Json,
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin.jsonc")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        include_live_rpc: true,
        no_color: true,
    };

    // Assert
    assert_eq!(request.render_mode, StatusRenderMode::Json);
    assert!(request.include_live_rpc);
    assert!(request.no_color);
}

#[test]
fn status_collector_input_keeps_rpc_config_and_detection_evidence_typed() {
    // Arrange
    let config_resolution = config_resolution();
    let request = StatusRequest {
        render_mode: StatusRenderMode::Human,
        maybe_config_path: None,
        maybe_data_dir: None,
        maybe_network: Some(NetworkSelection::Regtest),
        include_live_rpc: true,
        no_color: false,
    };

    // Act
    let input = StatusCollectorInput {
        request,
        config_resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: Some(StatusLiveRpcAdapterInput {
            endpoint: "http://127.0.0.1:8332".to_string(),
            auth_source: StatusRpcAuthSource::CookieFile {
                path: PathBuf::from("/tmp/.cookie"),
            },
            timeout: Duration::from_secs(2),
        }),
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Assert
    assert_eq!(input.request.render_mode, StatusRenderMode::Human);
    assert!(input.maybe_live_rpc.is_some());
    assert!(input.detection_evidence.detected_installations.is_empty());
}

#[test]
fn stopped_status_keeps_live_fields_unavailable() {
    // Arrange
    let input = status_input(Vec::new());

    // Act
    let snapshot = collect_status_snapshot(&input, None);
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["config"]["datadir"]["state"], "available");
    assert_eq!(decoded["config"]["datadir"]["value"], "/tmp/open-bitcoin");
    assert_eq!(decoded["sync"]["network"]["state"], "unavailable");
    assert_eq!(decoded["sync"]["chain_tip"]["state"], "unavailable");
    assert_eq!(decoded["sync"]["sync_progress"]["state"], "unavailable");
    assert_eq!(decoded["peers"]["peer_counts"]["state"], "unavailable");
    assert_eq!(decoded["mempool"]["transactions"]["state"], "unavailable");
    assert_eq!(
        decoded["wallet"]["trusted_balance_sats"]["state"],
        "unavailable"
    );
    assert_eq!(decoded["wallet"]["freshness"]["state"], "unavailable");
    assert_eq!(decoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert!(
        decoded["health_signals"]
            .as_array()
            .expect("health signals")
            .is_empty()
    );
    assert_eq!(decoded["build"]["version"], env!("CARGO_PKG_VERSION"));
}

#[test]
fn fake_live_rpc_maps_into_shared_status_snapshot() {
    // Arrange
    let input = status_input(vec![detected_installation()]);
    let rpc = FakeStatusRpcClient::running();

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["node"]["version"], "/Satoshi:29.3.0/");
    assert_eq!(decoded["config"]["datadir"]["value"], "/tmp/open-bitcoin");
    assert_eq!(decoded["sync"]["network"]["value"], "regtest");
    assert_eq!(decoded["sync"]["chain_tip"]["value"]["height"], 144);
    assert_eq!(
        decoded["sync"]["chain_tip"]["value"]["block_hash"],
        "00aabb"
    );
    assert_eq!(
        decoded["sync"]["sync_progress"]["value"]["block_height"],
        144
    );
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["inbound"], 2);
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["outbound"], 5);
    assert_eq!(decoded["mempool"]["transactions"]["value"], 12);
    assert_eq!(decoded["wallet"]["trusted_balance_sats"]["value"], 50_000);
    assert_eq!(decoded["wallet"]["freshness"]["value"], "fresh");
    assert_eq!(decoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(decoded["logs"]["path"]["state"], "unavailable");
    assert_eq!(
        decoded["metrics"]["retention"]["sample_interval_seconds"],
        30
    );
    assert_eq!(decoded["health_signals"][0]["source"], "detection");
    assert!(
        decoded["health_signals"]
            .to_string()
            .contains("/tmp/core/.bitcoin/bitcoin.conf")
    );
    assert!(decoded["health_signals"].to_string().contains("uncertain"));
    assert_eq!(decoded["build"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(decoded["build"]["build_time"]["state"], "available");
    assert_eq!(decoded["build"]["target"]["state"], "available");
    assert_eq!(decoded["build"]["profile"]["state"], "available");
}

#[test]
fn rpc_failure_produces_unreachable_snapshot_not_process_failure() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::failing("auth failed");

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "unreachable");
    assert_eq!(decoded["sync"]["network"]["state"], "unavailable");
    assert!(
        decoded["sync"]["network"]["value"]["reason"]
            .as_str()
            .expect("reason")
            .contains("auth failed")
    );
    assert!(
        decoded["health_signals"]
            .to_string()
            .contains("auth failed")
    );
}

#[test]
fn wallet_rpc_failure_keeps_node_running_and_marks_wallet_unavailable() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::wallet_failing(StatusRpcError::from_rpc_detail(
        RpcErrorDetail::new(
            RpcErrorCode::WalletNotSpecified,
            "Multiple wallets are loaded. Please select which wallet to use by requesting the RPC through the /wallet/<walletname> URI path.",
        ),
    ));

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["sync"]["network"]["value"], "regtest");
    assert_eq!(
        decoded["wallet"]["trusted_balance_sats"]["state"],
        "unavailable"
    );
    assert!(
        decoded["wallet"]["trusted_balance_sats"]["value"]["reason"]
            .as_str()
            .expect("wallet reason")
            .contains("Multiple wallets are loaded")
    );
    assert!(
        decoded["health_signals"]
            .as_array()
            .expect("health signals")
            .iter()
            .any(|signal| signal["source"] == "wallet")
    );
}

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
fn status_wallet_rpc_access_uses_sole_loaded_wallet() {
    // Arrange
    let store = managed_wallet_store("sole", &["alpha"], None);

    // Act
    let access = resolve_status_wallet_rpc_access(Some(store.path()));

    // Assert
    assert_eq!(access, StatusWalletRpcAccess::Wallet("alpha".to_string()));
}

#[test]
fn status_wallet_rpc_access_prefers_selected_wallet_when_multiple_loaded() {
    // Arrange
    let store = managed_wallet_store("selected", &["alpha", "beta"], Some("beta"));

    // Act
    let access = resolve_status_wallet_rpc_access(Some(store.path()));

    // Assert
    assert_eq!(access, StatusWalletRpcAccess::Wallet("beta".to_string()));
}

#[test]
fn status_wallet_rpc_access_marks_ambiguous_wallets_unavailable() {
    // Arrange
    let multi_store = managed_wallet_store("multi", &["alpha", "beta"], None);

    // Act
    let multi_access = resolve_status_wallet_rpc_access(Some(multi_store.path()));

    // Assert
    assert!(matches!(
        multi_access,
        StatusWalletRpcAccess::Unavailable { .. }
    ));
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
            latest_stop_reason: FieldAvailability::<SyncStopReasonStatus>::unavailable(
                "sync latest stop reason unavailable",
            ),
            last_error: FieldAvailability::unavailable("sync error unavailable"),
            recovery_category: FieldAvailability::unavailable("no recovery category recorded"),
            recovery_action: FieldAvailability::unavailable("sync recovery unavailable"),
            resource_pressure: FieldAvailability::unavailable("sync pressure unavailable"),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 1,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
        },
        mempool: MempoolStatus {
            transactions: FieldAvailability::available(3),
        },
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

#[test]
fn collect_status_snapshot_with_fake_running_manager_sets_service_fields_to_available_true() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(true),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: None,
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert!(
        matches!(
            &snapshot.service.manager,
            open_bitcoin_node::status::FieldAvailability::Available(_)
        ),
        "service.manager should be available when running manager injected"
    );
    assert_eq!(
        snapshot.service.installed,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.installed should be true when state is Running"
    );
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.enabled should be true when state is Running"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.running should be true when state is Running"
    );
}

#[test]
fn collect_status_snapshot_with_fake_installed_manager_sets_installed_true_enabled_false() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Installed,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: None,
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.installed,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.installed should be true when state is Installed"
    );
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.enabled should be false when state is Installed (not Enabled/Running)"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.running should be false when state is Installed"
    );
}

#[test]
fn collect_status_snapshot_uses_manager_enabled_state_over_state_inference() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Failed,
        maybe_enabled: Some(true),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: Some("systemctl is-active=failed".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.enabled should preserve manager evidence even when state is Failed"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.running should remain false when state is not Running"
    );
}

#[test]
fn collect_status_snapshot_preserves_running_when_startup_is_not_enabled() {
    // Arrange
    let fake = FakeServiceManager::new(ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(false),
        maybe_service_file_path: Some(PathBuf::from("/tmp/test.plist")),
        maybe_manager_diagnostics: Some("launchctl service is running but disabled".to_string()),
        maybe_log_path: None,
        maybe_log_path_unavailable_reason: Some("service log path unavailable".to_string()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_data_dir_unavailable_reason: None,
    });
    let input = StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(Box::new(fake)),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    };

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.enabled,
        open_bitcoin_node::status::FieldAvailability::available(false),
        "service.enabled should come from manager evidence instead of Running inference"
    );
    assert_eq!(
        snapshot.service.running,
        open_bitcoin_node::status::FieldAvailability::available(true),
        "service.running should still be true when the manager reports Running"
    );
}

#[test]
fn collect_status_snapshot_with_error_manager_falls_back_to_unavailable() {
    // Arrange
    struct ErrorServiceManager;
    impl crate::operator::service::ServiceManager for ErrorServiceManager {
        fn install(
            &self,
            _request: &crate::operator::service::ServiceInstallRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn uninstall(
            &self,
            _request: &crate::operator::service::ServiceUninstallRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn enable(
            &self,
            _request: &crate::operator::service::ServiceEnableRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn disable(
            &self,
            _request: &crate::operator::service::ServiceDisableRequest,
        ) -> Result<crate::operator::service::ServiceCommandOutcome, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "test".to_string(),
            })
        }
        fn status(&self) -> Result<ServiceStateSnapshot, ServiceError> {
            Err(ServiceError::UnsupportedPlatform {
                reason: "platform not supported in test".to_string(),
            })
        }
    }

    let path = temp_path("phase63-manager-error-sync-preservation");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };
    let store = FjallNodeStore::open(&path).expect("open sync state store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                maybe_sync_state: Some(phase62_durable_sync_state()),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save sync state metadata");
    drop(store);

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(Box::new(ErrorServiceManager), resolution);

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.lifecycle,
        FieldAvailability::available(ServiceLifecycleStatus::UnavailableManager)
    );
    assert_eq!(
        snapshot.service.manager,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.installed,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.enabled,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.running,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.service_file_path,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.log_path,
        FieldAvailability::unavailable(
            "service manager unavailable: unsupported platform: platform not supported in test"
        )
    );
    assert_eq!(
        snapshot.service.diagnostics,
        FieldAvailability::available(
            "unsupported platform: platform not supported in test".to_string()
        )
    );

    let FieldAvailability::Available(configured_targets) = snapshot.sync.configured_targets else {
        panic!("configured targets should survive service manager failure");
    };
    assert_eq!(configured_targets.target_outbound_peers, 4);
    assert_eq!(configured_targets.maybe_target_header_height, Some(840_200));

    let FieldAvailability::Available(attempt_counters) = snapshot.sync.attempt_counters else {
        panic!("attempt counters should survive service manager failure");
    };
    assert_eq!(attempt_counters.attempted_peers, 3);
    assert_eq!(attempt_counters.connected_peers, 2);
    assert_eq!(attempt_counters.failed_peers, 1);
    assert_eq!(attempt_counters.max_sync_rounds, 8);

    let FieldAvailability::Available(stop_reason) = snapshot.sync.latest_stop_reason else {
        panic!("latest stop reason should survive service manager failure");
    };
    assert_eq!(stop_reason.label, "target_header_reached");
    assert_eq!(
        snapshot.sync.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData)
    );
}

#[test]
fn service_restart_resume_status_surfaces_clean_same_datadir_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-clean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };
    let store = FjallNodeStore::open(&path).expect("open sync state store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: true,
                maybe_sync_state: Some(phase62_durable_sync_state()),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save clean runtime metadata");
    drop(store);

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.datadir,
        FieldAvailability::available(path.display().to_string())
    );
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(true)
    );
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::available(ServicePriorShutdownStatus::Clean)
    );
    assert_eq!(
        restart_resume.durable_progress,
        FieldAvailability::available(ServiceResumeProgressStatus {
            downloaded_block_height: 840_006,
            connected_block_height: 840_004,
            maybe_downloaded_block_hash: Some("22".repeat(32)),
            maybe_connected_block_hash: Some("11".repeat(32)),
        })
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::available(ServiceStaleInflightStatus::Cleared)
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData)
    );
    assert_eq!(
        restart_resume.next_action,
        FieldAvailability::available(
            "Retry sync after peer backoff or choose a different peer.".to_string()
        )
    );
}

#[test]
fn service_restart_resume_status_surfaces_unclean_stale_inflight_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-unclean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };
    let mut durable_sync_state = phase62_durable_sync_state();
    durable_sync_state.sync.resource_pressure =
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 2,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 64,
            max_sync_rounds: 8,
            outbound_peers: 2,
            target_outbound_peers: 4,
        });
    let store = FjallNodeStore::open(&path).expect("open sync state store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: false,
                maybe_sync_state: Some(durable_sync_state),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save unclean runtime metadata");
    drop(store);

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::available(ServicePriorShutdownStatus::Unclean)
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::available(ServiceStaleInflightStatus::StaleRequestsRecorded)
    );
}

#[test]
fn service_restart_resume_status_reports_datadir_mismatch() {
    // Arrange
    let path = temp_path("service-restart-resume-datadir-mismatch");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };
    let store = FjallNodeStore::open(&path).expect("open sync state store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: true,
                maybe_sync_state: Some(phase62_durable_sync_state()),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save runtime metadata");
    drop(store);

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path);
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/different-open-bitcoin"),
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(false)
    );
}

#[test]
fn service_restart_resume_status_reports_unavailable_selected_datadir() {
    // Arrange
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/open-bitcoin"),
        ))),
        OperatorConfigResolution {
            maybe_data_dir: None,
            ..config_resolution()
        },
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.restart_resume,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: datadir unavailable"
        )
    );
}

#[test]
fn service_restart_resume_status_prefers_storage_recovery_action() {
    // Arrange
    let path = temp_path("service-restart-resume-storage-action");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };
    let store = FjallNodeStore::open(&path).expect("open sync state store");
    store
        .save_runtime_metadata(
            &RuntimeMetadata {
                last_clean_shutdown: false,
                maybe_last_recovery_action: Some(open_bitcoin_node::StorageRecoveryAction::Repair),
                maybe_sync_state: Some(phase62_durable_sync_state()),
                ..RuntimeMetadata::default()
            },
            PersistMode::Sync,
        )
        .expect("save runtime metadata");
    drop(store);

    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(path.clone());
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            &path,
        ))),
        resolution,
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should be available");
    };
    assert_eq!(
        restart_resume.next_action,
        FieldAvailability::available(
            "Run the storage repair flow before restarting normal operation.".to_string()
        )
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::available(SyncRecoveryCategory::StoreCorruption)
    );
}

#[test]
fn service_restart_resume_status_reports_unavailable_runtime_metadata() {
    // Arrange
    let input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            Path::new("/tmp/nonexistent-open-bitcoin-status"),
        ))),
        OperatorConfigResolution {
            maybe_data_dir: Some(PathBuf::from("/tmp/nonexistent-open-bitcoin-status")),
            ..config_resolution()
        },
    );

    // Act
    let snapshot = collect_status_snapshot(&input, None);

    // Assert
    assert_eq!(
        snapshot.service.restart_resume,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: runtime metadata unavailable"
        )
    );
}

fn service_snapshot(
    state: ServiceLifecycleState,
    maybe_enabled: Option<bool>,
    data_dir: &Path,
) -> ServiceStateSnapshot {
    ServiceStateSnapshot {
        state,
        maybe_enabled,
        maybe_service_file_path: Some(PathBuf::from("/tmp/open-bitcoin-node.service")),
        maybe_manager_diagnostics: None,
        maybe_log_path: Some(PathBuf::from("/tmp/logs/open-bitcoin.log")),
        maybe_log_path_unavailable_reason: None,
        maybe_data_dir: Some(data_dir.to_path_buf()),
        maybe_data_dir_unavailable_reason: None,
    }
}

fn status_input_with_manager(
    manager: Box<dyn crate::operator::service::ServiceManager>,
    config_resolution: OperatorConfigResolution,
) -> StatusCollectorInput {
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: None,
            include_live_rpc: false,
            no_color: false,
        },
        config_resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: Some(manager),
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

fn phase62_durable_sync_state() -> DurableSyncState {
    DurableSyncState {
        sync: SyncStatus {
            network: FieldAvailability::available("mainnet".to_string()),
            chain_tip: FieldAvailability::unavailable("chain tip unavailable"),
            sync_progress: FieldAvailability::available(SyncProgress {
                header_height: 840_100,
                block_height: 840_004,
                downloaded_block_height: 840_006,
                connected_block_height: 840_004,
                maybe_downloaded_block_hash: Some("22".repeat(32)),
                maybe_connected_block_hash: Some("11".repeat(32)),
                progress_ratio: 840_004.0 / 840_100.0,
                messages_processed: 7,
                headers_received: 3,
                blocks_received: 1,
            }),
            lifecycle: FieldAvailability::available(SyncLifecycleState::Active),
            phase: FieldAvailability::available("block_download".to_string()),
            configured_targets: FieldAvailability::available(SyncConfiguredTargets {
                target_outbound_peers: 4,
                maybe_target_header_height: Some(840_200),
            }),
            attempt_counters: FieldAvailability::available(SyncAttemptCounters {
                attempted_peers: 3,
                connected_peers: 2,
                failed_peers: 1,
                max_sync_rounds: 8,
            }),
            progress_signal: FieldAvailability::available(SyncProgressSignal::AwaitingBlocks),
            lag: FieldAvailability::available(SyncLagStatus {
                headers_remaining: 0,
                blocks_remaining: 96,
            }),
            last_successful_progress_unix_seconds: FieldAvailability::available(1_717_000_000),
            latest_stop_reason: FieldAvailability::available(SyncStopReasonStatus {
                label: "target_header_reached".to_string(),
                message:
                    "sync header target reached: target_header_height=840200 best_header_height=840200"
                        .to_string(),
            }),
            last_error: FieldAvailability::available("peer stalled before block connect".to_string()),
            recovery_category: FieldAvailability::available(SyncRecoveryCategory::InvalidPeerData),
            recovery_action: FieldAvailability::available(
                "Retry sync after peer backoff or choose a different peer.".to_string(),
            ),
            resource_pressure: FieldAvailability::available(SyncResourcePressure {
                blocks_in_flight: 0,
                max_header_requests_in_flight_per_peer: 1,
                max_headers_per_message: 2_000,
                max_blocks_in_flight_per_peer: 16,
                max_blocks_in_flight_total: 64,
                max_messages_per_peer: 64,
                max_sync_rounds: 8,
                outbound_peers: 2,
                target_outbound_peers: 4,
            }),
        },
        peers: PeerStatus {
            peer_counts: FieldAvailability::available(PeerCounts {
                inbound: 0,
                outbound: 2,
            }),
            recent_peers: FieldAvailability::unavailable("peer telemetry unavailable"),
        },
        health_signals: Vec::new(),
        updated_at_unix_seconds: 1_717_000_010,
    }
}

fn status_input(detected_installations: Vec<DetectedInstallation>) -> StatusCollectorInput {
    status_input_with_service_candidates(detected_installations, Vec::new())
}

fn status_input_with_service_candidates(
    detected_installations: Vec<DetectedInstallation>,
    service_candidates: Vec<ServiceCandidate>,
) -> StatusCollectorInput {
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Human,
            maybe_config_path: None,
            maybe_data_dir: None,
            maybe_network: Some(NetworkSelection::Regtest),
            include_live_rpc: true,
            no_color: false,
        },
        config_resolution: config_resolution(),
        detection_evidence: StatusDetectionEvidence {
            detected_installations,
            service_candidates,
        },
        maybe_live_rpc: Some(StatusLiveRpcAdapterInput {
            endpoint: "http://127.0.0.1:18443".to_string(),
            auth_source: StatusRpcAuthSource::CookieFile {
                path: PathBuf::from("/tmp/open-bitcoin/.cookie"),
            },
            timeout: Duration::from_secs(2),
        }),
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

fn config_resolution() -> OperatorConfigResolution {
    OperatorConfigResolution {
        path_reports: vec![
            OperatorConfigPathReport {
                source: OperatorConfigSource::Defaults,
                kind: OperatorConfigPathKind::ConfigFile,
                path: PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc"),
                present: false,
            },
            OperatorConfigPathReport {
                source: OperatorConfigSource::BitcoinConf,
                kind: OperatorConfigPathKind::BitcoinConf,
                path: PathBuf::from("/tmp/open-bitcoin/bitcoin.conf"),
                present: false,
            },
        ],
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc")),
        maybe_bitcoin_conf_path: Some(PathBuf::from("/tmp/open-bitcoin/bitcoin.conf")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_log_dir: Some(PathBuf::from("/tmp/open-bitcoin/logs")),
        maybe_metrics_store_path: Some(PathBuf::from("/tmp/open-bitcoin/metrics")),
        ..OperatorConfigResolution::default()
    }
}

fn detected_installation() -> DetectedInstallation {
    DetectedInstallation {
        product_family: ProductFamily::Unknown,
        confidence: DetectionConfidence::Low,
        uncertainty: vec![DetectionUncertainty::ProductAmbiguous],
        source_paths: vec![
            DetectionSourcePath {
                kind: DetectionSourcePathKind::DataDir,
                path: PathBuf::from("/tmp/core/.bitcoin"),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::ConfigFile,
                path: PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf"),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::CookieFile,
                path: PathBuf::from("/tmp/core/.bitcoin/.cookie"),
                present: true,
            },
        ],
        maybe_data_dir: Some(PathBuf::from("/tmp/core/.bitcoin")),
        maybe_config_file: Some(PathBuf::from("/tmp/core/.bitcoin/bitcoin.conf")),
        maybe_cookie_file: Some(PathBuf::from("/tmp/core/.bitcoin/.cookie")),
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::LegacyWalletFile,
            path: PathBuf::from("/tmp/core/.bitcoin/wallet.dat"),
            maybe_name: None,
            present: true,
            product_family: ProductFamily::Unknown,
            product_confidence: DetectionConfidence::Low,
            chain_scope: crate::operator::detect::WalletChainScope::Mainnet,
        }],
    }
}

fn detected_service_candidate() -> ServiceCandidate {
    ServiceCandidate {
        product_family: ProductFamily::Unknown,
        manager: ServiceManager::Systemd,
        service_name: "bitcoind".to_string(),
        path: PathBuf::from("/tmp/systemd/bitcoind.service"),
        present: true,
    }
}

#[derive(Debug, Clone)]
struct FakeStatusRpcClient {
    maybe_node_error: Option<StatusRpcError>,
    maybe_wallet_error: Option<StatusRpcError>,
}

impl FakeStatusRpcClient {
    fn running() -> Self {
        Self {
            maybe_node_error: None,
            maybe_wallet_error: None,
        }
    }

    fn failing(message: &str) -> Self {
        Self {
            maybe_node_error: Some(StatusRpcError::new(message)),
            maybe_wallet_error: None,
        }
    }

    fn wallet_failing(error: StatusRpcError) -> Self {
        Self {
            maybe_node_error: None,
            maybe_wallet_error: Some(error),
        }
    }

    fn maybe_node_error(&self) -> Result<(), StatusRpcError> {
        match &self.maybe_node_error {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }

    fn maybe_wallet_error(&self) -> Result<(), StatusRpcError> {
        self.maybe_node_error()?;
        match &self.maybe_wallet_error {
            Some(error) => Err(error.clone()),
            None => Ok(()),
        }
    }
}

impl StatusRpcClient for FakeStatusRpcClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
        Ok(GetNetworkInfoResponse {
            version: 29_300,
            subversion: "/Satoshi:29.3.0/".to_string(),
            protocolversion: 70_016,
            localservices: "0000000000000409".to_string(),
            localrelay: true,
            connections: 7,
            connections_in: 2,
            connections_out: 5,
            relayfee: 1_000,
            incrementalfee: 1_000,
            warnings: vec!["network warning".to_string()],
        })
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
        Ok(GetBlockchainInfoResponse {
            chain: "regtest".to_string(),
            blocks: 144,
            headers: 150,
            maybe_best_block_hash: Some("00aabb".to_string()),
            maybe_median_time_past: Some(1_777_225_000),
            verificationprogress: 0.96,
            initialblockdownload: false,
            warnings: vec!["chain warning".to_string()],
        })
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        self.maybe_node_error()?;
        Ok(GetMempoolInfoResponse {
            size: 12,
            bytes: 2048,
            usage: 4096,
            total_fee_sats: 320,
            maxmempool: 300_000_000,
            mempoolminfee: 1_000,
            minrelaytxfee: 1_000,
            loaded: true,
        })
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        self.maybe_wallet_error()?;
        Ok(GetWalletInfoResponse {
            network: "regtest".to_string(),
            descriptor_count: 2,
            utxo_count: 1,
            maybe_tip_height: Some(144),
            maybe_tip_median_time_past: Some(1_777_225_000),
        })
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        self.maybe_wallet_error()?;
        Ok(GetBalancesResponse {
            mine: WalletBalanceDetails {
                trusted_sats: 50_000,
                untrusted_pending_sats: 0,
                immature_sats: 0,
            },
        })
    }
}

fn managed_wallet_store(
    test_name: &str,
    wallet_names: &[&str],
    maybe_selected_wallet_name: Option<&str>,
) -> TempDirGuard {
    let path = temp_path(test_name);
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("wallet store");
    let mut registry = WalletRegistry::default();

    for wallet_name in wallet_names {
        registry
            .create_wallet(
                &store,
                *wallet_name,
                Wallet::new(AddressNetwork::Regtest),
                PersistMode::Sync,
            )
            .expect("create wallet");
    }
    if let Some(selected_wallet_name) = maybe_selected_wallet_name {
        registry
            .set_selected_wallet(&store, selected_wallet_name, PersistMode::Sync)
            .expect("select wallet");
    }

    TempDirGuard { path }
}

fn temp_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-status-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

struct TempDirGuard {
    path: PathBuf,
}

impl TempDirGuard {
    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        remove_dir_if_exists(&self.path);
    }
}

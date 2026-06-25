// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    fs::{self, File},
    io,
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
    BestKnownTipStatus, BuildProvenance, ConfigStatus, FieldAvailability,
    INBOUND_STATUS_UNAVAILABLE_REASON, InboundAdmissionEvent, InboundHandshakeStatusCounts,
    InboundPeerServingStatus, MempoolStatus, NodeRuntimeState, NodeStatus,
    OpenBitcoinStatusSnapshot, PeerCounts, PeerStatus, ServiceLifecycleStatus, ServiceStatus,
    StayCurrentStatus, SyncAttemptCounters, SyncConfiguredTargets, SyncProgressSignal, SyncStatus,
    SyncStopReasonStatus, WalletFreshness, WalletScanProgress, WalletStatus,
};
use open_bitcoin_node::storage::FJALL_LOCK_FILE_NAME;
use open_bitcoin_rpc::{
    RpcErrorCode, RpcErrorDetail,
    method::{
        GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse,
        GetNetworkInfoResponse, GetWalletInfoResponse, OpenBitcoinNetworkStatusResponse,
        WalletBalanceDetails,
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
fn inbound_status_fake_live_rpc_maps_into_shared_status_snapshot() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::running_with_inbound_status();

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["inbound"], 2);
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["outbound"], 5);
    assert_eq!(decoded["peers"]["inbound"]["state"], "available");
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["listener_state"],
        "listening"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["bound_endpoints"][0],
        "127.0.0.1:18444"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["preflight_reason"],
        "ready"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["admitted_inbound_peers"],
        2
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["rejected_inbound_peers"],
        3
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["latest_admission_event"]["value"]["reason"],
        "duplicate_peer_id"
    );
}

#[test]
fn inbound_status_missing_method_keeps_peer_counts_available() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::network_status_failing(StatusRpcError::from_rpc_detail(
        RpcErrorDetail::new(RpcErrorCode::MethodNotFound, "Method not found"),
    ));

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["peers"]["peer_counts"]["state"], "available");
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["inbound"], 2);
    assert_eq!(decoded["peers"]["peer_counts"]["value"]["outbound"], 5);
    assert_eq!(decoded["peers"]["inbound"]["state"], "unavailable");
    let reason = decoded["peers"]["inbound"]["value"]["reason"]
        .as_str()
        .expect("inbound unavailable reason");
    assert!(reason.contains("openbitcoinnetworkstatus"));
    assert!(reason.contains("Method not found"));
    assert_ne!(reason, INBOUND_STATUS_UNAVAILABLE_REASON);
}

#[test]
fn inbound_status_snapshot_does_not_render_rpc_secrets() {
    // Arrange
    let mut input = status_input(Vec::new());
    input.maybe_live_rpc = Some(StatusLiveRpcAdapterInput {
        endpoint: "http://rpcuser:super-secret@127.0.0.1:18443".to_string(),
        auth_source: StatusRpcAuthSource::CookieFile {
            path: PathBuf::from("/tmp/open-bitcoin/super-secret.cookie"),
        },
        timeout: Duration::from_secs(2),
    });
    let rpc = FakeStatusRpcClient::running_with_inbound_status();

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");

    // Assert
    assert!(!rendered.contains("super-secret"));
    assert!(!rendered.contains("rpcuser"));
    assert!(!rendered.contains(".cookie"));
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
fn status_recovery_evidence_stopped_empty_datadir_does_not_create_fjall_files() {
    // Arrange
    let path = temp_path("recovery-evidence-empty-datadir");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("empty datadir");
    let _guard = TempDirGuard { path: path.clone() };
    let input = status_input_for_data_dir(&path);

    // Act
    let snapshot = collect_status_snapshot(&input, None);
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "stopped");
    assert_eq!(decoded["recovery_evidence"]["state"], "unavailable");
    assert_eq!(
        decoded["recovery_evidence"]["value"]["reason"],
        "recovery evidence unavailable: no storage, lock, service, or RPC signal"
    );
    assert_eq!(
        decoded["metrics"]["availability"]["reason"],
        "metrics history unavailable: probe-only status does not open Fjall stores"
    );
    assert_empty_dir(&path);
}

#[test]
fn status_recovery_evidence_stale_lock_reports_read_only_inspection() {
    // Arrange
    let path = temp_path("recovery-evidence-stale-lock");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("datadir");
    fs::write(path.join(FJALL_LOCK_FILE_NAME), "").expect("stale lock");
    let _guard = TempDirGuard { path: path.clone() };
    let input = status_input_for_data_dir(&path);

    // Act
    let snapshot = collect_status_snapshot(&input, None);
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["recovery_evidence"]["state"], "available");
    assert_eq!(
        decoded["recovery_evidence"]["value"]["category"],
        "storage_lock_contention"
    );
    assert_eq!(
        decoded["recovery_evidence"]["value"]["cause"],
        "stale_lock_evidence"
    );
    assert_eq!(
        decoded["recovery_evidence"]["value"]["action_class"],
        "read_only_inspection"
    );
}

#[test]
fn status_recovery_evidence_concurrent_datadir_uses_service_and_rpc_evidence() {
    // Arrange
    let path = temp_path("recovery-evidence-concurrent-datadir");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("datadir");
    let _guard = TempDirGuard { path: path.clone() };
    let lock_path = path.join(FJALL_LOCK_FILE_NAME);
    let lock_file = File::create(&lock_path).expect("lock file");
    lock_file.try_lock().expect("hold lock");
    let _lock_guard = lock_file;
    let input = status_input_with_running_manager_and_live_rpc(&path);
    let rpc = FakeStatusRpcClient::running();

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["node"]["state"], "running");
    assert_eq!(decoded["recovery_evidence"]["state"], "available");
    assert_eq!(
        decoded["recovery_evidence"]["value"]["category"],
        "storage_lock_contention"
    );
    assert_eq!(
        decoded["recovery_evidence"]["value"]["cause"],
        "concurrent_datadir_use"
    );
}

#[test]
fn status_recovery_evidence_missing_datadir_remains_explicit_unavailable_json() {
    // Arrange
    let path = temp_path("recovery-evidence-missing-datadir");
    remove_dir_if_exists(&path);
    let input = status_input_for_data_dir(&path);

    // Act
    let snapshot = collect_status_snapshot(&input, None);
    let rendered = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&rendered).expect("decode status json");

    // Assert
    assert_eq!(decoded["recovery_evidence"]["state"], "unavailable");
    assert_eq!(
        decoded["recovery_evidence"]["value"]["reason"],
        "recovery evidence unavailable: no storage, lock, service, or RPC signal"
    );
}

#[test]
fn status_recovery_evidence_render_human_line_follows_sync_recovery() {
    // Arrange
    let path = temp_path("recovery-evidence-render-position");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("datadir");
    fs::write(path.join(FJALL_LOCK_FILE_NAME), "").expect("stale lock");
    let _guard = TempDirGuard { path: path.clone() };
    let snapshot = collect_status_snapshot(&status_input_for_data_dir(&path), None);

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let lines = rendered.lines().collect::<Vec<_>>();
    let sync_recovery_index = lines
        .iter()
        .position(|line| line.starts_with("Sync recovery:"))
        .expect("sync recovery line");

    // Assert
    assert!(
        lines
            .get(sync_recovery_index + 1)
            .expect("recovery evidence line")
            .starts_with("Recovery evidence:")
    );
}

#[test]
fn status_recovery_evidence_render_human_available_labels() {
    // Arrange
    let path = temp_path("recovery-evidence-render-available");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("datadir");
    fs::write(path.join(FJALL_LOCK_FILE_NAME), "").expect("stale lock");
    let _guard = TempDirGuard { path: path.clone() };
    let snapshot = collect_status_snapshot(&status_input_for_data_dir(&path), None);

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "Recovery evidence: category=storage_lock_contention cause=stale_lock_evidence action_class=read_only_inspection next_action="
    ));
}

#[test]
fn status_recovery_evidence_render_human_unavailable_reason() {
    // Arrange
    let path = temp_path("recovery-evidence-render-unavailable");
    remove_dir_if_exists(&path);
    let snapshot = collect_status_snapshot(&status_input_for_data_dir(&path), None);

    // Act
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains(
        "Recovery evidence: Unavailable: recovery evidence unavailable: no storage, lock, service, or RPC signal"
    ));
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

    let path = temp_path("phase63-manager-error-probe-only");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

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

    assert!(matches!(
        snapshot.sync.configured_targets,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.sync.attempt_counters,
        FieldAvailability::Unavailable { .. }
    ));
    assert!(matches!(
        snapshot.sync.latest_stop_reason,
        FieldAvailability::Unavailable { .. }
    ));
    assert_eq!(
        snapshot.sync.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
}

#[test]
fn service_restart_resume_status_surfaces_same_datadir_without_runtime_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-clean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

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
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
    assert_eq!(
        restart_resume.next_action,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
}

#[test]
fn service_restart_resume_status_does_not_load_unclean_stale_inflight_metadata() {
    // Arrange
    let path = temp_path("service-restart-resume-unclean");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

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
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.stale_inflight,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
}

#[test]
fn service_restart_resume_status_reports_datadir_mismatch() {
    // Arrange
    let path = temp_path("service-restart-resume-datadir-mismatch");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

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
fn service_restart_resume_status_does_not_load_storage_recovery_action() {
    // Arrange
    let path = temp_path("service-restart-resume-storage-action");
    remove_dir_if_exists(&path);
    let _guard = TempDirGuard { path: path.clone() };

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
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
        )
    );
    assert_eq!(
        restart_resume.recovery_category,
        FieldAvailability::unavailable("no recovery category recorded")
    );
}

#[test]
fn service_restart_resume_status_reports_probe_only_runtime_metadata_unavailable() {
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
    let FieldAvailability::Available(restart_resume) = snapshot.service.restart_resume else {
        panic!("restart/resume evidence should keep service same-datadir evidence");
    };
    assert_eq!(
        restart_resume.same_datadir,
        FieldAvailability::available(true)
    );
    assert_eq!(
        restart_resume.prior_shutdown,
        FieldAvailability::unavailable(
            "service restart/resume evidence unavailable: probe-only status does not open Fjall stores"
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

fn status_input(detected_installations: Vec<DetectedInstallation>) -> StatusCollectorInput {
    status_input_with_service_candidates(detected_installations, Vec::new())
}

fn status_input_for_data_dir(data_dir: &Path) -> StatusCollectorInput {
    let mut resolution = config_resolution();
    resolution.maybe_data_dir = Some(data_dir.to_path_buf());
    resolution.maybe_log_dir = None;
    resolution.maybe_metrics_store_path = None;
    StatusCollectorInput {
        request: StatusRequest {
            render_mode: StatusRenderMode::Json,
            maybe_config_path: None,
            maybe_data_dir: Some(data_dir.to_path_buf()),
            maybe_network: Some(NetworkSelection::Regtest),
            include_live_rpc: false,
            no_color: true,
        },
        config_resolution: resolution,
        detection_evidence: StatusDetectionEvidence {
            detected_installations: Vec::new(),
            service_candidates: Vec::new(),
        },
        maybe_live_rpc: None,
        maybe_service_manager: None,
        wallet_rpc_access: StatusWalletRpcAccess::Root,
    }
}

fn status_input_with_running_manager_and_live_rpc(data_dir: &Path) -> StatusCollectorInput {
    let mut input = status_input_with_manager(
        Box::new(FakeServiceManager::new(service_snapshot(
            ServiceLifecycleState::Running,
            Some(true),
            data_dir,
        ))),
        {
            let mut resolution = config_resolution();
            resolution.maybe_data_dir = Some(data_dir.to_path_buf());
            resolution.maybe_log_dir = None;
            resolution.maybe_metrics_store_path = None;
            resolution
        },
    );
    input.request.render_mode = StatusRenderMode::Json;
    input.request.include_live_rpc = true;
    input.maybe_live_rpc = Some(StatusLiveRpcAdapterInput {
        endpoint: "http://127.0.0.1:18443".to_string(),
        auth_source: StatusRpcAuthSource::CookieFile {
            path: data_dir.join(".cookie"),
        },
        timeout: Duration::from_secs(2),
    });
    input
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
    maybe_network_status_error: Option<StatusRpcError>,
    maybe_network_status: Option<OpenBitcoinNetworkStatusResponse>,
    maybe_wallet_error: Option<StatusRpcError>,
}

impl FakeStatusRpcClient {
    fn running() -> Self {
        Self {
            maybe_node_error: None,
            maybe_network_status_error: None,
            maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
            }),
            maybe_wallet_error: None,
        }
    }

    fn running_with_inbound_status() -> Self {
        Self {
            maybe_network_status: Some(inbound_status_response()),
            ..Self::running()
        }
    }

    fn failing(message: &str) -> Self {
        Self {
            maybe_node_error: Some(StatusRpcError::new(message)),
            maybe_network_status_error: None,
            maybe_network_status: None,
            maybe_wallet_error: None,
        }
    }

    fn network_status_failing(error: StatusRpcError) -> Self {
        Self {
            maybe_network_status_error: Some(error),
            maybe_network_status: None,
            ..Self::running()
        }
    }

    fn wallet_failing(error: StatusRpcError) -> Self {
        Self {
            maybe_node_error: None,
            maybe_network_status_error: None,
            maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
            }),
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

    fn get_open_bitcoin_network_status(
        &self,
    ) -> Result<OpenBitcoinNetworkStatusResponse, StatusRpcError> {
        self.maybe_node_error()?;
        if let Some(error) = &self.maybe_network_status_error {
            return Err(error.clone());
        }

        Ok(self
            .maybe_network_status
            .clone()
            .unwrap_or_else(|| OpenBitcoinNetworkStatusResponse {
                inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                    INBOUND_STATUS_UNAVAILABLE_REASON,
                ),
            }))
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

fn inbound_status_response() -> OpenBitcoinNetworkStatusResponse {
    OpenBitcoinNetworkStatusResponse {
        inbound: FieldAvailability::available(InboundPeerServingStatus {
            listener_state: "listening".to_string(),
            bound_endpoints: vec!["127.0.0.1:18444".to_string()],
            preflight_reason: "ready".to_string(),
            admitted_inbound_peers: 2,
            rejected_inbound_peers: 3,
            handshake: InboundHandshakeStatusCounts {
                awaiting_version: 1,
                awaiting_verack: 0,
                established: 2,
                disconnected: 1,
            },
            duplicate_rejects: 1,
            self_connection_rejects: 1,
            cap_rejects: 1,
            reserved_slot_rejects: 0,
            latest_admission_event: FieldAvailability::available(InboundAdmissionEvent {
                outcome: "rejected".to_string(),
                reason: "duplicate_peer_id".to_string(),
                slot_class: "ordinary".to_string(),
                message: "duplicate inbound peer id rejected".to_string(),
            }),
            permissioned_inbound_peers: 0,
            protected_inbound_peers: 0,
            permission_class: "ordinary_inbound".to_string(),
            active_permission_effects: Vec::new(),
            inactive_permission_effects: Vec::new(),
            latest_permission_decision: FieldAvailability::unavailable(
                "inbound permission decision evidence unavailable",
            ),
        }),
    }
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

fn assert_empty_dir(path: &Path) {
    let entries = fs::read_dir(path)
        .expect("read datadir")
        .collect::<Result<Vec<_>, _>>()
        .expect("datadir entries");
    assert!(
        entries.is_empty(),
        "datadir should remain empty: {entries:?}"
    );
}

struct TempDirGuard {
    path: PathBuf,
}

impl Drop for TempDirGuard {
    fn drop(&mut self) {
        remove_dir_if_exists(&self.path);
    }
}

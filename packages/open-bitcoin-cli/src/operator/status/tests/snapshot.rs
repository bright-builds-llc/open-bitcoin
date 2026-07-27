// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
    let snapshot: OpenBitcoinStatusSnapshot = collect_status_snapshot(&input, Some(&rpc));
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
fn fake_live_rpc_maps_metrics_from_open_bitcoin_network_status() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient {
        maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
            inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                INBOUND_STATUS_UNAVAILABLE_REASON,
            ),
            relay: RelayEvidenceStatus::default(),
            block_relay: BlockRelayEvidenceStatus::default_unavailable(),
            metrics: MetricsStatus::available_with_samples(
                MetricRetentionPolicy::default(),
                vec![MetricSample::new(
                    MetricKind::InboundAdmittedPeerCount,
                    2.0,
                    1_777_225_022,
                )],
            ),
        }),
        ..FakeStatusRpcClient::running()
    };

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));

    // Assert
    assert_eq!(snapshot.metrics.samples.len(), 1);
    assert_eq!(
        snapshot.metrics.samples[0].kind,
        MetricKind::InboundAdmittedPeerCount
    );
    assert_eq!(snapshot.metrics.samples[0].value, 2.0);
}

#[test]
fn operator_status_renders_relay_evidence_from_open_bitcoin_network_status() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient {
        maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
            inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                INBOUND_STATUS_UNAVAILABLE_REASON,
            ),
            relay: relay_evidence_status_fixture(),
            block_relay: block_relay_evidence_status_fixture(),
            metrics: MetricsStatus::default(),
        }),
        ..FakeStatusRpcClient::running()
    };

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");

    // Assert
    assert_eq!(
        decoded["mempool"]["relay"]["outcome_counters"]["value"]["accepted_count"],
        1
    );
    assert_eq!(
        decoded["mempool"]["relay"]["outcome_counters"]["value"]["rebroadcast_deferred_count"],
        10
    );
    assert_eq!(
        decoded["mempool"]["relay"]["recovery_counters"]["value"]["recovered_count"],
        11
    );
    assert_eq!(
        decoded["mempool"]["relay"]["recovery_counters"]["value"]["dropped_evicted_count"],
        16
    );
    assert_eq!(
        decoded["mempool"]["relay"]["activation"]["value"]["enabled"],
        true
    );
    assert_eq!(
        decoded["mempool"]["relay"]["download_eligibility"]["value"]["eligible_peer_count"],
        1
    );
    assert_eq!(
        decoded["mempool"]["relay"]["download_eligibility"]["value"]["permission_required_count"],
        4
    );
    assert_eq!(
        decoded["mempool"]["relay"]["mempool_admission"]["state"],
        "implemented"
    );
    assert_eq!(
        decoded["mempool"]["relay"]["rebroadcast"]["state"],
        "deferred"
    );
    assert!(human.contains("Relay evidence: accepted_count=1 rejected_count=2"));
    assert!(human.contains(
        "Relay recovery: recovered_count=11 dropped_confirmed_count=12 dropped_duplicate_count=13 dropped_missing_parent_count=14 dropped_policy_incompatible_count=15 dropped_evicted_count=16"
    ));
    assert!(human.contains("Mempool evidence: Implemented: mempool_admission"));
    assert!(
        human.contains("Rebroadcast: deferred: Deferred: rebroadcast relay evidence not projected")
    );
    assert!(human.contains(
        "Public relay: Intentionally different: public relay readiness is intentionally not claimed"
    ));
    for forbidden in [
        "0100000001",
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "wtxid",
        "127.0.0.1:18444",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!human.contains(forbidden), "human leaked {forbidden}");
        assert!(!json.contains(forbidden), "json leaked {forbidden}");
    }
}

#[test]
fn operator_status_block_relay_maps_shared_contract_and_human_lines() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient {
        maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
            inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                INBOUND_STATUS_UNAVAILABLE_REASON,
            ),
            relay: RelayEvidenceStatus::default(),
            block_relay: block_relay_evidence_status_fixture(),
            metrics: MetricsStatus::default(),
        }),
        ..FakeStatusRpcClient::running()
    };

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");

    // Assert
    assert_eq!(
        decoded["block_relay"]["block_serving"]["activation"]["value"]["block_serving_enabled"],
        true
    );
    assert_eq!(
        decoded["block_relay"]["negotiation"]["value"]["version2_high_bandwidth_count"],
        3
    );
    assert_eq!(
        decoded["block_relay"]["reconstruction"]["value"]["compact_malformed_count"],
        1
    );
    assert_eq!(
        decoded["block_relay"]["cleanup"]["value"]["compact_cleanup_count"],
        3
    );
    assert!(human.contains("Block relay evidence"));
    assert!(
        human.contains(
            "Block relay activation: block_serving_enabled=true compact_relay_enabled=true"
        )
    );
    assert!(human.contains(
        "Compact reconstruction: compact_reconstructed_count=4 compact_reconstruction_failed_count=1 compact_malformed_count=1"
    ));
    assert!(human.contains(
        "Compact cleanup: compact_cleanup_count=3 compact_download_peer_disconnect_count=1 compact_download_timeout_count=1"
    ));
    for forbidden in [
        "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef",
        "cmpctblock",
        "127.0.0.1:18444",
        "peer_id",
        "permission_string",
        "credential",
        "cookie",
        "secret",
        "dynamic_label",
    ] {
        assert!(!human.contains(forbidden), "human leaked {forbidden}");
        assert!(!json.contains(forbidden), "json leaked {forbidden}");
    }
}

#[test]
fn operator_status_block_relay_fallback_uses_default_unavailable_contract() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::network_status_failing(StatusRpcError::from_rpc_detail(
        RpcErrorDetail::new(RpcErrorCode::MethodNotFound, "Method not found"),
    ));

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("status json");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");

    // Assert
    assert_eq!(
        decoded["block_relay"]["block_serving"]["activation"]["state"],
        "unavailable"
    );
    assert_eq!(
        decoded["block_relay"]["block_serving"]["activation"]["value"]["reason"],
        "block serving evidence unavailable"
    );
    assert_eq!(
        decoded["block_relay"]["cleanup"]["value"]["compact_cleanup_count"],
        0
    );
    assert!(human.contains("Block relay evidence"));
    assert!(
        human.contains("Block relay activation: Unavailable: block serving evidence unavailable")
    );
}

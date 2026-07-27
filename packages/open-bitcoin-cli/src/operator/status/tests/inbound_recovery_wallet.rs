// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

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
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["permissioned_inbound_peers"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["protected_inbound_peers"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["permission_class"],
        "protected_inbound"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["active_permission_effects"],
        serde_json::json!([
            "admission_protected",
            "eviction_policy_protected",
            "download_serving_policy_input"
        ])
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["inactive_permission_effects"],
        serde_json::json!([
            "inactive_relay",
            "inactive_mempool",
            "inactive_blockfilters"
        ])
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["latest_permission_decision"]["value"]["permission_class"],
        "protected_inbound"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["latest_permission_decision"]["value"]["active_permission_effects"],
        serde_json::json!(["admission_protected", "download_serving_policy_input"])
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["latest_permission_decision"]["value"]["inactive_permission_effects"],
        serde_json::json!(["inactive_relay"])
    );
    let inbound = decoded["peers"]["inbound"]["value"]
        .as_object()
        .expect("inbound status object");
    for field_name in [
        "local_advertisement_candidates",
        "suppressed_advertisements",
        "getaddr_responses_served",
        "getaddr_requests_suppressed",
        "learned_address_entries",
        "learned_address_rejections",
        "latest_address_decision",
        "eviction_candidates_evaluated",
        "disconnects_requested",
        "discouraged_peers",
        "active_bans",
        "expired_bans",
        "manual_unbans",
        "misbehavior_observations",
        "protected_no_actions",
        "latest_peer_policy_decision",
    ] {
        assert!(inbound.contains_key(field_name), "missing {field_name}");
    }
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["local_advertisement_candidates"][0]["source"],
        "source_local_listener"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["suppressed_advertisements"][0]["reason"],
        "not_publicly_routable"
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["getaddr_responses_served"],
        3
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["getaddr_requests_suppressed"],
        2
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["learned_address_entries"],
        5
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["learned_address_rejections"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["eviction_candidates_evaluated"],
        2
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["disconnects_requested"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["misbehavior_observations"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["protected_no_actions"],
        1
    );
    assert_eq!(
        decoded["peers"]["inbound"]["value"]["latest_peer_policy_decision"]["value"]["label"],
        "misbehavior_policy_decision"
    );
}

#[test]
fn inbound_status_output_names_bounded_address_behavior_without_relay_claims() {
    // Arrange
    let input = status_input(Vec::new());
    let rpc = FakeStatusRpcClient::running_with_inbound_status();

    // Act
    let snapshot = collect_status_snapshot(&input, Some(&rpc));
    let rendered = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(rendered.contains("bounded getaddr"));
    for forbidden in [
        "full address relay",
        "peer discovery support",
        "public inbound by default",
        "address_bytes",
        "peer_id=",
        "operator_loopback",
        "raw_permission",
    ] {
        assert!(!rendered.contains(forbidden), "leaked {forbidden}");
    }
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
    for forbidden in [
        "super-secret",
        "rpcuser",
        ".cookie",
        "operator_loopback",
        "operator-loopback",
        "in,noban",
        "127.0.0.1 permission class",
        "rpc_password",
        "cookie",
    ] {
        assert!(!rendered.contains(forbidden), "leaked {forbidden}");
    }
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

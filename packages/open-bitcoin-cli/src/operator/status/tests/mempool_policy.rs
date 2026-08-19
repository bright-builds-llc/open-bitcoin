// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

const POLICY_LABELS: [&str; 14] = [
    "Virtual size:",
    "Accounted usage:",
    "Accounted capacity:",
    "Static relay floor:",
    "Rolling mempool floor:",
    "Effective admission floor:",
    "Incremental relay fee:",
    "Pressure removals:",
    "Eviction (relay):",
    "Checkpoint:",
    "Recovery:",
    "Retry:",
    "Admission states:",
    "Relay states:",
];

fn available_policy_mempool() -> MempoolStatus {
    MempoolStatus {
        transactions: FieldAvailability::available(12),
        relay: relay_evidence_status_fixture(),
        resources: FieldAvailability::available(MempoolResourcesGroup {
            virtual_size: 2_048,
            accounted_usage: 4_096,
            accounted_capacity: 300_000_000,
            transaction_count: 12,
        }),
        fee_floors: FieldAvailability::available(MempoolFeeFloorsGroup {
            static_relay_floor: 1_000,
            rolling_mempool_floor: 2_000,
            effective_admission_floor: 2_000,
            incremental_relay_fee: 1_000,
        }),
        pressure: FieldAvailability::available(MempoolPressureGroup {
            pressure_removal_count: 3,
            decay_half_life_label: "half_life_12h".to_string(),
        }),
        eviction: FieldAvailability::available(MempoolEvictionGroup {
            pressure_removal_count: 3,
        }),
        checkpoint: FieldAvailability::available(MempoolCheckpointGroup {
            outcome: "succeeded".to_string(),
            overdue: false,
            persistence_strength: "sync".to_string(),
            age_seconds: Some(12),
            loss_bound_seconds: Some(30),
            dirty_generation_present: false,
        }),
        recovery: FieldAvailability::available(MempoolRecoveryGroup {
            recovered_count: 1,
            dropped_confirmed_count: 2,
            dropped_duplicate_count: 3,
            dropped_missing_parent_count: 4,
            dropped_policy_incompatible_count: 5,
            dropped_expired_count: 6,
            dropped_evicted_count: 7,
        }),
        retry: FieldAvailability::available(MempoolRetryGroup {
            eligible: 1,
            queued: 2,
            attempted: 0,
            emitted: 3,
            requested: 4,
            served: 5,
            suppressed: 6,
            relay_disabled: 0,
            cleared: 7,
        }),
        admission: FieldAvailability::available(MempoolAdmissionGroup {
            accepted: 8,
            still_present: 9,
            cleared: 10,
        }),
    }
}

fn live_rpc_with_policy_mempool() -> FakeStatusRpcClient {
    FakeStatusRpcClient {
        maybe_network_status: Some(OpenBitcoinNetworkStatusResponse {
            inbound: FieldAvailability::<InboundPeerServingStatus>::unavailable(
                INBOUND_STATUS_UNAVAILABLE_REASON,
            ),
            relay: relay_evidence_status_fixture(),
            block_relay: BlockRelayEvidenceStatus::default_unavailable(),
            metrics: MetricsStatus::default(),
            mempool: available_policy_mempool(),
        }),
        ..FakeStatusRpcClient::running()
    }
}

fn rendered_live_human_status() -> String {
    let snapshot = collect_status_snapshot(
        &status_input(Vec::new()),
        Some(&live_rpc_with_policy_mempool()),
    );
    render_status(&snapshot, StatusRenderMode::Human).expect("human status")
}

#[test]
fn human_status_inserts_virtual_size_before_relay_evidence() {
    // Arrange
    let human = rendered_live_human_status();
    let lines = human.lines().collect::<Vec<_>>();

    // Act
    let maybe_mempool = lines.iter().position(|line| line.starts_with("Mempool:"));
    let maybe_virtual_size = lines
        .iter()
        .position(|line| line.starts_with("Virtual size:"));
    let maybe_relay_evidence = lines
        .iter()
        .position(|line| line.starts_with("Relay evidence:"));

    // Assert
    let mempool_idx = maybe_mempool.expect("Mempool line");
    let virtual_size_idx = maybe_virtual_size.expect("Virtual size line");
    let relay_evidence_idx = maybe_relay_evidence.expect("Relay evidence line");
    assert!(mempool_idx < virtual_size_idx);
    assert!(virtual_size_idx < relay_evidence_idx);
    assert_eq!(lines[virtual_size_idx], "Virtual size: 2048 vbytes");
}

#[test]
fn human_status_uses_accounted_usage_not_knots_bytes_label() {
    // Arrange
    let human = rendered_live_human_status();

    // Act
    let has_knots_bytes_label = human.lines().any(|line| {
        line.starts_with("bytes:")
            || line.starts_with("Bytes:")
            || line.starts_with("usage:")
            || line.starts_with("maxmempool:")
    });

    // Assert
    assert!(human.contains("Accounted usage: 4096 accounted bytes"));
    assert!(human.contains("Accounted capacity: 300000000 accounted bytes"));
    assert!(!has_knots_bytes_label);
}

#[test]
fn stopped_status_policy_lines_share_transactions_unavailable_reason() {
    // Arrange
    let snapshot = collect_status_snapshot(&status_input(Vec::new()), None);

    // Act
    let human = render_status(&snapshot, StatusRenderMode::Human).expect("human status");

    // Assert
    assert!(human.contains("Mempool: Unavailable: node stopped"));
    for label in POLICY_LABELS {
        let line = human
            .lines()
            .find(|candidate| candidate.starts_with(label))
            .unwrap_or_else(|| panic!("missing {label}"));
        assert!(
            line.contains("Unavailable: node stopped"),
            "{label} used a different unavailable reason: {line}"
        );
    }
}

#[test]
fn json_status_has_mempool_fee_floors_and_no_last_package_members() {
    // Arrange
    let snapshot = collect_status_snapshot(
        &status_input(Vec::new()),
        Some(&live_rpc_with_policy_mempool()),
    );

    // Act
    let json = render_status(&snapshot, StatusRenderMode::Json).expect("json status");
    let decoded: serde_json::Value = serde_json::from_str(&json).expect("decode status json");
    let mempool = &decoded["mempool"];

    // Assert
    assert_eq!(mempool["fee_floors"]["state"], "available");
    assert_eq!(
        mempool["fee_floors"]["value"]["effective_admission_floor"],
        2_000
    );
    assert_eq!(mempool["resources"]["state"], "available");
    assert_eq!(mempool["admission"]["state"], "available");
    for forbidden_key in [
        "last_package",
        "package_members",
        "members",
        "fingerprint",
        "txid",
        "wtxid",
    ] {
        assert!(
            mempool.get(forbidden_key).is_none(),
            "snapshot.mempool leaked {forbidden_key}"
        );
        assert!(
            !json.contains(&format!("\"{forbidden_key}\"")),
            "json leaked {forbidden_key}"
        );
    }
}

#[test]
fn human_status_forbids_mempoolminfee_label() {
    // Arrange
    let human = rendered_live_human_status();

    // Act
    let has_effective_admission = human.contains("Effective admission floor: 2000 sat/kvB");

    // Assert
    assert!(has_effective_admission);
    assert!(!human.contains("mempoolminfee"));
    assert!(human.contains("Static relay floor: 1000 sat/kvB"));
    assert!(human.contains("Rolling mempool floor: 2000 sat/kvB"));
    assert!(human.contains("Incremental relay fee: 1000 sat/kvB"));
}

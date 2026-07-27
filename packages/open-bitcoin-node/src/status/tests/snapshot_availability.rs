// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn stopped_node_snapshot_keeps_unavailable_live_fields_explicit() {
    // Arrange / Act
    let snapshot = stopped_snapshot();
    let encoded = serde_json::to_value(&snapshot).expect("snapshot json");

    // Assert
    assert_eq!(snapshot.node.state, NodeRuntimeState::Stopped);
    assert_eq!(encoded["sync"]["network"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["chain_tip"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["sync_progress"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lifecycle"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["phase"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["configured_targets"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["attempt_counters"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["progress_signal"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["lag"]["state"], "unavailable");
    assert_eq!(
        encoded["sync"]["last_successful_progress_unix_seconds"]["state"],
        "unavailable"
    );
    assert_eq!(
        encoded["sync"]["latest_stop_reason"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["sync"]["last_error"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_category"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["recovery_action"]["state"], "unavailable");
    assert_eq!(encoded["sync"]["resource_pressure"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["peer_counts"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["recent_peers"]["state"], "unavailable");
    assert_eq!(encoded["peers"]["inbound"]["state"], "unavailable");
    assert_eq!(encoded["mempool"]["transactions"]["state"], "unavailable");
    assert_eq!(
        encoded["wallet"]["trusted_balance_sats"]["state"],
        "unavailable"
    );
    assert_eq!(encoded["wallet"]["freshness"]["state"], "unavailable");
    assert_eq!(encoded["wallet"]["scan_progress"]["state"], "unavailable");
    assert_eq!(encoded["config"]["datadir"]["state"], "available");
    assert_eq!(encoded["logs"]["retention"]["max_files"], 14);
    assert_eq!(
        encoded["metrics"]["retention"]["sample_interval_seconds"],
        30
    );
    assert_eq!(encoded["resource_bounds"]["state"], "unavailable");
}

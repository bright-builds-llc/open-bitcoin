// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_metrics_history_appends_across_runs() {
    // Arrange
    let path = temp_store_path("metrics-history");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        let mut transport =
            ScriptedTransport::new(vec![version_verack_script(0), version_verack_script(1)]);

        // Act
        runtime
            .sync_once(&mut transport, 1_777_225_022)
            .expect("first sync");
        runtime
            .sync_once(&mut transport, 1_777_225_052)
            .expect("second sync");
    }

    // Assert
    let reopened = FjallNodeStore::open(&path).expect("reopen store");
    let metrics = reopened
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    let mut sync_height_timestamps = metrics
        .samples
        .iter()
        .filter(|sample| sample.kind == MetricKind::SyncHeight)
        .map(|sample| sample.timestamp_unix_seconds)
        .collect::<Vec<_>>();
    sync_height_timestamps.sort_unstable();
    sync_height_timestamps.dedup();
    assert!(sync_height_timestamps.contains(&1_777_225_022));
    assert!(sync_height_timestamps.contains(&1_777_225_052));
    assert!(sync_height_timestamps.len() >= 2);

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_appends_inbound_status_samples_with_sync_samples() {
    // Arrange
    let path = temp_store_path("metrics-inbound");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let inbound = inbound_status_for_metrics();
    runtime
        .set_inbound_metric_status_provider(move || FieldAvailability::available(inbound.clone()));
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::SyncHeight && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundResourcePressureActiveCount
            && sample.value == 16.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_omits_inbound_samples_when_status_unavailable() {
    // Arrange
    let path = temp_store_path("metrics-inbound-unavailable");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    runtime.set_inbound_metric_status_provider(inbound_status_unavailable);
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );
    assert!(!metrics.samples.iter().any(|sample| matches!(
        sample.kind,
        MetricKind::InboundAdmittedPeerCount
            | MetricKind::InboundRejectedPeerCount
            | MetricKind::InboundCapRejectCount
            | MetricKind::InboundReservedSlotRejectCount
            | MetricKind::InboundDuplicateRejectCount
            | MetricKind::InboundSelfConnectionRejectCount
            | MetricKind::InboundPermissionedAdmitCount
            | MetricKind::InboundProtectedAdmitCount
            | MetricKind::InboundInactivePermissionEffectCount
            | MetricKind::InboundPermissionValidationFailureCount
            | MetricKind::InboundEvictionCandidateCount
            | MetricKind::InboundDisconnectCount
            | MetricKind::InboundActiveBanCount
            | MetricKind::InboundMisbehaviorObservationCount
            | MetricKind::InboundProtectedNoActionCount
            | MetricKind::InboundResourcePressureActiveCount
            | MetricKind::InboundReadQueuePressureCount
            | MetricKind::InboundWriteQueuePressureCount
            | MetricKind::InboundRequestCapReachedCount
            | MetricKind::InboundPayloadRejectedCount
            | MetricKind::InboundTimeoutDisconnectCount
            | MetricKind::InboundChurnRejectedCount
            | MetricKind::InboundReconnectSuppressedCount
    )));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_appends_block_relay_status_samples_with_sync_samples() {
    // Arrange
    let path = temp_store_path("metrics-block-relay");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let block_relay = block_relay_status_for_metrics();
    let snapshot = crate::network::BlockRelayRuntimeEvidenceSnapshot {
        status: block_relay,
        served_count: 9,
    };
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, Some(&snapshot), 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::SyncHeight && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::CompactAnnouncedCount
            && sample.value == 6.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::BlockServedCount
            && sample.value == 9.0
            && sample.timestamp_unix_seconds == 1_777_225_022
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn persist_metrics_omits_block_relay_samples_without_snapshot() {
    // Arrange
    let path = temp_store_path("metrics-block-relay-unavailable");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let summary = runtime.snapshot_summary();

    // Act
    runtime
        .persist_metrics(&summary, None, 1_777_225_022)
        .expect("persist metrics");

    // Assert
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );
    assert!(!metrics.samples.iter().any(|sample| matches!(
        sample.kind,
        MetricKind::BlockServedCount
            | MetricKind::BlockServingSuppressedCount
            | MetricKind::CompactAnnouncedCount
            | MetricKind::CompactReconstructedCount
            | MetricKind::CompactMissingTxRequestedCount
            | MetricKind::CompactFallbackCount
            | MetricKind::CompactMalformedCount
            | MetricKind::CompactTimeoutCount
            | MetricKind::CompactCleanupCount
    )));

    remove_dir_if_exists(&path);
}

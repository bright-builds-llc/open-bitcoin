// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use open_bitcoin_network::{
    BlockRelayActivationPolicy, BlockServingActivationConfig, CompactRelayActivationConfig,
};

use super::*;
use crate::network::PeerEmission;

const PROJECTION_TIMESTAMP: i64 = 1_777_225_405;

fn is_block_relay_metric(kind: MetricKind) -> bool {
    matches!(
        kind,
        MetricKind::BlockServedCount
            | MetricKind::BlockServingSuppressedCount
            | MetricKind::CompactAnnouncedCount
            | MetricKind::CompactReconstructedCount
            | MetricKind::CompactMissingTxRequestedCount
            | MetricKind::CompactFallbackCount
            | MetricKind::CompactMalformedCount
            | MetricKind::CompactTimeoutCount
            | MetricKind::CompactCleanupCount
    )
}

fn run_one_sync_tick(runtime: &mut DurableSyncRuntime) -> SyncRunSummary {
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    runtime
        .sync_once(&mut transport, PROJECTION_TIMESTAMP)
        .expect("sync tick")
}

fn open_authoritative_block_relay_runtime(
    store: FjallNodeStore,
    config: SyncRuntimeConfig,
) -> DurableSyncRuntime {
    DurableSyncRuntime::open_with_block_relay_activation(
        store,
        config,
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
    )
    .expect("runtime")
}

fn record_compact_activity_and_nine_block_writes(runtime: &mut DurableSyncRuntime) {
    let peer_id = 123_505;
    runtime
        .network
        .connect_outbound_peer(peer_id, PROJECTION_TIMESTAMP)
        .expect("connect evidence peer");
    for message in [
        WireNetworkMessage::Version(VersionMessage::default()),
        WireNetworkMessage::Verack,
        WireNetworkMessage::SendCompact(open_bitcoin_codec::SendCompactMessage {
            announce: true,
            version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
        }),
    ] {
        runtime
            .network
            .receive_sync_message(
                peer_id,
                message,
                PROJECTION_TIMESTAMP,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("record compact handshake");
    }

    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0);
    runtime
        .network
        .connect_local_block(&block, runtime.verify_flags, runtime.consensus_params)
        .expect("connect evidence block");
    let announcement = runtime
        .network
        .announce_block(peer_id, &block)
        .expect("announce compact block")
        .expect("compact message");
    assert!(matches!(announcement, WireNetworkMessage::CompactBlock(_)));
    let (_, _, receipt) = PeerEmission::new(peer_id, announcement, block_hash(&block.header))
        .expect("compact emission")
        .into_parts();
    runtime
        .network
        .complete_peer_emission(receipt)
        .expect("complete compact write");

    let inventory = InventoryList::new(vec![InventoryVector {
        inventory_type: InventoryType::Block,
        object_hash: block_hash(&block.header).into(),
    }]);
    for _ in 0..2 {
        let outbound = runtime
            .network
            .receive_sync_message(
                peer_id,
                WireNetworkMessage::GetData(inventory.clone()),
                PROJECTION_TIMESTAMP,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("serve evidence block")
            .outbound;
        assert!(
            outbound
                .iter()
                .any(|message| matches!(message, WireNetworkMessage::Block(_)))
        );
    }

    let block_message = WireNetworkMessage::Block(block);
    for _ in 0..9 {
        runtime
            .network
            .acknowledge_wire_message_written(&block_message)
            .expect("authoritative block-write acknowledgement");
    }
}

#[test]
fn phase123_unobserved_authoritative_network_omits_block_relay_metrics_and_log() {
    // Arrange
    let path = temp_store_path("phase123-unobserved-authoritative-projection");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        DurableSyncRuntime::open(store, sync_config_with_log_dir(&log_dir)).expect("runtime");

    // Act
    run_one_sync_tick(&mut runtime);
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    let records = load_structured_log_records(&log_dir);

    // Assert
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| sample.kind == MetricKind::SyncHeight)
    );
    assert!(
        !metrics
            .samples
            .iter()
            .any(|sample| is_block_relay_metric(sample.kind))
    );
    assert!(
        !records
            .iter()
            .any(|record| record.source == BLOCK_RELAY_LOG_SOURCE)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase123_sync_network_compact_activity_projects_same_snapshot_to_metrics_and_log() {
    // Arrange
    let path = temp_store_path("phase123-authoritative-projection");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        open_authoritative_block_relay_runtime(store, sync_config_with_log_dir(&log_dir));
    record_compact_activity_and_nine_block_writes(&mut runtime);
    let status = runtime
        .network
        .block_relay_evidence_status()
        .expect("authoritative block-relay status");
    let FieldAvailability::Available(eligibility) = status.block_serving.eligibility else {
        panic!("block-serving eligibility should be observed");
    };
    assert_eq!(eligibility.eligible_peer_count, 2);
    assert_eq!(
        runtime
            .network
            .block_served_write_count()
            .expect("authoritative block write count"),
        9
    );

    // Act
    run_one_sync_tick(&mut runtime);
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    let records = load_structured_log_records(&log_dir);
    let block_relay_record = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE)
        .expect("block relay log");

    // Assert
    assert!(
        metrics
            .samples
            .iter()
            .any(|sample| { sample.kind == MetricKind::BlockServedCount && sample.value == 9.0 })
    );
    assert!(
        metrics.samples.iter().any(|sample| {
            sample.kind == MetricKind::CompactAnnouncedCount && sample.value == 1.0
        })
    );
    assert!(block_relay_record.message.contains("block_served_count=9"));
    assert!(
        block_relay_record
            .message
            .contains("compact_announced_count=1")
    );

    remove_dir_if_exists(&path);
}

#[test]
fn authoritative_operator_snapshot_feeds_block_relay_metrics_and_log() {
    // Arrange
    let path = temp_store_path("phase127-authoritative-operator-projection");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime =
        open_authoritative_block_relay_runtime(store, sync_config_with_log_dir(&log_dir));
    record_compact_activity_and_nine_block_writes(&mut runtime);
    let snapshot = runtime
        .network
        .operator_snapshot()
        .expect("authoritative operator snapshot");

    // Act
    run_one_sync_tick(&mut runtime);
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");
    let records = load_structured_log_records(&log_dir);
    let block_relay_record = records
        .iter()
        .find(|record| record.source == BLOCK_RELAY_LOG_SOURCE)
        .expect("block relay log");

    // Assert
    assert_eq!(snapshot.block_served_count(), 9);
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::BlockServedCount
            && sample.value == snapshot.block_served_count() as f64
    }));
    assert!(block_relay_record.message.contains(&format!(
        "block_served_count={}",
        snapshot.block_served_count()
    )));

    remove_dir_if_exists(&path);
}

#[test]
fn phase123_inbound_metric_provider_remains_unchanged() {
    // Arrange
    let path = temp_store_path("phase123-inbound-provider-unchanged");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
    let inbound = inbound_status_for_metrics();
    runtime
        .set_inbound_metric_status_provider(move || FieldAvailability::available(inbound.clone()));

    // Act
    run_one_sync_tick(&mut runtime);
    let metrics = runtime
        .store()
        .load_metrics_snapshot()
        .expect("load metrics")
        .expect("metrics snapshot");

    // Assert
    assert!(metrics.samples.iter().any(|sample| {
        sample.kind == MetricKind::InboundResourcePressureActiveCount && sample.value == 16.0
    }));
    assert!(
        !metrics
            .samples
            .iter()
            .any(|sample| is_block_relay_metric(sample.kind))
    );

    remove_dir_if_exists(&path);
}

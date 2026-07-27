// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase71_synthetic_long_chain_exercises_resource_bounds_without_public_network() {
    // Arrange
    const SYNTHETIC_BLOCKS: usize = 48;

    let path = temp_store_path("phase71-synthetic-long-chain");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let blocks = {
        let mut blocks = Vec::with_capacity(SYNTHETIC_BLOCKS);
        let mut previous_hash = BlockHash::from_byte_array([0_u8; 32]);
        for height in 0..SYNTHETIC_BLOCKS {
            let block = build_block(previous_hash, height as u32);
            previous_hash = block_hash(&block.header);
            blocks.push(block);
        }
        blocks
    };
    let all_headers = blocks
        .iter()
        .map(|block| block.header.clone())
        .collect::<Vec<_>>();
    let first_peer_blocks = blocks
        .iter()
        .take(9)
        .cloned()
        .map(WireNetworkMessage::Block);
    let second_peer_blocks = blocks
        .iter()
        .skip(9)
        .take(10)
        .cloned()
        .map(WireNetworkMessage::Block);
    let mut first_peer_script = headers_script(47, all_headers);
    first_peer_script.extend(first_peer_blocks);
    let mut second_peer_script = version_verack_script(47);
    second_peer_script.extend(second_peer_blocks);
    let config = SyncRuntimeConfig {
        manual_peers: vec![
            SyncPeerAddress::manual("127.0.0.1", 18_444),
            SyncPeerAddress::manual("127.0.0.1", 18_445),
        ],
        dns_seeds: Vec::new(),
        target_outbound_peers: 2,
        max_blocks_in_flight_per_peer: 2,
        max_blocks_in_flight_total: 4,
        max_messages_per_peer: 12,
        max_rounds: 32,
        max_peer_retries: 0,
        maybe_log_dir: Some(log_dir.clone()),
        ..sync_config()
    };
    assert!(config.dns_seeds.is_empty());
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, config.clone()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![first_peer_script, second_peer_script]);
    let load_pressure = |runtime: &DurableSyncRuntime| {
        let metadata = runtime
            .store()
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata");
        let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");
        match durable_sync_state.sync.resource_pressure {
            FieldAvailability::Available(pressure) => pressure,
            FieldAvailability::Unavailable { reason } => {
                panic!("missing synthetic long-chain resource pressure: {reason}")
            }
        }
    };

    // Act
    let partial_summary = runtime
        .sync_once(&mut transport, i64::from(blocks[18].header.time))
        .expect("partial synthetic sync");
    let partial_pressure = load_pressure(&runtime);
    let metrics_status = runtime
        .store()
        .load_metrics_status(MetricRetentionPolicy::default())
        .expect("metrics status");
    let metrics_retention = MetricRetentionPolicy::default();
    let log_retention = LogRetentionPolicy::default();
    let log_status = load_log_status(&log_dir, LogRetentionPolicy::default(), 10);
    drop(runtime);

    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let mut reopened_runtime =
        DurableSyncRuntime::open(reopened_store, config).expect("reopened runtime");
    let reopened_summary = reopened_runtime.snapshot_summary();
    let connected_index =
        usize::try_from(reopened_summary.best_block_height).expect("connected height fits usize");
    let connected_hash = block_hash(&blocks[connected_index].header);
    let next_missing_hash = block_hash(&blocks[connected_index + 1].header);
    let mut resume_transport =
        ScriptedTransport::new(vec![version_verack_script(47), version_verack_script(47)]);
    let resume_summary = reopened_runtime
        .sync_once(
            &mut resume_transport,
            i64::from(blocks[connected_index + 1].header.time),
        )
        .expect("resume sync");
    let resume_pressure = load_pressure(&reopened_runtime);
    let resume_requested_hashes = getdata_block_hashes(&resume_transport.sent_messages());

    // Assert
    assert_eq!(blocks.len(), 48);
    assert_eq!(partial_summary.best_header_height, 47);
    assert!(partial_summary.blocks_received > 0);
    assert!(partial_summary.best_block_height < partial_summary.best_header_height);
    assert!(partial_pressure.blocks_in_flight <= 4);
    assert!(partial_pressure.outbound_peers <= 2);
    assert_eq!(partial_pressure.target_outbound_peers, 2);
    assert_eq!(partial_pressure.max_blocks_in_flight_per_peer, 2);
    assert_eq!(partial_pressure.max_blocks_in_flight_total, 4);
    assert_eq!(partial_pressure.max_messages_per_peer, 12);
    assert_eq!(partial_pressure.max_sync_rounds, 32);

    assert_eq!(metrics_status.retention, MetricRetentionPolicy::default());
    assert_eq!(metrics_retention.sample_interval_seconds, 30);
    assert_eq!(metrics_retention.max_samples_per_series, 2_880);
    assert_eq!(metrics_retention.max_age_seconds, 86_400);
    assert_eq!(log_status.retention, LogRetentionPolicy::default());
    assert_eq!(log_retention.max_files, 14);
    assert_eq!(log_retention.max_age_days, 14);
    assert_eq!(log_retention.max_total_bytes, 268_435_456);

    assert!(reopened_runtime.inflight_blocks.is_empty());
    assert_eq!(reopened_summary.best_header_height, 47);
    assert_eq!(
        reopened_summary.downloaded_block_height,
        partial_summary.downloaded_block_height
    );
    assert_eq!(
        reopened_summary.best_block_height,
        partial_summary.best_block_height
    );
    assert_eq!(
        reopened_summary.maybe_connected_block_hash,
        partial_summary.maybe_connected_block_hash
    );
    assert!(!resume_requested_hashes.contains(&connected_hash));
    assert!(resume_requested_hashes.contains(&next_missing_hash));
    assert_eq!(resume_summary.best_header_height, 47);
    assert!(resume_pressure.blocks_in_flight <= 4);
    assert!(resume_pressure.outbound_peers <= 2);
    assert_eq!(resume_pressure.max_messages_per_peer, 12);
    assert_eq!(resume_pressure.max_sync_rounds, 32);

    drop(reopened_runtime);
    remove_dir_if_exists(&path);
}

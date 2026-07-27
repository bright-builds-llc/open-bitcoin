// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn bounded_unattended_cycles_preserve_resource_pressure_and_retention() {
    // Arrange
    let path = temp_store_path("bounded-unattended-cycles");
    let log_dir = path.join("logs");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.60", 18_444),
                SyncPeerAddress::manual("198.51.100.61", 18_445),
                SyncPeerAddress::manual("198.51.100.62", 18_446),
                SyncPeerAddress::manual("198.51.100.63", 18_447),
                SyncPeerAddress::manual("198.51.100.64", 18_448),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 2,
            max_messages_per_peer: 3,
            max_rounds: 5,
            max_peer_retries: 0,
            retry_backoff_ms: 10_000,
            max_blocks_in_flight_per_peer: 2,
            max_blocks_in_flight_total: 4,
            maybe_log_dir: Some(log_dir.clone()),
            ..sync_config()
        },
    )
    .expect("runtime");
    let invalid_headers_script = |time: u32| {
        let genesis = header(BlockHash::from_byte_array([0_u8; 32]), time);
        let mut stale_child = header(block_hash(&genesis), time.saturating_add(1));
        stale_child.time = genesis.time;
        vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 1,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::Headers(HeadersMessage {
                headers: vec![genesis, stale_child],
            }),
        ]
    };
    let mut transport = ScriptedTransport::with_connect_results(vec![
        Ok(Vec::new()),
        Ok(invalid_headers_script(100)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
        Ok(invalid_headers_script(120)),
        Ok(version_verack_script(0)),
    ]);
    let mut resolver = ScriptedResolver::new(Vec::new());
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
                panic!("missing sync resource pressure: {reason}")
            }
        }
    };
    let assert_bounded_pressure = |pressure: &SyncResourcePressure| {
        assert!(pressure.blocks_in_flight <= 4);
        assert_eq!(pressure.max_header_requests_in_flight_per_peer, 1);
        assert_eq!(pressure.max_headers_per_message, 2_000);
        assert_eq!(pressure.max_blocks_in_flight_per_peer, 2);
        assert_eq!(pressure.max_blocks_in_flight_total, 4);
        assert_eq!(pressure.max_messages_per_peer, 3);
        assert_eq!(pressure.max_sync_rounds, 5);
        assert!(pressure.outbound_peers <= 2);
        assert_eq!(pressure.target_outbound_peers, 2);
    };

    // Act
    let first_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_200)
        .expect("first sync");
    // durable storage writes are synchronous adapter calls with no queued write backlog.
    let first_pressure = load_pressure(&runtime);
    let first_backoff_keys = runtime.peer_backoff.keys().cloned().collect::<Vec<_>>();
    let second_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_201)
        .expect("second sync");
    let second_pressure = load_pressure(&runtime);
    let third_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_211)
        .expect("third sync");
    let third_pressure = load_pressure(&runtime);
    let metrics_retention = MetricRetentionPolicy::default();
    let log_retention = LogRetentionPolicy::default();
    let records = load_structured_log_records(&log_dir);

    // Assert
    assert_eq!(first_summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert_eq!(first_summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        first_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(first_summary.connected_peers, 2);
    assert_eq!(first_summary.failed_peers, 1);
    assert_bounded_pressure(&first_pressure);
    assert_eq!(
        first_backoff_keys,
        vec!["127.0.0.1:18444".to_string(), "127.0.0.1:18445".to_string()]
    );
    // peer retry state is keyed by resolved endpoint.
    assert!(
        first_backoff_keys
            .iter()
            .all(|key| key.starts_with("127.0.0.1:"))
    );
    assert!(first_backoff_keys.len() <= runtime.config.target_outbound_peers);
    assert!(first_backoff_keys.len() <= runtime.config.candidate_peers().len());

    assert_eq!(
        second_summary.peer_outcomes[0].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        second_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert_eq!(
        second_summary.peer_outcomes[1].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        second_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert_eq!(second_summary.connected_peers, 2);
    assert_bounded_pressure(&second_pressure);

    assert_eq!(
        third_summary.peer_outcomes[0].state,
        PeerSyncState::Connected
    );
    assert_eq!(third_summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        third_summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(
        third_summary.peer_outcomes[2].state,
        PeerSyncState::Connected
    );
    assert_eq!(third_summary.connected_peers, 2);
    assert_eq!(runtime.peer_backoff.len(), 1);
    assert!(runtime.peer_backoff.contains_key("127.0.0.1:18445"));
    assert!(runtime.peer_backoff.len() <= runtime.config.target_outbound_peers);
    assert_bounded_pressure(&third_pressure);

    assert_eq!(metrics_retention.sample_interval_seconds, 30);
    assert_eq!(metrics_retention.max_samples_per_series, 2_880);
    assert_eq!(metrics_retention.max_age_seconds, 86_400);
    assert_eq!(log_retention.max_files, 14);
    assert_eq!(log_retention.max_age_days, 14);
    assert_eq!(log_retention.max_total_bytes, 268_435_456);
    assert!(!records.is_empty());
    assert!(records.len() <= 32);

    remove_dir_if_exists(&path);
}

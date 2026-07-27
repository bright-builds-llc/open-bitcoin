// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn contextual_invalid_headers_fail_with_typed_invalid_data() {
    // Arrange
    let path = temp_store_path("invalid-header");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_177)
        .expect("sync summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert!(matches!(
        summary.peer_outcomes.as_slice(),
        [PeerSyncOutcome {
            maybe_failure_reason: Some(PeerFailureReason::InvalidData),
            ..
        }]
    ));
    assert!(summary.health_signals.iter().any(|signal| {
        signal.message == "sync peer sent invalid data: inspect peer compatibility"
    }));

    remove_dir_if_exists(&path);
}

#[test]
fn peer_contribution_rejects_invalid_headers_without_credit() {
    // Arrange
    let path = temp_store_path("peer-contribution-invalid-headers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut transport = ScriptedTransport::new(vec![script]);
    let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_178)
        .expect("sync summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    assert!(matches!(
        summary.peer_outcomes.as_slice(),
        [PeerSyncOutcome {
            maybe_failure_reason: Some(PeerFailureReason::InvalidData),
            contribution: PeerContribution {
                messages_processed: 3,
                headers_received: 0,
                blocks_received: 0,
            },
            maybe_last_activity_unix_seconds: Some(1_777_225_178),
            ..
        }]
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn retry_backoff_waiting_and_stalled_peers_remain_uncredited() {
    // Arrange
    let path = temp_store_path("peer-contribution-waiting-stalled");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.45", 18_444)],
            dns_seeds: Vec::new(),
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.45", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.45", 18_444)]),
    ]);

    // Act
    let stalled_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_180)
        .expect("first sync");
    let waiting_summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_181)
        .expect("second sync");

    // Assert
    assert_eq!(
        stalled_summary.peer_outcomes[0].state,
        PeerSyncState::Stalled
    );
    assert_eq!(stalled_summary.connected_peers, 0);
    assert_eq!(
        stalled_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Stall)
    );
    assert_eq!(
        stalled_summary.peer_outcomes[0].contribution,
        PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert!(
        waiting_summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("retry backoff wait_seconds=9"))
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].contribution,
        PeerContribution {
            messages_processed: 0,
            headers_received: 0,
            blocks_received: 0,
        }
    );

    remove_dir_if_exists(&path);
}

#[test]
fn mixed_peer_failures_rotate_to_replacement_without_corrupting_state() {
    // Arrange
    let path = temp_store_path("mixed-peer-failures");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let genesis = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let mut stale_child = header(block_hash(&genesis), 2);
    stale_child.time = genesis.time;
    let invalid_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 1,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Verack,
        WireNetworkMessage::Headers(HeadersMessage {
            headers: vec![genesis, stale_child],
        }),
    ];
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.40", 18_444),
                SyncPeerAddress::manual("198.51.100.41", 18_445),
                SyncPeerAddress::manual("198.51.100.42", 18_446),
            ],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            max_messages_per_peer: 3,
            target_outbound_peers: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::with_connect_results(vec![
        Err(SyncRuntimeError::Io {
            peer: "198.51.100.40:18444".to_string(),
            message: "scripted disconnect".to_string(),
        }),
        Ok(invalid_script),
        Ok(vec![
            WireNetworkMessage::Version(VersionMessage {
                start_height: 0,
                ..VersionMessage::default()
            }),
            WireNetworkMessage::Verack,
            WireNetworkMessage::SendHeaders,
        ]),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.40", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.41", 18_445)]),
        Ok(vec![resolved_manual_peer("198.51.100.42", 18_446)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_183)
        .expect("sync");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");

    // Assert
    assert_eq!(summary.attempted_peers, 3);
    assert_eq!(summary.failed_peers, 2);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Connect)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Failed);
    assert_eq!(
        summary.peer_outcomes[1].maybe_failure_reason,
        Some(PeerFailureReason::InvalidData)
    );
    assert_eq!(summary.peer_outcomes[2].state, PeerSyncState::Connected);
    assert_eq!(runtime.snapshot_summary().best_block_height, 0);
    assert_eq!(
        durable_sync_state.sync.lifecycle,
        FieldAvailability::available(SyncLifecycleState::Active)
    );
    assert_eq!(
        durable_sync_state.sync.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 16,
            max_blocks_in_flight_total: 64,
            max_messages_per_peer: 3,
            max_sync_rounds: 8,
            outbound_peers: 1,
            target_outbound_peers: 1,
        })
    );
}

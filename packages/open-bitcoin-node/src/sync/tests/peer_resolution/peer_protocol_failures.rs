// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn duplicate_version_peer_is_failed_and_replaced_without_progress_credit() {
    // Arrange
    let path = temp_store_path("duplicate-version-replacement");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let duplicate_version_script = vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::Version(VersionMessage {
            start_height: 0,
            ..VersionMessage::default()
        }),
    ];
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.23", 18_444),
                SyncPeerAddress::manual("198.51.100.24", 18_445),
            ],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![duplicate_version_script, version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.23", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.24", 18_445)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_191)
        .expect("summary");
    let metadata = runtime
        .store()
        .load_runtime_metadata()
        .expect("load runtime metadata")
        .expect("runtime metadata");
    let durable_sync_state = metadata.maybe_sync_state.expect("durable sync state");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let rejected = &summary.peer_outcomes[0];
    assert_eq!(rejected.state, PeerSyncState::Failed);
    assert_eq!(
        rejected.maybe_failure_reason,
        Some(PeerFailureReason::Compatibility)
    );
    assert_eq!(rejected.contribution.headers_received, 0);
    assert_eq!(rejected.contribution.blocks_received, 0);
    assert!(
        rejected
            .maybe_error
            .as_ref()
            .is_some_and(|message| { message.contains("duplicate version") })
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
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
            max_messages_per_peer: 8,
            max_sync_rounds: 8,
            outbound_peers: 1,
            target_outbound_peers: 1,
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn wrong_network_peer_is_failed_without_progress_credit() {
    // Arrange
    let path = temp_store_path("wrong-network-peer");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.25", 18_444)],
            dns_seeds: Vec::new(),
            max_peer_retries: 0,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::with_connect_results(vec![Err(SyncRuntimeError::InvalidMagic {
            expected: SyncNetwork::Regtest.magic().to_bytes(),
            actual: SyncNetwork::Mainnet.magic().to_bytes(),
        })]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.25",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_192)
        .expect("summary");

    // Assert
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.headers_received, 0);
    assert_eq!(summary.blocks_received, 0);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Failed);
    assert_eq!(
        outcome.maybe_failure_reason,
        Some(PeerFailureReason::InvalidMagic)
    );
    assert_eq!(outcome.contribution.messages_processed, 0);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_outcome_captures_peer_capabilities_and_endpoint() {
    // Arrange
    let path = temp_store_path("peer-capabilities");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.30", 18_444)],
            dns_seeds: Vec::new(),
            max_messages_per_peer: 4,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![vec![
        WireNetworkMessage::Version(VersionMessage {
            start_height: 3,
            ..VersionMessage::default()
        }),
        WireNetworkMessage::WtxidRelay,
        WireNetworkMessage::SendHeaders,
        WireNetworkMessage::Verack,
    ]]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.30",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_199)
        .expect("summary");

    // Assert
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(
        outcome.maybe_resolved_endpoint.as_deref(),
        Some("127.0.0.1:18444")
    );
    let capabilities = outcome
        .maybe_capabilities
        .as_ref()
        .expect("peer capabilities");
    assert!(capabilities.services_bits > 0);
    assert_eq!(capabilities.start_height, 3);
    assert!(capabilities.wtxidrelay);
    assert!(capabilities.prefers_headers);
}

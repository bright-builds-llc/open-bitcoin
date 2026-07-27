// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_once_with_resolver_records_resolution_failures() {
    // Arrange
    let path = temp_store_path("resolver-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("seed.invalid", 18_444)],
            dns_seeds: Vec::new(),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(Vec::new());
    let mut resolver = ScriptedResolver::new(vec![Err(SyncRuntimeError::AddressResolution {
        peer: "seed.invalid:18444".to_string(),
        message: "scripted lookup failure".to_string(),
    })]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_166)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::AddressResolution)
    );
    assert!(
        summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("address resolution failed"))
    );
}

#[test]
fn sync_once_rotates_to_alternative_peer_after_stall() {
    // Arrange
    let path = temp_store_path("peer-rotation");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.10", 18_444),
                SyncPeerAddress::manual("198.51.100.11", 18_444),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 3,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        Vec::new(),
        headers_script(1, vec![header(BlockHash::from_byte_array([0_u8; 32]), 2)]),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![ResolvedSyncPeerAddress::new(
            SyncPeerAddress::manual("198.51.100.10", 18_444),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18_444),
        )]),
        Ok(vec![ResolvedSyncPeerAddress::new(
            SyncPeerAddress::manual("198.51.100.11", 18_444),
            SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 2)), 18_444),
        )]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_177)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.peer_outcomes.len(), 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Stall)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_eq!(summary.peer_outcomes[1].contribution.headers_received, 1);
}

#[test]
fn sync_once_retry_backoff_wait_replaces_peer() {
    // Arrange
    let path = temp_store_path("backoff-replacement");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.12", 18_444),
                SyncPeerAddress::manual("198.51.100.13", 18_445),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 2,
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        Vec::new(),
        version_verack_script(0),
        version_verack_script(0),
    ]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.12", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.13", 18_445)]),
        Ok(vec![resolved_manual_peer("198.51.100.12", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.13", 18_445)]),
    ]);
    runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_177)
        .expect("first sync");

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_178)
        .expect("second sync");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.failed_peers, 0);
    assert_eq!(summary.peer_outcomes.len(), 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Waiting);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert!(
        summary.peer_outcomes[0]
            .maybe_error
            .as_ref()
            .is_some_and(|message| message.contains("consecutive_failures=1"))
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
}

#[test]
fn sync_once_waiting_backoff_projects_waiting_for_peers_phase() {
    // Arrange
    let path = temp_store_path("backoff-waiting-phase");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.14", 18_444)],
            dns_seeds: Vec::new(),
            retry_backoff_ms: 10_000,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![Vec::new()]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.14", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.14", 18_444)]),
    ]);
    runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_180)
        .expect("first sync");

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_181)
        .expect("second sync");
    let sync_status = summary.sync_status(SyncNetwork::Regtest);
    let peer_status = summary.peer_status();
    let log_records = summary.structured_log_records(1_777_225_181);

    // Assert
    assert_eq!(summary.attempted_peers, 0);
    assert_eq!(summary.connected_peers, 0);
    assert_eq!(summary.failed_peers, 0);
    assert_eq!(
        sync_status.phase,
        FieldAvailability::available("waiting_for_peers".to_string())
    );
    assert!(matches!(
        peer_status.recent_peers,
        FieldAvailability::Available(ref peers)
            if peers.first().is_some_and(|peer| peer.state == "waiting")
    ));
    assert!(log_records.iter().any(|record| {
        record.level == StructuredLogLevel::Warn
            && record.source == "sync"
            && record.message.contains("retry backoff")
    }));
}

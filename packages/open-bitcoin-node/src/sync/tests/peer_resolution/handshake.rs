// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn manual_peer_completes_handshake_before_idle() {
    // Arrange
    let path = temp_store_path("manual-handshake-idle");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.22", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.22",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_189)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(summary.failed_peers, 0);
    assert!(summary.health_signals.is_empty());
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(outcome.contribution.messages_processed, 2);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert_eq!(outcome.maybe_failure_reason, None);
    assert!(outcome.maybe_capabilities.is_some());
    assert_eq!(summary.best_header_height, 0);
    assert_eq!(summary.best_block_height, 0);

    remove_dir_if_exists(&path);
}

#[test]
fn dns_seed_peer_completes_handshake_before_idle() {
    // Arrange
    let path = temp_store_path("dns-handshake-idle");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: Vec::new(),
            dns_seeds: vec!["seed.example.invalid".to_string()],
            target_outbound_peers: 1,
            max_messages_per_peer: 8,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![ResolvedSyncPeerAddress::new(
        SyncPeerAddress::dns_seed("seed.example.invalid", 18_444),
        SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 18_444),
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_190)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    let outcome = &summary.peer_outcomes[0];
    assert_eq!(outcome.state, PeerSyncState::Connected);
    assert_eq!(outcome.peer.source, SyncPeerSource::DnsSeed);
    assert_eq!(outcome.contribution.headers_received, 0);
    assert_eq!(outcome.contribution.blocks_received, 0);
    assert_eq!(outcome.maybe_failure_reason, None);

    remove_dir_if_exists(&path);
}

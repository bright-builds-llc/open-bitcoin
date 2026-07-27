// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn no_configured_peers_is_a_typed_error() {
    // Arrange
    let path = temp_store_path("no-peers");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: Vec::new(),
            dns_seeds: Vec::new(),
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![]);

    // Act
    let error = runtime
        .sync_once(&mut transport, 1)
        .expect_err("no peers configured");

    // Assert
    assert_eq!(error, SyncRuntimeError::NoPeersConfigured);

    remove_dir_if_exists(&path);
}

#[test]
fn connect_failures_are_reported_as_peer_outcomes() {
    // Arrange
    let path = temp_store_path("connect-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            max_peer_retries: 0,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::failing();

    // Act
    let summary = runtime.sync_once(&mut transport, 1).expect("summary");

    // Assert
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Failed);
    assert!(summary.peer_outcomes[0].maybe_error.is_some());
    assert_eq!(summary.health_signals.len(), 1);

    remove_dir_if_exists(&path);
}

#[test]
fn sync_networks_select_matching_consensus_pow_rules() {
    // Arrange
    let mainnet = SyncNetwork::Mainnet.consensus_params();
    let testnet = SyncNetwork::Testnet.consensus_params();
    let signet = SyncNetwork::Signet.consensus_params();
    let regtest = SyncNetwork::Regtest.consensus_params();

    // Act / Assert
    assert_eq!(mainnet.pow_limit_bits, 0x1d00_ffff);
    assert!(!mainnet.allow_min_difficulty_blocks);
    assert!(!mainnet.no_pow_retargeting);
    assert_eq!(testnet.pow_limit_bits, 0x1d00_ffff);
    assert!(testnet.allow_min_difficulty_blocks);
    assert!(!testnet.no_pow_retargeting);
    assert_eq!(signet.pow_limit_bits, 0x1e03_77ae);
    assert!(!signet.allow_min_difficulty_blocks);
    assert_eq!(regtest.pow_limit_bits, EASY_BITS);
    assert!(regtest.allow_min_difficulty_blocks);
    assert!(regtest.no_pow_retargeting);
}

#[test]
#[ignore = "requires public Bitcoin network; set OPEN_BITCOIN_LIVE_SYNC_SMOKE=1 to run"]
fn live_network_smoke_is_explicitly_opt_in() {
    if std::env::var("OPEN_BITCOIN_LIVE_SYNC_SMOKE")
        .ok()
        .as_deref()
        != Some("1")
    {
        return;
    }

    let path = temp_store_path("live");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::dns_seed("seed.bitcoin.sipa.be", 8333)],
            dns_seeds: Vec::new(),
            max_messages_per_peer: 2,
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("runtime");
    let mut transport = TcpPeerTransport;

    let _summary = runtime
        .sync_once(&mut transport, 1_777_225_022)
        .expect("live sync smoke");

    remove_dir_if_exists(&path);
}

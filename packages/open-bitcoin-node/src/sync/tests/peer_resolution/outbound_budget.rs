// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn sync_status_preserves_configured_target_outbound_peer_count() {
    // Arrange
    let path = temp_store_path("configured-target-outbound");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![SyncPeerAddress::manual("198.51.100.15", 18_444)],
            dns_seeds: Vec::new(),
            target_outbound_peers: 3,
            max_messages_per_peer: 2,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport = ScriptedTransport::new(vec![version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![Ok(vec![resolved_manual_peer(
        "198.51.100.15",
        18_444,
    )])]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_182)
        .expect("sync");
    let sync_status = summary.sync_status(SyncNetwork::Regtest);

    // Assert
    assert_eq!(summary.target_outbound_peers, 3);
    assert_eq!(
        sync_status.resource_pressure,
        FieldAvailability::available(SyncResourcePressure {
            blocks_in_flight: 0,
            max_header_requests_in_flight_per_peer: 1,
            max_headers_per_message: 2_000,
            max_blocks_in_flight_per_peer: 0,
            max_blocks_in_flight_total: 0,
            max_messages_per_peer: 0,
            max_sync_rounds: 0,
            outbound_peers: 1,
            target_outbound_peers: 3,
        })
    );
}

#[test]
fn sync_once_stops_after_target_outbound_peer_budget_is_met() {
    // Arrange
    let path = temp_store_path("target-outbound-budget");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(
        store,
        SyncRuntimeConfig {
            manual_peers: vec![
                SyncPeerAddress::manual("198.51.100.20", 18_444),
                SyncPeerAddress::manual("198.51.100.21", 18_444),
            ],
            dns_seeds: Vec::new(),
            target_outbound_peers: 1,
            ..sync_config()
        },
    )
    .expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![version_verack_script(0), version_verack_script(0)]);
    let mut resolver = ScriptedResolver::new(vec![
        Ok(vec![resolved_manual_peer("198.51.100.20", 18_444)]),
        Ok(vec![resolved_manual_peer("198.51.100.21", 18_444)]),
    ]);

    // Act
    let summary = runtime
        .sync_once_with_resolver(&mut transport, &mut resolver, 1_777_225_188)
        .expect("summary");

    // Assert
    assert_eq!(summary.attempted_peers, 1);
    assert_eq!(summary.peer_outcomes.len(), 1);
}

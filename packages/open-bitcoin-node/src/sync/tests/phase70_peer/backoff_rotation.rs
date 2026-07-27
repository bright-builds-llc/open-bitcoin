// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase70_stalled_peer_backoff_does_not_consume_rotation_slot() {
    // Arrange
    let path = temp_store_path("phase70-peer-stall-rotation");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ScriptedTransport::new(vec![
        Vec::new(),
        version_verack_script(0),
        version_verack_script(0),
    ]);

    // Act
    let stalled_summary = runtime
        .sync_once(&mut transport, 1_777_225_300)
        .expect("first sync");
    let waiting_summary = runtime
        .sync_once(&mut transport, 1_777_225_301)
        .expect("second sync");

    // Assert
    assert_eq!(stalled_summary.attempted_peers, 2);
    assert_eq!(stalled_summary.connected_peers, 1);
    assert_eq!(
        stalled_summary.peer_outcomes[0].state,
        PeerSyncState::Stalled
    );
    assert_eq!(
        stalled_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Stall)
    );
    assert_eq!(
        stalled_summary.peer_outcomes[1].state,
        PeerSyncState::Connected
    );
    assert!(
        stalled_summary
            .health_signals
            .iter()
            .any(|signal| { signal.message == "peer stalled before sending more sync messages" })
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
        waiting_summary.health_signals.iter().any(|signal| {
            signal.message == "peer waiting for retry backoff before next attempt"
        })
    );

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_incompatible_peer_rotates_with_typed_backoff() {
    // Arrange
    let path = temp_store_path("phase70-peer-incompatible-rotation");
    remove_dir_if_exists(&path);
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
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport =
        ScriptedTransport::new(vec![duplicate_version_script, version_verack_script(0)]);

    // Act
    let summary = runtime
        .sync_once(&mut transport, 1_777_225_302)
        .expect("sync");

    // Assert
    assert_eq!(summary.attempted_peers, 2);
    assert_eq!(summary.failed_peers, 1);
    assert_eq!(summary.connected_peers, 1);
    assert_eq!(
        summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Compatibility)
    );
    assert_eq!(summary.peer_outcomes[1].state, PeerSyncState::Connected);
    assert_first_peer_backoff(&runtime);

    remove_dir_if_exists(&path);
}

#[test]
fn phase70_disconnect_backoff_reports_waiting_and_tries_other_peer() {
    // Arrange
    let path = temp_store_path("phase70-peer-disconnect-backoff");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("store");
    let mut runtime = DurableSyncRuntime::open(store, rotation_config()).expect("runtime");
    let mut transport = ScriptedTransport::with_connect_results(vec![
        Err(SyncRuntimeError::Network {
            message: "scripted disconnect".to_string(),
        }),
        Ok(version_verack_script(0)),
        Ok(version_verack_script(0)),
    ]);

    // Act
    let failed_summary = runtime
        .sync_once(&mut transport, 1_777_225_303)
        .expect("first sync");
    let waiting_summary = runtime
        .sync_once(&mut transport, 1_777_225_304)
        .expect("second sync");

    // Assert
    assert_eq!(failed_summary.attempted_peers, 2);
    assert_eq!(failed_summary.failed_peers, 1);
    assert_eq!(
        failed_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::Network)
    );
    assert_eq!(
        failed_summary.peer_outcomes[1].state,
        PeerSyncState::Connected
    );
    assert_eq!(waiting_summary.attempted_peers, 1);
    assert_eq!(
        waiting_summary.peer_outcomes[0].state,
        PeerSyncState::Waiting
    );
    assert_eq!(
        waiting_summary.peer_outcomes[0].maybe_failure_reason,
        Some(PeerFailureReason::RetryBackoff)
    );
    assert_eq!(
        waiting_summary.peer_outcomes[1].state,
        PeerSyncState::Connected
    );

    remove_dir_if_exists(&path);
}

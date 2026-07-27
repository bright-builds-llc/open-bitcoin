// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase123_closed_receive_ends_session() {
    // Arrange
    let path = temp_store_path("phase123-closed-session");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let mut transport = TimingTransport::new(vec![SyncPeerReceiveOutcome::Closed]);
    let mut resolver = timing_resolver();
    let mut clock = || 4_000;

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 4_000, &mut clock)
        .expect("closed summary");

    // Assert
    assert_eq!(summary.messages_processed, 0);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Stalled);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_target_mismatch_is_not_written_to_current_session() {
    // Arrange
    let path = temp_store_path("phase123-target-mismatch");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    start_other_peer_compact_download(&mut runtime, 99, &compact_block, 5_000);
    let sent = Rc::new(RefCell::new(Vec::new()));
    let session = TimingSession {
        outcomes: vec![SyncPeerReceiveOutcome::Idle].into(),
        sent: Rc::clone(&sent),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 5_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1;

    // Act
    let result = runtime.sync_connected_peer(session, &peer, 1, 1, 5_000, &mut clock);

    // Assert
    let failure = result.expect_err("target mismatch must fail");
    assert!(matches!(
        failure.error,
        SyncRuntimeError::Network { ref message }
            if message == "compact timeout action target does not match connected session"
    ));
    assert!(
        !sent
            .borrow()
            .iter()
            .any(|message| { is_full_block_getdata_for_hash(message, expected_hash) })
    );
    remove_dir_if_exists(&path);
}

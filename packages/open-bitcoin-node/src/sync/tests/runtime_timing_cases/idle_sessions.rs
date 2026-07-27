// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase123_idle_before_timeout_retains_session_without_fallback_or_progress() {
    // Arrange
    let path = temp_store_path("phase123-idle-before-timeout");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]);
    let mut resolver = timing_resolver();
    let clock_calls = Rc::new(RefCell::new(0_usize));
    let clock_call_count = Rc::clone(&clock_calls);
    let mut clock = move || {
        *clock_call_count.borrow_mut() += 1;
        1_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS - 1
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 1_000, &mut clock)
        .expect("idle sync summary");

    // Assert
    assert_eq!(*clock_calls.borrow(), 3);
    assert_eq!(summary.messages_processed, 2);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    assert!(!transport.sent_messages().iter().any(is_full_block_getdata));
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_idle_after_fake_clock_emits_same_peer_full_block_fallback() {
    // Arrange
    let path = temp_store_path("phase123-idle-after-timeout");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let mut transport = TimingTransport::new(compact_download_script(&compact_block));
    let mut resolver = timing_resolver();
    let mut clock_values = [
        2_000,
        2_000,
        2_000,
        2_000,
        2_000 + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
    ]
    .into_iter();
    let mut clock = || clock_values.next().expect("scripted clock value");

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 2_000, &mut clock)
        .expect("timed-out compact sync summary");

    // Assert
    assert_eq!(summary.messages_processed, 4);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| { is_full_block_getdata_for_hash(message, expected_hash) })
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_message_after_idle_uses_session_clock_for_compact_timeout() {
    // Arrange
    let path = temp_store_path("phase123-message-after-idle-clock");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping { nonce: 123 }),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]);
    let sent = Rc::clone(&transport.sent);
    let mut resolver = timing_resolver();
    let compact_received_at = 7_000;
    let mut clock_values = [
        6_000,
        6_000,
        6_000,
        compact_received_at,
        compact_received_at,
        compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS - 1,
        compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS - 1,
        compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
    ]
    .into_iter();
    let mut clock = || {
        let now = clock_values.next().expect("scripted clock value");
        if now > compact_received_at + COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS {
            assert!(!sent.borrow().iter().any(is_full_block_getdata));
        }
        now
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 6_000, &mut clock)
        .expect("late compact sync summary");

    // Assert
    assert_eq!(summary.messages_processed, 5);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| is_full_block_getdata_for_hash(message, expected_hash))
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_idle_wake_does_not_consume_message_budget() {
    // Arrange
    let path = temp_store_path("phase123-idle-message-budget");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 5);
    let compact_block = compact_block_fixture(&mut runtime);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping { nonce: 123 }),
    ]);
    let mut resolver = timing_resolver();
    let mut clock = || 3_000;

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 3_000, &mut clock)
        .expect("idle budget summary");

    // Assert
    assert_eq!(summary.messages_processed, 5);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_idle_session_without_compact_work_yields_after_first_wake() {
    // Arrange
    let path = temp_store_path("phase123-bounded-idle-session");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let receive_calls = Rc::new(RefCell::new(0_usize));
    let session = PerpetualIdleSession {
        receive_calls: Rc::clone(&receive_calls),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut clock = || 3_500;

    // Act
    let progress = runtime
        .sync_connected_peer(session, &peer, 1, 1, 3_500, &mut clock)
        .expect("bounded idle session");

    // Assert
    assert_eq!(*receive_calls.borrow(), 3);
    assert_eq!(progress.state, PeerSyncState::Connected);
    assert_eq!(progress.messages_processed, 2);
    remove_dir_if_exists(&path);
}

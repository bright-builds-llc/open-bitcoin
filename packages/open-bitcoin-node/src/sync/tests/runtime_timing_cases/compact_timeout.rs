// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase123_compact_download_survives_five_second_idle_cadence_until_timeout() {
    // Arrange
    let path = temp_store_path("phase123-compact-default-idle-cadence");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let started_at = 8_000;
    let mut outcomes = vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
    ];
    outcomes.extend((0..13).map(|_| SyncPeerReceiveOutcome::Idle));
    let mut transport = TimingTransport::new(outcomes);
    let sent = Rc::clone(&transport.sent);
    let mut resolver = timing_resolver();
    let mut clock_calls = 0_usize;
    let mut elapsed = 0_i64;
    let mut clock = || {
        clock_calls += 1;
        if clock_calls <= 4 {
            return started_at;
        }
        elapsed += 5;
        assert!(!sent.borrow().iter().any(is_full_block_getdata));
        started_at + elapsed
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, started_at, &mut clock)
        .expect("compact cadence sync summary");

    // Assert
    assert_eq!(elapsed, COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 5);
    assert_eq!(clock_calls, 17);
    assert_eq!(summary.messages_processed, 4);
    assert_eq!(summary.peer_outcomes[0].state, PeerSyncState::Connected);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| is_full_block_getdata_for_hash(message, expected_hash))
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_compact_timeout_fallback_consumes_matching_block_before_yield() {
    // Arrange
    let path = temp_store_path("phase123-compact-fallback-response");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = connectable_compact_block_fixture(&mut runtime);
    let expected_hash = block_hash(&compact_block.header);
    let expected_hash_hex = block_hash_hex(expected_hash);
    let started_at = i64::from(compact_block.header.time) + 1;
    let mut outcomes = vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
    ];
    outcomes.extend((0..13).map(|_| SyncPeerReceiveOutcome::Idle));
    outcomes.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Block(
        compact_block.clone(),
    )));
    outcomes.push(SyncPeerReceiveOutcome::Idle);
    let mut transport = TimingTransport::new(outcomes);
    let sent = Rc::clone(&transport.sent);
    let mut resolver = timing_resolver();
    let mut clock_calls = 0_usize;
    let mut elapsed = 0_i64;
    let mut clock = || {
        clock_calls += 1;
        if clock_calls <= 4 {
            return started_at;
        }
        elapsed += 5;
        if clock_calls <= 17 {
            assert!(!sent.borrow().iter().any(is_full_block_getdata));
        } else {
            assert!(sent.borrow().iter().any(is_full_block_getdata));
        }
        started_at + elapsed
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, started_at, &mut clock)
        .expect("compact fallback response summary");

    // Assert
    assert_eq!(clock_calls, 19);
    assert_eq!(summary.messages_processed, 5);
    assert_eq!(summary.peer_outcomes[0].contribution.blocks_received, 1);
    assert_eq!(
        summary.maybe_downloaded_block_hash,
        Some(expected_hash_hex.clone())
    );
    assert_eq!(summary.maybe_connected_block_hash, Some(expected_hash_hex));
    assert!(runtime.inflight_blocks.is_empty());
    assert!(
        runtime
            .store()
            .load_block(expected_hash)
            .expect("load fallback block")
            .is_some()
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase123_slow_messages_without_idle_timestamp_compact_at_receipt() {
    // Arrange
    let path = temp_store_path("phase123-slow-message-clock");
    remove_dir_if_exists(&path);
    let mut runtime = timing_runtime(&path, 8);
    let compact_block = compact_block_fixture(&mut runtime);
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(
            &compact_block,
        ))),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Closed,
    ]);
    let mut resolver = timing_resolver();
    let clock_calls = Rc::new(RefCell::new(0_usize));
    let clock_call_count = Rc::clone(&clock_calls);
    let mut clock_values = [9_000, 9_005, 9_010, 9_300, 9_305].into_iter();
    let mut clock = move || {
        *clock_call_count.borrow_mut() += 1;
        clock_values.next().expect("scripted clock value")
    };

    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, 9_000, &mut clock)
        .expect("slow-message clock summary");

    // Assert
    assert_eq!(*clock_calls.borrow(), 5);
    assert_eq!(summary.messages_processed, 4);
    assert!(!transport.sent_messages().iter().any(is_full_block_getdata));
    remove_dir_if_exists(&path);
}

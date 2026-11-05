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

#[test]
fn phase167_compact_runtime_handoff_message_timeout_keeps_response_pending() {
    // Arrange
    let path = temp_store_path("phase167-message-timeout-handoff");
    let mut runtime = timing_runtime(&path, 8);
    let block = connectable_compact_block_fixture(&mut runtime);
    let hash = block_hash(&block.header);
    let future = build_block(hash, 3);
    let future_hash = block_hash(&future.header);
    runtime.config.max_blocks_in_flight_total = 1;
    let started = i64::from(block.header.time) + 1;
    let mut transport = TimingTransport::new(vec![
        SyncPeerReceiveOutcome::Message(version_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Verack),
        SyncPeerReceiveOutcome::Message(send_compact_message()),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::CompactBlock(compact_payload(&block))),
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Headers(
            open_bitcoin_network::HeadersMessage {
                headers: vec![block.header.clone(), future.header],
            },
        )),
        SyncPeerReceiveOutcome::Idle,
        SyncPeerReceiveOutcome::Message(WireNetworkMessage::Block(block)),
        SyncPeerReceiveOutcome::Idle,
    ]);
    let mut resolver = timing_resolver();
    let mut calls = 0;
    let observed_network = runtime.network.clone();
    let mut clock = || {
        calls += 1;
        if calls == 7 {
            assert!(
                !observed_network
                    .peer_requested_blocks(1)
                    .expect("before response scheduling")
                    .contains(&future_hash)
            );
        }
        if calls <= 4 {
            started
        } else {
            started + 60 + calls - 4
        }
    };
    // Act
    let summary = runtime
        .sync_once_with_resolver_and_clock(&mut transport, &mut resolver, started, &mut clock)
        .expect("message timeout session");
    let retained = runtime
        .store()
        .load_block(hash)
        .expect("fallback body")
        .is_some();
    remove_dir_if_exists(&path);
    // Assert
    assert_eq!(calls, 8);
    assert_eq!(summary.messages_processed, 6);
    assert!(retained);
    assert!(
        transport
            .sent_messages()
            .iter()
            .any(|message| is_full_block_getdata_for_hash(message, hash))
    );
}

fn handoff_peer(runtime: &mut DurableSyncRuntime, peer: PeerId) {
    runtime
        .network
        .connect_outbound_peer(peer, 0)
        .expect("real peer");
    for message in [
        version_message(),
        WireNetworkMessage::Verack,
        send_compact_message(),
    ] {
        runtime
            .network
            .receive_sync_message(
                peer,
                message,
                0,
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("real handshake");
    }
}

fn handoff_registered(
    runtime: &mut DurableSyncRuntime,
    peer: PeerId,
    hashes: &[BlockHash],
) -> Vec<(PeerId, WireNetworkMessage)> {
    runtime
        .network
        .request_missing_blocks(peer, hashes)
        .expect("actual ordinary registration")
        .into_iter()
        .map(|message| (peer, message))
        .collect()
}

fn handoff_session(
    runtime: &mut DurableSyncRuntime,
    messages: Vec<SyncPeerReceiveOutcome>,
    started: i64,
) -> (
    Result<crate::sync::PeerProgress, Box<crate::sync::PeerFailure>>,
    Vec<WireNetworkMessage>,
) {
    let sent = Rc::new(RefCell::new(Vec::new()));
    let session = TimingSession {
        outcomes: messages.into(),
        sent: Rc::clone(&sent),
    };
    let peer = resolved_manual_peer("127.0.0.1", 18_444);
    let mut calls = 0;
    let mut clock = || {
        calls += 1;
        if calls <= 4 { started } else { started + 61 }
    };
    let result = runtime.sync_connected_peer(session, &peer, 1, 1, started, &mut clock);
    let messages = sent.borrow().clone();
    (result, messages)
}

#[test]
fn phase167_compact_runtime_handoff_batch_preserves_packets_unique_slots_and_resends() {
    use crate::sync::block_reconcile::handoff_requested_messages;
    // Arrange
    let path = temp_store_path("phase167-packet-handoff");
    let mut runtime = timing_runtime(&path, 8);
    compact_block_fixture(&mut runtime);
    handoff_peer(&mut runtime, 99);
    let hashes = [
        BlockHash::from_byte_array([91; 32]),
        BlockHash::from_byte_array([92; 32]),
    ];
    let actual = handoff_registered(&mut runtime, 99, &hashes);
    let mut packet = actual[0].1.clone();
    let WireNetworkMessage::GetData(ref mut inventory) = packet else {
        panic!("actual request");
    };
    inventory.inventory[0].inventory_type = InventoryType::WitnessBlock;
    inventory.inventory.push(inventory.inventory[0].clone());
    let packets = vec![
        (99, WireNetworkMessage::Ping { nonce: 8 }),
        (99, packet.clone()),
        (99, packet.clone()),
    ];
    runtime.config.max_blocks_in_flight_total = 2;
    let before = format!(
        "{:?}",
        runtime.network.peer_manager_snapshot().expect("manager")
    );
    // Act
    let first =
        handoff_requested_messages(&mut runtime, 99, packets.clone()).expect("unique batch");
    let resent = handoff_requested_messages(&mut runtime, 99, packets).expect("resend at capacity");
    let sent = Rc::new(RefCell::new(Vec::new()));
    let mut session = TimingSession {
        outcomes: VecDeque::new(),
        sent: Rc::clone(&sent),
    };
    runtime
        .send_all(&mut session, 99, &first)
        .expect("real transport delivery");
    // Assert
    assert_eq!(
        first,
        vec![
            WireNetworkMessage::Ping { nonce: 8 },
            packet.clone(),
            packet
        ]
    );
    assert_eq!(resent, first);
    assert_eq!(*sent.borrow(), first);
    assert_eq!(runtime.inflight_blocks, hashes.into_iter().collect());
    assert_eq!(
        format!(
            "{:?}",
            runtime.network.peer_manager_snapshot().expect("manager")
        ),
        before
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase167_compact_runtime_handoff_rejects_unregistered_and_foreign_batches_atomically() {
    use crate::sync::block_reconcile::handoff_requested_messages;
    // Arrange
    let path = temp_store_path("phase167-handoff-refusals");
    let mut runtime = timing_runtime(&path, 8);
    compact_block_fixture(&mut runtime);
    handoff_peer(&mut runtime, 99);
    let hash = BlockHash::from_byte_array([91; 32]);
    let actual = handoff_registered(
        &mut runtime,
        99,
        &[hash, BlockHash::from_byte_array([96; 32])],
    );
    let mut unknown = actual[0].1.clone();
    if let WireNetworkMessage::GetData(ref mut inventory) = unknown {
        inventory.inventory[0].object_hash = BlockHash::from_byte_array([93; 32]).into();
    }
    let before = format!(
        "{:?}",
        runtime.network.peer_manager_snapshot().expect("manager")
    );
    // Act / Assert
    for batch in [
        vec![(99, actual[0].1.clone()), (100, actual[0].1.clone())],
        vec![(99, actual[0].1.clone()), (99, unknown)],
    ] {
        assert!(matches!(
            handoff_requested_messages(&mut runtime, 99, batch),
            Err(SyncRuntimeError::Network { .. })
        ));
        assert!(runtime.inflight_blocks.is_empty());
        assert_eq!(
            format!(
                "{:?}",
                runtime.network.peer_manager_snapshot().expect("manager")
            ),
            before
        );
    }
    runtime.config.max_blocks_in_flight_total = 1;
    assert!(matches!(
        handoff_requested_messages(&mut runtime, 99, actual),
        Err(SyncRuntimeError::ResourceLimit { .. })
    ));
    assert!(runtime.inflight_blocks.is_empty());
    remove_dir_if_exists(&path);
}

#[test]
fn phase167_compact_runtime_handoff_overflow_runs_real_teardown_without_delivery() {
    use crate::sync::block_reconcile::handoff_requested_messages;
    // Arrange
    let path = temp_store_path("phase167-handoff-global-overflow");
    let mut runtime = timing_runtime(&path, 8);
    let block = compact_block_fixture(&mut runtime);
    let hash = block_hash(&block.header);
    handoff_peer(&mut runtime, 99);
    let other = BlockHash::from_byte_array([94; 32]);
    let batch = handoff_registered(&mut runtime, 99, &[other]);
    handoff_requested_messages(&mut runtime, 99, batch).expect("existing runtime occupancy");
    runtime.config.max_blocks_in_flight_total = 1;
    let mut messages = compact_download_script(&block);
    messages.truncate(4);
    messages.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping {
        nonce: 9,
    }));
    // Act
    let (result, sent) = handoff_session(&mut runtime, messages, 100);
    // Assert
    assert!(matches!(
        result.expect_err("terminal overflow").error,
        SyncRuntimeError::ResourceLimit { .. }
    ));
    assert!(
        !sent
            .iter()
            .any(|message| is_full_block_getdata_for_hash(message, hash))
    );
    assert_eq!(runtime.inflight_blocks, [other].into_iter().collect());
    let manager = runtime
        .network
        .peer_manager_snapshot()
        .expect("post teardown");
    assert!(manager.peer_state(1).is_none());
    assert!(manager.block_request_peer_order(hash).is_empty());
    assert_eq!(manager.block_request_peer_order(other), vec![99]);
    remove_dir_if_exists(&path);
}

#[test]
fn phase167_compact_runtime_handoff_receipts_and_teardown_preserve_other_same_hash() {
    use crate::sync::block_reconcile::handoff_requested_messages;
    for route in 0..3 {
        // Arrange
        let path = temp_store_path(&format!("phase167-shared-pending-{route}"));
        let mut runtime = timing_runtime(&path, 8);
        let block = connectable_compact_block_fixture(&mut runtime);
        let hash = block_hash(&block.header);
        handoff_peer(&mut runtime, 99);
        let other = BlockHash::from_byte_array([95; 32]);
        let batch = handoff_registered(&mut runtime, 99, &[hash, other]);
        handoff_requested_messages(&mut runtime, 99, batch).expect("two real pending hashes");
        let mut messages = compact_download_script(&block);
        messages.truncate(if route == 2 { 3 } else { 4 });
        if route == 1 {
            messages.push(SyncPeerReceiveOutcome::Message(
                WireNetworkMessage::NotFound(open_bitcoin_network::InventoryList::new(vec![
                    open_bitcoin_core::primitives::InventoryVector {
                        inventory_type: InventoryType::Block,
                        object_hash: hash.into(),
                    },
                ])),
            ));
        }
        if route == 2 {
            messages.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Block(
                block,
            )));
        }
        messages.push(SyncPeerReceiveOutcome::Closed);
        // Act
        let (result, _) = handoff_session(&mut runtime, messages, 100);
        result.expect("real session receipt/teardown");
        // Assert
        assert_eq!(
            runtime.inflight_blocks,
            [hash, other].into_iter().collect(),
            "route={route}"
        );
        let manager = runtime
            .network
            .peer_manager_snapshot()
            .expect("remaining peer");
        assert_eq!(manager.block_request_peer_order(hash), vec![99]);
        assert_eq!(manager.block_request_peer_order(other), vec![99]);
        assert!(manager.peer_state(1).is_none());
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_compact_runtime_handoff_error_after_manager_release_cleans_global_hash() {
    // Arrange
    let path = temp_store_path("phase167-error-after-release");
    let mut runtime = timing_runtime(&path, 8);
    let block = connectable_compact_block_fixture(&mut runtime);
    let hash = block_hash(&block.header);
    let mut invalid = block.clone();
    invalid.transactions[0].lock_time += 1;
    let mut messages = compact_download_script(&block);
    messages.truncate(4);
    messages.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping {
        nonce: 9,
    }));
    messages.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Block(
        invalid,
    )));
    // Act
    let (result, sent) = handoff_session(&mut runtime, messages, i64::from(block.header.time) + 1);
    // Assert
    let failure = result.expect_err("actual invalid body after request release");
    assert!(
        failure.error.to_string().contains("bad-txnmrklroot"),
        "{}",
        failure.error
    );
    assert!(
        sent.iter()
            .any(|message| is_full_block_getdata_for_hash(message, hash))
    );
    assert!(runtime.inflight_blocks.is_empty());
    assert!(
        runtime
            .network
            .peer_manager_snapshot()
            .expect("teardown")
            .peer_state(1)
            .is_none()
    );
    remove_dir_if_exists(&path);
}

#[test]
fn phase167_compact_runtime_handoff_preserves_receive_and_cleanup_errors() {
    use crate::sync::session::request_cleanup_result;
    // Arrange
    let path = temp_store_path("phase167-cleanup-error-preservation");
    let mut runtime = timing_runtime(&path, 8);
    let mut invalid = connectable_compact_block_fixture(&mut runtime);
    handoff_peer(&mut runtime, 99);
    handoff_registered(&mut runtime, 99, &[block_hash(&invalid.header)]);
    invalid.transactions[0].lock_time += 1;
    let received = runtime.network.receive_sync_message(
        99,
        WireNetworkMessage::Block(invalid),
        0,
        runtime.verify_flags,
        runtime.consensus_params,
    );
    let primary = SyncRuntimeError::from(received.expect_err("real invalid body receive"));
    let cleanup = SyncRuntimeError::from(
        runtime
            .network
            .peer_requested_blocks(778)
            .expect_err("real cleanup capture failure"),
    );
    // Act
    let combined = request_cleanup_result::<()>(Err(primary.clone()), Err(cleanup.clone()))
        .expect_err("both errors");
    // Assert
    assert!(combined.to_string().contains(&primary.to_string()));
    assert!(combined.to_string().contains(&cleanup.to_string()));
    assert_eq!(
        request_cleanup_result::<()>(Err(primary.clone()), Ok(())),
        Err(primary)
    );
    assert_eq!(
        request_cleanup_result::<()>(Ok(()), Err(cleanup.clone())),
        Err(cleanup)
    );
    remove_dir_if_exists(&path);
}

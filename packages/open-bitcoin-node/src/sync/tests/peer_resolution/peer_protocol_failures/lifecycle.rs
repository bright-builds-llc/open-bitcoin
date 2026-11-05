// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;

#[test]
fn phase167_peer_session_protocol_removal_reconciles_ordinary_compact_and_fallback() {
    for route in 0..3 {
        // Arrange
        let (path, mut runtime, block) =
            lifetime_fixture(&format!("phase167-protocol-owned-{route}"));
        let hash = block_hash(&block.header);
        let exclusive = BlockHash::from_byte_array([82; 32]);
        let other = BlockHash::from_byte_array([83; 32]);
        let mut requests = owned_peer(&mut runtime, 1);
        lifetime_handshake(&runtime, 99);
        lifetime_register(&mut runtime, 99, &[hash, other]);
        lifetime_register(&mut runtime, 1, &[exclusive]);
        let mut progress = PeerProgress::new(
            resolved_manual_peer("127.0.0.1", 18444),
            runtime.config.network,
            1,
        );
        if route == 0 {
            lifetime_register(&mut runtime, 1, &[hash]);
        } else {
            requests
                .dispatch(
                    &mut runtime,
                    WireNetworkMessage::SendCompact(SendCompactMessage {
                        announce: true,
                        version: 2,
                    }),
                    0,
                    &[],
                    &mut progress,
                )
                .expect("authorization")
                .expect("negotiation");
            requests
                .dispatch(
                    &mut runtime,
                    WireNetworkMessage::CompactBlock(lifetime_compact(&block, false)),
                    i64::from(block.header.time),
                    &[],
                    &mut progress,
                )
                .expect("authorization")
                .expect("actual partial");
            if route == 2 {
                requests
                    .expire(&mut runtime, i64::from(block.header.time) + 61)
                    .expect("actual fallback grant");
            }
        }
        requests
            .renew(&mut runtime, &[])
            .expect("retained observation");
        let manager = runtime
            .network
            .peer_manager_snapshot()
            .expect("other before");
        let other_state = manager.peer_state(99).expect("other").clone();
        // Act
        let error = requests
            .dispatch(
                &mut runtime,
                WireNetworkMessage::Version(VersionMessage::default()),
                1,
                &[],
                &mut progress,
            )
            .expect("actual dispatch authorization")
            .expect_err("actual duplicate version");
        requests
            .finish(&mut runtime)
            .expect("authorized absence, no second disconnect");
        // Assert
        assert!(matches!(error, SyncRuntimeError::PeerCompatibility { .. }));
        assert_eq!(runtime.inflight_blocks, [hash, other].into_iter().collect());
        let after = runtime.network.peer_manager_snapshot().expect("after");
        assert!(after.peer_state(1).is_none());
        assert_eq!(after.peer_state(99), Some(&other_state));
        assert_eq!(after.block_request_peer_order(hash), vec![99]);
        assert_eq!(after.block_request_peer_order(other), vec![99]);
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_compact_completion_and_every_finish_keep_real_fallback_accounting() {
    for (blocktxn, shared) in [(false, false), (false, true), (true, false), (true, true)] {
        for finish in 0..3 {
            // Arrange
            let (path, mut runtime, block) = lifetime_fixture(&format!(
                "phase167-compact-lifetime-{blocktxn}-{shared}-{finish}"
            ));
            let hash = block_hash(&block.header);
            let other = BlockHash::from_byte_array([84; 32]);
            lifetime_handshake(&runtime, 99);
            let retained = [
                if shared {
                    hash
                } else {
                    BlockHash::from_byte_array([85; 32])
                },
                other,
            ];
            lifetime_register(&mut runtime, 99, &retained);
            let before = runtime
                .network
                .peer_manager_snapshot()
                .expect("other before")
                .peer_state(99)
                .expect("other")
                .clone();
            let before_tip = runtime.network.maybe_chain_tip().expect("tip");
            let mut outcomes: Vec<_> = version_verack_script(101)
                .into_iter()
                .chain([
                    WireNetworkMessage::SendCompact(SendCompactMessage {
                        announce: true,
                        version: 2,
                    }),
                    WireNetworkMessage::CompactBlock(lifetime_compact(&block, false)),
                ])
                .map(SyncPeerReceiveOutcome::Message)
                .collect();
            outcomes.push(SyncPeerReceiveOutcome::Idle);
            if !shared && finish != 2 {
                // A consumed compact attempt cannot be restarted by another announcement.
                outcomes.push(SyncPeerReceiveOutcome::Message(
                    WireNetworkMessage::CompactBlock(lifetime_compact(&block, blocktxn)),
                ));
                if finish == 0 {
                    outcomes.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Ping {
                        nonce: 9,
                    }));
                }
                // The emitted full fetch is completed by its actual full-body response.
                outcomes.push(SyncPeerReceiveOutcome::Message(WireNetworkMessage::Block(
                    block.clone(),
                )));
            }
            let sent = Rc::new(RefCell::new(Vec::new()));
            let session = CompactSession {
                outcomes: outcomes.into(),
                sent: Rc::clone(&sent),
                network: runtime.network.clone(),
                remove_on_getdata: finish == 2,
                hash,
                manager_only: false,
            };
            let peer = resolved_manual_peer("127.0.0.1", 18444);
            let start = i64::from(block.header.time) + 1;
            let mut calls = 0;
            let mut clock = || {
                calls += 1;
                if calls <= 4 { start } else { start + 61 }
            };
            // Act
            let result = runtime.sync_connected_peer(session, &peer, 1, 1, start, &mut clock);
            // Assert
            result.expect("real completion or legitimately absent original");
            let full_fetches: Vec<_> = sent
                .borrow()
                .iter()
                .flat_map(|message| {
                    let WireNetworkMessage::GetData(inventory) = message else {
                        return Vec::new();
                    };
                    inventory
                        .inventory
                        .iter()
                        .filter(|item| BlockHash::from(item.object_hash) == hash)
                        .map(|item| item.inventory_type)
                        .collect::<Vec<_>>()
                })
                .collect();
            if shared {
                assert!(
                    full_fetches.is_empty(),
                    "later peer emitted full fetch: {full_fetches:?}"
                );
            } else {
                assert_eq!(
                    full_fetches,
                    vec![open_bitcoin_core::primitives::InventoryType::WitnessBlock]
                );
            }
            assert_eq!(runtime.inflight_blocks, retained.into_iter().collect());
            let after = runtime.network.peer_manager_snapshot().expect("after");
            assert_eq!(after.peer_state(99), Some(&before));
            assert_eq!(
                after.block_request_peer_order(hash),
                if shared { vec![99] } else { Vec::new() }
            );
            assert_eq!(after.block_request_peer_order(retained[0]), vec![99]);
            assert_eq!(after.block_request_peer_order(other), vec![99]);
            assert!(after.peer_state(1).is_none());
            if shared || finish == 2 {
                assert_eq!(
                    runtime.network.maybe_chain_tip().expect("unchanged tip"),
                    before_tip
                );
                assert!(
                    runtime
                        .store
                        .load_block(hash)
                        .expect("unaccepted body")
                        .is_none()
                );
            } else if finish != 2 {
                assert_eq!(
                    runtime
                        .network
                        .maybe_chain_tip()
                        .expect("actual accepted completion tip")
                        .map(|tip| tip.block_hash),
                    Some(hash)
                );
                assert!(
                    runtime
                        .store
                        .load_block(hash)
                        .expect("actual complete stored body")
                        .is_some()
                );
            }
            drop(runtime);
            remove_dir_if_exists(&path);
        }
    }
}

#[test]
fn phase167_peer_session_stale_bound_initial_idle_handoff_and_policy_have_no_effects() {
    for operation in 0..4 {
        // Arrange
        let (path, mut runtime, block) =
            lifetime_fixture(&format!("phase167-stale-scheduling-{operation}"));
        let hash = block_hash(&block.header);
        let mut requests = owned_peer(&mut runtime, 1);
        let issued = runtime
            .network
            .request_missing_blocks(1, &[hash])
            .expect("genuine old request");
        block_reconcile::handoff_requested_messages(
            &mut runtime,
            1,
            issued.iter().cloned().map(|message| (1, message)).collect(),
        )
        .expect("genuine old slots");
        requests
            .renew(&mut runtime, &issued)
            .expect("earned observation");
        let replacement = runtime.network.clone();
        replacement.disconnect_peer(1).expect("actual old removal");
        lifetime_handshake(&runtime, 1);
        lifetime_handshake(&runtime, 99);
        runtime
            .network
            .receive_sync_message(
                99,
                WireNetworkMessage::CompactBlock(lifetime_compact(&block, false)),
                i64::from(block.header.time),
                runtime.verify_flags,
                runtime.consensus_params,
            )
            .expect("real other compact tick target");
        let future = build_block(hash, 102);
        lifetime_admit_headers(&runtime, &[block.header.clone(), future.header]);
        let manager_before = format!(
            "{:?}",
            replacement
                .peer_manager_snapshot()
                .expect("replacement before")
        );
        let slots = runtime.inflight_blocks.clone();
        let mut progress = PeerProgress::new(
            resolved_manual_peer("127.0.0.1", 18444),
            runtime.config.network,
            1,
        );
        // Act
        let error = match operation {
            0 => requests.request_missing(&mut runtime).map(|_| ()),
            1 => requests
                .expire(&mut runtime, i64::from(block.header.time) + 61)
                .map(|_| ()),
            2 => requests
                .handoff(
                    &mut runtime,
                    issued.into_iter().map(|message| (1, message)).collect(),
                )
                .map(|_| ()),
            _ => requests.record_ignored(&mut runtime, 1, &mut progress, &[hash]),
        };
        // Assert
        assert!(error.is_err());
        assert!(requests.finish(&mut runtime).is_err());
        assert_eq!(
            format!(
                "{:?}",
                replacement
                    .peer_manager_snapshot()
                    .expect("replacement unchanged")
            ),
            manager_before
        );
        assert_eq!(runtime.inflight_blocks, slots);
        assert_eq!(progress.messages_processed, 0);
        assert!(progress.maybe_last_activity_unix_seconds.is_none());
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_actual_socket_charges_decode_and_message_limit_without_stale_credit() {
    for route in 0..5 {
        // Arrange
        let (path, mut runtime, _) = lifetime_fixture(&format!("phase167-socket-charge-{route}"));
        runtime.config.max_messages_per_peer = if route == 2 || route == 4 { 2 } else { 1 };
        let (session, _server, peer, reads) = metered_socket(&runtime, route);
        let network = runtime.network.clone();
        let mut calls = 0;
        let mut clock = || {
            calls += 1;
            if route == 1 {
                network
                    .disconnect_peer(1)
                    .expect("removed after actual decode");
                network.add_inbound_peer(1).expect("actual replacement");
            }
            2_000
        };
        // Act
        let result = runtime.sync_connected_peer(session, &peer, 1, 1, 1_999, &mut clock);
        // Assert
        assert_eq!(reads.get(), if route == 2 || route == 4 { 2 } else { 1 });
        assert_metered_result(route, result);
        assert_eq!(
            calls,
            if route == 3 {
                0
            } else if route == 2 || route == 4 {
                2
            } else {
                1
            }
        );
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_absent_after_handoff_and_poison_keep_authorized_accounting() {
    for poison in [false, true] {
        // Arrange
        let (path, mut runtime, block) =
            lifetime_fixture(&format!("phase167-handoff-removal-{poison}"));
        let hash = block_hash(&block.header);
        let mut requests = owned_peer(&mut runtime, 1);
        let mut progress = PeerProgress::new(
            resolved_manual_peer("127.0.0.1", 18444),
            runtime.config.network,
            1,
        );
        requests
            .dispatch(
                &mut runtime,
                WireNetworkMessage::SendCompact(SendCompactMessage {
                    announce: true,
                    version: 2,
                }),
                0,
                &[],
                &mut progress,
            )
            .expect("authorized")
            .expect("negotiated");
        requests
            .dispatch(
                &mut runtime,
                WireNetworkMessage::CompactBlock(lifetime_compact(&block, false)),
                i64::from(block.header.time),
                &[],
                &mut progress,
            )
            .expect("authorized")
            .expect("actual partial");
        let actual = requests
            .expire(&mut runtime, i64::from(block.header.time) + 61)
            .expect("actual Idle-equivalent handoff");
        assert!(getdata_block_hashes(&actual).contains(&hash));
        assert!(runtime.inflight_blocks.contains(&hash));
        let before = runtime.inflight_blocks.clone();
        if poison {
            runtime.network.poison_for_test();
        } else {
            runtime
                .network
                .disconnect_peer(1)
                .expect("actual external removal");
        }
        // Act
        let renewed = requests.renew(&mut runtime, &actual);
        let finished = requests.finish(&mut runtime);
        // Assert
        if poison {
            assert!(renewed.is_err());
            assert!(finished.is_err());
            assert_eq!(runtime.inflight_blocks, before);
        } else {
            assert!(!renewed.expect("authorized absent renewal"));
            finished.expect("authorized absence");
            assert!(runtime.inflight_blocks.is_empty());
        }
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_real_known_prefix_survives_later_store_error_but_not_stale_commit() {
    for stale in [false, true] {
        // Arrange
        let (path, mut runtime, known) =
            lifetime_fixture(&format!("phase167-preparation-prefix-{stale}"));
        let known_hash = block_hash(&known.header);
        let corrupt = build_block(known_hash, 102);
        let corrupt_hash = block_hash(&corrupt.header);
        lifetime_admit_headers(&runtime, &[known.header.clone(), corrupt.header.clone()]);
        runtime
            .store
            .save_block(&known, PersistMode::Sync)
            .expect("actual first known body");
        runtime
            .store
            .write_raw_for_test(
                StorageNamespace::BlockIndex,
                &format!("block:{}", block_hash_hex(corrupt_hash)),
                vec![0xff],
            )
            .expect("deliberately malformed later body");
        let original = SyncRuntimeError::from(
            runtime
                .store
                .load_block(corrupt_hash)
                .expect_err("genuine decode error"),
        );
        let mut requests = owned_peer(&mut runtime, 1);
        if stale {
            runtime.network.disconnect_peer(1).expect("actual removal");
            lifetime_handshake(&runtime, 1);
        }
        let before = format!(
            "{:?}",
            runtime.network.peer_manager_snapshot().expect("before")
        );
        let slots = runtime.inflight_blocks.clone();
        // Act
        let error = requests
            .request_missing(&mut runtime)
            .expect_err("real later storage error");
        // Assert
        assert!(error.to_string().contains(&original.to_string()));
        assert_eq!(runtime.inflight_blocks, slots);
        if stale {
            assert!(error.to_string().contains("Replaced"));
            assert_eq!(
                format!(
                    "{:?}",
                    runtime
                        .network
                        .peer_manager_snapshot()
                        .expect("unchanged replacement")
                ),
                before
            );
        } else {
            assert_eq!(error, original);
        }
        let probe = runtime
            .network
            .request_missing_blocks(1, &[known_hash])
            .expect("actual known-membership probe");
        assert_eq!(getdata_block_hashes(&probe).is_empty(), !stale);
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_manager_membership_without_handoff_does_not_authorize_receipt_credit() {
    for notfound in [false, true] {
        // Arrange
        let (path, mut runtime, block) =
            lifetime_fixture(&format!("phase167-no-runtime-grant-{notfound}"));
        let hash = block_hash(&block.header);
        let before = runtime
            .network
            .chainstate_snapshot()
            .expect("before accepted state");
        assert!(!runtime.inflight_blocks.contains(&hash));
        let receipt = if notfound {
            notfound_for_block(hash)
        } else {
            WireNetworkMessage::Block(block)
        };
        let outcomes = version_verack_script(101)
            .into_iter()
            .chain([receipt])
            .map(SyncPeerReceiveOutcome::Message)
            .collect();
        let session = CompactSession {
            outcomes,
            sent: Rc::default(),
            network: runtime.network.clone(),
            remove_on_getdata: false,
            hash,
            manager_only: true,
        };
        let peer = resolved_manual_peer("127.0.0.1", 18444);
        let mut clock = || 2_000;
        // Act
        let progress = runtime
            .sync_connected_peer(session, &peer, 1, 1, 1_999, &mut clock)
            .expect("actual ignored-policy/NotFound path");
        // Assert
        assert_eq!(progress.blocks_received, 0);
        assert_eq!(progress.messages_processed, 3);
        if notfound {
            assert!(progress.maybe_failure_reason.is_none());
        }
        assert_eq!(
            runtime
                .network
                .chainstate_snapshot()
                .expect("accepted state"),
            before
        );
        assert!(
            runtime
                .store
                .load_block(hash)
                .expect("unrequested body remains unavailable")
                .is_none()
        );
        assert!(runtime.inflight_blocks.is_empty());
        drop(runtime);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn phase167_peer_session_old_outbox_finisher_keeps_replacement_queue_readiness_and_emissions() {
    // Arrange
    let (path, mut runtime, block) = lifetime_fixture("phase167-outbox-replacement");
    let expected = Rc::new(RefCell::new(Vec::new()));
    let registration = Rc::new(RefCell::new(None));
    let outboxes = runtime.announcement_outboxes();
    let session = ReplacedOutboxSession {
        inbound: version_verack_script(101).into(),
        network: runtime.network.clone(),
        store: runtime.store.clone(),
        outboxes: outboxes.clone(),
        block,
        flags: runtime.verify_flags,
        params: runtime.consensus_params,
        expected: Rc::clone(&expected),
        registration: Rc::clone(&registration),
        replaced: false,
    };
    let peer = resolved_manual_peer("127.0.0.1", 18444);
    let mut clock = || 2_000;
    // Act
    let error = runtime
        .sync_connected_peer(session, &peer, 1, 1, 1_999, &mut clock)
        .expect_err("owned registration was replaced");
    // Assert
    assert!(
        error
            .error
            .to_string()
            .contains("outbox registration was replaced")
    );
    let snapshots = outboxes.snapshots().expect("replacement still registered");
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].peer_id(), 1);
    assert_eq!(snapshots[0].queued_messages(), expected.borrow().len());
    let emissions = outboxes.take_peer_emissions(1).expect("replacement queue");
    assert_eq!(
        emissions
            .iter()
            .map(|emission| emission.message().clone())
            .collect::<Vec<_>>(),
        *expected.borrow()
    );
    let mut notification = registration
        .borrow_mut()
        .take()
        .expect("actual replacement readiness");
    let mut notified = std::pin::pin!(notification.notified());
    assert!(matches!(
        std::future::Future::poll(
            notified.as_mut(),
            &mut std::task::Context::from_waker(std::task::Waker::noop())
        ),
        std::task::Poll::Ready(())
    ));
    for emission in emissions {
        runtime
            .network
            .abort_peer_emission(emission.into_parts().2)
            .expect("replacement owner aborts its own actual emission");
    }
    outboxes
        .unregister_peer(&runtime.network, 1)
        .expect("replacement owner cleanup");
    drop(runtime);
    remove_dir_if_exists(&path);
}

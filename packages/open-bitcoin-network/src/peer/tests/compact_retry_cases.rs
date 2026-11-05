// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::super::block_requests::{CompactRequestRules, RequestPosition};
use super::*;

#[test]
fn phase167_compact_retry_later_collision_releases_only_later_without_getdata() {
    // Arrange
    let first = 90;
    let later = 2;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(first);
    manager.add_outbound_peer(later, 0).expect("later peer");
    complete_outbound_handshake(&mut manager, later, 2);
    process_high_bandwidth_sendcmpct(&mut manager, later);
    manager
        .handle_fixture_compact_block_download(
            first,
            payload.clone(),
            explicit_empty_compact_receive_facts(),
            100,
            false,
        )
        .expect("first request");
    let mut colliding = payload;
    colliding.short_ids.push(colliding.short_ids[0]);

    // Act
    let actions = manager
        .handle_fixture_compact_block_download(
            later,
            colliding,
            explicit_empty_compact_receive_facts(),
            10,
            false,
        )
        .expect("later semantic failure");

    // Assert
    assert!(
        actions.is_empty(),
        "later peer must not send a full fetch: {actions:?}"
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![first]);
    assert_eq!(
        manager.peer_requested_blocks(first).expect("first"),
        vec![hash]
    );
    assert!(
        manager
            .peer_requested_blocks(later)
            .expect("later")
            .is_empty()
    );
}

#[test]
fn phase167_compact_retry_first_collision_keeps_one_full_fetch() {
    // Arrange
    let peer_id = 91;
    let (mut manager, mut payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    payload.short_ids.push(payload.short_ids[0]);

    // Act
    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
            false,
        )
        .expect("first semantic failure");

    // Assert
    assert!(
        matches!(actions.as_slice(), [PeerAction::Send(WireNetworkMessage::GetData(list))]
        if list.inventory.len() == 1
            && list.inventory[0].inventory_type == InventoryType::WitnessBlock
            && list.inventory[0].object_hash == hash.into())
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
}

#[test]
fn phase167_compact_retry_consumed_reannounce_is_silent_and_matching_blocktxn_is_misbehavior() {
    // Arrange
    let peer_id = 92;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    let mut colliding = payload.clone();
    colliding.short_ids.push(colliding.short_ids[0]);
    manager
        .handle_fixture_compact_block_download(
            peer_id,
            colliding,
            explicit_empty_compact_receive_facts(),
            100,
            false,
        )
        .expect("first fallback");

    // Act
    let reannounce = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            101,
            false,
        )
        .expect("consumed announcement");
    let stray = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: BlockHash::from_byte_array([77; 32]),
                transactions: vec![],
            }),
            101,
        )
        .expect("unrelated stray");
    let duplicate = manager
        .handle_message(
            peer_id,
            WireNetworkMessage::BlockTxn(BlockTransactions {
                block_hash: hash,
                transactions: vec![],
            }),
            102,
        )
        .expect("matching duplicate");

    // Assert
    assert!(reannounce.is_empty());
    assert!(stray.is_empty());
    assert_eq!(
        duplicate,
        vec![PeerAction::Disconnect(
            DisconnectReason::CompactBlockMisbehavior
        )]
    );
    assert!(manager.block_request_peer_order(hash).is_empty());
}

#[test]
fn phase167_compact_retry_later_timeout_is_silent_and_first_timeout_uses_original_start() {
    // Arrange
    let first = 93;
    let later = 3;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(first);
    manager.add_outbound_peer(later, 0).expect("later peer");
    complete_outbound_handshake(&mut manager, later, 2);
    process_high_bandwidth_sendcmpct(&mut manager, later);
    for (peer, start) in [(first, 100), (later, 10)] {
        manager
            .handle_fixture_compact_block_download(
                peer,
                payload.clone(),
                explicit_empty_compact_receive_facts(),
                start,
                false,
            )
            .expect("active request");
    }

    // Act / Assert
    assert!(manager.expire_compact_download_timeouts(70).is_empty());
    assert!(manager.expire_compact_download_timeouts(71).is_empty());
    assert_eq!(manager.block_request_peer_order(hash), vec![first]);
    assert!(manager.expire_compact_download_timeouts(160).is_empty());
    let fallback = manager.expire_compact_download_timeouts(161);
    assert!(
        matches!(fallback.as_slice(), [(93, PeerAction::Send(WireNetworkMessage::GetData(list)))]
        if list.inventory[0].inventory_type == InventoryType::WitnessBlock
            && list.inventory[0].object_hash == hash.into())
    );
    assert!(manager.expire_compact_download_timeouts(200).is_empty());
    assert_eq!(manager.block_request_peer_order(hash), vec![first]);
}

#[test]
fn phase167_compact_retry_non_witness_version_selects_legacy_block_fetch() {
    // Arrange
    let peer_id = 94;
    let (_, mut payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    payload.short_ids.push(payload.short_ids[0]);
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    let genesis = mined_header(BlockHash::from_byte_array([0; 32]), 1);
    let tip = mined_header(block_hash(&genesis), 2);
    phase115_seed_header_chain(&mut manager, &[genesis, tip]);
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    manager
        .handle_message(
            peer_id,
            WireNetworkMessage::Version(crate::VersionMessage {
                services: ServiceFlags::NETWORK,
                ..Default::default()
            }),
            11,
        )
        .expect("non-WITNESS version");
    manager
        .handle_message(peer_id, WireNetworkMessage::Verack, 12)
        .expect("verack");
    process_high_bandwidth_sendcmpct(&mut manager, peer_id);

    // Act
    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
            false,
        )
        .expect("fallback");

    // Assert
    assert!(
        matches!(actions.as_slice(), [PeerAction::Send(WireNetworkMessage::GetData(list))]
        if list.inventory.len() == 1
            && list.inventory[0].inventory_type == InventoryType::Block
            && list.inventory[0].object_hash == hash.into())
    );
}

#[test]
fn phase167_compact_retry_distinct_consumed_hashes_stay_bounded_and_can_reenroll() {
    // Arrange
    let peer_id = 95;
    let (mut manager, template, _, _) = phase115_prepare_compact_download_manager(peer_id);
    manager.max_blocks_in_flight_per_peer = 2;
    let parent = template.header.previous_block_hash;
    let mut hashes = Vec::new();

    // Act / Assert
    for index in 0..16 {
        for slot in 0..4 {
            let mut payload = template.clone();
            payload.header = mined_header(parent, 200 + index * 4 + slot);
            payload.short_ids.push(payload.short_ids[0]);
            let hash = block_hash(&payload.header);
            hashes.push(hash);
            let actions = manager
                .handle_fixture_compact_block_download(
                    peer_id,
                    payload,
                    explicit_empty_compact_receive_facts(),
                    100,
                    false,
                )
                .expect("distinct semantic fallback");
            if slot < 2 {
                assert!(matches!(
                    actions.as_slice(),
                    [PeerAction::Send(WireNetworkMessage::GetData(_))]
                ));
            } else {
                assert!(actions.is_empty(), "at-cap fallback must remain silent");
            }
            assert!(
                manager
                    .peer_requested_blocks(peer_id)
                    .expect("bounded membership")
                    .len()
                    <= 2
            );
        }
        manager.cleanup_all_compact_downloads(CompactDownloadCleanupCause::Reorg);
        assert!(
            manager
                .peer_requested_blocks(peer_id)
                .expect("terminal cleanup")
                .is_empty()
        );
        for hash in &hashes {
            assert!(manager.block_request_peer_order(*hash).is_empty());
        }
    }
    let mut reenrolled = template;
    reenrolled.header = mined_header(parent, 200);
    reenrolled.short_ids.push(reenrolled.short_ids[0]);
    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            reenrolled,
            explicit_empty_compact_receive_facts(),
            200,
            false,
        )
        .expect("genuine reenrollment");
    assert!(matches!(
        actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetData(_))]
    ));
    assert_eq!(manager.block_request_peer_order(hashes[0]), vec![peer_id]);
}

#[test]
fn phase167_compact_retry_at_capacity_adds_no_partial_or_request() {
    // Arrange
    let peer_id = 96;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    manager.max_blocks_in_flight_per_peer = 0;

    // Act
    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            100,
            false,
        )
        .expect("bounded capacity refusal");

    // Assert
    assert!(actions.is_empty());
    assert!(manager.block_request_peer_order(hash).is_empty());
    assert!(
        manager
            .peer_requested_blocks(peer_id)
            .expect("peer")
            .is_empty()
    );
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| state.in_flight.is_empty())
    );
}

#[test]
fn phase167_compact_retry_detached_ordinary_request_does_not_resend_on_timeout() {
    // Arrange
    let peer_id = 97;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    assert!(
        manager
            .register_block_request(peer_id, hash, None, 2)
            .expect("ordinary prior request")
    );
    let start = 100;
    let actions = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            start,
            false,
        )
        .expect("selected compact request");
    assert!(matches!(
        actions.as_slice(),
        [PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))]
    ));
    manager.detach_compact_block_request(peer_id, hash);
    assert_eq!(
        manager.maybe_block_request_enforce_segwit(peer_id, hash),
        None
    );

    // Act
    let expired = manager.expire_compact_download_timeouts(
        start + crate::COMPACT_BLOCK_DOWNLOAD_TIMEOUT_SECONDS + 1,
    );

    // Assert
    assert!(expired.is_empty(), "detached compact owner cannot resend");
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert_eq!(
        manager
            .peer_requested_blocks(peer_id)
            .expect("ordinary request"),
        vec![hash]
    );
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| !state.in_flight.contains_key(&hash))
    );
}

#[test]
fn phase167_compact_owned_oversized_candidate_refuses_before_peer_authority() {
    // Arrange
    let peer_id = 98;
    let other_peer = 99;
    let (mut manager, mut payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    let other_hash = BlockHash::from_byte_array([0x98; 32]);
    manager
        .add_outbound_peer(other_peer, 0)
        .expect("unrelated peer");
    assert!(
        manager
            .register_block_request(other_peer, other_hash, None, 2)
            .expect("unrelated ordinary request")
    );
    let mut oversized = phase115_sample_transaction(0x98);
    oversized.outputs[0].script_pubkey =
        ScriptBuf::from_serialized_bytes(vec![
            0;
            usize::try_from(open_bitcoin_codec::MAX_SIZE + 1)
                .expect("bounded codec oversize")
        ]);
    let synthetic_wtxid = Wtxid::from_byte_array([0x98; 32]);
    let selector =
        open_bitcoin_codec::short_id_selector_from_header_and_nonce(&payload.header, payload.nonce);
    payload.short_ids = vec![open_bitcoin_consensus::compact_short_id_for_wtxid(
        selector,
        &synthetic_wtxid,
    )];
    let candidates = [(&synthetic_wtxid, &oversized)];

    // Act
    let error = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            CompactBlockReceiveFacts {
                candidates: &candidates,
                extra: &[],
            },
            100,
            false,
        )
        .expect_err("oversized owned candidate cannot publish or register");

    // Assert
    assert_eq!(
        error,
        NetworkError::Codec(open_bitcoin_codec::CodecError::CompactSizeTooLarge(
            open_bitcoin_codec::MAX_SIZE + 1,
        ))
    );
    assert!(manager.block_request_peer_order(hash).is_empty());
    assert!(
        manager
            .peer_requested_blocks(peer_id)
            .expect("peer")
            .is_empty()
    );
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_none_or(|state| state.in_flight.is_empty())
    );
    assert_eq!(
        manager.block_request_peer_order(other_hash),
        vec![other_peer]
    );
}

#[test]
fn phase167_compact_conflicting_held_fixture_rule_refuses_before_partial_install() {
    // Arrange
    let peer_id = 100;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    assert!(
        manager
            .register_block_request(
                peer_id,
                hash,
                Some(CompactRequestRules::Fixture {
                    enforce_segwit: true,
                }),
                2,
            )
            .expect("held fixture rule without partial")
    );
    assert!(manager.maybe_compact_attempt(peer_id, hash).is_none());
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_none_or(|state| state.in_flight.is_empty())
    );

    // Act
    let error = manager
        .handle_fixture_compact_block_download(
            peer_id,
            payload,
            explicit_empty_compact_receive_facts(),
            101,
            false,
        )
        .expect_err("conflicting held rule must refuse registration");

    // Assert
    assert!(
        matches!(error, NetworkError::InvalidHeader { reject_reason, .. }
        if reject_reason == "compact-request-context-mismatch")
    );
    assert_eq!(
        manager.maybe_block_request_enforce_segwit(peer_id, hash),
        Some(true)
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert!(
        manager
            .compact_download_peer_state(peer_id)
            .is_some_and(|state| state.in_flight.is_empty())
    );
}

#[test]
fn phase167_block_request_nonmember_and_inconsistent_peer_flag_do_not_create_order() {
    // Arrange
    let first = 101;
    let other = 102;
    let (mut manager, _, _, hash) = phase115_prepare_compact_download_manager(first);
    manager.add_outbound_peer(other, 0).expect("other peer");
    assert!(
        manager
            .register_block_request(first, hash, None, 2)
            .expect("first ordinary request")
    );
    assert_eq!(
        manager.request_position(other, hash),
        RequestPosition::Absent
    );
    manager
        .peers
        .get_mut(&other)
        .expect("other peer state")
        .requested_blocks
        .insert(hash);

    // Act
    let accepted = manager
        .register_block_request(
            other,
            hash,
            Some(CompactRequestRules::Fixture {
                enforce_segwit: true,
            }),
            2,
        )
        .expect("inconsistent peer flag refuses without mutation");

    // Assert
    assert!(!accepted);
    assert_eq!(manager.block_request_peer_order(hash), vec![first]);
    assert_eq!(
        manager.request_position(other, hash),
        RequestPosition::Absent
    );
}

#[test]
fn phase167_finish_block_requests_clears_two_compact_peers_but_keeps_unrelated_order() {
    // Arrange
    let first = 103;
    let second = 104;
    let (mut manager, payload, _, hash) = phase115_prepare_compact_download_manager(first);
    manager.add_outbound_peer(second, 0).expect("second peer");
    complete_outbound_handshake(&mut manager, second, 2);
    process_high_bandwidth_sendcmpct(&mut manager, second);
    for peer_id in [first, second] {
        manager
            .handle_fixture_compact_block_download(
                peer_id,
                payload.clone(),
                explicit_empty_compact_receive_facts(),
                100,
                false,
            )
            .expect("held compact partial");
    }
    let unrelated_hash = BlockHash::from_byte_array([0xa4; 32]);
    assert_ne!(hash, unrelated_hash);
    assert!(
        manager
            .register_block_request(first, unrelated_hash, None, 3)
            .expect("unrelated request")
    );
    assert_eq!(manager.block_request_peer_order(hash), vec![first, second]);

    // Act
    let cleared = manager.finish_block_requests(hash);

    // Assert
    assert_eq!(cleared, 2);
    assert!(manager.block_request_peer_order(hash).is_empty());
    assert_eq!(
        manager.peer_requested_blocks(first).expect("first"),
        vec![unrelated_hash]
    );
    assert!(
        manager
            .peer_requested_blocks(second)
            .expect("second")
            .is_empty()
    );
    for peer_id in [first, second] {
        assert!(
            manager
                .compact_download_peer_state(peer_id)
                .is_some_and(|state| !state.in_flight.contains_key(&hash))
        );
    }
}

#[test]
fn phase167_detach_absent_compact_request_preserves_unrelated_ordinary_order() {
    // Arrange
    let peer_id = 105;
    let (mut manager, _, _, hash) = phase115_prepare_compact_download_manager(peer_id);
    assert!(
        manager
            .register_block_request(peer_id, hash, None, 2)
            .expect("ordinary request")
    );
    let absent = BlockHash::from_byte_array([0xa5; 32]);

    // Act
    manager.detach_compact_block_request(peer_id, absent);

    // Assert
    assert_eq!(manager.block_request_peer_order(hash), vec![peer_id]);
    assert!(manager.block_request_peer_order(absent).is_empty());
    assert_eq!(
        manager.peer_requested_blocks(peer_id).expect("peer"),
        vec![hash]
    );
}

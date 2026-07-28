// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/mempool_persist.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/test/functional/mempool_persist.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/mempool_accept.py

use super::*;

#[test]
fn phase126_compact_announcement_uses_injected_nonce_once() {
    // Arrange
    let peer_id = 126_101;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let nonce_calls = Cell::new(0);
    let expected_nonce = 0x0123_4567_89ab_cdef;

    // Act
    let maybe_message = network
        .announce_block_with_nonce(
            peer_id,
            &block,
            phase126_compact_announcement_decision(),
            || {
                nonce_calls.set(nonce_calls.get() + 1);
                Ok::<u64, ()>(expected_nonce)
            },
        )
        .expect("compact announcement");

    // Assert
    let Some(WireNetworkMessage::CompactBlock(payload)) = maybe_message else {
        panic!("expected compact block announcement");
    };
    assert_eq!(payload.nonce, expected_nonce);
    assert_eq!(nonce_calls.get(), 1);
}

#[test]
fn phase126_compact_announcement_entropy_failure_uses_safe_fallback_without_compact_evidence() {
    // Arrange
    let peer_id = 126_102;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);

    // Act
    let maybe_message = network
        .announce_block_with_nonce(
            peer_id,
            &block,
            phase126_compact_announcement_decision(),
            || Err::<u64, ()>(()),
        )
        .expect("safe fallback");

    // Assert
    assert!(matches!(maybe_message, Some(WireNetworkMessage::Inv(_))));
    assert!(
        network
            .peer_manager()
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .is_empty()
    );
    let encoded =
        serde_json::to_value(network.block_relay_evidence_status()).expect("block relay evidence");
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_inventory_fallback_count"],
        0
    );
}

#[test]
fn phase126_non_compact_announcement_actions_do_not_request_entropy() {
    // Arrange
    let peer_id = 126_103;
    let mut network = compact_relay_enabled_managed_network(peer_id);
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let cases = [
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::AnnounceHeaders,
                reason: CompactAnnouncementReason::CompactHighBandwidthNotRequested,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
                },
            },
            "headers",
        ),
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::AnnounceInventory,
                reason: CompactAnnouncementReason::CompactRelayDisabled,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::LocalActivationDisabled,
                },
            },
            "inventory",
        ),
        (
            CompactAnnouncementDecision {
                action: CompactAnnouncementAction::Suppress,
                reason: CompactAnnouncementReason::CompactBlockUnavailable,
                eligibility: CompactAnnouncementEligibility::Ineligible {
                    reason: CompactAnnouncementEligibilityReason::BlockUnavailable,
                },
            },
            "suppression",
        ),
    ];

    // Act
    let outcomes: Vec<_> = cases
        .into_iter()
        .map(|(decision, label)| {
            let nonce_calls = Cell::new(0);
            let maybe_message = network
                .announce_block_with_nonce(peer_id, &block, decision, || {
                    nonce_calls.set(nonce_calls.get() + 1);
                    Ok::<u64, ()>(0)
                })
                .expect("non-compact announcement");
            (label, nonce_calls.get(), maybe_message)
        })
        .collect();

    // Assert
    assert!(matches!(
        outcomes.as_slice(),
        [
            ("headers", 0, Some(WireNetworkMessage::Headers(_))),
            ("inventory", 0, Some(WireNetworkMessage::Inv(_))),
            ("suppression", 0, None),
        ]
    ));
}

#[test]
fn phase116_block_relay_evidence_status_defaults_to_unavailable_until_observed() {
    let network = compact_relay_enabled_managed_network(116_001);

    let status = network.block_relay_evidence_status();
    let encoded = serde_json::to_value(status).expect("block relay evidence");

    assert_eq!(
        encoded["block_serving"]["activation"]["state"],
        "unavailable"
    );
}

#[test]
fn phase123_block_acknowledgement_increments_private_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_201);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let message = WireNetworkMessage::Block(block);

    // Act
    network.acknowledge_wire_message_written(&message);
    let snapshot: crate::network::BlockRelayRuntimeEvidenceSnapshot =
        network.block_relay_runtime_evidence_snapshot();

    // Assert
    assert_eq!(network.block_served_write_count(), 1);
    assert_eq!(snapshot.served_count, 1);
    assert!(matches!(
        snapshot.status.block_serving.activation,
        crate::status::FieldAvailability::Available(_)
    ));
}

#[test]
fn phase123_non_block_acknowledgement_does_not_increment_private_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_202);
    let message = WireNetworkMessage::Verack;

    // Act
    network.acknowledge_wire_message_written(&message);

    // Assert
    assert_eq!(network.block_served_write_count(), 0);
}

#[test]
fn phase123_public_block_relay_status_omits_runtime_served_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(123_203);
    let block = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    let message = WireNetworkMessage::Block(block);
    network.acknowledge_wire_message_written(&message);

    // Act
    let encoded = serde_json::to_value(network.block_relay_evidence_status())
        .expect("serialize public block-relay status");

    // Assert
    assert!(!encoded.to_string().contains("served_count"));
    assert_eq!(
        encoded["block_serving"]["eligibility"]["value"]["eligible_peer_count"],
        0
    );
    assert_eq!(
        encoded["block_serving"]["status"]["value"]["validated_count"],
        0
    );
}

#[test]
fn phase116_block_relay_evidence_projects_negotiation_serving_download_and_cleanup() {
    let mut network = compact_relay_enabled_managed_network(116_002);
    let peer_id = 116_002;
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");
    let maybe_announce = network
        .announce_block(peer_id, &genesis)
        .expect("announce block");
    assert!(
        matches!(
            maybe_announce.as_ref(),
            Some(WireNetworkMessage::CompactBlock(_))
        ),
        "HB-eligible announce must emit CompactBlock, got {maybe_announce:?}"
    );
    let emission = network
        .prepare_peer_emission(
            peer_id,
            maybe_announce.expect("compact announcement"),
            block_hash(&genesis.header),
        )
        .expect("supported compact emission");
    let (_, _, capability) = emission.into_parts();
    network
        .record_peer_emission(peer_id, capability.acknowledge_write().into_parts().1)
        .expect("complete compact write");

    let served = network
        .receive_message(
            peer_id,
            WireNetworkMessage::GetData(block_getdata_inventory(&genesis)),
            3,
            verify_flags(),
            consensus_params(),
        )
        .expect("serve block")
        .outbound;
    assert!(
        served
            .iter()
            .any(|message| matches!(message, WireNetworkMessage::Block(_)))
    );

    let actions = network
        .peer_manager
        .handle_compact_block_download(
            peer_id,
            compact_payload_with_missing_short_id(block_hash(&genesis.header)),
            open_bitcoin_network::CompactBlockReceiveFacts {
                candidates: &[],
                extra: &[],
            },
            4,
        )
        .expect("compact block");
    network.note_block_relay_observed();
    network.record_compact_download_evidence(&actions);
    assert!(actions.iter().any(|action| matches!(
        action,
        open_bitcoin_network::PeerAction::Send(WireNetworkMessage::GetBlockTxn(_))
    )));

    let status = network.block_relay_evidence_status();
    let encoded = serde_json::to_value(&status).expect("block relay evidence");
    assert_eq!(
        encoded["block_serving"]["activation"]["value"]["block_serving_enabled"],
        true
    );
    assert_eq!(
        encoded["negotiation"]["value"]["version2_high_bandwidth_count"],
        1
    );
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        1
    );
    assert_eq!(
        encoded["block_serving"]["eligibility"]["value"]["eligible_peer_count"],
        1
    );
    assert_eq!(
        encoded["missing_transaction"]["value"]["compact_missing_tx_requested_count"],
        1
    );
    assert_eq!(encoded["in_flight"]["value"]["in_flight_count"], 1);
    assert_eq!(
        encoded["in_flight"]["value"]["peers_with_in_flight_count"],
        1
    );

    network.disconnect_peer(peer_id).expect("disconnect");

    let cleaned = serde_json::to_value(network.block_relay_evidence_status())
        .expect("block relay evidence after cleanup");
    assert_eq!(
        cleaned["cleanup"]["value"]["compact_download_peer_disconnect_count"],
        1
    );
}

#[test]
fn phase118_low_bandwidth_announce_does_not_increment_compact_announced_count() {
    // Arrange
    let mut network = compact_relay_enabled_managed_network(118_003);
    let peer_id = 118_003;
    network
        .connect_outbound_peer(peer_id, 1)
        .expect("connect outbound");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Version(open_bitcoin_network::VersionMessage::default()),
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("version");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::Verack,
            1,
            verify_flags(),
            consensus_params(),
        )
        .expect("verack");
    network
        .receive_message(
            peer_id,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: open_bitcoin_codec::BIP152_COMPACT_BLOCKS_VERSION,
            }),
            2,
            verify_flags(),
            consensus_params(),
        )
        .expect("sendcmpct low-bandwidth");

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 500_000_000);
    network
        .connect_local_block(&genesis, verify_flags(), consensus_params())
        .expect("connect genesis");

    // Act
    let maybe_announce = network
        .announce_block(peer_id, &genesis)
        .expect("announce block");

    // Assert
    assert!(
        !matches!(maybe_announce, Some(WireNetworkMessage::CompactBlock(_))),
        "low-bandwidth path must not emit CompactBlock, got {maybe_announce:?}"
    );
    assert!(
        matches!(
            maybe_announce,
            Some(WireNetworkMessage::Headers(_)) | Some(WireNetworkMessage::Inv(_))
        ),
        "low-bandwidth path should fall back to Headers or Inv, got {maybe_announce:?}"
    );
    let encoded =
        serde_json::to_value(network.block_relay_evidence_status()).expect("block relay evidence");
    assert_eq!(
        encoded["announcement"]["value"]["compact_announced_count"],
        0
    );
}

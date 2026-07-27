// Parity breadcrumbs:
// - packages/bitcoin-knots/src/headerssync.cpp
// - packages/bitcoin-knots/src/sync.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/test/functional/p2p_handshake.py
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use super::*;

#[test]
fn phase113_sendcmpct_version2_high_bandwidth_updates_peer_compact_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_001, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_001,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");

    // Assert
    assert!(actions.is_empty());
    let compact = &manager.peer_state(113_001).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_version2_low_bandwidth_updates_peer_compact_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_002, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_002,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");

    // Assert
    assert!(actions.is_empty());
    let compact = &manager.peer_state(113_002).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_high_to_low_clears_high_bandwidth_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_003, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_003,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
    manager
        .handle_message(
            113_003,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            2,
        )
        .expect("sendcmpct low-bandwidth should process");

    // Assert
    let compact = &manager.peer_state(113_003).expect("peer").compact_relay;
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_low_to_high_clears_low_bandwidth_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_004, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_004,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct low-bandwidth should process");
    manager
        .handle_message(
            113_004,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            2,
        )
        .expect("sendcmpct high-bandwidth should process");

    // Assert
    let compact = &manager.peer_state(113_004).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_sendcmpct_unsupported_version_records_evidence_without_disconnect() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_005, 0).expect("peer");

    // Act
    let actions = manager
        .handle_message(
            113_005,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 3,
            }),
            1,
        )
        .expect("unsupported sendcmpct should process without disconnecting");

    // Assert
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Disconnect(_))));
    let compact = &manager.peer_state(113_005).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Unsupported { version: 3 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(compact.maybe_unsupported_version, Some(3));
}

#[test]
fn phase113_unsupported_sendcmpct_does_not_clear_existing_version2_capability() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_006, 0).expect("peer");

    // Act
    manager
        .handle_message(
            113_006,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 2,
            }),
            1,
        )
        .expect("sendcmpct high-bandwidth should process");
    let unsupported_actions = manager
        .handle_message(
            113_006,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");

    // Assert
    assert!(!unsupported_actions
        .iter()
        .any(|action| matches!(action, PeerAction::Disconnect(_))));
    let compact = &manager.peer_state(113_006).expect("peer").compact_relay;
    assert_eq!(
        compact.capability,
        CompactRelayCapability::Supported { version: 2 }
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(compact.maybe_unsupported_version, Some(1));
}

#[test]
fn phase113_transaction_relay_messages_do_not_activate_compact_relay_state() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_inbound_peer(113_007).expect("peer");
    let transaction = open_bitcoin_primitives::Transaction::default();
    let txid = transaction_txid(&transaction).expect("txid");

    // Act
    let wtxidrelay_actions = manager
        .handle_message(113_007, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay should process");
    let inventory_actions = manager
        .handle_message(
            113_007,
            WireNetworkMessage::Inv(InventoryList::new(vec![InventoryVector {
                inventory_type: InventoryType::Transaction,
                object_hash: txid.into(),
            }])),
            2,
        )
        .expect("transaction inventory should process");

    // Assert
    assert!(wtxidrelay_actions.is_empty());
    assert!(!inventory_actions.iter().any(|action| {
        matches!(
            action,
            PeerAction::Send(WireNetworkMessage::CompactBlock(_))
        )
    }));
    let peer = manager.peer_state(113_007).expect("peer");
    assert!(peer.remote_wtxidrelay);
    assert_eq!(
        peer.compact_relay.capability,
        CompactRelayCapability::Unknown
    );
    assert_eq!(
        peer.compact_relay.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn phase113_block_serving_enabled_without_compact_relay_does_not_announce_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_042, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_042);
    let input = PeerCompactAnnouncementInput {
        activation: BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig::default(),
        },
        peer_has_previous_header: true,
        peer_has_current_header: false,
        status: compact_available_status(),
        resource_gate: compact_available_resource_gate(),
    };

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_042, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
}

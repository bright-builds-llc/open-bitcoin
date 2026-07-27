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
fn phase128_compact_relay_local_offer_defaults_to_not_offered() {
    // Arrange / Act
    let state = LocalCompactRelayOfferState::default();

    // Assert
    assert_eq!(state, LocalCompactRelayOfferState::NotOffered);
}

#[test]
fn phase128_compact_relay_local_offer_schedules_exact_version2_low_bandwidth_message() {
    // Arrange
    let mut state = LocalCompactRelayOfferState::default();

    // Act
    let maybe_message = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(true),
        true,
        Some(BIP152_MIN_PROTOCOL_VERSION),
    );

    // Assert
    assert_eq!(
        maybe_message,
        Some(SendCompactMessage {
            announce: false,
            version: 2,
        })
    );
    assert_eq!(state, LocalCompactRelayOfferState::Scheduled { version: 2 });
}

#[test]
fn phase128_compact_relay_local_offer_is_one_shot() {
    // Arrange
    let mut state = LocalCompactRelayOfferState::default();
    let first = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(true),
        true,
        Some(BIP152_MIN_PROTOCOL_VERSION),
    );

    // Act
    let second = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(true),
        true,
        Some(BIP152_MIN_PROTOCOL_VERSION),
    );

    // Assert
    assert!(first.is_some());
    assert_eq!(second, None);
    assert_eq!(state, LocalCompactRelayOfferState::Scheduled { version: 2 });
}

#[test]
fn phase128_compact_relay_local_offer_fails_closed_when_activation_is_disabled() {
    // Arrange
    let mut state = LocalCompactRelayOfferState::default();

    // Act
    let maybe_message = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(false),
        true,
        Some(BIP152_MIN_PROTOCOL_VERSION),
    );

    // Assert
    assert_eq!(maybe_message, None);
    assert_eq!(state, LocalCompactRelayOfferState::NotOffered);
}

#[test]
fn phase128_compact_relay_local_offer_fails_closed_before_established_handshake() {
    // Arrange
    let mut state = LocalCompactRelayOfferState::default();

    // Act
    let maybe_message = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(true),
        false,
        Some(BIP152_MIN_PROTOCOL_VERSION),
    );

    // Assert
    assert_eq!(maybe_message, None);
    assert_eq!(state, LocalCompactRelayOfferState::NotOffered);
}

#[test]
fn phase128_compact_relay_local_offer_fails_closed_for_unsupported_protocol() {
    // Arrange
    let mut state = LocalCompactRelayOfferState::default();

    // Act
    let maybe_message = maybe_schedule_local_compact_offer(
        &mut state,
        compact_announcement_activation(true),
        true,
        Some(BIP152_MIN_PROTOCOL_VERSION - 1),
    );

    // Assert
    assert_eq!(maybe_message, None);
    assert_eq!(state, LocalCompactRelayOfferState::NotOffered);
}

#[test]
fn phase128_enabled_handshake_appends_one_low_bandwidth_offer_after_existing_verack_action() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    manager.add_outbound_peer(128_001, 10).expect("peer");
    manager
        .handle_message(
            128_001,
            WireNetworkMessage::Version(crate::VersionMessage {
                start_height: 3,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");

    // Act
    let actions = manager
        .handle_message(128_001, WireNetworkMessage::Verack, 12)
        .expect("verack");

    // Assert
    assert!(matches!(
        actions.first(),
        Some(PeerAction::Send(WireNetworkMessage::GetHeaders { .. }))
    ));
    assert_eq!(
        actions.last(),
        Some(&PeerAction::Send(WireNetworkMessage::SendCompact(
            SendCompactMessage {
                announce: false,
                version: 2,
            }
        )))
    );
    assert_eq!(
        actions
            .iter()
            .filter(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_))))
            .count(),
        1
    );
}

#[test]
fn phase128_duplicate_verack_does_not_repeat_local_compact_offer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    manager.add_outbound_peer(128_002, 10).expect("peer");
    manager
        .handle_message(
            128_002,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            11,
        )
        .expect("version");
    let first = manager
        .handle_message(128_002, WireNetworkMessage::Verack, 12)
        .expect("first verack");

    // Act
    let duplicate = manager
        .handle_message(128_002, WireNetworkMessage::Verack, 13)
        .expect("duplicate verack");

    // Assert
    assert_eq!(
        first
            .iter()
            .filter(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_))))
            .count(),
        1
    );
    assert!(duplicate.is_empty());
}

#[test]
fn phase128_disabled_handshake_does_not_emit_local_compact_offer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(128_003, 10).expect("peer");
    manager
        .handle_message(
            128_003,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            11,
        )
        .expect("version");

    // Act
    let actions = manager
        .handle_message(128_003, WireNetworkMessage::Verack, 12)
        .expect("verack");

    // Assert
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_)))));
}

#[test]
fn phase128_unsupported_protocol_handshake_does_not_emit_local_compact_offer() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    manager.add_outbound_peer(128_004, 10).expect("peer");
    manager
        .handle_message(
            128_004,
            WireNetworkMessage::Version(crate::VersionMessage {
                version: BIP152_MIN_PROTOCOL_VERSION - 1,
                ..crate::VersionMessage::default()
            }),
            11,
        )
        .expect("version");

    // Act
    let actions = manager
        .handle_message(128_004, WireNetworkMessage::Verack, 12)
        .expect("verack");

    // Assert
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_)))));
}

#[test]
fn phase128_local_offer_preserves_remote_high_then_low_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    manager.add_outbound_peer(128_005, 10).expect("peer");
    manager
        .handle_message(
            128_005,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            11,
        )
        .expect("version");
    process_high_bandwidth_sendcmpct(&mut manager, 128_005);

    // Act
    let actions = manager
        .handle_message(128_005, WireNetworkMessage::Verack, 12)
        .expect("verack");
    process_low_bandwidth_sendcmpct(&mut manager, 128_005);

    // Assert
    assert!(actions
        .iter()
        .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_)))));
    let state = manager.peer_state(128_005).expect("peer");
    assert_eq!(
        state.local_compact_relay_offer,
        LocalCompactRelayOfferState::Scheduled { version: 2 }
    );
    assert_eq!(
        state.compact_relay.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.compact_relay.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase128_local_offer_preserves_remote_low_then_high_preference() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.set_block_relay_activation_policy(compact_announcement_activation(true));
    manager.add_outbound_peer(128_006, 10).expect("peer");
    manager
        .handle_message(
            128_006,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            11,
        )
        .expect("version");
    process_low_bandwidth_sendcmpct(&mut manager, 128_006);

    // Act
    let actions = manager
        .handle_message(128_006, WireNetworkMessage::Verack, 12)
        .expect("verack");
    process_high_bandwidth_sendcmpct(&mut manager, 128_006);

    // Assert
    assert!(actions
        .iter()
        .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_)))));
    let state = manager.peer_state(128_006).expect("peer");
    assert_eq!(
        state.local_compact_relay_offer,
        LocalCompactRelayOfferState::Scheduled { version: 2 }
    );
    assert_eq!(
        state.compact_relay.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.compact_relay.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
}

#[test]
fn phase128_transaction_relay_activation_does_not_enable_local_compact_offer() {
    // Arrange
    let mut manager = relay_download_manager(true);
    manager.add_outbound_peer(128_007, 10).expect("peer");
    manager
        .handle_message(
            128_007,
            WireNetworkMessage::Version(crate::VersionMessage::default()),
            11,
        )
        .expect("version");

    // Act
    let actions = manager
        .handle_message(128_007, WireNetworkMessage::Verack, 12)
        .expect("verack");

    // Assert
    assert!(manager.relay_download_policy.activation.enabled);
    assert!(
        !manager
            .block_relay_activation_policy()
            .compact_relay
            .enabled
    );
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Send(WireNetworkMessage::SendCompact(_)))));
}

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
fn phase113_high_low_high_toggle_refreshes_recorded_eligibility() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_035, 0).expect("peer");
    let input = || {
        compact_announcement_input(
            true,
            true,
            false,
            compact_available_status(),
            compact_available_resource_gate(),
        )
    };

    // Act
    process_high_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("high-bandwidth decision");
    let high_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;
    process_low_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("low-bandwidth decision");
    let low_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;
    process_high_bandwidth_sendcmpct(&mut manager, 113_035);
    manager
        .decide_compact_announcement_for_peer(113_035, input())
        .expect("restored high-bandwidth decision");
    let restored_eligibility = manager
        .peer_state(113_035)
        .expect("peer")
        .compact_relay
        .announcement_eligibility;

    // Assert
    assert_eq!(high_eligibility, CompactAnnouncementEligibility::Eligible);
    assert_eq!(
        low_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
        }
    );
    assert_eq!(
        restored_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_unsupported_compact_version_without_supported_preference_uses_inventory_fallback_without_disconnect(
) {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_036, 0).expect("peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(
            113_036,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: true,
                version: 3,
            }),
            1,
        )
        .expect("unsupported sendcmpct should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_036, input)
        .expect("compact announcement decision");

    // Assert
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Disconnect(_))));
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(decision.reason.as_str(), "compact_unsupported_version");
}

#[test]
fn phase113_unsupported_compact_version_after_supported_high_bandwidth_still_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_037, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_037);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(
            113_037,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_037, input)
        .expect("compact announcement decision");

    // Assert
    assert!(!actions
        .iter()
        .any(|action| matches!(action, PeerAction::Disconnect(_))));
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_037).expect("peer").compact_relay;
    assert_eq!(compact.maybe_unsupported_version, Some(1));
}

#[test]
fn phase113_peer_already_has_current_header_uses_headers_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_038, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_038);
    let input = compact_announcement_input(
        true,
        true,
        true,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_038, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(decision.reason.as_str(), "compact_peer_already_has_header");
}

#[test]
fn phase113_missing_header_or_unavailable_block_never_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_039, 0).expect("peer");
    manager.add_outbound_peer(113_040, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_039);
    process_high_bandwidth_sendcmpct(&mut manager, 113_040);
    let missing_header_input = compact_announcement_input(
        true,
        false,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );
    let unavailable_block_input = compact_announcement_input(
        true,
        true,
        false,
        compact_unavailable_status(),
        compact_available_resource_gate(),
    );

    // Act
    let missing_header_decision = manager
        .decide_compact_announcement_for_peer(113_039, missing_header_input)
        .expect("missing-header compact announcement decision");
    let unavailable_block_decision = manager
        .decide_compact_announcement_for_peer(113_040, unavailable_block_input)
        .expect("unavailable-block compact announcement decision");

    // Assert
    assert_ne!(
        missing_header_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        missing_header_decision.reason.as_str(),
        "compact_header_continuity_missing"
    );
    assert_ne!(
        unavailable_block_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        unavailable_block_decision.action,
        CompactAnnouncementAction::Suppress
    );
    assert_eq!(
        unavailable_block_decision.reason.as_str(),
        "compact_block_unavailable"
    );
}

#[test]
fn phase113_wtxidrelay_does_not_activate_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_041, 0).expect("peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let actions = manager
        .handle_message(113_041, WireNetworkMessage::WtxidRelay, 1)
        .expect("wtxidrelay should process");
    let decision = manager
        .decide_compact_announcement_for_peer(113_041, input)
        .expect("compact announcement decision");

    // Assert
    assert!(actions.is_empty());
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
    assert_eq!(
        manager
            .peer_state(113_041)
            .expect("peer")
            .compact_relay
            .capability,
        CompactRelayCapability::Unknown
    );
}

#[test]
fn phase113_download_permission_does_not_grant_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            113_043,
            permission_decision(["in", "download"]),
        ))
        .expect("download-permission inbound peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_043, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
}

#[test]
fn phase113_protected_inbound_permission_does_not_grant_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager
        .add_inbound_peer_record(permissioned_inbound_record(
            113_044,
            protected_permission_decision(),
        ))
        .expect("protected inbound peer");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_044, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_peer_not_negotiated");
}

#[test]
fn phase113_default_activation_policy_suppresses_compact_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_045, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_045);
    let input = PeerCompactAnnouncementInput {
        activation: BlockRelayActivationPolicy::default(),
        peer_has_previous_header: true,
        peer_has_current_header: false,
        status: compact_available_status(),
        resource_gate: compact_available_resource_gate(),
    };

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_045, input)
        .expect("compact announcement decision");

    // Assert
    assert!(!BlockRelayActivationPolicy::default().compact_relay.enabled);
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
}

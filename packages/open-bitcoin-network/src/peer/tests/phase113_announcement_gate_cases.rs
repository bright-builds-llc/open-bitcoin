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
fn phase122_compact_announcement_provenance_is_idempotent_and_bounded() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    let peer_id = 122_001;
    manager.add_outbound_peer(peer_id, 0).expect("peer");
    assert!(
        manager
            .peer_state(peer_id)
            .expect("peer")
            .compact_announcements
            .is_empty()
    );
    let hashes = (0..=super::super::MAX_COMPACT_ANNOUNCEMENT_PROVENANCE)
        .map(|index| BlockHash::from_byte_array([index as u8; 32]))
        .collect::<Vec<_>>();

    // Act
    manager
        .record_compact_block_announcement(peer_id, hashes[0])
        .expect("first record");
    manager
        .record_compact_block_announcement(peer_id, hashes[0])
        .expect("duplicate record");
    for block_hash in hashes.iter().copied().skip(1) {
        manager
            .record_compact_block_announcement(peer_id, block_hash)
            .expect("bounded record");
    }

    // Assert
    let provenance = &manager
        .peer_state(peer_id)
        .expect("peer")
        .compact_announcements;
    assert_eq!(
        provenance.len(),
        super::super::MAX_COMPACT_ANNOUNCEMENT_PROVENANCE
    );
    assert!(!provenance.contains(&hashes[0]));
    assert!(provenance.contains(hashes.last().expect("last hash")));
}

#[test]
fn phase113_compact_announcement_all_gates_allow_compact_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_021, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_021);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_021, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    assert_eq!(
        manager
            .peer_state(113_021)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_compact_announcement_disabled_local_activation_uses_inventory_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_022, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_022);
    let input = compact_announcement_input(
        false,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_022, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(decision.reason.as_str(), "compact_relay_disabled");
    assert_eq!(
        manager
            .peer_state(113_022)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::LocalActivationDisabled,
        }
    );
}

#[test]
fn phase113_compact_announcement_missing_previous_header_uses_headers_fallback() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_023, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_023);
    let input = compact_announcement_input(
        true,
        false,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_023, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(
        decision.reason.as_str(),
        "compact_header_continuity_missing"
    );
    assert_eq!(
        manager
            .peer_state(113_023)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HeaderContinuityMissing,
        }
    );
}

#[test]
fn phase113_compact_announcement_unavailable_block_suppresses() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_024, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_024);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_unavailable_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_024, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::Suppress);
    assert_eq!(decision.reason.as_str(), "compact_block_unavailable");
    assert_eq!(
        manager
            .peer_state(113_024)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::BlockUnavailable,
        }
    );
}

#[test]
fn phase113_compact_announcement_resource_limit_suppresses() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_025, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_025);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_limited_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_025, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::Suppress);
    assert_eq!(decision.reason.as_str(), "compact_resource_limited");
    assert_eq!(
        manager
            .peer_state(113_025)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::ResourceLimited,
        }
    );
}

#[test]
fn phase113_compact_announcement_refreshes_eligibility_across_high_low_high_toggles() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_026, 0).expect("peer");
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
    process_high_bandwidth_sendcmpct(&mut manager, 113_026);
    let high_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("high bandwidth decision");
    process_low_bandwidth_sendcmpct(&mut manager, 113_026);
    let low_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("low bandwidth decision");
    process_high_bandwidth_sendcmpct(&mut manager, 113_026);
    let restored_high_decision = manager
        .decide_compact_announcement_for_peer(113_026, input())
        .expect("restored high bandwidth decision");

    // Assert
    assert_eq!(
        high_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        low_decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    assert_eq!(
        low_decision.eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
        }
    );
    assert_eq!(
        restored_high_decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        manager
            .peer_state(113_026)
            .expect("peer")
            .compact_relay
            .announcement_eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn phase113_compact_announcement_preserves_supported_preference_after_unsupported_sendcmpct() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_027, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_027);
    manager
        .handle_message(
            113_027,
            WireNetworkMessage::SendCompact(SendCompactMessage {
                announce: false,
                version: 1,
            }),
            2,
        )
        .expect("unsupported sendcmpct should process");
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_027, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_027).expect("peer").compact_relay;
    assert_eq!(compact.maybe_unsupported_version, Some(1));
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_low_bandwidth_compact_peer_uses_headers_fallback_for_direct_announcement() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_031, 0).expect("peer");
    manager
        .handle_message(113_031, WireNetworkMessage::SendHeaders, 1)
        .expect("sendheaders should process");
    process_low_bandwidth_sendcmpct(&mut manager, 113_031);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_031, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_031).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
}

#[test]
fn phase113_low_bandwidth_compact_peer_uses_inventory_fallback_without_sendheaders() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_032, 0).expect("peer");
    process_low_bandwidth_sendcmpct(&mut manager, 113_032);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_032, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_032).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_high_to_low_toggle_never_announces_compact() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_033, 0).expect("peer");
    process_high_bandwidth_sendcmpct(&mut manager, 113_033);
    process_low_bandwidth_sendcmpct(&mut manager, 113_033);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_033, input)
        .expect("compact announcement decision");

    // Assert
    assert_ne!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(
        decision.reason.as_str(),
        "compact_high_bandwidth_not_requested"
    );
    let compact = &manager.peer_state(113_033).expect("peer").compact_relay;
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

#[test]
fn phase113_low_to_high_toggle_all_gates_allow_compact_block() {
    // Arrange
    let mut manager = PeerManager::new(local_config());
    manager.add_outbound_peer(113_034, 0).expect("peer");
    process_low_bandwidth_sendcmpct(&mut manager, 113_034);
    process_high_bandwidth_sendcmpct(&mut manager, 113_034);
    let input = compact_announcement_input(
        true,
        true,
        false,
        compact_available_status(),
        compact_available_resource_gate(),
    );

    // Act
    let decision = manager
        .decide_compact_announcement_for_peer(113_034, input)
        .expect("compact announcement decision");

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason.as_str(), "compact_announced");
    let compact = &manager.peer_state(113_034).expect("peer").compact_relay;
    assert_eq!(
        compact.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        compact.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
}

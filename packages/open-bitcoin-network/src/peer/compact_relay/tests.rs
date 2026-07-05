// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/test/functional/p2p_compactblocks.py

use super::*;
use crate::block_serving::{
    BlockServingOutcomeLabel, BlockServingResourceGateDecision, BlockServingStatusDecision,
    BlockServingStatusLabel, CompactRelayActivationConfig,
};

#[test]
fn compact_relay_default_state_is_unknown() {
    // Arrange / Act
    let state = CompactRelayPeerState::default();

    // Assert
    assert_eq!(state.capability, CompactRelayCapability::Unknown);
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(state.maybe_unsupported_version, None);
}

#[test]
fn compact_relay_version2_high_bandwidth_requests_only_high_bandwidth() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    let outcome = state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });

    // Assert
    assert_eq!(
        state.capability,
        CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }
    );
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(
        outcome.reason,
        CompactRelayNegotiationReason::Version2HighBandwidth
    );
}

#[test]
fn compact_relay_version2_low_bandwidth_requests_only_low_bandwidth() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    let outcome = state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });

    // Assert
    assert_eq!(
        state.capability,
        CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }
    );
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(
        outcome.reason,
        CompactRelayNegotiationReason::Version2LowBandwidth
    );
}

#[test]
fn compact_relay_high_to_low_uses_last_supported_preference() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });
    state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });

    // Assert
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn compact_relay_low_to_high_uses_last_supported_preference() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });
    state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });

    // Assert
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
}

#[test]
fn compact_relay_unsupported_before_supported_records_evidence_only() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    let outcome = state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: 3,
    });

    // Assert
    assert_eq!(
        state.capability,
        CompactRelayCapability::Unsupported { version: 3 }
    );
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::Unknown
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(state.maybe_unsupported_version, Some(3));
    assert_eq!(
        outcome.reason,
        CompactRelayNegotiationReason::UnsupportedVersion
    );
}

#[test]
fn compact_relay_unsupported_after_supported_preserves_version2_preference() {
    // Arrange
    let mut state = CompactRelayPeerState::default();

    // Act
    state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });
    state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: 1,
    });

    // Assert
    assert_eq!(
        state.capability,
        CompactRelayCapability::Supported {
            version: BIP152_COMPACT_BLOCKS_VERSION,
        }
    );
    assert_eq!(
        state.high_bandwidth_preference,
        CompactRelayPreference::Requested
    );
    assert_eq!(
        state.low_bandwidth_preference,
        CompactRelayPreference::NotRequested
    );
    assert_eq!(
        state.announcement_eligibility,
        CompactAnnouncementEligibility::Unknown
    );
    assert_eq!(state.maybe_unsupported_version, Some(1));
}

#[test]
fn compact_announcement_all_gates_allow_compact_block() {
    // Arrange
    let input = compact_announcement_input(
        compact_activation(true),
        supported_high_bandwidth_state(),
        false,
        true,
        false,
        available_status(),
        available_resource_gate(),
    );

    // Act
    let decision = decide_compact_announcement(input);

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason, CompactAnnouncementReason::CompactAnnounced);
    assert_eq!(
        decision.eligibility,
        CompactAnnouncementEligibility::Eligible
    );
}

#[test]
fn compact_announcement_gate_reasons_map_to_ineligible_reasons() {
    // Arrange
    let cases = [
        (
            CompactAnnouncementReason::CompactRelayDisabled,
            CompactAnnouncementEligibilityReason::LocalActivationDisabled,
            "compact_relay_disabled",
        ),
        (
            CompactAnnouncementReason::CompactPeerNotNegotiated,
            CompactAnnouncementEligibilityReason::PeerNotNegotiated,
            "compact_peer_not_negotiated",
        ),
        (
            CompactAnnouncementReason::CompactUnsupportedVersion,
            CompactAnnouncementEligibilityReason::UnsupportedVersion,
            "compact_unsupported_version",
        ),
        (
            CompactAnnouncementReason::CompactHighBandwidthNotRequested,
            CompactAnnouncementEligibilityReason::HighBandwidthNotRequested,
            "compact_high_bandwidth_not_requested",
        ),
        (
            CompactAnnouncementReason::CompactHeaderContinuityMissing,
            CompactAnnouncementEligibilityReason::HeaderContinuityMissing,
            "compact_header_continuity_missing",
        ),
        (
            CompactAnnouncementReason::CompactPeerAlreadyHasHeader,
            CompactAnnouncementEligibilityReason::PeerAlreadyHasHeader,
            "compact_peer_already_has_header",
        ),
        (
            CompactAnnouncementReason::CompactBlockUnavailable,
            CompactAnnouncementEligibilityReason::BlockUnavailable,
            "compact_block_unavailable",
        ),
        (
            CompactAnnouncementReason::CompactResourceLimited,
            CompactAnnouncementEligibilityReason::ResourceLimited,
            "compact_resource_limited",
        ),
        (
            CompactAnnouncementReason::CompactHeadersFallback,
            CompactAnnouncementEligibilityReason::PeerNotNegotiated,
            "compact_headers_fallback",
        ),
        (
            CompactAnnouncementReason::CompactInventoryFallback,
            CompactAnnouncementEligibilityReason::PeerNotNegotiated,
            "compact_inventory_fallback",
        ),
    ];

    for (reason, expected_reason, expected_label) in cases {
        // Act
        let decision =
            CompactAnnouncementDecision::new(CompactAnnouncementAction::Suppress, reason);

        // Assert
        assert_eq!(decision.reason.as_str(), expected_label);
        assert_eq!(
            decision.eligibility,
            CompactAnnouncementEligibility::Ineligible {
                reason: expected_reason,
            }
        );
    }
}

#[test]
fn compact_announcement_unsupported_without_supported_preference_uses_unsupported_reason() {
    // Arrange
    let mut peer_state = CompactRelayPeerState::default();
    peer_state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: 3,
    });
    let input = compact_announcement_input(
        compact_activation(true),
        peer_state,
        false,
        true,
        false,
        available_status(),
        available_resource_gate(),
    );

    // Act
    let decision = decide_compact_announcement(input);

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(
        decision.reason,
        CompactAnnouncementReason::CompactUnsupportedVersion
    );
    assert_eq!(decision.reason.as_str(), "compact_unsupported_version");
}

#[test]
fn compact_announcement_rejects_non_v2_supported_capability() {
    // Arrange
    let peer_state = CompactRelayPeerState {
        capability: CompactRelayCapability::Supported { version: 3 },
        high_bandwidth_preference: CompactRelayPreference::Requested,
        low_bandwidth_preference: CompactRelayPreference::NotRequested,
        announcement_eligibility: CompactAnnouncementEligibility::Unknown,
        maybe_unsupported_version: None,
    };
    let input = compact_announcement_input(
        compact_activation(true),
        peer_state,
        false,
        true,
        false,
        available_status(),
        available_resource_gate(),
    );

    // Act
    let decision = decide_compact_announcement(input);

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceInventory
    );
    assert_eq!(
        decision.reason,
        CompactAnnouncementReason::CompactUnsupportedVersion
    );
    assert_eq!(
        decision.eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::UnsupportedVersion
        }
    );
}

#[test]
fn compact_announcement_supported_preference_overrides_later_unsupported_evidence() {
    // Arrange
    let mut peer_state = supported_high_bandwidth_state();
    peer_state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: 1,
    });
    let input = compact_announcement_input(
        compact_activation(true),
        peer_state,
        false,
        true,
        false,
        available_status(),
        available_resource_gate(),
    );

    // Act
    let decision = decide_compact_announcement(input);

    // Assert
    assert_eq!(
        decision.action,
        CompactAnnouncementAction::AnnounceCompactBlock
    );
    assert_eq!(decision.reason, CompactAnnouncementReason::CompactAnnounced);
    assert_eq!(peer_state.maybe_unsupported_version, Some(1));
}

#[test]
fn compact_announcement_gate_order_prefers_high_bandwidth_before_header_checks() {
    // Arrange
    let mut peer_state = supported_high_bandwidth_state();
    peer_state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });
    let input = compact_announcement_input(
        compact_activation(true),
        peer_state,
        true,
        false,
        false,
        available_status(),
        available_resource_gate(),
    );

    // Act
    let decision = decide_compact_announcement(input);

    // Assert
    assert_eq!(decision.action, CompactAnnouncementAction::AnnounceHeaders);
    assert_eq!(
        decision.reason,
        CompactAnnouncementReason::CompactHighBandwidthNotRequested
    );
}

#[test]
fn compact_announcement_status_and_resource_gates_suppress() {
    // Arrange
    let unavailable_input = compact_announcement_input(
        compact_activation(true),
        supported_high_bandwidth_state(),
        false,
        true,
        false,
        unavailable_status(),
        available_resource_gate(),
    );
    let limited_input = compact_announcement_input(
        compact_activation(true),
        supported_high_bandwidth_state(),
        false,
        true,
        false,
        available_status(),
        limited_resource_gate(),
    );

    // Act
    let unavailable_decision = decide_compact_announcement(unavailable_input);
    let limited_decision = decide_compact_announcement(limited_input);

    // Assert
    assert_eq!(
        unavailable_decision.action,
        CompactAnnouncementAction::Suppress
    );
    assert_eq!(
        unavailable_decision.reason,
        CompactAnnouncementReason::CompactBlockUnavailable
    );
    assert_eq!(limited_decision.action, CompactAnnouncementAction::Suppress);
    assert_eq!(
        limited_decision.reason,
        CompactAnnouncementReason::CompactResourceLimited
    );
}

#[test]
fn compact_announcement_record_decision_updates_only_eligibility() {
    // Arrange
    let mut peer_state = supported_high_bandwidth_state();
    peer_state.apply_send_compact(SendCompactMessage {
        announce: false,
        version: 1,
    });
    let before = peer_state;
    let decision = CompactAnnouncementDecision::new(
        CompactAnnouncementAction::Suppress,
        CompactAnnouncementReason::CompactResourceLimited,
    );

    // Act
    peer_state.record_announcement_decision(&decision);

    // Assert
    assert_eq!(
        peer_state.announcement_eligibility,
        CompactAnnouncementEligibility::Ineligible {
            reason: CompactAnnouncementEligibilityReason::ResourceLimited,
        }
    );
    assert_eq!(peer_state.capability, before.capability);
    assert_eq!(
        peer_state.high_bandwidth_preference,
        before.high_bandwidth_preference
    );
    assert_eq!(
        peer_state.low_bandwidth_preference,
        before.low_bandwidth_preference
    );
    assert_eq!(
        peer_state.maybe_unsupported_version,
        before.maybe_unsupported_version
    );
}

fn compact_activation(enabled: bool) -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        compact_relay: CompactRelayActivationConfig { enabled },
        ..BlockRelayActivationPolicy::default()
    }
}

fn supported_high_bandwidth_state() -> CompactRelayPeerState {
    let mut state = CompactRelayPeerState::default();
    state.apply_send_compact(SendCompactMessage {
        announce: true,
        version: BIP152_COMPACT_BLOCKS_VERSION,
    });
    state
}

fn compact_announcement_input(
    activation: BlockRelayActivationPolicy,
    peer_state: CompactRelayPeerState,
    peer_prefers_headers: bool,
    peer_has_previous_header: bool,
    peer_has_current_header: bool,
    status: BlockServingStatusDecision,
    resource_gate: BlockServingResourceGateDecision,
) -> CompactAnnouncementInput {
    CompactAnnouncementInput {
        activation,
        peer_state,
        peer_prefers_headers,
        peer_has_previous_header,
        peer_has_current_header,
        status,
        resource_gate,
    }
}

fn available_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Available,
        allow_storage_read: true,
        may_serve_block: true,
    }
}

fn unavailable_status() -> BlockServingStatusDecision {
    BlockServingStatusDecision {
        label: BlockServingStatusLabel::Unavailable,
        allow_storage_read: false,
        may_serve_block: false,
    }
}

fn available_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        allow_storage_read: true,
        may_serve_block: true,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

fn limited_resource_gate() -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockRequestCapReached,
        allow_storage_read: false,
        may_serve_block: false,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

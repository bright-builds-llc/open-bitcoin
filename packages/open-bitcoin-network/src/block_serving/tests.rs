// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;
use crate::{
    INBOUND_PERMISSION_TOKENS_FIELD, InactivePermissionEffectLabel, LocalPeerConfig,
    PeerConnectionClass, PeerPermissionSet, PermissionEffectLabel, ServiceFlags,
};

fn activated_policy() -> BlockRelayActivationPolicy {
    BlockRelayActivationPolicy {
        block_serving: BlockServingActivationConfig { enabled: true },
        compact_relay: CompactRelayActivationConfig::default(),
    }
}

fn input(connection_class: PeerConnectionClass) -> BlockServingEligibilityInput {
    BlockServingEligibilityInput {
        activation: activated_policy(),
        inbound_serving_enabled: true,
        connection_class,
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        status_available: true,
    }
}

fn classify(input: BlockServingEligibilityInput) -> BlockServingEligibilityDecision {
    classify_block_serving_eligibility(&input)
}

fn status_facts(
    chain_position: BlockServingChainPosition,
    validation_state: BlockServingValidationState,
    data_availability: BlockServingDataAvailability,
) -> BlockServingStatusFacts {
    BlockServingStatusFacts {
        chain_position,
        validation_state,
        data_availability,
        suppressed: false,
    }
}

#[test]
fn block_serving_and_compact_relay_default_to_disabled() {
    // Arrange
    let block_serving = BlockServingActivationConfig::default();
    let compact_relay = CompactRelayActivationConfig::default();
    let policy = BlockRelayActivationPolicy::default();
    let input = BlockServingEligibilityInput {
        activation: policy,
        inbound_serving_enabled: true,
        connection_class: PeerConnectionClass::Outbound,
        active_permission_effects: Vec::new(),
        inactive_permission_effects: Vec::new(),
        status_available: true,
    };

    // Act
    let decision = classify(input);

    // Assert
    assert!(!block_serving.enabled);
    assert!(!compact_relay.enabled);
    assert!(!policy.block_serving.enabled);
    assert!(!policy.compact_relay.enabled);
    assert_eq!(decision.reason, BlockServingEligibilityReason::Disabled);
    assert_eq!(decision.reason.as_str(), "disabled");
    assert!(!decision.eligible);
    assert!(!decision.advertises_public_service);
}

#[test]
fn outbound_and_manual_peers_require_activation_and_available_status() {
    // Arrange
    let cases = [
        PeerConnectionClass::Outbound,
        PeerConnectionClass::ManualConfigured,
    ];

    // Act
    let decisions: Vec<_> = cases
        .into_iter()
        .map(|connection_class| {
            let disabled = BlockServingEligibilityInput {
                activation: BlockRelayActivationPolicy::default(),
                inbound_serving_enabled: true,
                connection_class,
                active_permission_effects: Vec::new(),
                inactive_permission_effects: Vec::new(),
                status_available: true,
            };
            let unavailable = BlockServingEligibilityInput {
                activation: activated_policy(),
                inbound_serving_enabled: true,
                connection_class,
                active_permission_effects: Vec::new(),
                inactive_permission_effects: Vec::new(),
                status_available: false,
            };
            let eligible = input(connection_class);

            (
                classify(disabled),
                classify(unavailable),
                classify(eligible),
            )
        })
        .collect();

    // Assert
    for (disabled, unavailable, eligible) in decisions {
        assert_eq!(disabled.reason, BlockServingEligibilityReason::Disabled);
        assert_eq!(
            unavailable.reason,
            BlockServingEligibilityReason::StatusUnavailable,
        );
        assert_eq!(eligible.reason, BlockServingEligibilityReason::Eligible);
        assert!(!disabled.eligible);
        assert!(!unavailable.eligible);
        assert!(eligible.eligible);
        assert!(!eligible.advertises_public_service);
    }
}

#[test]
fn inbound_peers_require_scoped_download_serving_permission() {
    // Arrange
    let ordinary = input(PeerConnectionClass::OrdinaryInbound);
    let protected = BlockServingEligibilityInput {
        active_permission_effects: vec![PermissionEffectLabel::AdmissionProtected],
        ..input(PeerConnectionClass::ProtectedInbound)
    };
    let no_inbound_serving = BlockServingEligibilityInput {
        inbound_serving_enabled: false,
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        ..input(PeerConnectionClass::PermissionedInbound)
    };
    let inactive_filter = BlockServingEligibilityInput {
        inactive_permission_effects: vec![InactivePermissionEffectLabel::BlockFilters],
        ..input(PeerConnectionClass::PermissionedInbound)
    };
    let unavailable = BlockServingEligibilityInput {
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        status_available: false,
        ..input(PeerConnectionClass::PermissionedInbound)
    };
    let eligible = BlockServingEligibilityInput {
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        ..input(PeerConnectionClass::PermissionedInbound)
    };

    // Act
    let ordinary_decision = classify(ordinary);
    let protected_decision = classify(protected);
    let no_inbound_serving_decision = classify(no_inbound_serving);
    let inactive_filter_decision = classify(inactive_filter);
    let unavailable_decision = classify(unavailable);
    let eligible_decision = classify(eligible);

    // Assert
    assert_eq!(
        ordinary_decision.reason,
        BlockServingEligibilityReason::PermissionRequired,
    );
    assert_eq!(
        protected_decision.reason,
        BlockServingEligibilityReason::ProtectedNotServing,
    );
    assert_eq!(protected_decision.reason.as_str(), "protected_not_serving");
    assert_eq!(
        no_inbound_serving_decision.reason,
        BlockServingEligibilityReason::InboundServingRequired,
    );
    assert_eq!(
        inactive_filter_decision.reason,
        BlockServingEligibilityReason::PermissionEffectInactive,
    );
    assert_eq!(
        inactive_filter_decision.reason.as_str(),
        "permission_effect_inactive",
    );
    assert_eq!(
        unavailable_decision.reason,
        BlockServingEligibilityReason::StatusUnavailable,
    );
    assert_eq!(unavailable_decision.reason.as_str(), "status_unavailable");
    assert_eq!(
        eligible_decision.reason,
        BlockServingEligibilityReason::Eligible,
    );
    assert!(!ordinary_decision.eligible);
    assert!(!protected_decision.eligible);
    assert!(!no_inbound_serving_decision.eligible);
    assert!(!inactive_filter_decision.eligible);
    assert!(!unavailable_decision.eligible);
    assert!(eligible_decision.eligible);
}

#[test]
fn permissioned_inbound_without_download_reports_inactive_effects() {
    // Arrange
    let input = BlockServingEligibilityInput {
        inactive_permission_effects: vec![InactivePermissionEffectLabel::BlockFilters],
        ..input(PeerConnectionClass::PermissionedInbound)
    };

    // Act
    let decision = classify(input);

    // Assert
    assert_eq!(
        decision.reason,
        BlockServingEligibilityReason::PermissionEffectInactive,
    );
    assert!(!decision.eligible);
}

#[test]
fn permissioned_inbound_without_download_requires_permission() {
    // Arrange
    let input = input(PeerConnectionClass::PermissionedInbound);

    // Act
    let decision = classify(input);

    // Assert
    assert_eq!(
        decision.reason,
        BlockServingEligibilityReason::PermissionRequired,
    );
    assert!(!decision.eligible);
}

#[test]
fn permission_expansions_do_not_activate_serving_or_compact_relay() {
    // Arrange
    let permission_sets = ["download", "noban", "forceinbound", "all"]
        .into_iter()
        .map(|permission| {
            PeerPermissionSet::parse(INBOUND_PERMISSION_TOKENS_FIELD, ["in", permission])
        })
        .collect::<Result<Vec<_>, _>>();
    assert!(
        permission_sets.is_ok(),
        "test permission tokens should parse"
    );
    let permission_sets = permission_sets.unwrap_or_default();

    // Act
    let decisions: Vec<_> = permission_sets
        .into_iter()
        .map(|permission_set| {
            let policy = BlockRelayActivationPolicy::default();
            let input = BlockServingEligibilityInput {
                activation: policy,
                inbound_serving_enabled: true,
                connection_class: PeerConnectionClass::PermissionedInbound,
                active_permission_effects: permission_set.active_effects(),
                inactive_permission_effects: permission_set.inactive_effects(),
                status_available: true,
            };

            (policy, classify(input))
        })
        .collect();

    // Assert
    for (policy, decision) in decisions {
        assert!(!policy.block_serving.enabled);
        assert!(!policy.compact_relay.enabled);
        assert_eq!(
            decision.reason,
            BlockServingEligibilityReason::ActivationRequired,
        );
        assert!(!decision.eligible);
        assert!(!decision.advertises_public_service);
    }
}

#[test]
fn block_serving_activation_does_not_change_service_bits() {
    // Arrange
    let policies = [
        BlockRelayActivationPolicy::default(),
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig::default(),
        },
        BlockRelayActivationPolicy {
            block_serving: BlockServingActivationConfig { enabled: true },
            compact_relay: CompactRelayActivationConfig { enabled: true },
        },
    ];

    // Act
    let decisions: Vec<_> = policies
        .into_iter()
        .map(|activation| {
            let input = BlockServingEligibilityInput {
                activation,
                inbound_serving_enabled: true,
                connection_class: PeerConnectionClass::Outbound,
                active_permission_effects: Vec::new(),
                inactive_permission_effects: Vec::new(),
                status_available: true,
            };

            classify(input)
        })
        .collect();
    let services = LocalPeerConfig::default().services;

    // Assert
    assert_eq!(services, ServiceFlags::NETWORK | ServiceFlags::WITNESS);
    assert!(decisions.iter().all(|decision| {
        !decision.advertises_public_service
            && matches!(
                decision.reason,
                BlockServingEligibilityReason::Disabled | BlockServingEligibilityReason::Eligible
            )
    }));
}

#[test]
fn eligibility_reason_labels_are_stable() {
    // Arrange
    let reasons = [
        BlockServingEligibilityReason::Eligible,
        BlockServingEligibilityReason::Disabled,
        BlockServingEligibilityReason::ActivationRequired,
        BlockServingEligibilityReason::InboundServingRequired,
        BlockServingEligibilityReason::PermissionRequired,
        BlockServingEligibilityReason::ProtectedNotServing,
        BlockServingEligibilityReason::StatusUnavailable,
        BlockServingEligibilityReason::PermissionEffectInactive,
    ];

    // Act
    let labels: Vec<_> = reasons.into_iter().map(|reason| reason.as_str()).collect();

    // Assert
    assert_eq!(
        labels,
        vec![
            "eligible",
            "disabled",
            "activation_required",
            "inbound_serving_required",
            "permission_required",
            "protected_not_serving",
            "status_unavailable",
            "permission_effect_inactive",
        ],
    );
}

#[test]
fn block_serving_status_active_validated_available_block_allows_later_storage_read_and_serving() {
    // Arrange
    let facts = status_facts(
        BlockServingChainPosition::Active,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Available,
    );

    // Act
    let decision = classify_block_serving_status(&facts);

    // Assert
    assert_eq!(decision.label, BlockServingStatusLabel::Available);
    assert_eq!(decision.label.as_str(), "available");
    assert!(decision.allow_storage_read);
    assert!(decision.may_serve_block);
}

#[test]
fn block_serving_status_recent_valid_available_block_is_later_servable_but_stale_and_side_chain_are_not()
 {
    // Arrange
    let recent_valid = status_facts(
        BlockServingChainPosition::RecentValid,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Available,
    );
    let stale = status_facts(
        BlockServingChainPosition::Stale,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Available,
    );
    let side_chain = status_facts(
        BlockServingChainPosition::SideChain,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Available,
    );

    // Act
    let recent_valid_decision = classify_block_serving_status(&recent_valid);
    let stale_decision = classify_block_serving_status(&stale);
    let side_chain_decision = classify_block_serving_status(&side_chain);

    // Assert
    assert_eq!(
        recent_valid_decision.label,
        BlockServingStatusLabel::Available,
    );
    assert!(recent_valid_decision.allow_storage_read);
    assert!(recent_valid_decision.may_serve_block);
    assert_eq!(stale_decision.label, BlockServingStatusLabel::Stale);
    assert_eq!(stale_decision.label.as_str(), "stale");
    assert!(!stale_decision.allow_storage_read);
    assert!(!stale_decision.may_serve_block);
    assert_eq!(
        side_chain_decision.label,
        BlockServingStatusLabel::SideChain
    );
    assert_eq!(side_chain_decision.label.as_str(), "side_chain");
    assert!(!side_chain_decision.allow_storage_read);
    assert!(!side_chain_decision.may_serve_block);
}

#[test]
fn block_serving_status_unsafe_or_incomplete_facts_never_allow_storage_reads_or_serving() {
    // Arrange
    let cases = [
        (
            status_facts(
                BlockServingChainPosition::Active,
                BlockServingValidationState::Validated,
                BlockServingDataAvailability::Pruned,
            ),
            BlockServingStatusLabel::Pruned,
        ),
        (
            status_facts(
                BlockServingChainPosition::Active,
                BlockServingValidationState::Validated,
                BlockServingDataAvailability::Unavailable,
            ),
            BlockServingStatusLabel::Unavailable,
        ),
        (
            status_facts(
                BlockServingChainPosition::Unknown,
                BlockServingValidationState::Validated,
                BlockServingDataAvailability::Available,
            ),
            BlockServingStatusLabel::Unknown,
        ),
        (
            status_facts(
                BlockServingChainPosition::Active,
                BlockServingValidationState::Unvalidated,
                BlockServingDataAvailability::Available,
            ),
            BlockServingStatusLabel::Unvalidated,
        ),
        (
            BlockServingStatusFacts {
                suppressed: true,
                ..status_facts(
                    BlockServingChainPosition::Active,
                    BlockServingValidationState::Validated,
                    BlockServingDataAvailability::Available,
                )
            },
            BlockServingStatusLabel::Suppressed,
        ),
    ];

    // Act
    let decisions: Vec<_> = cases
        .into_iter()
        .map(|(facts, expected_label)| (classify_block_serving_status(&facts), expected_label))
        .collect();

    // Assert
    for (decision, expected_label) in decisions {
        assert_eq!(decision.label, expected_label);
        assert!(!decision.allow_storage_read);
        assert!(!decision.may_serve_block);
    }
}

#[test]
fn block_serving_status_validated_but_data_unknown_block_is_not_later_servable() {
    // Arrange
    let facts = status_facts(
        BlockServingChainPosition::Active,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Unknown,
    );

    // Act
    let decision = classify_block_serving_status(&facts);

    // Assert
    assert_eq!(decision.label, BlockServingStatusLabel::Validated);
    assert_eq!(decision.label.as_str(), "validated");
    assert!(!decision.allow_storage_read);
    assert!(!decision.may_serve_block);
}

#[test]
fn block_serving_status_labels_are_stable() {
    // Arrange
    let labels = [
        BlockServingStatusLabel::Validated,
        BlockServingStatusLabel::Available,
        BlockServingStatusLabel::Stale,
        BlockServingStatusLabel::SideChain,
        BlockServingStatusLabel::Pruned,
        BlockServingStatusLabel::Unavailable,
        BlockServingStatusLabel::Unvalidated,
        BlockServingStatusLabel::Unknown,
        BlockServingStatusLabel::Suppressed,
    ];

    // Act
    let rendered: Vec<_> = labels
        .into_iter()
        .map(BlockServingStatusLabel::as_str)
        .collect();

    // Assert
    assert_eq!(
        rendered,
        vec![
            "validated",
            "available",
            "stale",
            "side_chain",
            "pruned",
            "unavailable",
            "unvalidated",
            "unknown",
            "suppressed",
        ],
    );
}

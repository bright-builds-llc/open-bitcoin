// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;
use crate::peer::DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER;
use crate::{
    ConnectionChurnInput, INBOUND_PERMISSION_TOKENS_FIELD, InactivePermissionEffectLabel,
    InboundHandshakeState, LocalPeerConfig, PHASE94_CONNECTION_CHURN_WINDOW_SECONDS,
    PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW, PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
    PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW, PHASE94_REPEATED_FAILURE_WINDOW_SECONDS,
    PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS, PeerConnectionClass, PeerPermissionSet,
    PermissionEffectLabel, QueuePressureInput, ReconnectSuppressionInput, RepeatedFailureInput,
    RequestPressureInput, ResourceGovernancePolicy, ResourceLifecycleLabel, ResourcePressureLabel,
    ResourceTimeoutInput, ServiceFlags,
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

fn eligible_decision() -> BlockServingEligibilityDecision {
    classify(input(PeerConnectionClass::Outbound))
}

fn permissioned_eligible_decision(
    connection_class: PeerConnectionClass,
) -> BlockServingEligibilityDecision {
    classify(BlockServingEligibilityInput {
        active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
        ..input(connection_class)
    })
}

fn available_status_decision() -> BlockServingStatusDecision {
    classify_block_serving_status(&status_facts(
        BlockServingChainPosition::Active,
        BlockServingValidationState::Validated,
        BlockServingDataAvailability::Available,
    ))
}

fn gate_input() -> BlockServingResourceGateInput {
    BlockServingResourceGateInput {
        eligibility: eligible_decision(),
        status: available_status_decision(),
        queue_pressure: QueuePressureInput::default(),
        request_pressure: RequestPressureInput::default(),
        maybe_timeout: None,
        maybe_churn: None,
        maybe_repeated_failure: None,
        reconnect: ReconnectSuppressionInput::default(),
        maybe_cleanup: None,
    }
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

#[test]
fn block_serving_resource_gate_respects_phase94_block_request_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let at_cap = BlockServingResourceGateInput {
        request_pressure: RequestPressureInput {
            requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
            ..RequestPressureInput::default()
        },
        ..gate_input()
    };
    let over_cap = BlockServingResourceGateInput {
        request_pressure: RequestPressureInput {
            requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
            ..RequestPressureInput::default()
        },
        ..gate_input()
    };

    // Act
    let accepted = evaluate_block_serving_resource_gate(&policy, at_cap);
    let rejected = evaluate_block_serving_resource_gate(&policy, over_cap);

    // Assert
    assert_eq!(
        PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER,
        DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
    );
    assert_eq!(
        accepted.label,
        BlockServingOutcomeLabel::BlockServingEligible
    );
    assert_eq!(accepted.label.as_str(), "block_serving_eligible");
    assert!(accepted.allow_storage_read);
    assert!(accepted.may_serve_block);
    assert_eq!(
        rejected.label,
        BlockServingOutcomeLabel::BlockRequestCapReached,
    );
    assert_eq!(rejected.label.as_str(), "block_request_cap_reached");
    assert!(!rejected.allow_storage_read);
    assert!(!rejected.may_serve_block);
    assert_eq!(
        rejected
            .maybe_resource_event
            .as_ref()
            .map(|event| event.label.as_str()),
        Some(ResourcePressureLabel::RequestCapReached.as_str()),
    );
}

#[test]
fn block_serving_resource_gate_counts_permissioned_and_protected_peers_under_same_cap() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let cases = [
        BlockServingResourceGateInput {
            eligibility: permissioned_eligible_decision(PeerConnectionClass::PermissionedInbound),
            request_pressure: RequestPressureInput {
                requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
                active_permission_effects: vec![PermissionEffectLabel::DownloadServingPolicyInput],
                inactive_permission_effects: vec![
                    InactivePermissionEffectLabel::Relay,
                    InactivePermissionEffectLabel::ForceRelay,
                ],
                ..RequestPressureInput::default()
            },
            ..gate_input()
        },
        BlockServingResourceGateInput {
            eligibility: permissioned_eligible_decision(PeerConnectionClass::ProtectedInbound),
            request_pressure: RequestPressureInput {
                requested_blocks_in_flight: PHASE94_MAX_INBOUND_BLOCK_REQUESTS_PER_PEER + 1,
                active_permission_effects: vec![
                    PermissionEffectLabel::DownloadServingPolicyInput,
                    PermissionEffectLabel::AdmissionProtected,
                ],
                inactive_permission_effects: vec![InactivePermissionEffectLabel::Mempool],
                ..RequestPressureInput::default()
            },
            ..gate_input()
        },
    ];

    // Act
    let decisions: Vec<_> = cases
        .into_iter()
        .map(|input| evaluate_block_serving_resource_gate(&policy, input))
        .collect();

    // Assert
    for decision in decisions {
        assert_eq!(
            decision.label,
            BlockServingOutcomeLabel::BlockRequestCapReached,
        );
        assert!(!decision.allow_storage_read);
        assert!(!decision.may_serve_block);
    }
}

#[test]
fn block_serving_resource_gate_preserves_existing_resource_suppression_labels() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let now = 1_000;
    let cases = [
        (
            BlockServingResourceGateInput {
                queue_pressure: QueuePressureInput {
                    peer_read_queue_bytes: policy.max_peer_read_queue_bytes + 1,
                    ..QueuePressureInput::default()
                },
                ..gate_input()
            },
            ResourcePressureLabel::ReadQueuePressure.as_str(),
        ),
        (
            BlockServingResourceGateInput {
                maybe_timeout: Some(ResourceTimeoutInput {
                    handshake_state: InboundHandshakeState::Handshaking,
                    connected_at_unix_seconds: now - PHASE94_SLOW_HANDSHAKE_TIMEOUT_SECONDS - 1,
                    last_activity_unix_seconds: now,
                    now_unix_seconds: now,
                }),
                ..gate_input()
            },
            ResourceLifecycleLabel::SlowHandshake.as_str(),
        ),
        (
            BlockServingResourceGateInput {
                maybe_churn: Some(ConnectionChurnInput {
                    window_started_unix_seconds: now - PHASE94_CONNECTION_CHURN_WINDOW_SECONDS,
                    now_unix_seconds: now,
                    connection_attempts_in_window: PHASE94_MAX_CONNECTIONS_PER_CHURN_WINDOW + 1,
                }),
                ..gate_input()
            },
            ResourceLifecycleLabel::ConnectionChurnLimited.as_str(),
        ),
        (
            BlockServingResourceGateInput {
                maybe_repeated_failure: Some(RepeatedFailureInput {
                    window_started_unix_seconds: now - PHASE94_REPEATED_FAILURE_WINDOW_SECONDS,
                    now_unix_seconds: now,
                    failures_in_window: PHASE94_MAX_REPEATED_FAILURES_PER_WINDOW + 1,
                }),
                ..gate_input()
            },
            ResourceLifecycleLabel::RepeatedFailureLimited.as_str(),
        ),
        (
            BlockServingResourceGateInput {
                reconnect: ReconnectSuppressionInput {
                    banned: true,
                    discouraged: false,
                },
                ..gate_input()
            },
            ResourceLifecycleLabel::ReconnectSuppressedBanned.as_str(),
        ),
        (
            BlockServingResourceGateInput {
                reconnect: ReconnectSuppressionInput {
                    banned: false,
                    discouraged: true,
                },
                ..gate_input()
            },
            ResourceLifecycleLabel::ReconnectSuppressedDiscouraged.as_str(),
        ),
    ];

    // Act
    let decisions: Vec<_> = cases
        .into_iter()
        .map(|(input, expected_label)| {
            (
                evaluate_block_serving_resource_gate(&policy, input),
                expected_label,
            )
        })
        .collect();

    // Assert
    for (decision, expected_label) in decisions {
        assert_eq!(
            decision.label,
            BlockServingOutcomeLabel::BlockServingSuppressed,
        );
        assert!(!decision.allow_storage_read);
        assert!(!decision.may_serve_block);
        assert_eq!(
            decision
                .maybe_resource_event
                .as_ref()
                .map(|event| event.label.as_str()),
            Some(expected_label),
        );
    }
}

#[test]
fn block_serving_resource_outcome_labels_are_stable() {
    // Arrange
    let labels = [
        BlockServingOutcomeLabel::BlockServingDisabled,
        BlockServingOutcomeLabel::BlockServingEligible,
        BlockServingOutcomeLabel::BlockServingSuppressed,
        BlockServingOutcomeLabel::BlockStatusUnavailable,
        BlockServingOutcomeLabel::BlockStatusPruned,
        BlockServingOutcomeLabel::BlockStatusUnvalidated,
        BlockServingOutcomeLabel::BlockRequestCapReached,
        BlockServingOutcomeLabel::BlockInFlightCleanupReleased,
        BlockServingOutcomeLabel::BlockInFlightCleanupPeerRemoved,
        BlockServingOutcomeLabel::BlockInFlightCleanupTimeout,
        BlockServingOutcomeLabel::BlockInFlightCleanupRestart,
        BlockServingOutcomeLabel::BlockInFlightLimitStillReached,
    ];

    // Act
    let rendered: Vec<_> = labels
        .into_iter()
        .map(BlockServingOutcomeLabel::as_str)
        .collect();

    // Assert
    assert_eq!(
        rendered,
        vec![
            "block_serving_disabled",
            "block_serving_eligible",
            "block_serving_suppressed",
            "block_status_unavailable",
            "block_status_pruned",
            "block_status_unvalidated",
            "block_request_cap_reached",
            "block_inflight_cleanup_released",
            "block_inflight_cleanup_peer_removed",
            "block_inflight_cleanup_timeout",
            "block_inflight_cleanup_restart",
            "block_inflight_limit_still_reached",
        ],
    );
}

#[test]
fn block_inflight_cleanup_classifies_release_peer_timeout_and_restart_without_serving() {
    // Arrange
    let causes = [
        (
            BlockInFlightCleanupCause::ReceivedBlock,
            BlockServingOutcomeLabel::BlockInFlightCleanupReleased,
        ),
        (
            BlockInFlightCleanupCause::NotFound,
            BlockServingOutcomeLabel::BlockInFlightCleanupReleased,
        ),
        (
            BlockInFlightCleanupCause::PeerDisconnect,
            BlockServingOutcomeLabel::BlockInFlightCleanupPeerRemoved,
        ),
        (
            BlockInFlightCleanupCause::Timeout,
            BlockServingOutcomeLabel::BlockInFlightCleanupTimeout,
        ),
        (
            BlockInFlightCleanupCause::RuntimeRestart,
            BlockServingOutcomeLabel::BlockInFlightCleanupRestart,
        ),
    ];
    let policy = ResourceGovernancePolicy::default();

    // Act
    let decisions: Vec<_> = causes
        .into_iter()
        .map(|(cause, expected_label)| {
            let cleanup = BlockInFlightCleanupInput {
                cause,
                blocks_in_flight_before: 2,
                released_blocks: 1,
                remaining_blocks_in_flight: 1,
                max_blocks_in_flight_per_peer: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
                max_blocks_in_flight_total: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER * 2,
            };
            let cleanup_decision = classify_block_inflight_cleanup(&cleanup);
            let gate_decision = evaluate_block_serving_resource_gate(
                &policy,
                BlockServingResourceGateInput {
                    maybe_cleanup: Some(cleanup),
                    ..gate_input()
                },
            );
            (cleanup_decision, gate_decision, expected_label)
        })
        .collect();

    // Assert
    for (cleanup_decision, gate_decision, expected_label) in decisions {
        assert_eq!(cleanup_decision.label, expected_label);
        assert_eq!(gate_decision.label, expected_label);
        assert!(!cleanup_decision.limit_still_reached);
        assert!(!gate_decision.allow_storage_read);
        assert!(!gate_decision.may_serve_block);
    }
}

#[test]
fn block_inflight_cleanup_reports_limit_still_reached_without_serving() {
    // Arrange
    let cleanup = BlockInFlightCleanupInput {
        cause: BlockInFlightCleanupCause::NotFound,
        blocks_in_flight_before: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER + 1,
        released_blocks: 1,
        remaining_blocks_in_flight: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
        max_blocks_in_flight_per_peer: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
        max_blocks_in_flight_total: DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER * 2,
    };

    // Act
    let decision = classify_block_inflight_cleanup(&cleanup);

    // Assert
    assert_eq!(
        decision.label,
        BlockServingOutcomeLabel::BlockInFlightLimitStillReached,
    );
    assert_eq!(
        decision.label.as_str(),
        "block_inflight_limit_still_reached"
    );
    assert!(decision.limit_still_reached);
    assert_eq!(decision.released_blocks, 1);
    assert_eq!(
        decision.remaining_blocks_in_flight,
        DEFAULT_MAX_BLOCKS_IN_FLIGHT_PER_PEER,
    );
}

#[test]
fn block_serving_resource_gate_blocks_disabled_and_nonservable_statuses_before_storage_read() {
    // Arrange
    let policy = ResourceGovernancePolicy::default();
    let disabled = BlockServingResourceGateInput {
        eligibility: classify(BlockServingEligibilityInput {
            activation: BlockRelayActivationPolicy::default(),
            ..input(PeerConnectionClass::Outbound)
        }),
        ..gate_input()
    };
    let status_unavailable_eligibility = BlockServingResourceGateInput {
        eligibility: classify(BlockServingEligibilityInput {
            status_available: false,
            ..input(PeerConnectionClass::Outbound)
        }),
        ..gate_input()
    };
    let statuses = [
        BlockServingStatusLabel::Validated,
        BlockServingStatusLabel::Stale,
        BlockServingStatusLabel::SideChain,
        BlockServingStatusLabel::Pruned,
        BlockServingStatusLabel::Unavailable,
        BlockServingStatusLabel::Unvalidated,
        BlockServingStatusLabel::Unknown,
        BlockServingStatusLabel::Suppressed,
    ];

    // Act
    let disabled_decision = evaluate_block_serving_resource_gate(&policy, disabled);
    let status_unavailable_eligibility_decision =
        evaluate_block_serving_resource_gate(&policy, status_unavailable_eligibility);
    let status_decisions: Vec<_> = statuses
        .into_iter()
        .map(|label| {
            evaluate_block_serving_resource_gate(
                &policy,
                BlockServingResourceGateInput {
                    status: BlockServingStatusDecision {
                        label,
                        allow_storage_read: false,
                        may_serve_block: false,
                    },
                    ..gate_input()
                },
            )
        })
        .collect();

    // Assert
    assert_eq!(
        disabled_decision.label,
        BlockServingOutcomeLabel::BlockServingDisabled,
    );
    assert!(!disabled_decision.allow_storage_read);
    assert!(!disabled_decision.may_serve_block);
    assert_eq!(
        status_unavailable_eligibility_decision.label,
        BlockServingOutcomeLabel::BlockStatusUnavailable,
    );
    assert!(!status_unavailable_eligibility_decision.allow_storage_read);
    assert!(!status_unavailable_eligibility_decision.may_serve_block);
    for decision in status_decisions {
        assert!(!decision.allow_storage_read);
        assert!(!decision.may_serve_block);
    }
}

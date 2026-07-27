// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;

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

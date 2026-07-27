// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use super::*;

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
fn block_serving_status_recent_valid_available_block_is_later_servable_but_stale_and_side_chain_are_not(
) {
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

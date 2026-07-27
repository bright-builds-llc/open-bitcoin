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

mod eligibility_cases;
mod resource_gate_cases;
mod status_cases;

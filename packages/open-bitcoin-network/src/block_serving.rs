// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use crate::resource::ConnectionChurnInput;
use crate::{
    InactivePermissionEffectLabel, InboundResourceEvent, PeerConnectionClass,
    PermissionEffectLabel, QueuePressureInput, ReconnectSuppressionInput, RepeatedFailureInput,
    RequestPressureInput, ResourceGovernanceDecision, ResourceGovernancePolicy,
    ResourcePressureLabel, ResourceTimeoutInput,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockServingActivationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CompactRelayActivationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BlockRelayActivationPolicy {
    pub block_serving: BlockServingActivationConfig,
    pub compact_relay: CompactRelayActivationConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockServingEligibilityInput {
    pub activation: BlockRelayActivationPolicy,
    pub inbound_serving_enabled: bool,
    pub connection_class: PeerConnectionClass,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    pub status_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockServingEligibilityDecision {
    pub eligible: bool,
    pub reason: BlockServingEligibilityReason,
    pub active_permission_effects: Vec<PermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
    pub advertises_public_service: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingEligibilityReason {
    Eligible,
    Disabled,
    ActivationRequired,
    InboundServingRequired,
    PermissionRequired,
    ProtectedNotServing,
    StatusUnavailable,
    PermissionEffectInactive,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingChainPosition {
    Active,
    RecentValid,
    Stale,
    SideChain,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingValidationState {
    Validated,
    Unvalidated,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingDataAvailability {
    Available,
    Pruned,
    Unavailable,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockServingStatusFacts {
    pub chain_position: BlockServingChainPosition,
    pub validation_state: BlockServingValidationState,
    pub data_availability: BlockServingDataAvailability,
    pub suppressed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockServingStatusDecision {
    pub label: BlockServingStatusLabel,
    pub allow_storage_read: bool,
    pub may_serve_block: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingStatusLabel {
    Validated,
    Available,
    Stale,
    SideChain,
    Pruned,
    Unavailable,
    Unvalidated,
    Unknown,
    Suppressed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockServingOutcomeLabel {
    BlockServingDisabled,
    BlockServingEligible,
    BlockServingSuppressed,
    BlockStatusUnavailable,
    BlockStatusPruned,
    BlockStatusUnvalidated,
    BlockRequestCapReached,
    BlockInFlightCleanupReleased,
    BlockInFlightCleanupPeerRemoved,
    BlockInFlightCleanupTimeout,
    BlockInFlightCleanupRestart,
    BlockInFlightLimitStillReached,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockServingResourceGateInput {
    pub eligibility: BlockServingEligibilityDecision,
    pub status: BlockServingStatusDecision,
    pub queue_pressure: QueuePressureInput,
    pub request_pressure: RequestPressureInput,
    pub maybe_timeout: Option<ResourceTimeoutInput>,
    pub maybe_churn: Option<ConnectionChurnInput>,
    pub maybe_repeated_failure: Option<RepeatedFailureInput>,
    pub reconnect: ReconnectSuppressionInput,
    pub maybe_cleanup: Option<BlockInFlightCleanupInput>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockServingResourceGateDecision {
    pub label: BlockServingOutcomeLabel,
    pub allow_storage_read: bool,
    pub may_serve_block: bool,
    pub maybe_resource_event: Option<InboundResourceEvent>,
    pub maybe_cleanup: Option<BlockInFlightCleanupDecision>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BlockInFlightCleanupCause {
    ReceivedBlock,
    NotFound,
    PeerDisconnect,
    Timeout,
    RuntimeRestart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockInFlightCleanupInput {
    pub cause: BlockInFlightCleanupCause,
    pub blocks_in_flight_before: usize,
    pub released_blocks: usize,
    pub remaining_blocks_in_flight: usize,
    pub max_blocks_in_flight_per_peer: usize,
    pub max_blocks_in_flight_total: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BlockInFlightCleanupDecision {
    pub label: BlockServingOutcomeLabel,
    pub released_blocks: usize,
    pub remaining_blocks_in_flight: usize,
    pub limit_still_reached: bool,
}

impl BlockServingEligibilityReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::Disabled => "disabled",
            Self::ActivationRequired => "activation_required",
            Self::InboundServingRequired => "inbound_serving_required",
            Self::PermissionRequired => "permission_required",
            Self::ProtectedNotServing => "protected_not_serving",
            Self::StatusUnavailable => "status_unavailable",
            Self::PermissionEffectInactive => "permission_effect_inactive",
        }
    }
}

impl BlockServingStatusLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Validated => "validated",
            Self::Available => "available",
            Self::Stale => "stale",
            Self::SideChain => "side_chain",
            Self::Pruned => "pruned",
            Self::Unavailable => "unavailable",
            Self::Unvalidated => "unvalidated",
            Self::Unknown => "unknown",
            Self::Suppressed => "suppressed",
        }
    }
}

impl BlockServingOutcomeLabel {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::BlockServingDisabled => "block_serving_disabled",
            Self::BlockServingEligible => "block_serving_eligible",
            Self::BlockServingSuppressed => "block_serving_suppressed",
            Self::BlockStatusUnavailable => "block_status_unavailable",
            Self::BlockStatusPruned => "block_status_pruned",
            Self::BlockStatusUnvalidated => "block_status_unvalidated",
            Self::BlockRequestCapReached => "block_request_cap_reached",
            Self::BlockInFlightCleanupReleased => "block_inflight_cleanup_released",
            Self::BlockInFlightCleanupPeerRemoved => "block_inflight_cleanup_peer_removed",
            Self::BlockInFlightCleanupTimeout => "block_inflight_cleanup_timeout",
            Self::BlockInFlightCleanupRestart => "block_inflight_cleanup_restart",
            Self::BlockInFlightLimitStillReached => "block_inflight_limit_still_reached",
        }
    }
}

pub fn classify_block_serving_eligibility(
    input: &BlockServingEligibilityInput,
) -> BlockServingEligibilityDecision {
    let reason = classify_block_serving_eligibility_reason(input);
    let eligible = reason == BlockServingEligibilityReason::Eligible;

    BlockServingEligibilityDecision {
        eligible,
        reason,
        active_permission_effects: input.active_permission_effects.clone(),
        inactive_permission_effects: input.inactive_permission_effects.clone(),
        advertises_public_service: false,
    }
}

pub fn classify_block_serving_status(
    facts: &BlockServingStatusFacts,
) -> BlockServingStatusDecision {
    let label = classify_block_serving_status_label(facts);
    let may_serve_block = label == BlockServingStatusLabel::Available;

    BlockServingStatusDecision {
        label,
        allow_storage_read: may_serve_block,
        may_serve_block,
    }
}

pub fn evaluate_block_serving_resource_gate(
    policy: &ResourceGovernancePolicy,
    input: BlockServingResourceGateInput,
) -> BlockServingResourceGateDecision {
    if !input.eligibility.eligible {
        return blocked_gate_decision(eligibility_label(input.eligibility.reason), None);
    }

    if let Some(label) = status_gate_label(input.status.label) {
        return blocked_gate_decision(label, None);
    }

    if let Some(decision) = resource_gate_decision(policy.decide_queue(input.queue_pressure)) {
        return decision;
    }

    if let Some(decision) = resource_gate_decision(policy.decide_request(input.request_pressure)) {
        return decision;
    }

    if let Some(timeout) = input.maybe_timeout
        && let Some(decision) = resource_gate_decision(policy.decide_timeout(timeout))
    {
        return decision;
    }

    if let Some(churn) = input.maybe_churn
        && let Some(decision) = resource_gate_decision(policy.decide_churn(churn))
    {
        return decision;
    }

    if let Some(repeated_failure) = input.maybe_repeated_failure
        && let Some(decision) =
            resource_gate_decision(policy.decide_repeated_failure(repeated_failure))
    {
        return decision;
    }

    if let Some(decision) = resource_gate_decision(policy.decide_reconnect(input.reconnect)) {
        return decision;
    }

    if let Some(cleanup) = input.maybe_cleanup {
        let cleanup_decision = classify_block_inflight_cleanup(&cleanup);
        return BlockServingResourceGateDecision {
            label: cleanup_decision.label,
            allow_storage_read: false,
            may_serve_block: false,
            maybe_resource_event: None,
            maybe_cleanup: Some(cleanup_decision),
        };
    }

    BlockServingResourceGateDecision {
        label: BlockServingOutcomeLabel::BlockServingEligible,
        allow_storage_read: true,
        may_serve_block: true,
        maybe_resource_event: None,
        maybe_cleanup: None,
    }
}

pub fn classify_block_inflight_cleanup(
    input: &BlockInFlightCleanupInput,
) -> BlockInFlightCleanupDecision {
    let limit_still_reached = input.remaining_blocks_in_flight
        >= input.max_blocks_in_flight_per_peer
        || input.remaining_blocks_in_flight >= input.max_blocks_in_flight_total;
    let label = if limit_still_reached {
        BlockServingOutcomeLabel::BlockInFlightLimitStillReached
    } else {
        cleanup_label(input.cause)
    };

    BlockInFlightCleanupDecision {
        label,
        released_blocks: input.released_blocks,
        remaining_blocks_in_flight: input.remaining_blocks_in_flight,
        limit_still_reached,
    }
}

fn classify_block_serving_eligibility_reason(
    input: &BlockServingEligibilityInput,
) -> BlockServingEligibilityReason {
    let has_download_serving_effect = input
        .active_permission_effects
        .contains(&PermissionEffectLabel::DownloadServingPolicyInput);

    if !input.activation.block_serving.enabled {
        if has_download_serving_effect {
            return BlockServingEligibilityReason::ActivationRequired;
        }
        return BlockServingEligibilityReason::Disabled;
    }

    if matches!(
        input.connection_class,
        PeerConnectionClass::Outbound | PeerConnectionClass::ManualConfigured
    ) {
        return status_sensitive_reason(input);
    }

    if !input.inbound_serving_enabled {
        return BlockServingEligibilityReason::InboundServingRequired;
    }

    if input.connection_class == PeerConnectionClass::OrdinaryInbound {
        return BlockServingEligibilityReason::PermissionRequired;
    }

    if input.connection_class == PeerConnectionClass::ProtectedInbound
        && !has_download_serving_effect
    {
        return BlockServingEligibilityReason::ProtectedNotServing;
    }

    if !has_download_serving_effect {
        if !input.inactive_permission_effects.is_empty() {
            return BlockServingEligibilityReason::PermissionEffectInactive;
        }
        return BlockServingEligibilityReason::PermissionRequired;
    }

    status_sensitive_reason(input)
}

fn eligibility_label(reason: BlockServingEligibilityReason) -> BlockServingOutcomeLabel {
    if reason == BlockServingEligibilityReason::StatusUnavailable {
        return BlockServingOutcomeLabel::BlockStatusUnavailable;
    }

    BlockServingOutcomeLabel::BlockServingDisabled
}

fn status_gate_label(label: BlockServingStatusLabel) -> Option<BlockServingOutcomeLabel> {
    match label {
        BlockServingStatusLabel::Available => None,
        BlockServingStatusLabel::Pruned => Some(BlockServingOutcomeLabel::BlockStatusPruned),
        BlockServingStatusLabel::Unvalidated => {
            Some(BlockServingOutcomeLabel::BlockStatusUnvalidated)
        }
        BlockServingStatusLabel::Suppressed => {
            Some(BlockServingOutcomeLabel::BlockServingSuppressed)
        }
        BlockServingStatusLabel::Validated
        | BlockServingStatusLabel::Stale
        | BlockServingStatusLabel::SideChain
        | BlockServingStatusLabel::Unavailable
        | BlockServingStatusLabel::Unknown => {
            Some(BlockServingOutcomeLabel::BlockStatusUnavailable)
        }
    }
}

fn resource_gate_decision(
    decision: ResourceGovernanceDecision,
) -> Option<BlockServingResourceGateDecision> {
    let event = match decision {
        ResourceGovernanceDecision::Accept => return None,
        ResourceGovernanceDecision::Backpressure(event)
        | ResourceGovernanceDecision::Disconnect(event)
        | ResourceGovernanceDecision::RecordMisbehavior(event) => event,
    };
    let label = if event.label == ResourcePressureLabel::RequestCapReached.as_str() {
        BlockServingOutcomeLabel::BlockRequestCapReached
    } else {
        BlockServingOutcomeLabel::BlockServingSuppressed
    };

    Some(blocked_gate_decision(label, Some(event)))
}

fn blocked_gate_decision(
    label: BlockServingOutcomeLabel,
    maybe_resource_event: Option<InboundResourceEvent>,
) -> BlockServingResourceGateDecision {
    BlockServingResourceGateDecision {
        label,
        allow_storage_read: false,
        may_serve_block: false,
        maybe_resource_event,
        maybe_cleanup: None,
    }
}

fn cleanup_label(cause: BlockInFlightCleanupCause) -> BlockServingOutcomeLabel {
    match cause {
        BlockInFlightCleanupCause::ReceivedBlock | BlockInFlightCleanupCause::NotFound => {
            BlockServingOutcomeLabel::BlockInFlightCleanupReleased
        }
        BlockInFlightCleanupCause::PeerDisconnect => {
            BlockServingOutcomeLabel::BlockInFlightCleanupPeerRemoved
        }
        BlockInFlightCleanupCause::Timeout => BlockServingOutcomeLabel::BlockInFlightCleanupTimeout,
        BlockInFlightCleanupCause::RuntimeRestart => {
            BlockServingOutcomeLabel::BlockInFlightCleanupRestart
        }
    }
}

fn status_sensitive_reason(input: &BlockServingEligibilityInput) -> BlockServingEligibilityReason {
    if input.status_available {
        return BlockServingEligibilityReason::Eligible;
    }

    BlockServingEligibilityReason::StatusUnavailable
}

fn classify_block_serving_status_label(facts: &BlockServingStatusFacts) -> BlockServingStatusLabel {
    if facts.suppressed {
        return BlockServingStatusLabel::Suppressed;
    }

    if facts.chain_position == BlockServingChainPosition::Unknown
        || facts.validation_state == BlockServingValidationState::Unknown
    {
        return BlockServingStatusLabel::Unknown;
    }

    if facts.validation_state == BlockServingValidationState::Unvalidated {
        return BlockServingStatusLabel::Unvalidated;
    }

    match facts.chain_position {
        BlockServingChainPosition::Stale => return BlockServingStatusLabel::Stale,
        BlockServingChainPosition::SideChain => return BlockServingStatusLabel::SideChain,
        BlockServingChainPosition::Active
        | BlockServingChainPosition::RecentValid
        | BlockServingChainPosition::Unknown => {}
    }

    match facts.data_availability {
        BlockServingDataAvailability::Available => BlockServingStatusLabel::Available,
        BlockServingDataAvailability::Pruned => BlockServingStatusLabel::Pruned,
        BlockServingDataAvailability::Unavailable => BlockServingStatusLabel::Unavailable,
        BlockServingDataAvailability::Unknown => BlockServingStatusLabel::Validated,
    }
}

#[cfg(test)]
mod tests;

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use crate::{InactivePermissionEffectLabel, PeerConnectionClass, PermissionEffectLabel};

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

fn status_sensitive_reason(input: &BlockServingEligibilityInput) -> BlockServingEligibilityReason {
    if input.status_available {
        return BlockServingEligibilityReason::Eligible;
    }

    BlockServingEligibilityReason::StatusUnavailable
}

#[cfg(test)]
mod tests;

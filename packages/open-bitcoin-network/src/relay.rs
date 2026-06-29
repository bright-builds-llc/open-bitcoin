// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_permissions.py

use crate::{InactivePermissionEffectLabel, PeerConnectionClass, RelayPermissionEffectLabel};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RelayActivationConfig {
    pub enabled: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelayEligibilityReason {
    Eligible,
    Disabled,
    ActivationRequired,
    InboundServingRequired,
    PermissionRequired,
    ProtectedNotRelay,
    PermissionEffectInactive,
}

impl RelayEligibilityReason {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Eligible => "eligible",
            Self::Disabled => "disabled",
            Self::ActivationRequired => "activation_required",
            Self::InboundServingRequired => "inbound_serving_required",
            Self::PermissionRequired => "permission_required",
            Self::ProtectedNotRelay => "protected_not_relay",
            Self::PermissionEffectInactive => "permission_effect_inactive",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayEligibilityInput {
    pub activation: RelayActivationConfig,
    pub inbound_serving_enabled: bool,
    pub connection_class: PeerConnectionClass,
    pub relay_permission_effects: Vec<RelayPermissionEffectLabel>,
    pub inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RelayEligibilityDecision {
    pub eligible: bool,
    pub reason: RelayEligibilityReason,
    pub relay_permission_effects: Vec<RelayPermissionEffectLabel>,
    pub version_message_relay: bool,
}

pub fn classify_relay_eligibility(input: &RelayEligibilityInput) -> RelayEligibilityDecision {
    let has_relay_permission_effect = !input.relay_permission_effects.is_empty();
    let reason = classify_relay_eligibility_reason(input, has_relay_permission_effect);
    let eligible = reason == RelayEligibilityReason::Eligible;

    RelayEligibilityDecision {
        eligible,
        reason,
        relay_permission_effects: input.relay_permission_effects.clone(),
        version_message_relay: eligible,
    }
}

fn classify_relay_eligibility_reason(
    input: &RelayEligibilityInput,
    has_relay_permission_effect: bool,
) -> RelayEligibilityReason {
    if !input.activation.enabled {
        if has_relay_permission_effect {
            return RelayEligibilityReason::ActivationRequired;
        }
        return RelayEligibilityReason::Disabled;
    }

    if matches!(
        input.connection_class,
        PeerConnectionClass::Outbound | PeerConnectionClass::ManualConfigured
    ) {
        return RelayEligibilityReason::Eligible;
    }

    if !input.inbound_serving_enabled {
        return RelayEligibilityReason::InboundServingRequired;
    }

    if has_relay_permission_effect {
        return RelayEligibilityReason::Eligible;
    }

    if input.connection_class == PeerConnectionClass::ProtectedInbound {
        return RelayEligibilityReason::ProtectedNotRelay;
    }

    if !input.inactive_permission_effects.is_empty() {
        return RelayEligibilityReason::PermissionEffectInactive;
    }

    RelayEligibilityReason::PermissionRequired
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{LocalPeerConfig, ServiceFlags};

    fn input(
        connection_class: PeerConnectionClass,
        relay_permission_effects: Vec<RelayPermissionEffectLabel>,
    ) -> RelayEligibilityInput {
        RelayEligibilityInput {
            activation: RelayActivationConfig { enabled: true },
            inbound_serving_enabled: true,
            connection_class,
            relay_permission_effects,
            inactive_permission_effects: Vec::new(),
        }
    }

    fn classify(input: RelayEligibilityInput) -> RelayEligibilityDecision {
        classify_relay_eligibility(&input)
    }

    #[test]
    fn relay_activation_defaults_to_disabled() {
        // Arrange
        let input = RelayEligibilityInput {
            activation: RelayActivationConfig::default(),
            inbound_serving_enabled: true,
            connection_class: PeerConnectionClass::Outbound,
            relay_permission_effects: Vec::new(),
            inactive_permission_effects: Vec::new(),
        };

        // Act
        let decision = classify(input);

        // Assert
        assert!(!RelayActivationConfig::default().enabled);
        assert_eq!(decision.reason, RelayEligibilityReason::Disabled);
        assert!(!decision.eligible);
        assert!(!decision.version_message_relay);
    }

    #[test]
    fn relay_eligibility_matrix_covers_connection_classes() {
        // Arrange
        let relay_effect = vec![RelayPermissionEffectLabel::TransactionRelayPolicyInput];
        let cases = [
            (
                PeerConnectionClass::Outbound,
                Vec::new(),
                RelayEligibilityReason::Eligible,
            ),
            (
                PeerConnectionClass::ManualConfigured,
                Vec::new(),
                RelayEligibilityReason::Eligible,
            ),
            (
                PeerConnectionClass::OrdinaryInbound,
                Vec::new(),
                RelayEligibilityReason::PermissionRequired,
            ),
            (
                PeerConnectionClass::PermissionedInbound,
                relay_effect.clone(),
                RelayEligibilityReason::Eligible,
            ),
            (
                PeerConnectionClass::ProtectedInbound,
                Vec::new(),
                RelayEligibilityReason::ProtectedNotRelay,
            ),
        ];

        // Act
        let decisions: Vec<RelayEligibilityDecision> = cases
            .into_iter()
            .map(|(connection_class, relay_permission_effects, _reason)| {
                classify(input(connection_class, relay_permission_effects))
            })
            .collect();

        // Assert
        assert_eq!(decisions[0].reason, RelayEligibilityReason::Eligible);
        assert_eq!(decisions[1].reason, RelayEligibilityReason::Eligible);
        assert_eq!(
            decisions[2].reason,
            RelayEligibilityReason::PermissionRequired,
        );
        assert_eq!(decisions[3].reason, RelayEligibilityReason::Eligible);
        assert_eq!(
            decisions[4].reason,
            RelayEligibilityReason::ProtectedNotRelay,
        );
        assert!(decisions[0].eligible);
        assert!(decisions[1].eligible);
        assert!(!decisions[2].eligible);
        assert!(decisions[3].eligible);
        assert!(!decisions[4].eligible);
    }

    #[test]
    fn protected_admission_is_not_relay_eligibility() {
        // Arrange
        let input = input(PeerConnectionClass::ProtectedInbound, Vec::new());

        // Act
        let decision = classify(input);

        // Assert
        assert_eq!(decision.reason, RelayEligibilityReason::ProtectedNotRelay);
        assert!(!decision.eligible);
    }

    #[test]
    fn inbound_relay_permission_requires_activation_and_inbound_serving() {
        // Arrange
        let relay_permission_effects =
            vec![RelayPermissionEffectLabel::TransactionRelayPolicyInput];
        let disabled = RelayEligibilityInput {
            activation: RelayActivationConfig::default(),
            inbound_serving_enabled: true,
            connection_class: PeerConnectionClass::PermissionedInbound,
            relay_permission_effects: relay_permission_effects.clone(),
            inactive_permission_effects: Vec::new(),
        };
        let inbound_serving_disabled = RelayEligibilityInput {
            activation: RelayActivationConfig { enabled: true },
            inbound_serving_enabled: false,
            connection_class: PeerConnectionClass::PermissionedInbound,
            relay_permission_effects: relay_permission_effects.clone(),
            inactive_permission_effects: Vec::new(),
        };
        let enabled = RelayEligibilityInput {
            activation: RelayActivationConfig { enabled: true },
            inbound_serving_enabled: true,
            connection_class: PeerConnectionClass::PermissionedInbound,
            relay_permission_effects,
            inactive_permission_effects: Vec::new(),
        };

        // Act
        let disabled_decision = classify(disabled);
        let no_serving_decision = classify(inbound_serving_disabled);
        let enabled_decision = classify(enabled);

        // Assert
        assert_eq!(
            disabled_decision.reason,
            RelayEligibilityReason::ActivationRequired,
        );
        assert_eq!(
            no_serving_decision.reason,
            RelayEligibilityReason::InboundServingRequired,
        );
        assert_eq!(enabled_decision.reason, RelayEligibilityReason::Eligible);
        assert!(!disabled_decision.eligible);
        assert!(!no_serving_decision.eligible);
        assert!(enabled_decision.eligible);
        assert!(enabled_decision.version_message_relay);
    }

    #[test]
    fn filter_permissions_stay_inactive_for_relay_eligibility() {
        // Arrange
        let input = RelayEligibilityInput {
            activation: RelayActivationConfig { enabled: true },
            inbound_serving_enabled: true,
            connection_class: PeerConnectionClass::PermissionedInbound,
            relay_permission_effects: Vec::new(),
            inactive_permission_effects: vec![
                InactivePermissionEffectLabel::BloomFilter,
                InactivePermissionEffectLabel::BlockFilters,
            ],
        };

        // Act
        let decision = classify(input);

        // Assert
        assert_eq!(
            decision.reason,
            RelayEligibilityReason::PermissionEffectInactive,
        );
        assert!(!decision.eligible);
        assert!(!decision.version_message_relay);
    }

    #[test]
    fn relay_activation_does_not_change_service_bits() {
        // Arrange
        let config = LocalPeerConfig::default();

        // Act
        let services = config.services;

        // Assert
        assert_eq!(services, ServiceFlags::NETWORK | ServiceFlags::WITNESS);
    }

    #[test]
    fn relay_eligibility_reason_labels_are_stable() {
        // Arrange
        let reasons = [
            RelayEligibilityReason::Eligible,
            RelayEligibilityReason::Disabled,
            RelayEligibilityReason::ActivationRequired,
            RelayEligibilityReason::InboundServingRequired,
            RelayEligibilityReason::PermissionRequired,
            RelayEligibilityReason::ProtectedNotRelay,
            RelayEligibilityReason::PermissionEffectInactive,
        ];

        // Act
        let labels: Vec<&str> = reasons.into_iter().map(|reason| reason.as_str()).collect();

        // Assert
        assert_eq!(
            labels,
            vec![
                "eligible",
                "disabled",
                "activation_required",
                "inbound_serving_required",
                "permission_required",
                "protected_not_relay",
                "permission_effect_inactive",
            ],
        );
    }
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/protocol.h
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/src/node/txdownloadman.h
// - packages/bitcoin-knots/src/node/txdownloadman_impl.cpp
// - packages/bitcoin-knots/src/txorphanage.h
// - packages/bitcoin-knots/src/txorphanage.cpp
// - packages/bitcoin-knots/src/txrequest.h
// - packages/bitcoin-knots/src/txrequest.cpp
// - packages/bitcoin-knots/test/functional/p2p_orphan_handling.py
// - packages/bitcoin-knots/test/functional/p2p_tx_download.py
// - packages/bitcoin-knots/test/functional/p2p_getdata.py

use crate::{
    InactivePermissionEffectLabel, LocalPeerConfig, PeerConnectionClass, RelayActivationConfig,
    RelayEligibilityDecision, RelayEligibilityInput, RelayPermissionEffectLabel,
    classify_relay_eligibility,
};

use super::{ConnectionRole, PeerManager, PeerState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct RelayDownloadPolicy {
    pub activation: RelayActivationConfig,
    pub inbound_serving_enabled: bool,
}

impl PeerManager {
    pub fn with_relay_download_policy(
        local_config: LocalPeerConfig,
        max_blocks_in_flight_per_peer: usize,
        relay_download_policy: RelayDownloadPolicy,
    ) -> Self {
        let mut manager =
            Self::with_max_blocks_in_flight(local_config, max_blocks_in_flight_per_peer);
        manager.relay_download_policy = relay_download_policy;
        manager
    }

    pub fn set_relay_download_policy(&mut self, relay_download_policy: RelayDownloadPolicy) {
        self.relay_download_policy = relay_download_policy;
    }
}

pub(super) fn relay_download_eligibility(
    peer: &PeerState,
    policy: RelayDownloadPolicy,
) -> RelayEligibilityDecision {
    let (connection_class, relay_permission_effects, inactive_permission_effects) =
        relay_download_inputs(peer);
    relay_download_eligibility_for_class(
        policy,
        connection_class,
        relay_permission_effects,
        inactive_permission_effects,
    )
}

fn relay_download_inputs(
    peer: &PeerState,
) -> (
    PeerConnectionClass,
    Vec<RelayPermissionEffectLabel>,
    Vec<InactivePermissionEffectLabel>,
) {
    match peer.role {
        ConnectionRole::Outbound => (PeerConnectionClass::Outbound, Vec::new(), Vec::new()),
        ConnectionRole::Inbound => {
            let Some(record) = peer.maybe_inbound_record.as_ref() else {
                return (PeerConnectionClass::OrdinaryInbound, Vec::new(), Vec::new());
            };
            (
                record.connection_class,
                record
                    .permission_decision
                    .relay_permission_effects()
                    .to_vec(),
                record.permission_decision.inactive_effects().to_vec(),
            )
        }
    }
}

fn relay_download_eligibility_for_class(
    policy: RelayDownloadPolicy,
    connection_class: PeerConnectionClass,
    relay_permission_effects: Vec<RelayPermissionEffectLabel>,
    inactive_permission_effects: Vec<InactivePermissionEffectLabel>,
) -> RelayEligibilityDecision {
    classify_relay_eligibility(&RelayEligibilityInput {
        activation: policy.activation,
        inbound_serving_enabled: policy.inbound_serving_enabled,
        connection_class,
        relay_permission_effects,
        inactive_permission_effects,
    })
}

#[cfg(test)]
mod tests {
    use crate::{RelayEligibilityReason, RelayPermissionEffectLabel};

    use super::*;

    fn enabled_policy() -> RelayDownloadPolicy {
        RelayDownloadPolicy {
            activation: RelayActivationConfig { enabled: true },
            inbound_serving_enabled: true,
        }
    }

    #[test]
    fn default_policy_is_relay_disabled() {
        // Arrange
        let policy = RelayDownloadPolicy::default();

        // Act
        let decision = relay_download_eligibility_for_class(
            policy,
            PeerConnectionClass::Outbound,
            Vec::new(),
            Vec::new(),
        );

        // Assert
        assert_eq!(decision.reason, RelayEligibilityReason::Disabled);
        assert!(!decision.eligible);
    }

    #[test]
    fn setter_updates_runtime_relay_download_policy() {
        // Arrange
        let mut manager = PeerManager::new(LocalPeerConfig::default());
        let policy = enabled_policy();

        // Act
        manager.set_relay_download_policy(policy);

        // Assert
        assert_eq!(manager.relay_download_policy, policy);
    }

    #[test]
    fn manual_configured_peers_are_eligible_when_relay_is_enabled() {
        // Arrange
        let policy = enabled_policy();

        // Act
        let decision = relay_download_eligibility_for_class(
            policy,
            PeerConnectionClass::ManualConfigured,
            Vec::new(),
            Vec::new(),
        );

        // Assert
        assert_eq!(decision.reason, RelayEligibilityReason::Eligible);
        assert!(decision.eligible);
    }

    #[test]
    fn inbound_without_admission_record_is_ordinary_inbound_for_downloads() {
        // Arrange
        let peer = PeerState::new(ConnectionRole::Inbound);

        // Act
        let decision = relay_download_eligibility(&peer, enabled_policy());

        // Assert
        assert_eq!(decision.reason, RelayEligibilityReason::PermissionRequired);
        assert!(!decision.eligible);
    }

    #[test]
    fn permissioned_inbound_requires_scoped_relay_effect() {
        // Arrange
        let policy = enabled_policy();

        // Act
        let without_relay_effect = relay_download_eligibility_for_class(
            policy,
            PeerConnectionClass::PermissionedInbound,
            Vec::new(),
            Vec::new(),
        );
        let with_relay_effect = relay_download_eligibility_for_class(
            policy,
            PeerConnectionClass::PermissionedInbound,
            vec![RelayPermissionEffectLabel::TransactionRelayPolicyInput],
            Vec::new(),
        );

        // Assert
        assert_eq!(
            without_relay_effect.reason,
            RelayEligibilityReason::PermissionRequired,
        );
        assert!(!without_relay_effect.eligible);
        assert_eq!(with_relay_effect.reason, RelayEligibilityReason::Eligible);
        assert!(with_relay_effect.eligible);
    }

    #[test]
    fn protected_only_inbound_is_not_relay_eligible() {
        // Arrange
        let policy = enabled_policy();

        // Act
        let decision = relay_download_eligibility_for_class(
            policy,
            PeerConnectionClass::ProtectedInbound,
            Vec::new(),
            Vec::new(),
        );

        // Assert
        assert_eq!(decision.reason, RelayEligibilityReason::ProtectedNotRelay);
        assert!(!decision.eligible);
    }
}

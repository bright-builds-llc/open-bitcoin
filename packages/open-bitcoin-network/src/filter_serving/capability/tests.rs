// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_permissions.h
// - packages/bitcoin-knots/src/net_permissions.cpp
// - packages/bitcoin-knots/src/init.cpp

use crate::{
    BasicPeerServingCapability, BasicPeerServingPrerequisite, InactivePermissionEffectLabel,
    PeerPermissionSet, PeerPermissionToken, PermissionEffectLabel, ServiceFlags,
};
use crate::{ParsedPeerPermissionClass, PeerPermissionClassRegistry};

#[test]
fn phase160_capability_requested_effective_prerequisite_matrix() {
    // Arrange
    let cases: &[(&[&str], bool, bool)] = &[
        (&[], false, false),
        (&["all"], true, false),
        (&["blockfilters"], true, true),
        (&["all", "blockfilters"], true, true),
        (&["noban"], false, false),
        (&["download"], false, false),
        (&["forceinbound"], false, false),
        (&["mempool", "noban", "addr", "relay"], false, false),
        (&["bloomfilter"], false, false),
    ];
    for &(tokens, effective, explicit) in cases {
        let permissions = PeerPermissionSet::parse("permissions", tokens).expect("valid tokens");
        for general in [false, true] {
            // Act
            let capability = BasicPeerServingCapability::from_permissions(general, &permissions);

            // Assert
            assert_eq!(capability.effective_permission, effective);
            assert_eq!(capability.explicit_permission_requested, explicit);
            assert_eq!(capability.advertised(), general || effective);
            assert_eq!(
                permissions.requested_token(PeerPermissionToken::BlockFilters),
                explicit
            );
            assert_eq!(
                capability.validate_basic_index(true),
                BasicPeerServingPrerequisite::Accepted
            );
            assert_eq!(
                capability.validate_basic_index(false),
                if general || explicit {
                    BasicPeerServingPrerequisite::BasicIndexRequired
                } else {
                    BasicPeerServingPrerequisite::Accepted
                }
            );
        }
    }
}

#[test]
fn phase160_capability_admission_conditionally_classifies_filter_only() {
    // Arrange
    let class = ParsedPeerPermissionClass::parse("peer", ["127.0.0.1"], ["in", "all"])
        .expect("valid class");
    let decision =
        PeerPermissionClassRegistry::new([class]).resolve_inbound("127.0.0.1".parse().expect("IP"));

    // Act
    let configured = decision.clone().with_basic_filter_serving(false);

    // Assert
    assert!(
        decision
            .inactive_effects()
            .contains(&InactivePermissionEffectLabel::BlockFilters)
    );
    assert!(
        !configured
            .inactive_effects()
            .contains(&InactivePermissionEffectLabel::BlockFilters)
    );
    assert!(
        configured
            .inactive_effects()
            .contains(&InactivePermissionEffectLabel::BloomFilter)
    );
    assert!(
        configured
            .active_effects()
            .contains(&PermissionEffectLabel::BlockFiltersServingPolicyInput)
    );
    assert_eq!(
        configured.relay_permission_effects(),
        decision.relay_permission_effects()
    );
    assert_eq!(
        configured.clone().with_basic_filter_serving(false),
        configured
    );
}

#[test]
fn phase160_capability_permission_overlay_does_not_change_service_base() {
    // Arrange
    let permissions = PeerPermissionSet::parse("permissions", ["all"]).expect("valid tokens");
    let capability = BasicPeerServingCapability::from_permissions(false, &permissions);
    let base = ServiceFlags::NETWORK_LIMITED | ServiceFlags::WITNESS;

    // Act
    let services = capability.own_services(base);

    // Assert
    assert_eq!(services, base | ServiceFlags::COMPACT_FILTERS);
    assert!(!base.contains(ServiceFlags::COMPACT_FILTERS));
    assert!(!services.contains(ServiceFlags::NETWORK));
    assert_eq!(services.bits() & (1 << 2), 0);
}

#[test]
fn phase160_capability_outbound_class_grammar_stays_rejected() {
    // Arrange
    let tokens = ["out", "blockfilters"];

    // Act
    let parsed = ParsedPeerPermissionClass::parse("peer", ["127.0.0.1"], tokens);

    // Assert
    assert_eq!(
        parsed
            .expect_err("outbound grammar remains unsupported")
            .reason(),
        "outbound_direction_unsupported"
    );
}

#[test]
fn phase160_capability_admission_preserves_explicit_request_provenance() {
    // Arrange
    let all = ParsedPeerPermissionClass::parse("peer", ["127.0.0.1"], ["in", "all"])
        .expect("valid all class");
    let explicit =
        ParsedPeerPermissionClass::parse("peer", ["127.0.0.1"], ["in", "all", "blockfilters"])
            .expect("valid explicit class");
    let address = "127.0.0.1".parse().expect("literal IP");

    // Act
    let all_decision = PeerPermissionClassRegistry::new([all]).resolve_inbound(address);
    let explicit_decision = PeerPermissionClassRegistry::new([explicit]).resolve_inbound(address);

    // Assert
    assert_ne!(all_decision, explicit_decision);
}

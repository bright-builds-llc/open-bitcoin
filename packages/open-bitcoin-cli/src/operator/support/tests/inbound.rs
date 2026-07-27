// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;

#[test]
fn inbound_support_redacts_raw_phase94_resource_governance_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-resource-governance-redaction");
    let mut status = phase94_status_with_resource_governance_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.latest_resource_governance_decision =
        FieldAvailability::available(InboundResourceGovernanceEvent {
            outcome: "rejected 127.0.0.1:18444 198.51.100.94:8333 peer-94".to_string(),
            reason: "invalid_checksum peer_id=94 raw_endpoint=0.0.0.0:8333 [2001:db8:95::1]:8333"
                .to_string(),
            label: "payload_rejected payload_bytes=[00] raw_permission".to_string(),
            source: "source_inbound_resource_governance permission_string=in,noban".to_string(),
            message: "node.example:8333 0.0.0.0:8333 ::1 config=operator rpc_password=phase95 credential=phase95 secret=phase95 cookie=phase95"
                .to_string(),
            next_action: "peer-94 payload_bytes raw_endpoint permission_string config=operator"
                .to_string(),
        });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);
    let decision = &serialized["status"]["peers"]["inbound"]["value"]["latest_resource_governance_decision"]
        ["value"];
    let decision_json_text = serde_json::to_string_pretty(decision).expect("decision json");
    let decision_markdown_line = markdown
        .lines()
        .find(|line| line.contains("Latest resource governance decision:"))
        .expect("resource governance decision line");

    // Assert
    for field in [
        "outcome",
        "reason",
        "label",
        "source",
        "message",
        "next_action",
    ] {
        assert_eq!(
            decision[field],
            json!("redacted_resource_governance_evidence"),
            "unexpected {field} redaction"
        );
    }
    for rendered in [&decision_json_text, decision_markdown_line] {
        assert!(rendered.contains("redacted_resource_governance_evidence"));
        for forbidden in [
            "127.0.0.1:",
            "198.51.100.94:8333",
            "[2001:db8:95::1]:8333",
            "node.example:8333",
            "0.0.0.0:",
            "::1",
            "peer_id=",
            "peer-",
            "raw_endpoint",
            "payload_bytes",
            "raw_permission",
            "permission_string",
            "config=",
            "rpc_password",
            "credential",
            "secret",
            "cookie=",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn inbound_support_redacts_raw_phase92_address_boundary_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-address-redaction");
    let mut status = phase92_status_with_address_boundary_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.local_advertisement_candidates[0].source =
        "source_local_listener 198.51.100.92:8333 address_bytes=[127,0,0,1]".to_string();
    inbound.suppressed_advertisements[0].message =
        "local evidence only peer_id=92 ::1 operator_loopback raw_permission".to_string();
    inbound.latest_address_decision = FieldAvailability::available(InboundAddressDecisionEvent {
        outcome: "suppressed".to_string(),
        reason: "permission_policy_denied".to_string(),
        label: "getaddr_suppressed".to_string(),
        source: "source_inbound_addr".to_string(),
        message: "bounded getaddr denied [2001:db8:92::1]:8333 0.0.0.0:8333 peer_id=92 raw_permission config=operator"
            .to_string(),
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "127.0.0.1:",
            "198.51.100.92:8333",
            "[2001:db8:92::1]:8333",
            "0.0.0.0:",
            "::1",
            "address_bytes",
            "peer_id=",
            "operator_loopback",
            "raw_permission",
            "full address relay support",
            "peer discovery support",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
    assert!(markdown.contains("redacted_address_evidence"));
}

#[test]
fn inbound_support_redacts_raw_phase93_peer_policy_material() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-peer-policy-redaction");
    let mut status = phase93_status_with_peer_policy_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "peer-93-disconnect".to_string(),
        reason: "operator-loopback-secret".to_string(),
        label: "peer_id=93".to_string(),
        source: "node.example:8333".to_string(),
        message: "peer-93 127.0.0.1:18444 198.51.100.93:8333 raw_permission permission_string credential=phase96 secret=phase96 cookie=phase96 config=operator"
            .to_string(),
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        assert!(rendered.contains("redacted_peer_policy_label"));
        for forbidden in [
            "peer-93",
            "peer_id=93",
            "node.example:8333",
            "198.51.100.93:8333",
            "127.0.0.1:",
            "operator-loopback-secret",
            "raw_permission",
            "permission_string",
            "credential=phase96",
            "secret=phase96",
            "cookie=phase96",
            "config=operator",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
}

#[test]
fn inbound_support_redaction_preserves_safe_phase96_peer_policy_labels() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-peer-policy-safe-labels");
    let mut status = phase93_status_with_peer_policy_evidence();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.active_bans = 1;
    inbound.manual_unbans = 1;
    inbound.misbehavior_observations = 1;
    inbound.protected_no_actions = 1;
    inbound.latest_peer_policy_decision = FieldAvailability::available(InboundPeerPolicyEvent {
        outcome: "protected_no_action".to_string(),
        reason: "unbanned".to_string(),
        label: "ban_active".to_string(),
        source: "source_peer_policy_runtime_bridge".to_string(),
        message: "protected_no_action ban_active unbanned source_peer_policy_runtime_bridge"
            .to_string(),
    });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    for rendered in [&json_text, &markdown] {
        for expected in [
            "protected_no_action",
            "unbanned",
            "ban_active",
            "source_peer_policy_runtime_bridge",
        ] {
            assert!(rendered.contains(expected), "missing {expected}");
        }
    }
}

#[test]
fn inbound_support_json_and_markdown_redact_raw_permission_config_evidence() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-permission-redaction");
    let mut status = phase91_status_with_permissioned_inbound();
    let FieldAvailability::Available(inbound) = &mut status.peers.inbound else {
        panic!("inbound status fixture should be available");
    };
    inbound.permission_class = "operator_loopback".to_string();
    inbound.active_permission_effects = vec![
        "admission_protected".to_string(),
        "in,noban,forceinbound".to_string(),
    ];
    inbound.inactive_permission_effects =
        vec!["inactive_relay".to_string(), "peer_id=91".to_string()];
    inbound.latest_permission_decision =
        FieldAvailability::available(InboundPermissionDecisionEvent {
            outcome: "admitted".to_string(),
            reason: "admitted".to_string(),
            permission_class: "operator_loopback".to_string(),
            active_permission_effects: vec![
                "admission_protected".to_string(),
                "in,noban,forceinbound".to_string(),
            ],
            inactive_permission_effects: vec![
                "inactive_relay".to_string(),
                "peer_id=91".to_string(),
            ],
            message: "operator_loopback in,noban,forceinbound peer_id=91 127.0.0.1:18444 rpc_password cookie=phase91-secret"
                .to_string(),
        });
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let json_text = serde_json::to_string_pretty(&bundle).expect("support json");
    let markdown = render::render_support_markdown(&bundle);
    let inbound = &serialized["status"]["peers"]["inbound"]["value"];
    let latest_decision = &inbound["latest_permission_decision"]["value"];

    // Assert
    assert_eq!(
        inbound["permission_class"],
        json!("redacted_permission_class")
    );
    assert_eq!(
        inbound["active_permission_effects"],
        json!(["admission_protected", "redacted_permission_effect"])
    );
    assert_eq!(
        inbound["inactive_permission_effects"],
        json!(["inactive_relay", "redacted_permission_effect"])
    );
    assert_eq!(
        latest_decision["permission_class"],
        json!("redacted_permission_class")
    );
    assert_eq!(
        latest_decision["message"],
        json!("inbound permission decision admitted as redacted_permission_class")
    );
    for rendered in [&json_text, &markdown] {
        for forbidden in [
            "operator_loopback",
            "in,noban,forceinbound",
            "peer_id=",
            "127.0.0.1:",
            "rpc_password",
            "cookie=phase91-secret",
        ] {
            assert_absent(rendered, forbidden);
        }
    }
    assert!(markdown.contains("permission_class: redacted_permission_class"));
    assert!(
        markdown
            .contains("active_permission_effects: admission_protected, redacted_permission_effect")
    );
    assert!(
        markdown
            .contains("inactive_permission_effects: inactive_relay, redacted_permission_effect")
    );
}

#[test]
fn inbound_support_preserves_unavailable_reason_in_json_and_markdown() {
    // Arrange
    let temp = TestDirectory::new("inbound-support-unavailable");
    let mut status = phase72_status();
    status.peers.inbound = FieldAvailability::unavailable("inbound probe not collected");
    let bundle = phase77_support_bundle_with_status(temp.path(), status);

    // Act
    let serialized = serde_json::to_value(&bundle).expect("support bundle json");
    let markdown = render::render_support_markdown(&bundle);

    // Assert
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["state"],
        json!("unavailable")
    );
    assert_eq!(
        serialized["status"]["peers"]["inbound"]["value"]["reason"],
        json!("inbound probe not collected")
    );
    assert!(markdown.contains("## Inbound Serving"));
    assert!(markdown.contains("Status: Unavailable: inbound probe not collected"));
}

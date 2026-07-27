// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use super::*;

#[test]
fn open_bitcoin_config_default_disables_inbound_listener() {
    // Arrange / Act
    let config = OpenBitcoinConfig::default();
    let runtime = RuntimeConfig::default();

    // Assert
    assert!(!config.inbound.enabled);
    assert!(!runtime.inbound.enabled);
    assert_eq!(
        runtime.inbound.listen_addresses,
        vec!["127.0.0.1:18444".to_string()]
    );
    assert_eq!(runtime.inbound.max_peers, 8);
    assert_eq!(runtime.inbound.reserved_slots, 0);
    assert!(!runtime.inbound.allow_public);
}

#[test]
fn open_bitcoin_jsonc_accepts_inbound_listener_contract() {
    // Arrange
    let text = r#"
    {
      "inbound": {
        "enabled": true,
        "listen_addresses": ["127.0.0.1:18444"],
        "max_peers": 8,
        "reserved_slots": 1,
        "allow_public": false
      }
    }
    "#;
    let sandbox = TestDirectory::new("inbound-jsonc-contract");
    fs::write(sandbox.child("open-bitcoin.jsonc"), text).expect("open bitcoin config");

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("inbound runtime config");
    let preflight = classify_inbound_preflight(&runtime.inbound);

    // Assert
    assert!(config.inbound.enabled);
    assert_eq!(
        config.inbound.listen_addresses,
        vec!["127.0.0.1:18444".to_string()]
    );
    assert_eq!(config.inbound.max_peers, 8);
    assert_eq!(config.inbound.reserved_slots, 1);
    assert!(!config.inbound.allow_public);
    assert!(runtime.inbound.enabled);
    assert_eq!(
        runtime.inbound.listen_addresses,
        vec!["127.0.0.1:18444".to_string()]
    );
    assert_eq!(runtime.inbound.max_peers, 8);
    assert_eq!(runtime.inbound.reserved_slots, 1);
    assert!(!runtime.inbound.allow_public);
    assert_eq!(preflight.reason(), InboundPreflightReason::Ready);
    assert_eq!(preflight.ready_endpoints()[0].normalized, "127.0.0.1:18444");
}

#[test]
fn open_bitcoin_jsonc_accepts_inbound_permission_classes() {
    // Arrange
    let text = r#"
    {
      "inbound": {
        "permission_classes": [
          {
            "name": "operator_loopback",
            "addresses": ["127.0.0.1"],
            "permissions": ["in", "noban", "forceinbound", "download", "addr"]
          }
        ]
      }
    }
    "#;
    let sandbox = TestDirectory::new("inbound-permission-jsonc");
    fs::write(sandbox.child("open-bitcoin.jsonc"), text).expect("open bitcoin config");

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("inbound permission runtime config");
    let permissioned = runtime
        .inbound
        .permission_classes
        .resolve_inbound("127.0.0.1".parse().expect("literal ip"));
    let ordinary = runtime
        .inbound
        .permission_classes
        .resolve_inbound("127.0.0.2".parse().expect("literal ip"));

    // Assert
    assert_eq!(config.inbound.permission_classes.len(), 1);
    assert_eq!(
        config.inbound.permission_classes[0].name,
        "operator_loopback"
    );
    assert_eq!(
        permissioned.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert_eq!(
        permissioned.slot_class(),
        InboundAdmissionSlotClass::Reserved
    );
    assert!(
        permissioned
            .active_effects()
            .contains(&PermissionEffectLabel::AdmissionProtected)
    );
    assert!(
        permissioned
            .active_effects()
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
    assert!(
        permissioned
            .active_effects()
            .contains(&PermissionEffectLabel::AddressResponsePolicyInput)
    );
    assert!(permissioned.inactive_effects().is_empty());
    assert_eq!(
        ordinary.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
}

#[test]
fn open_bitcoin_jsonc_rejects_unknown_inbound_permission_class_fields() {
    // Arrange
    let text = r#"
    {
      "inbound": {
        "permission_classes": [
          {
            "name": "operator_loopback",
            "addresses": ["127.0.0.1"],
            "permissions": ["in", "noban"],
            "surprise": true
          }
        ]
      }
    }
    "#;

    // Act
    let error = parse_open_bitcoin_jsonc_config(text).expect_err("unknown field should fail");

    // Assert
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("surprise"));
}

#[test]
fn inbound_permission_config_rejects_malformed_classes_with_stable_errors() {
    let cases = [
        (
            "empty-name",
            r#"{ "name": "", "addresses": ["127.0.0.1"], "permissions": ["in", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].name value \"\" (empty_class_name): peer permission class name must not be empty",
        ),
        (
            "empty-addresses",
            r#"{ "name": "operator_loopback", "addresses": [], "permissions": ["in", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].addresses value \"\" (empty_address_list): peer permission class must include at least one literal IP address",
        ),
        (
            "cidr-address",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1/24"], "permissions": ["in", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].addresses value \"127.0.0.1/24\" (invalid_literal_ip_address): peer permission class address must be a literal IP address: 127.0.0.1/24",
        ),
        (
            "hostname-address",
            r#"{ "name": "operator_loopback", "addresses": ["localhost"], "permissions": ["in", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].addresses value \"localhost\" (invalid_literal_ip_address): peer permission class address must be a literal IP address: localhost",
        ),
        (
            "socket-endpoint",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1:8333"], "permissions": ["in", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].addresses value \"127.0.0.1:8333\" (invalid_literal_ip_address): peer permission class address must be a literal IP address: 127.0.0.1:8333",
        ),
        (
            "unsupported-token",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1"], "permissions": ["in", "oopsie"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].permissions token \"oopsie\" (unsupported_token): unsupported peer permission token: oopsie",
        ),
        (
            "direction-only",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1"], "permissions": ["in"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].permissions token \"in\" (direction_only): peer permission class sets only direction token: in",
        ),
        (
            "missing-in",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1"], "permissions": ["noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].permissions token \"in\" (missing_inbound_direction): peer permission class must include the in direction",
        ),
        (
            "out-combination",
            r#"{ "name": "operator_loopback", "addresses": ["127.0.0.1"], "permissions": ["in", "out", "noban"] }"#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[0].permissions token \"out\" (outbound_direction_unsupported): peer permission class cannot include the out direction in Phase 91",
        ),
        (
            "duplicate-address",
            r#"
            { "name": "operator_loopback", "addresses": ["127.0.0.1"], "permissions": ["in", "noban"] },
            { "name": "operator_second", "addresses": ["127.0.0.1"], "permissions": ["in", "download"] }
            "#,
            "Error resolving Open Bitcoin inbound config: inbound.permission_classes[1].addresses value \"127.0.0.1\" (duplicate_literal_ip_address): peer permission class address duplicates an earlier permission class.",
        ),
    ];

    for (label, class_json, expected_error) in cases {
        // Arrange
        let sandbox = TestDirectory::new(&format!("inbound-permission-invalid-{label}"));
        fs::write(
            sandbox.child("open-bitcoin.jsonc"),
            format!(
                r#"
                {{
                  "inbound": {{
                    "permission_classes": [{class_json}]
                  }}
                }}
                "#
            ),
        )
        .expect("open bitcoin config");

        // Act
        let error =
            load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
                .expect_err("invalid inbound permission class should fail");

        // Assert
        assert_eq!(error.to_string(), expected_error, "{label}");
    }
}

#[test]
fn open_bitcoin_jsonc_rejects_unknown_inbound_fields() {
    // Arrange
    let text = r#"
    {
      "inbound": {
        "enabled": false,
        "surprise": true
      }
    }
    "#;

    // Act
    let error = parse_open_bitcoin_jsonc_config(text).expect_err("unknown field should fail");

    // Assert
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("surprise"));
}

#[test]
fn inbound_config_rejects_zero_max_peers_and_reserved_slots_over_cap() {
    // Arrange
    let zero_max_sandbox = TestDirectory::new("inbound-zero-max");
    fs::write(
        zero_max_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "enabled": true,
            "listen_addresses": ["127.0.0.1:18444"],
            "max_peers": 0
          }
        }
        "#,
    )
    .expect("open bitcoin config");
    let reserved_over_cap_sandbox = TestDirectory::new("inbound-reserved-over-cap");
    fs::write(
        reserved_over_cap_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "enabled": true,
            "listen_addresses": ["127.0.0.1:18444"],
            "max_peers": 2,
            "reserved_slots": 3
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let zero_max_error = load_runtime_config_for_args(
        &[cli_arg("datadir", &zero_max_sandbox.path)],
        &zero_max_sandbox.path,
    )
    .expect_err("zero max peers should fail");
    let reserved_over_cap_error = load_runtime_config_for_args(
        &[cli_arg("datadir", &reserved_over_cap_sandbox.path)],
        &reserved_over_cap_sandbox.path,
    )
    .expect_err("reserved slots over cap should fail");

    // Assert
    assert_eq!(
        zero_max_error.to_string(),
        "Error resolving Open Bitcoin inbound config: inbound.max_peers must be greater than zero."
    );
    assert_eq!(
        reserved_over_cap_error.to_string(),
        "Error resolving Open Bitcoin inbound config: inbound.reserved_slots must be less than or equal to inbound.max_peers."
    );
}

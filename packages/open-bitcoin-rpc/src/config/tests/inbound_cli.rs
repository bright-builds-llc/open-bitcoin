// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use super::*;

#[test]
fn daemon_inbound_cli_override_can_enable_or_disable_open_bitcoin_jsonc() {
    // Arrange
    let disabled_sandbox = TestDirectory::new("daemon-inbound-cli-enable");
    fs::write(
        disabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "enabled": false,
            "listen_addresses": ["127.0.0.1:18444"],
            "max_peers": 8
          }
        }
        "#,
    )
    .expect("open bitcoin config");
    let enabled_sandbox = TestDirectory::new("daemon-inbound-cli-disable");
    fs::write(
        enabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "enabled": true,
            "listen_addresses": ["127.0.0.1:18444"],
            "max_peers": 8
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let cli_enabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &disabled_sandbox.path),
            os("-openbitcoininbound=1"),
            os("-openbitcoinmaxinbound=5"),
            os("-openbitcoinreservedslots=2"),
        ],
        &disabled_sandbox.path,
    )
    .expect("cli enables inbound");
    let cli_disabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &enabled_sandbox.path),
            os("-openbitcoininbound=0"),
        ],
        &enabled_sandbox.path,
    )
    .expect("cli disables inbound");

    // Assert
    assert!(cli_enabled.inbound.enabled);
    assert_eq!(cli_enabled.inbound.max_peers, 5);
    assert_eq!(cli_enabled.inbound.reserved_slots, 2);
    assert!(!cli_disabled.inbound.enabled);
}

#[test]
fn daemon_inbound_cli_listen_values_override_jsonc_in_order() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-inbound-cli-listen-order");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "enabled": true,
            "listen_addresses": ["127.0.0.1:18444"],
            "max_peers": 8
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &sandbox.path),
            os("-openbitcoinlisten=127.0.0.1:18445"),
            os("-openbitcoinlisten=[::1]:18446"),
        ],
        &sandbox.path,
    )
    .expect("cli listen override");

    // Assert
    assert_eq!(
        runtime.inbound.listen_addresses,
        vec!["127.0.0.1:18445".to_string(), "[::1]:18446".to_string(),]
    );
}

#[test]
fn daemon_inbound_permission_cli_overrides_jsonc_permission_classes() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-inbound-permission-cli-override");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "inbound": {
            "permission_classes": [
              {
                "name": "jsonc_loopback",
                "addresses": ["127.0.0.2"],
                "permissions": ["in", "noban", "download"]
              }
            ]
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &sandbox.path),
            os("-openbitcoininboundpermissionclass=name@127.0.0.1=in,noban,forceinbound,download,addr"),
            os("-openbitcoininboundpermissionclass=download_only@127.0.0.3=in,download"),
        ],
        &sandbox.path,
    )
    .expect("cli permission class override");
    let protected = runtime
        .inbound
        .permission_classes
        .resolve_inbound("127.0.0.1".parse().expect("literal ip"));
    let jsonc_replaced = runtime
        .inbound
        .permission_classes
        .resolve_inbound("127.0.0.2".parse().expect("literal ip"));
    let permissioned = runtime
        .inbound
        .permission_classes
        .resolve_inbound("127.0.0.3".parse().expect("literal ip"));

    // Assert
    assert_eq!(
        protected.connection_class(),
        PeerConnectionClass::ProtectedInbound
    );
    assert!(
        protected
            .active_effects()
            .contains(&PermissionEffectLabel::AdmissionProtected)
    );
    assert_eq!(
        jsonc_replaced.connection_class(),
        PeerConnectionClass::OrdinaryInbound
    );
    assert_eq!(
        permissioned.connection_class(),
        PeerConnectionClass::PermissionedInbound
    );
    assert!(
        permissioned
            .active_effects()
            .contains(&PermissionEffectLabel::DownloadServingPolicyInput)
    );
}

#[test]
fn daemon_inbound_permission_cli_rejects_missing_and_malformed_specs() {
    let cases = [
        (
            "missing-value",
            os("-openbitcoininboundpermissionclass"),
            "Error parsing command line arguments: Can not set -openbitcoininboundpermissionclass with no value. Please specify value with -openbitcoininboundpermissionclass=value.",
        ),
        (
            "missing-at",
            os("-openbitcoininboundpermissionclass=name=127.0.0.1=in,noban"),
            "Error parsing command line arguments: -openbitcoininboundpermissionclass must use <class_name>@<literal_ip>=<comma-separated tokens>.",
        ),
        (
            "missing-equals",
            os("-openbitcoininboundpermissionclass=name@127.0.0.1"),
            "Error parsing command line arguments: -openbitcoininboundpermissionclass must use <class_name>@<literal_ip>=<comma-separated tokens>.",
        ),
        (
            "missing-permissions",
            os("-openbitcoininboundpermissionclass=name@127.0.0.1="),
            "Error parsing command line arguments: -openbitcoininboundpermissionclass must use <class_name>@<literal_ip>=<comma-separated tokens>.",
        ),
    ];

    for (label, arg, expected_error) in cases {
        // Arrange
        let sandbox = TestDirectory::new(&format!("daemon-inbound-permission-cli-{label}"));

        // Act
        let error = load_runtime_config_for_args(&[arg], &sandbox.path)
            .expect_err("invalid permission CLI spec should fail");

        // Assert
        assert_eq!(error.to_string(), expected_error, "{label}");
    }
}

#[test]
fn daemon_inbound_rejects_baseline_listener_and_permission_keys() {
    for key in [
        "listen",
        "bind",
        "whitebind",
        "whitelist",
        "whitelistrelay",
        "whitelistforcerelay",
    ] {
        // Arrange
        let sandbox = TestDirectory::new(&format!("daemon-inbound-baseline-{key}"));
        let conf_path = sandbox.child("bitcoin.conf");
        fs::write(&conf_path, format!("{key}=127.0.0.1:18444\n")).expect("config");

        // Act
        let error = load_runtime_config_for_args(&[cli_arg("conf", &conf_path)], &sandbox.path)
            .expect_err("baseline listener key should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            format!("Error reading configuration file: Invalid configuration value {key}")
        );
    }
}

#[test]
fn daemon_inbound_rejects_baseline_permission_cli_parameters() {
    for key in [
        "whitelist",
        "whitebind",
        "whitelistrelay",
        "whitelistforcerelay",
    ] {
        // Arrange
        let sandbox = TestDirectory::new(&format!("daemon-inbound-baseline-cli-{key}"));
        let arg = os(&format!("-{key}=127.0.0.1"));

        // Act
        let error = load_runtime_config_for_args(&[arg], &sandbox.path)
            .expect_err("baseline permission CLI key should fail");

        // Assert
        assert_eq!(
            error.to_string(),
            format!("Error parsing command line arguments: Invalid parameter -{key}=127.0.0.1")
        );
    }
}

#[test]
fn daemon_inbound_cli_public_endpoint_stays_unsafe_without_allow_public() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-inbound-public-preflight");

    // Act
    let runtime = load_runtime_config_for_args(
        &[
            os("-openbitcoininbound=1"),
            os("-openbitcoinlisten=0.0.0.0:18444"),
        ],
        &sandbox.path,
    )
    .expect("public endpoint config loads");
    let preflight = classify_inbound_preflight(&runtime.inbound);
    let allowed_runtime = load_runtime_config_for_args(
        &[
            os("-openbitcoininbound=1"),
            os("-openbitcoinlisten=0.0.0.0:18444"),
            os("-openbitcoinallowpublic=1"),
        ],
        &sandbox.path,
    )
    .expect("public endpoint config loads with acknowledgement");
    let allowed_preflight = classify_inbound_preflight(&allowed_runtime.inbound);

    // Assert
    assert!(runtime.inbound.enabled);
    assert!(!runtime.inbound.allow_public);
    assert_eq!(preflight.reason(), InboundPreflightReason::UnsafeEndpoint);
    assert_eq!(preflight.diagnostics()[0].field, "inbound.listen_addresses");
    assert!(allowed_runtime.inbound.allow_public);
    assert_eq!(allowed_preflight.reason(), InboundPreflightReason::Ready);
}

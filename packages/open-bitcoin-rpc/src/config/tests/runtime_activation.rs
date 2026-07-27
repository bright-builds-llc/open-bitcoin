// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use super::*;

#[test]
fn open_bitcoin_jsonc_accepts_comments() {
    // Arrange
    let text = r#"
    {
      // Open Bitcoin-owned runtime settings.
      "metrics": {
        "enabled": true,
        "sample_interval_seconds": 30,
      },
      "logs": {
        "rotation": "daily",
        "max_files": 14,
      },
    }
    "#;

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");

    // Assert
    assert!(config.metrics.enabled);
    assert_eq!(config.metrics.sample_interval_seconds, 30);
    assert_eq!(config.logs.rotation, "daily");
    assert_eq!(config.logs.max_files, 14);
}

#[test]
fn open_bitcoin_jsonc_defaults_relay_activation_to_disabled() {
    // Arrange
    let sandbox = TestDirectory::new("relay-default");

    // Act
    let config = OpenBitcoinConfig::default();
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("default runtime config");

    // Assert
    assert!(!config.relay.enabled);
    assert!(!config.relay.to_activation_config().enabled);
    assert!(!runtime.relay.enabled);
}

#[test]
fn open_bitcoin_jsonc_accepts_relay_activation_enabled() {
    // Arrange
    let sandbox = TestDirectory::new("relay-jsonc-enabled");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "relay": {
            "enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let config = parse_open_bitcoin_jsonc_config(
        r#"
        {
          "relay": {
            "enabled": true
          }
        }
        "#,
    )
    .expect("jsonc config");
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("runtime config");

    // Assert
    assert!(config.relay.enabled);
    assert!(config.relay.to_activation_config().enabled);
    assert!(runtime.relay.enabled);
}

#[test]
fn open_bitcoin_jsonc_rejects_unknown_relay_fields() {
    // Arrange
    let text = r#"
    {
      "relay": {
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
fn open_bitcoin_jsonc_defaults_block_serving_activation_to_disabled() {
    // Arrange
    let sandbox = TestDirectory::new("block-serving-default");

    // Act
    let config = OpenBitcoinConfig::default();
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("default runtime config");

    // Assert
    assert!(!config.block_serving.enabled);
    assert!(!config.block_serving.compact_relay_enabled);
    assert!(
        !config
            .block_serving
            .to_activation_policy()
            .block_serving
            .enabled
    );
    assert!(
        !config
            .block_serving
            .to_activation_policy()
            .compact_relay
            .enabled
    );
    assert!(!runtime.block_serving.block_serving.enabled);
    assert!(!runtime.block_serving.compact_relay.enabled);
}

#[test]
fn open_bitcoin_jsonc_accepts_block_serving_activation_enabled() {
    // Arrange
    let sandbox = TestDirectory::new("block-serving-jsonc-enabled");
    let text = r#"
    {
      "block_serving": {
        "enabled": true,
        "compact_relay_enabled": true
      }
    }
    "#;
    fs::write(sandbox.child("open-bitcoin.jsonc"), text).expect("open bitcoin config");

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("runtime config");

    // Assert
    assert!(config.block_serving.enabled);
    assert!(config.block_serving.compact_relay_enabled);
    assert!(
        config
            .block_serving
            .to_activation_policy()
            .block_serving
            .enabled
    );
    assert!(
        config
            .block_serving
            .to_activation_policy()
            .compact_relay
            .enabled
    );
    assert!(runtime.block_serving.block_serving.enabled);
    assert!(runtime.block_serving.compact_relay.enabled);
}

#[test]
fn open_bitcoin_jsonc_rejects_unknown_block_serving_fields() {
    // Arrange
    let text = r#"
    {
      "block_serving": {
        "enabled": false,
        "archive": true
      }
    }
    "#;

    // Act
    let error = parse_open_bitcoin_jsonc_config(text).expect_err("unknown field should fail");

    // Assert
    assert!(error.to_string().contains("unknown field"));
    assert!(error.to_string().contains("archive"));
}

#[test]
fn daemon_block_serving_cli_override_can_enable_or_disable_open_bitcoin_jsonc() {
    // Arrange
    let disabled_sandbox = TestDirectory::new("daemon-block-serving-cli-enable");
    fs::write(
        disabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "block_serving": {
            "enabled": false,
            "compact_relay_enabled": false
          }
        }
        "#,
    )
    .expect("open bitcoin config");
    let enabled_sandbox = TestDirectory::new("daemon-block-serving-cli-disable");
    fs::write(
        enabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "block_serving": {
            "enabled": true,
            "compact_relay_enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let cli_enabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &disabled_sandbox.path),
            os("-openbitcoinblockserving"),
            os("-openbitcoincompactrelay"),
        ],
        &disabled_sandbox.path,
    )
    .expect("cli enables block serving");
    let cli_disabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &enabled_sandbox.path),
            os("-openbitcoinblockserving=0"),
            os("-noopenbitcoincompactrelay"),
        ],
        &enabled_sandbox.path,
    )
    .expect("cli disables block serving");

    // Assert
    assert!(cli_enabled.block_serving.block_serving.enabled);
    assert!(cli_enabled.block_serving.compact_relay.enabled);
    assert!(!cli_disabled.block_serving.block_serving.enabled);
    assert!(!cli_disabled.block_serving.compact_relay.enabled);
}

#[test]
fn daemon_block_serving_cli_accepts_negated_open_bitcoin_flag() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-block-serving-cli-negated");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "block_serving": {
            "enabled": true,
            "compact_relay_enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &sandbox.path),
            os("-noopenbitcoinblockserving"),
            os("-noopenbitcoincompactrelay"),
        ],
        &sandbox.path,
    )
    .expect("cli disables block serving");

    // Assert
    assert!(!runtime.block_serving.block_serving.enabled);
    assert!(!runtime.block_serving.compact_relay.enabled);
}

#[test]
fn daemon_relay_activation_does_not_enable_block_serving() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-relay-is-not-block-serving");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "relay": {
            "enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let jsonc_runtime =
        load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
            .expect("jsonc relay runtime");
    let cli_runtime = load_runtime_config_for_args(
        &[cli_arg("datadir", &sandbox.path), os("-openbitcoinrelay")],
        &sandbox.path,
    )
    .expect("cli relay runtime");

    // Assert
    assert!(jsonc_runtime.relay.enabled);
    assert!(cli_runtime.relay.enabled);
    assert!(!jsonc_runtime.block_serving.block_serving.enabled);
    assert!(!jsonc_runtime.block_serving.compact_relay.enabled);
    assert!(!cli_runtime.block_serving.block_serving.enabled);
    assert!(!cli_runtime.block_serving.compact_relay.enabled);
}

#[test]
fn daemon_relay_cli_override_can_enable_or_disable_open_bitcoin_jsonc() {
    // Arrange
    let disabled_sandbox = TestDirectory::new("daemon-relay-cli-enable");
    fs::write(
        disabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "relay": {
            "enabled": false
          }
        }
        "#,
    )
    .expect("open bitcoin config");
    let enabled_sandbox = TestDirectory::new("daemon-relay-cli-disable");
    fs::write(
        enabled_sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "relay": {
            "enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let cli_enabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &disabled_sandbox.path),
            os("-openbitcoinrelay=1"),
        ],
        &disabled_sandbox.path,
    )
    .expect("cli enables relay");
    let cli_disabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &enabled_sandbox.path),
            os("-openbitcoinrelay=0"),
        ],
        &enabled_sandbox.path,
    )
    .expect("cli disables relay");

    // Assert
    assert!(cli_enabled.relay.enabled);
    assert!(!cli_disabled.relay.enabled);
}

#[test]
fn daemon_relay_cli_accepts_negated_open_bitcoin_flag() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-relay-cli-negated");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "relay": {
            "enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(
        &[cli_arg("datadir", &sandbox.path), os("-noopenbitcoinrelay")],
        &sandbox.path,
    )
    .expect("cli disables relay");

    // Assert
    assert!(!runtime.relay.enabled);
}

#[test]
fn daemon_relay_cli_rejects_invalid_boolean() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-relay-cli-invalid-bool");

    // Act
    let error = load_runtime_config_for_args(&[os("-openbitcoinrelay=maybe")], &sandbox.path)
        .expect_err("invalid relay boolean should fail");

    // Assert
    assert_eq!(error.to_string(), "invalid boolean value: maybe");
}

#[test]
fn open_bitcoin_jsonc_accepts_wizard_onboarding_answers() {
    // Arrange
    let text = r#"
    {
      "onboarding": {
        "non_interactive": true,
        "completed_steps": ["network"],
        "wizard_answers": {
          "network": "signet",
          "datadir": "/tmp/open-bitcoin"
        }
      }
    }
    "#;

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");

    // Assert
    assert!(config.onboarding.non_interactive);
    assert_eq!(config.onboarding.completed_steps, vec!["network"]);
    assert_eq!(
        config.onboarding.wizard_answers.get("network"),
        Some(&"signet".to_string())
    );
    assert_eq!(
        config.onboarding.wizard_answers.get("datadir"),
        Some(&"/tmp/open-bitcoin".to_string())
    );
}

#[test]
fn open_bitcoin_jsonc_accepts_mainnet_sync_activation_contract() {
    // Arrange
    let text = r#"
    {
      "sync": {
        "network_enabled": true,
        "mode": "mainnet-ibd"
      }
    }
    "#;

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");

    // Assert
    assert!(config.sync.network_enabled);
    assert_eq!(config.sync.mode, "mainnet-ibd");
    assert_eq!(config.sync.maybe_manual_peers, None);
    assert_eq!(config.sync.maybe_dns_seeds, None);
    assert_eq!(config.sync.maybe_target_outbound_peers, None);
    assert_eq!(config.sync.maybe_target_header_height, None);
    assert_eq!(config.sync.maybe_max_messages_per_peer, None);
    assert_eq!(config.sync.maybe_max_rounds, None);
    assert_eq!(config.sync.maybe_max_blocks_in_flight_per_peer, None);
    assert_eq!(config.sync.maybe_max_blocks_in_flight_total, None);
}

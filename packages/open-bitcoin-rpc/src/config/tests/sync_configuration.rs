// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use super::*;

#[test]
fn open_bitcoin_jsonc_accepts_manual_peers_seed_overrides_and_resource_bounds() {
    // Arrange
    let text = r#"
    {
      "sync": {
        "network_enabled": true,
        "mode": "mainnet-ibd",
        "manual_peers": ["198.51.100.10:8333", "[2001:db8::7]:8334"],
        "dns_seeds": ["seed-one.example:8335", "seed-two.example"],
        "target_outbound_peers": 2,
        "target_header_height": 144,
        "max_messages_per_peer": 12,
        "max_rounds": 3,
        "max_blocks_in_flight_per_peer": 4,
        "max_blocks_in_flight_total": 10
      }
    }
    "#;

    // Act
    let config = parse_open_bitcoin_jsonc_config(text).expect("jsonc config");

    // Assert
    assert_eq!(
        config.sync.maybe_manual_peers,
        Some(vec![
            "198.51.100.10:8333".to_string(),
            "[2001:db8::7]:8334".to_string(),
        ])
    );
    assert_eq!(
        config.sync.maybe_dns_seeds,
        Some(vec![
            "seed-one.example:8335".to_string(),
            "seed-two.example".to_string(),
        ])
    );
    assert_eq!(config.sync.maybe_target_outbound_peers, Some(2));
    assert_eq!(config.sync.maybe_target_header_height, Some(144));
    assert_eq!(config.sync.maybe_max_messages_per_peer, Some(12));
    assert_eq!(config.sync.maybe_max_rounds, Some(3));
    assert_eq!(config.sync.maybe_max_blocks_in_flight_per_peer, Some(4));
    assert_eq!(config.sync.maybe_max_blocks_in_flight_total, Some(10));
}

#[test]
fn daemon_sync_loads_from_open_bitcoin_jsonc_when_explicitly_enabled() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-jsonc");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd"
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("mainnet sync config should load");

    // Assert
    assert_eq!(runtime.chain, AddressNetwork::Mainnet);
    assert_eq!(runtime.sync.mode, DaemonSyncMode::MainnetIbd);
    assert_eq!(runtime.sync.runtime.network, SyncNetwork::Mainnet);
    assert!(runtime.sync.is_enabled());
}

#[test]
fn daemon_sync_jsonc_applies_manual_peers_seed_overrides_and_resource_bounds() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-peer-config");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd",
            "manual_peers": ["198.51.100.10", "203.0.113.2:8334"],
            "dns_seeds": ["seed-one.example:8335"],
            "target_outbound_peers": 2,
            "target_header_height": 144,
            "max_messages_per_peer": 12,
            "max_rounds": 3,
            "max_blocks_in_flight_per_peer": 4,
            "max_blocks_in_flight_total": 10
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let runtime = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect("mainnet sync config should load");

    // Assert
    assert_eq!(runtime.sync.runtime.target_outbound_peers, 2);
    assert_eq!(runtime.sync.runtime.maybe_target_header_height, Some(144));
    assert_eq!(runtime.sync.runtime.max_messages_per_peer, 12);
    assert_eq!(runtime.sync.runtime.max_rounds, 3);
    assert_eq!(runtime.sync.runtime.max_blocks_in_flight_per_peer, 4);
    assert_eq!(runtime.sync.runtime.max_blocks_in_flight_total, 10);
    assert_eq!(
        runtime.sync.runtime.manual_peers,
        vec![
            SyncPeerAddress::manual("198.51.100.10", 8333),
            SyncPeerAddress::manual("203.0.113.2", 8334),
        ]
    );
    assert_eq!(
        runtime.sync.runtime.dns_seeds,
        vec!["seed-one.example:8335".to_string()]
    );
}

#[test]
fn daemon_sync_rejects_zero_resource_bounds() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-zero-bound");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd",
            "max_blocks_in_flight_total": 0
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let error = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect_err("zero resource bound should fail");

    // Assert
    assert_eq!(
        error.to_string(),
        "Error reading open-bitcoin.jsonc: sync.max_blocks_in_flight_total must be greater than zero."
    );
}

#[test]
fn daemon_sync_rejects_zero_target_header_height() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-zero-header-target");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd",
            "target_header_height": 0
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let error = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect_err("zero target header height should fail");

    // Assert
    assert_eq!(
        error.to_string(),
        "Error reading open-bitcoin.jsonc: sync.target_header_height must be greater than zero."
    );
}

#[test]
fn daemon_sync_cli_override_can_enable_or_disable_open_bitcoin_jsonc() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-cli");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd"
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let cli_enabled =
        load_runtime_config_for_args(&[os("-openbitcoinsync=mainnet-ibd")], &sandbox.path)
            .expect("cli sync enable");
    let cli_disabled = load_runtime_config_for_args(
        &[
            cli_arg("datadir", &sandbox.path),
            os("-openbitcoinsync=disabled"),
        ],
        &sandbox.path,
    )
    .expect("cli sync disable");

    // Assert
    assert_eq!(cli_enabled.sync.mode, DaemonSyncMode::MainnetIbd);
    assert!(cli_enabled.sync.is_enabled());
    assert_eq!(cli_disabled.sync.mode, DaemonSyncMode::Disabled);
    assert!(!cli_disabled.sync.is_enabled());
}

#[test]
fn daemon_sync_rejects_partial_or_non_mainnet_activation() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-rejections");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let partial_error =
        load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
            .expect_err("partial JSONC activation should fail");
    let non_mainnet_error = load_runtime_config_for_args(
        &[os("-regtest"), os("-openbitcoinsync=mainnet-ibd")],
        &sandbox.path,
    )
    .expect_err("non-mainnet activation should fail");

    // Assert
    assert_eq!(
        partial_error.to_string(),
        "Error reading open-bitcoin.jsonc: sync.network_enabled requires sync.mode = \"mainnet-ibd\" for daemon mainnet sync activation."
    );
    assert_eq!(
        non_mainnet_error.to_string(),
        "open-bitcoind mainnet sync activation requires -chain=main or -main; current chain is regtest."
    );
}

#[test]
fn daemon_sync_rejects_invalid_peer_config() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-invalid-peer-config");
    fs::write(
        sandbox.child("open-bitcoin.jsonc"),
        r#"
        {
          "sync": {
            "network_enabled": true,
            "mode": "mainnet-ibd",
            "manual_peers": ["localhost:not-a-port"]
          }
        }
        "#,
    )
    .expect("open bitcoin config");

    // Act
    let error = load_runtime_config_for_args(&[cli_arg("datadir", &sandbox.path)], &sandbox.path)
        .expect_err("invalid sync peer should fail");

    // Assert
    assert_eq!(error.to_string(), "invalid rpc port: not-a-port");
}

#[test]
fn daemon_sync_rejects_unreadable_explicit_open_bitcoin_jsonc_path() {
    // Arrange
    let sandbox = TestDirectory::new("daemon-sync-missing-jsonc");
    let missing_config = sandbox.child("missing-open-bitcoin.jsonc");

    // Act
    let error = load_runtime_config_for_args(
        &[cli_arg("openbitcoinconf", &missing_config)],
        &sandbox.path,
    )
    .expect_err("explicit config path should fail");

    // Assert
    assert_eq!(
        error.to_string(),
        format!(
            "Error reading open-bitcoin.jsonc: specified Open Bitcoin config file \"{}\" could not be opened.",
            missing_config.display()
        )
    );
}

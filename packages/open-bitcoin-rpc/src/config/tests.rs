// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use std::{
    ffi::OsString,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use open_bitcoin_node::{SyncNetwork, SyncPeerAddress, core::wallet::AddressNetwork};

use open_bitcoin_network::{InboundPreflightReason, classify_inbound_preflight};

use super::{
    ConfigPrecedence, ConfigSource, DEFAULT_COOKIE_FILE_NAME, DaemonSyncMode, OpenBitcoinConfig,
    RpcAuthConfig, RuntimeConfig, WalletRuntimeConfig, WalletRuntimeScope,
    load_runtime_config_for_args, parse_open_bitcoin_jsonc_config,
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-rpc-config-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn cli_arg(name: &str, value: &Path) -> OsString {
    OsString::from(format!("-{name}={}", value.display()))
}

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[test]
fn runtime_config_defaults_to_local_single_wallet_auth() {
    // Arrange
    let expected_bind = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 8332);

    // Act
    let runtime = RuntimeConfig::default();

    // Assert
    assert_eq!(runtime.chain, AddressNetwork::Mainnet);
    assert_eq!(runtime.maybe_data_dir, None);
    assert_eq!(runtime.rpc_server.bind_address, expected_bind);
    assert_eq!(runtime.rpc_client.endpoint.host, "127.0.0.1");
    assert_eq!(runtime.rpc_client.endpoint.port, expected_bind.port());
    assert_eq!(
        runtime.wallet.scope,
        WalletRuntimeScope::LocalOperatorSingleWallet
    );
    assert_eq!(
        runtime.wallet,
        WalletRuntimeConfig {
            scope: WalletRuntimeScope::LocalOperatorSingleWallet,
            coinbase_maturity: 100,
        }
    );
    assert_eq!(runtime.sync.mode, DaemonSyncMode::Disabled);
    assert!(!runtime.sync.is_enabled());
    assert!(matches!(
        runtime.rpc_server.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(_)
        }
    ));
    assert!(matches!(
        runtime.rpc_client.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(_)
        }
    ));
}

#[test]
fn rpcconnect_accepts_hostnames_and_preserves_port_precedence() {
    // Arrange
    let sandbox = TestDirectory::new("rpcconnect-hostnames");

    // Act
    let hostname_default =
        load_runtime_config_for_args(&[os("-rpcconnect=localhost")], &sandbox.path)
            .expect("hostname without port");
    let hostname_embedded = load_runtime_config_for_args(
        &[os("-regtest"), os("-rpcconnect=localhost:18442")],
        &sandbox.path,
    )
    .expect("hostname with embedded port");
    let explicit_port = load_runtime_config_for_args(
        &[
            os("-regtest"),
            os("-rpcconnect=localhost:18442"),
            os("-rpcport=18443"),
        ],
        &sandbox.path,
    )
    .expect("explicit port");
    let ipv4_embedded =
        load_runtime_config_for_args(&[os("-rpcconnect=127.0.0.1:8339")], &sandbox.path)
            .expect("ipv4 endpoint");
    let ipv6_embedded =
        load_runtime_config_for_args(&[os("-rpcconnect=[::1]:8339")], &sandbox.path)
            .expect("ipv6 endpoint");
    let server_bind_error =
        load_runtime_config_for_args(&[os("-rpcbind=localhost")], &sandbox.path)
            .expect_err("server bind keeps socket-only validation");

    // Assert
    assert_eq!(hostname_default.rpc_client.endpoint.host, "localhost");
    assert_eq!(hostname_default.rpc_client.endpoint.port, 8332);
    assert_eq!(hostname_embedded.rpc_client.endpoint.host, "localhost");
    assert_eq!(hostname_embedded.rpc_client.endpoint.port, 18_442);
    assert_eq!(explicit_port.rpc_client.endpoint.host, "localhost");
    assert_eq!(explicit_port.rpc_client.endpoint.port, 18_443);
    assert_eq!(ipv4_embedded.rpc_client.endpoint.host, "127.0.0.1");
    assert_eq!(ipv4_embedded.rpc_client.endpoint.port, 8339);
    assert_eq!(ipv6_embedded.rpc_client.endpoint.host, "::1");
    assert_eq!(ipv6_embedded.rpc_client.endpoint.port, 8339);
    assert_eq!(
        server_bind_error.to_string(),
        "invalid rpc address: localhost"
    );
}

#[test]
fn conf_cannot_be_set_in_configuration_files() {
    // Arrange
    let sandbox = TestDirectory::new("conf-setting");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(&conf_path, "conf=some.conf\n").expect("config");
    let cli_args = vec![cli_arg("conf", &conf_path)];

    // Act
    let direct_error =
        load_runtime_config_for_args(&cli_args, &sandbox.path).expect_err("conf must fail");

    // Assert
    assert_eq!(
        direct_error.to_string(),
        "Error reading configuration file: conf cannot be set in the configuration file; use includeconf= if you want to include additional config files",
    );

    // Arrange
    let include_path = sandbox.child("include.conf");
    fs::write(
        &conf_path,
        format!("includeconf={}\n", include_path.display()),
    )
    .expect("root config");
    fs::write(&include_path, "conf=some.conf\n").expect("include config");

    // Act
    let include_error =
        load_runtime_config_for_args(&cli_args, &sandbox.path).expect_err("included conf fails");

    // Assert
    assert_eq!(
        include_error.to_string(),
        "Error reading configuration file: conf cannot be set in the configuration file; use includeconf= if you want to include additional config files",
    );
}

#[test]
fn rpcpassword_with_hash_is_rejected() {
    // Arrange
    let sandbox = TestDirectory::new("rpcpassword-hash");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(
        &conf_path,
        "server=1\nrpcuser=someuser\nrpcpassword=some#pass\n",
    )
    .expect("config");
    let cli_args = vec![cli_arg("conf", &conf_path)];

    // Act
    let error = load_runtime_config_for_args(&cli_args, &sandbox.path).expect_err("hash must fail");

    // Assert
    assert_eq!(
        error.to_string(),
        "Error reading configuration file: parse error on line 3, using # in rpcpassword can be ambiguous and should be avoided",
    );
}

#[test]
fn cli_datadir_overrides_config_datadir() {
    // Arrange
    let sandbox = TestDirectory::new("datadir-precedence");
    let configured_data_dir = sandbox.child("configured");
    let cli_data_dir = sandbox.child("cli");
    fs::create_dir_all(&configured_data_dir).expect("configured datadir");
    fs::create_dir_all(&cli_data_dir).expect("cli datadir");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(
        &conf_path,
        format!("datadir={}\nserver=1\n", configured_data_dir.display()),
    )
    .expect("config");
    let base_args = vec![cli_arg("conf", &conf_path)];

    // Act
    let configured_runtime = load_runtime_config_for_args(&base_args, &sandbox.path)
        .expect("config datadir should load");
    let overridden_runtime = load_runtime_config_for_args(
        &[
            cli_arg("conf", &conf_path),
            cli_arg("datadir", &cli_data_dir),
        ],
        &sandbox.path,
    )
    .expect("cli datadir should win");

    // Assert
    assert_eq!(
        configured_runtime.maybe_data_dir,
        Some(configured_data_dir.clone())
    );
    assert_eq!(
        overridden_runtime.maybe_data_dir,
        Some(cli_data_dir.clone())
    );
    assert!(matches!(
        configured_runtime.rpc_server.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(ref cookie_file)
        } if cookie_file == &configured_data_dir.join(DEFAULT_COOKIE_FILE_NAME)
    ));
    assert!(matches!(
        overridden_runtime.rpc_server.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(ref cookie_file)
        } if cookie_file == &cli_data_dir.join(DEFAULT_COOKIE_FILE_NAME)
    ));
}

#[test]
fn auth_resolution_prefers_cookie_when_password_is_empty() {
    // Arrange
    let sandbox = TestDirectory::new("auth-resolution");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(
        &conf_path,
        "rpcuser=alice\nrpcpassword=\nrpccookiefile=custom.cookie\n",
    )
    .expect("config");

    // Act
    let cookie_runtime =
        load_runtime_config_for_args(&[cli_arg("conf", &conf_path)], &sandbox.path)
            .expect("empty password should use cookie auth");
    fs::write(&conf_path, "rpcuser=alice\nrpcpassword=secret\n").expect("config");
    let explicit_runtime =
        load_runtime_config_for_args(&[cli_arg("conf", &conf_path)], &sandbox.path)
            .expect("explicit auth should load");

    // Assert
    assert!(matches!(
        cookie_runtime.rpc_server.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(ref cookie_file)
        } if cookie_file == &sandbox.path.join("custom.cookie")
    ));
    assert_eq!(
        cookie_runtime.rpc_server.auth,
        cookie_runtime.rpc_client.auth
    );
    assert_eq!(
        explicit_runtime.rpc_server.auth,
        RpcAuthConfig::UserPassword {
            username: "alice".to_string(),
            password: "secret".to_string(),
        }
    );
    assert_eq!(
        explicit_runtime.rpc_client.auth,
        RpcAuthConfig::UserPassword {
            username: "alice".to_string(),
            password: "secret".to_string(),
        }
    );
}

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
fn daemon_inbound_rejects_baseline_listener_and_permission_keys() {
    for key in ["listen", "bind", "whitebind", "whitelist"] {
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

#[test]
fn open_bitcoin_jsonc_rejects_unknown_top_level_fields() {
    // Arrange
    let text = r#"{ "unknown": true }"#;

    // Act
    let error = parse_open_bitcoin_jsonc_config(text).expect_err("unknown field should fail");

    // Assert
    assert!(error.to_string().contains("unknown field"));
}

#[test]
fn config_precedence_orders_cli_env_jsonc_bitcoin_conf_cookie_defaults() {
    // Arrange / Act
    let sources = ConfigPrecedence::ordered_sources();

    // Assert
    assert_eq!(
        sources,
        [
            ConfigSource::CliFlags,
            ConfigSource::Environment,
            ConfigSource::OpenBitcoinJsonc,
            ConfigSource::BitcoinConf,
            ConfigSource::Cookies,
            ConfigSource::Defaults,
        ]
    );
}

#[test]
fn bitcoin_conf_rejects_open_bitcoin_only_keys() {
    // Arrange
    let sandbox = TestDirectory::new("open-bitcoin-only-keys");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(&conf_path, "dashboard=1\nservice=1\n").expect("config");

    // Act
    let error = load_runtime_config_for_args(&[cli_arg("conf", &conf_path)], &sandbox.path)
        .expect_err("open bitcoin keys must fail");

    // Assert
    assert_eq!(
        error.to_string(),
        "Error reading configuration file: Invalid configuration value dashboard"
    );
}

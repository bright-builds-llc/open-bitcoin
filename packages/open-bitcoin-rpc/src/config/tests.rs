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

use open_bitcoin_node::core::wallet::AddressNetwork;

use super::{
    ConfigPrecedence, ConfigSource, DEFAULT_COOKIE_FILE_NAME, RpcAuthConfig, RuntimeConfig,
    WalletRuntimeConfig, WalletRuntimeScope, load_runtime_config_for_args,
    parse_open_bitcoin_jsonc_config,
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

use std::{
    ffi::OsString,
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use open_bitcoin_node::core::wallet::AddressNetwork;

use super::{
    DEFAULT_COOKIE_FILE_NAME, RpcAuthConfig, RuntimeConfig, WalletRuntimeConfig,
    WalletRuntimeScope, load_runtime_config_for_test,
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
    assert_eq!(runtime.rpc_client.connect_address, expected_bind);
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
fn conf_cannot_be_set_in_configuration_files() {
    // Arrange
    let sandbox = TestDirectory::new("conf-setting");
    let conf_path = sandbox.child("bitcoin.conf");
    fs::write(&conf_path, "conf=some.conf\n").expect("config");
    let cli_args = vec![cli_arg("conf", &conf_path)];

    // Act
    let direct_error =
        load_runtime_config_for_test(&cli_args, &sandbox.path).expect_err("conf must fail");

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
        load_runtime_config_for_test(&cli_args, &sandbox.path).expect_err("included conf fails");

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
    let error = load_runtime_config_for_test(&cli_args, &sandbox.path).expect_err("hash must fail");

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
    let configured_runtime = load_runtime_config_for_test(&base_args, &sandbox.path)
        .expect("config datadir should load");
    let overridden_runtime = load_runtime_config_for_test(
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
        load_runtime_config_for_test(&[cli_arg("conf", &conf_path)], &sandbox.path)
            .expect("empty password should use cookie auth");
    fs::write(&conf_path, "rpcuser=alice\nrpcpassword=secret\n").expect("config");
    let explicit_runtime =
        load_runtime_config_for_test(&[cli_arg("conf", &conf_path)], &sandbox.path)
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

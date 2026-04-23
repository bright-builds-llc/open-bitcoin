use std::{
    ffi::OsString,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use open_bitcoin_rpc::config::RpcAuthConfig;

use crate::args::parse_cli_args;

use super::resolve_startup_config;

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-cli-startup-tests-{label}-{}",
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

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[test]
fn client_startup_resolves_conf_datadir_and_auth_precedence() {
    // Arrange
    let sandbox = TestDirectory::new("precedence");
    let default_data_dir = sandbox.child("default");
    let configured_data_dir = sandbox.child("configured");
    let cli_data_dir = sandbox.child("cli");
    fs::create_dir_all(&default_data_dir).expect("default datadir");
    fs::create_dir_all(&configured_data_dir).expect("configured datadir");
    fs::create_dir_all(&cli_data_dir).expect("cli datadir");

    let conf_path = sandbox.child("custom.conf");
    fs::write(
        &conf_path,
        format!(
            "datadir={}\nrpcconnect=198.51.100.8:8339\nrpcport=18443\nrpcuser=alice\nrpcpassword=\nrpccookiefile=custom.cookie\n",
            configured_data_dir.display()
        ),
    )
    .expect("config");

    let cookie_cli = parse_cli_args(
        &[
            os(&format!("-conf={}", conf_path.display())),
            os("getnetworkinfo"),
        ],
        "",
    )
    .expect("cookie startup args");
    let explicit_cli = parse_cli_args(
        &[
            os(&format!("-conf={}", conf_path.display())),
            os(&format!("-datadir={}", cli_data_dir.display())),
            os("-rpcconnect=203.0.113.25:9999"),
            os("-rpcport=8332"),
            os("-rpcuser=bob"),
            os("-rpcpassword=secret"),
            os("getnetworkinfo"),
        ],
        "",
    )
    .expect("explicit startup args");

    // Act
    let cookie_startup =
        resolve_startup_config(&cookie_cli.startup, &default_data_dir).expect("cookie auth");
    let explicit_startup =
        resolve_startup_config(&explicit_cli.startup, &default_data_dir).expect("explicit auth");

    // Assert
    assert_eq!(cookie_startup.conf_path, conf_path);
    assert_eq!(
        cookie_startup.maybe_data_dir,
        Some(configured_data_dir.clone())
    );
    assert_eq!(cookie_startup.rpc.host, "198.51.100.8");
    assert_eq!(cookie_startup.rpc.port, 18443);
    assert_eq!(
        cookie_startup.rpc.auth,
        RpcAuthConfig::Cookie {
            maybe_cookie_file: Some(configured_data_dir.join("custom.cookie")),
        }
    );

    assert_eq!(explicit_startup.conf_path, conf_path);
    assert_eq!(explicit_startup.maybe_data_dir, Some(cli_data_dir));
    assert_eq!(explicit_startup.rpc.host, "203.0.113.25");
    assert_eq!(explicit_startup.rpc.port, 8332);
    assert_eq!(
        explicit_startup.rpc.auth,
        RpcAuthConfig::UserPassword {
            username: "bob".to_string(),
            password: "secret".to_string(),
        }
    );
}

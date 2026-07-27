// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/httprpc.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp

use super::*;

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

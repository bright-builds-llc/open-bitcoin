// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::ffi::OsString;

use clap::Parser;

use super::{
    CliRoute, ConfigCommand, OperatorCli, OperatorCommand, OperatorOutputFormat,
    route_cli_invocation,
};

fn os(value: &str) -> OsString {
    OsString::from(value)
}

#[test]
fn status_command_parses_json_output() {
    // Arrange
    let argv = ["open-bitcoin", "status", "--format", "json"];

    // Act
    let parsed = OperatorCli::try_parse_from(argv).expect("operator cli should parse");

    // Assert
    assert_eq!(parsed.format, OperatorOutputFormat::Json);
    assert!(matches!(parsed.command, OperatorCommand::Status(_)));
}

#[test]
fn open_bitcoin_cli_routes_to_compat() {
    // Arrange
    let args = vec![os("-named"), os("getnetworkinfo")];

    // Act
    let route = route_cli_invocation("open-bitcoin-cli", &args).expect("route");

    // Assert
    assert_eq!(route, CliRoute::BitcoinCliCompat(args));
}

#[test]
fn open_bitcoin_routes_to_operator() {
    // Arrange
    let args = vec![os("config"), os("paths")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Config(config) = cli.command else {
        panic!("expected config command");
    };
    assert_eq!(config.command, ConfigCommand::Paths);
}

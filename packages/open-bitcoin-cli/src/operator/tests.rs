// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::ffi::OsString;

use clap::Parser;
use open_bitcoin_rpc::config::{ConfigPrecedence, ConfigSource};

use super::{
    CliRoute, ConfigCommand, OperatorCli, OperatorCommand, OperatorOutputFormat,
    config::OperatorConfigSource,
    onboarding::{OnboardingWriteDecision, ProposedConfigWrite},
    route_cli_invocation,
};

fn os(value: &str) -> OsString {
    OsString::from(value)
}

fn operator_source_from_rpc(source: ConfigSource) -> OperatorConfigSource {
    match source {
        ConfigSource::CliFlags => OperatorConfigSource::CliFlags,
        ConfigSource::Environment => OperatorConfigSource::Environment,
        ConfigSource::OpenBitcoinJsonc => OperatorConfigSource::OpenBitcoinJsonc,
        ConfigSource::BitcoinConf => OperatorConfigSource::BitcoinConf,
        ConfigSource::Cookies => OperatorConfigSource::Cookies,
        ConfigSource::Defaults => OperatorConfigSource::Defaults,
    }
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

#[test]
fn open_bitcoin_status_routes_to_operator_status() {
    // Arrange
    let args = vec![os("status")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    assert!(matches!(cli.command, OperatorCommand::Status(_)));
}

#[test]
fn operator_config_sources_follow_rpc_precedence_order() {
    // Arrange
    let rpc_sources = ConfigPrecedence::ordered_sources();

    // Act
    let operator_sources: Vec<_> = rpc_sources
        .into_iter()
        .map(operator_source_from_rpc)
        .collect();
    let operator_names: Vec<_> = operator_sources
        .iter()
        .map(|source| source.as_str())
        .collect();

    // Assert
    assert_eq!(operator_sources, OperatorConfigSource::ordered());
    assert_eq!(
        operator_names,
        vec![
            "cli_flags",
            "environment",
            "open_bitcoin_jsonc",
            "bitcoin_conf",
            "cookies",
            "defaults",
        ]
    );
}

#[test]
fn onboarding_write_decision_contract_covers_all_write_states() {
    // Arrange
    let proposed = ProposedConfigWrite {
        path: "/tmp/open-bitcoin.jsonc".into(),
        contents: "{ \"schema_version\": 1 }".to_string(),
        replaces_existing: false,
    };
    let decisions = [
        OnboardingWriteDecision::NoWrite {
            reason: "not approved".to_string(),
        },
        OnboardingWriteDecision::ProposedWrite {
            write: proposed.clone(),
        },
        OnboardingWriteDecision::ApprovedWrite { write: proposed },
    ];

    // Act
    let labels: Vec<_> = decisions
        .iter()
        .map(|decision| match decision {
            OnboardingWriteDecision::NoWrite { .. } => "no_write",
            OnboardingWriteDecision::ProposedWrite { .. } => "proposed_write",
            OnboardingWriteDecision::ApprovedWrite { .. } => "approved_write",
        })
        .collect();

    // Assert
    assert_eq!(labels, vec!["no_write", "proposed_write", "approved_write"]);
}

#[test]
fn status_contract_defers_final_json_dto_to_shared_status_binding_plan() {
    // Arrange
    let source = include_str!("status.rs");

    // Assert
    assert!(!source.contains("StatusJson"));
    assert!(!source.contains("StatusDto"));
    assert!(!source.contains("CliStatusSnapshot"));
    assert!(!source.contains("OpenBitcoinStatusSnapshot"));
}

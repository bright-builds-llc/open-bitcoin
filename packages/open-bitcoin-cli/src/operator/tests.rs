// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/test/functional/interface_bitcoin_cli.py

use std::{
    ffi::OsString,
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

use clap::Parser;
use open_bitcoin_rpc::config::{ConfigPrecedence, ConfigSource};
use serde_json::Value;

use super::{
    CliRoute, ConfigCommand, DashboardArgs, MigrationCommand, NetworkSelection, OperatorCli,
    OperatorCommand, OperatorOutputFormat, StatusArgs, SyncCommand,
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

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("time after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "open-bitcoin-operator-tests-{label}-{}-{timestamp}",
            std::process::id()
        ));
        fs::create_dir_all(&path).expect("test directory");
        Self { path }
    }

    fn child(&self, path: &str) -> PathBuf {
        self.path.join(path)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

fn decode_operator_json(outcome: &super::runtime::OperatorCommandOutcome) -> Value {
    serde_json::from_str(&outcome.stdout.text).expect("operator json")
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
fn open_bitcoin_sync_pause_routes_to_operator_sync() {
    // Arrange
    let args = vec![os("sync"), os("pause")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Sync(sync) = cli.command else {
        panic!("expected sync command");
    };
    assert_eq!(sync.command, SyncCommand::Pause);
}

#[test]
fn open_bitcoin_dashboard_routes_to_operator_dashboard() {
    // Arrange
    let args = vec![os("dashboard"), os("--tick-ms"), os("500")];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Dashboard(dashboard) = cli.command else {
        panic!("expected dashboard command");
    };
    assert_eq!(dashboard.tick_ms, 500);
}

#[test]
fn open_bitcoin_migrate_plan_routes_to_operator_command() {
    // Arrange
    let args = vec![
        os("migrate"),
        os("plan"),
        os("--source-datadir"),
        os("/tmp/core"),
    ];

    // Act
    let route = route_cli_invocation("open-bitcoin", &args).expect("route");

    // Assert
    let CliRoute::Operator(cli) = route else {
        panic!("expected operator route");
    };
    let OperatorCommand::Migrate(migration) = cli.command else {
        panic!("expected migrate command");
    };
    let MigrationCommand::Plan(plan) = migration.command;
    assert_eq!(
        plan.maybe_source_data_dir.as_deref(),
        Some(std::path::Path::new("/tmp/core"))
    );
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
fn status_contract_uses_shared_status_snapshot_without_renderer_dto() {
    // Arrange
    let source = include_str!("status.rs");

    // Assert
    assert!(!source.contains("StatusJson"));
    assert!(!source.contains("StatusDto"));
    assert!(!source.contains("CliStatusSnapshot"));
    assert!(source.contains("OpenBitcoinStatusSnapshot"));
}

#[test]
fn dashboard_command_is_no_longer_deferred_in_runtime() {
    // Arrange
    let source = include_str!("runtime.rs");

    // Assert
    assert!(!source.contains("dashboard command is deferred to Phase 19"));
    assert!(source.contains("run_dashboard"));
}

#[test]
fn status_rejects_removed_watch_flag() {
    // Arrange / Act
    let error = route_cli_invocation("open-bitcoin", &[os("status"), os("--watch")])
        .expect_err("status --watch should be rejected");

    // Assert
    assert!(error.to_string().contains("unexpected argument '--watch'"));
}

#[test]
fn status_attempts_live_rpc_without_implicit_bitcoin_conf_when_cookie_exists() {
    // Arrange
    let sandbox = TestDirectory::new("status-live-no-conf");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join(".cookie"), "__cookie__:fixture").expect("cookie");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Status(StatusArgs {}),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "unreachable");
}

#[test]
fn dashboard_reuses_live_rpc_bootstrap_without_implicit_bitcoin_conf() {
    // Arrange
    let sandbox = TestDirectory::new("dashboard-live-no-conf");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    fs::write(data_dir.join(".cookie"), "__cookie__:fixture").expect("cookie");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Dashboard(DashboardArgs { tick_ms: 1_000 }),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "unreachable");
}

#[test]
fn status_stays_stopped_when_configless_bootstrap_has_no_credentials() {
    // Arrange
    let sandbox = TestDirectory::new("status-stopped-no-credentials");
    let data_dir = sandbox.child("open-bitcoin");
    fs::create_dir_all(&data_dir).expect("datadir");
    let cli = OperatorCli {
        maybe_config_path: None,
        maybe_data_dir: Some(data_dir.clone()),
        maybe_network: Some(NetworkSelection::Regtest),
        format: OperatorOutputFormat::Json,
        no_color: true,
        command: OperatorCommand::Status(StatusArgs {}),
    };

    // Act
    let outcome = super::runtime::execute_operator_cli_with_default_data_dir(cli, data_dir.clone());
    let decoded = decode_operator_json(&outcome);

    // Assert
    assert_eq!(outcome.exit_code, super::runtime::OperatorExitCode::Success);
    assert_eq!(decoded["node"]["state"], "stopped");
}

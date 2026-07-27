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
    OperatorCommand, OperatorOutputFormat, ServiceCommand, SoakCommand, SoakPeerPolicyArg,
    SoakStopConditionArg, SoakStopReasonArg, StatusArgs, SupportCommand, SyncCommand,
    config::OperatorConfigSource,
    onboarding::{OnboardingWriteDecision, ProposedConfigWrite},
    route_cli_invocation, runtime,
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

mod routing;
mod status_bootstrap;

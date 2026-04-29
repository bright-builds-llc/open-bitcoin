// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/interfaces/node.h

use std::{fs, path::PathBuf};

use open_bitcoin_cli::operator::{
    NetworkSelection,
    config::{
        OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
        OperatorConfigSource,
    },
    dashboard::{
        DashboardRuntimeContext, DashboardServiceRuntime, collect_dashboard_snapshot,
        model::DashboardState, render_dashboard_text_snapshot,
    },
    service::{ServiceLifecycleState, ServiceStateSnapshot, fake::FakeServiceManager},
    status::{
        StatusCollectorInput, StatusDetectionEvidence, StatusRenderMode, StatusRequest,
        StatusRpcClient, StatusRpcError, StatusWalletRpcAccess, collect_status_snapshot,
        render_status,
    },
};
use open_bitcoin_node::{FjallNodeStore, PersistMode};
use open_bitcoin_rpc::method::{
    GetBalancesResponse, GetBlockchainInfoResponse, GetMempoolInfoResponse, GetNetworkInfoResponse,
    GetWalletInfoResponse, WalletBalanceDetails,
};

use crate::{
    error::BenchError,
    registry::{
        BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, OPERATOR_RUNTIME_MAPPING,
    },
    runtime_fixtures::{TempStoreDir, sample_metrics_storage_snapshot},
};

const STATUS_CASE_ID: &str = "operator-runtime.status-render";
const DASHBOARD_CASE_ID: &str = "operator-runtime.dashboard-projection";

pub const CASES: [BenchCase; 2] = [
    BenchCase {
        id: STATUS_CASE_ID,
        group: BenchGroupId::OperatorRuntime,
        description: "Collects operator status through the shared runtime collector and renders stable human and JSON output.",
        measurement: BenchMeasurement {
            focus: "status_render",
            fixture: "runtime_collected_status_snapshot",
            durability: BenchDurability::Ephemeral,
        },
        knots_mapping: &OPERATOR_RUNTIME_MAPPING,
        run_once: run_status_render_case,
    },
    BenchCase {
        id: DASHBOARD_CASE_ID,
        group: BenchGroupId::OperatorRuntime,
        description: "Collects the dashboard snapshot through the shared runtime path and renders projection plus text views.",
        measurement: BenchMeasurement {
            focus: "dashboard_projection",
            fixture: "runtime_collected_dashboard_snapshot",
            durability: BenchDurability::Ephemeral,
        },
        knots_mapping: &OPERATOR_RUNTIME_MAPPING,
        run_once: run_dashboard_projection_case,
    },
];

struct OperatorRuntimeFixture {
    _temp_dir: TempStoreDir,
    binary_path: PathBuf,
    data_dir: PathBuf,
    maybe_config_path: Option<PathBuf>,
    maybe_log_path: Option<PathBuf>,
    status_input: StatusCollectorInput,
    rpc_client: RunningStatusRpcClient,
}

struct DashboardOperatorRuntimeFixture {
    _temp_dir: TempStoreDir,
    context: DashboardRuntimeContext,
}

impl OperatorRuntimeFixture {
    fn new(case_id: &'static str) -> Result<Self, BenchError> {
        let temp_dir = TempStoreDir::new(case_id)?;
        let data_dir = temp_dir.path().join("datadir");
        let log_dir = temp_dir.path().join("logs");
        let metrics_dir = temp_dir.path().join("metrics");
        fs::create_dir_all(&data_dir)
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        fs::create_dir_all(&log_dir)
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        let config_path = data_dir.join("open-bitcoin.jsonc");
        let bitcoin_conf_path = data_dir.join("bitcoin.conf");
        let log_path = log_dir.join("open-bitcoin.log");
        let binary_path = data_dir.join("open-bitcoin-node");
        fs::write(&config_path, "{}\n")
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        fs::write(&bitcoin_conf_path, "regtest=1\n")
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        fs::write(&binary_path, b"#!/usr/bin/env false\n")
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        let store = FjallNodeStore::open(&metrics_dir)
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;
        store
            .save_metrics_snapshot(&sample_metrics_storage_snapshot(), PersistMode::Sync)
            .map_err(|error| BenchError::case_failed(case_id, error.to_string()))?;

        let status_input = StatusCollectorInput {
            request: StatusRequest {
                render_mode: StatusRenderMode::Human,
                maybe_config_path: Some(config_path.clone()),
                maybe_data_dir: Some(data_dir.clone()),
                maybe_network: Some(NetworkSelection::Regtest),
                include_live_rpc: true,
                no_color: true,
            },
            config_resolution: OperatorConfigResolution {
                path_reports: vec![
                    OperatorConfigPathReport {
                        source: OperatorConfigSource::Defaults,
                        kind: OperatorConfigPathKind::ConfigFile,
                        path: config_path.clone(),
                        present: true,
                    },
                    OperatorConfigPathReport {
                        source: OperatorConfigSource::BitcoinConf,
                        kind: OperatorConfigPathKind::BitcoinConf,
                        path: bitcoin_conf_path.clone(),
                        present: true,
                    },
                    OperatorConfigPathReport {
                        source: OperatorConfigSource::Defaults,
                        kind: OperatorConfigPathKind::DataDir,
                        path: data_dir.clone(),
                        present: true,
                    },
                    OperatorConfigPathReport {
                        source: OperatorConfigSource::Defaults,
                        kind: OperatorConfigPathKind::LogDirectory,
                        path: log_dir.clone(),
                        present: true,
                    },
                    OperatorConfigPathReport {
                        source: OperatorConfigSource::Defaults,
                        kind: OperatorConfigPathKind::MetricsStore,
                        path: metrics_dir.clone(),
                        present: true,
                    },
                ],
                maybe_config_path: Some(config_path.clone()),
                maybe_bitcoin_conf_path: Some(bitcoin_conf_path),
                maybe_data_dir: Some(data_dir.clone()),
                maybe_network: Some(NetworkSelection::Regtest),
                maybe_log_dir: Some(log_dir),
                maybe_metrics_store_path: Some(metrics_dir),
                ..OperatorConfigResolution::default()
            },
            detection_evidence: StatusDetectionEvidence {
                detected_installations: vec![],
            },
            maybe_live_rpc: None,
            maybe_service_manager: Some(Box::new(FakeServiceManager::new(running_service_state(
                Some(log_path.clone()),
            )))),
            wallet_rpc_access: StatusWalletRpcAccess::Root,
        };

        Ok(Self {
            _temp_dir: temp_dir,
            binary_path,
            data_dir,
            maybe_config_path: Some(config_path),
            maybe_log_path: Some(log_path),
            status_input,
            rpc_client: RunningStatusRpcClient::new(),
        })
    }

    fn into_dashboard_fixture(self) -> DashboardOperatorRuntimeFixture {
        let Self {
            _temp_dir,
            binary_path,
            data_dir,
            maybe_config_path,
            maybe_log_path,
            status_input,
            rpc_client,
        } = self;
        let context = DashboardRuntimeContext {
            render_mode: StatusRenderMode::Human,
            status_input,
            maybe_rpc_client: Some(Box::new(rpc_client)),
            service: DashboardServiceRuntime {
                binary_path,
                data_dir,
                maybe_config_path: maybe_config_path.clone(),
                maybe_log_path: maybe_log_path.clone(),
                manager: Box::new(FakeServiceManager::new(running_service_state(
                    maybe_log_path,
                ))),
            },
        };
        DashboardOperatorRuntimeFixture { _temp_dir, context }
    }
}

struct RunningStatusRpcClient {
    network_info: GetNetworkInfoResponse,
    blockchain_info: GetBlockchainInfoResponse,
    mempool_info: GetMempoolInfoResponse,
    wallet_info: GetWalletInfoResponse,
    balances: GetBalancesResponse,
}

impl RunningStatusRpcClient {
    fn new() -> Self {
        Self {
            network_info: GetNetworkInfoResponse {
                version: 100_000,
                subversion: "/open-bitcoin:0.1.0/".to_string(),
                protocolversion: 70_016,
                localservices: "0000000000000009".to_string(),
                localrelay: true,
                connections: 8,
                connections_in: 0,
                connections_out: 8,
                relayfee: 1,
                incrementalfee: 1,
                warnings: vec![],
            },
            blockchain_info: GetBlockchainInfoResponse {
                chain: "regtest".to_string(),
                blocks: 100,
                headers: 100,
                maybe_best_block_hash: Some(
                    "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
                ),
                maybe_median_time_past: Some(1_700_000_120),
                verificationprogress: 1.0,
                initialblockdownload: false,
                warnings: vec![],
            },
            mempool_info: GetMempoolInfoResponse {
                size: 12,
                bytes: 2_048,
                usage: 4_096,
                total_fee_sats: 5_000,
                maxmempool: 300_000_000,
                mempoolminfee: 1,
                minrelaytxfee: 1,
                loaded: true,
            },
            wallet_info: GetWalletInfoResponse {
                network: "regtest".to_string(),
                descriptor_count: 2,
                utxo_count: 2,
                maybe_tip_height: Some(100),
                maybe_tip_median_time_past: Some(1_700_000_120),
            },
            balances: GetBalancesResponse {
                mine: WalletBalanceDetails {
                    trusted_sats: 60_000,
                    untrusted_pending_sats: 0,
                    immature_sats: 0,
                },
            },
        }
    }
}

impl StatusRpcClient for RunningStatusRpcClient {
    fn get_network_info(&self) -> Result<GetNetworkInfoResponse, StatusRpcError> {
        Ok(self.network_info.clone())
    }

    fn get_blockchain_info(&self) -> Result<GetBlockchainInfoResponse, StatusRpcError> {
        Ok(self.blockchain_info.clone())
    }

    fn get_mempool_info(&self) -> Result<GetMempoolInfoResponse, StatusRpcError> {
        Ok(self.mempool_info.clone())
    }

    fn get_wallet_info(&self) -> Result<GetWalletInfoResponse, StatusRpcError> {
        Ok(self.wallet_info.clone())
    }

    fn get_balances(&self) -> Result<GetBalancesResponse, StatusRpcError> {
        Ok(self.balances.clone())
    }
}

fn run_status_render_case() -> Result<(), BenchError> {
    let fixture = OperatorRuntimeFixture::new(STATUS_CASE_ID)?;
    let snapshot = collect_status_snapshot(&fixture.status_input, Some(&fixture.rpc_client));
    let human = render_status(&snapshot, StatusRenderMode::Human)
        .map_err(|error| BenchError::case_failed(STATUS_CASE_ID, error.to_string()))?;
    let json = render_status(&snapshot, StatusRenderMode::Json)
        .map_err(|error| BenchError::case_failed(STATUS_CASE_ID, error.to_string()))?;

    if snapshot.metrics.samples.is_empty() {
        return Err(BenchError::case_failed(
            STATUS_CASE_ID,
            "runtime-collected status snapshot did not load seeded metrics history",
        ));
    }
    if !human.contains("Daemon: running")
        || !human.contains("Wallet freshness: fresh")
        || !human.contains("Metrics: available")
    {
        return Err(BenchError::case_failed(
            STATUS_CASE_ID,
            "human status render omitted expected runtime-collected fields",
        ));
    }
    if !json.contains("\"health_signals\"")
        || !json.contains("\"metrics\"")
        || !json.contains("\"service\"")
    {
        return Err(BenchError::case_failed(
            STATUS_CASE_ID,
            "json status render omitted expected runtime sections",
        ));
    }

    Ok(())
}

fn run_dashboard_projection_case() -> Result<(), BenchError> {
    let fixture = OperatorRuntimeFixture::new(DASHBOARD_CASE_ID)?.into_dashboard_fixture();
    let snapshot = collect_dashboard_snapshot(&fixture.context);
    let state = DashboardState::from_snapshot(&snapshot);
    let rendered = render_dashboard_text_snapshot(&snapshot);

    if state.sections.is_empty() || state.charts.is_empty() || state.actions.is_empty() {
        return Err(BenchError::case_failed(
            DASHBOARD_CASE_ID,
            "dashboard projection did not produce the expected sections, charts, and actions",
        ));
    }
    if snapshot.metrics.samples.is_empty() {
        return Err(BenchError::case_failed(
            DASHBOARD_CASE_ID,
            "dashboard projection did not include runtime-collected metrics history",
        ));
    }
    if !rendered.contains("Open Bitcoin Dashboard")
        || !rendered.contains("## Charts")
        || !rendered.contains("Build: version=")
    {
        return Err(BenchError::case_failed(
            DASHBOARD_CASE_ID,
            "dashboard text render omitted expected runtime-collected headings",
        ));
    }

    Ok(())
}

fn running_service_state(maybe_log_path: Option<PathBuf>) -> ServiceStateSnapshot {
    ServiceStateSnapshot {
        state: ServiceLifecycleState::Running,
        maybe_enabled: Some(true),
        maybe_service_file_path: None,
        maybe_manager_diagnostics: None,
        maybe_log_path,
        maybe_log_path_unavailable_reason: None,
    }
}

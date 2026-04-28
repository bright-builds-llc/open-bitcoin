// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoin-cli.cpp
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/interfaces/node.h

use open_bitcoin_cli::operator::{
    dashboard::{model::DashboardState, render_dashboard_text_snapshot},
    status::{StatusRenderMode, render_status},
};

use crate::{
    error::BenchError,
    registry::{
        BenchCase, BenchDurability, BenchGroupId, BenchMeasurement, OPERATOR_RUNTIME_MAPPING,
    },
    runtime_fixtures::sample_status_snapshot,
};

const STATUS_CASE_ID: &str = "operator-runtime.status-render";
const DASHBOARD_CASE_ID: &str = "operator-runtime.dashboard-projection";

pub const CASES: [BenchCase; 2] = [
    BenchCase {
        id: STATUS_CASE_ID,
        group: BenchGroupId::OperatorRuntime,
        description: "Renders the shared status snapshot as stable human and JSON output.",
        measurement: BenchMeasurement {
            focus: "status_render",
            fixture: "shared_status_snapshot",
            durability: BenchDurability::Ephemeral,
        },
        knots_mapping: &OPERATOR_RUNTIME_MAPPING,
        run_once: run_status_render_case,
    },
    BenchCase {
        id: DASHBOARD_CASE_ID,
        group: BenchGroupId::OperatorRuntime,
        description: "Projects the shared status snapshot into dashboard model and text views.",
        measurement: BenchMeasurement {
            focus: "dashboard_projection",
            fixture: "shared_status_snapshot",
            durability: BenchDurability::Ephemeral,
        },
        knots_mapping: &OPERATOR_RUNTIME_MAPPING,
        run_once: run_dashboard_projection_case,
    },
];

fn run_status_render_case() -> Result<(), BenchError> {
    let snapshot = sample_status_snapshot();
    let human = render_status(&snapshot, StatusRenderMode::Human)
        .map_err(|error| BenchError::case_failed(STATUS_CASE_ID, error.to_string()))?;
    let json = render_status(&snapshot, StatusRenderMode::Json)
        .map_err(|error| BenchError::case_failed(STATUS_CASE_ID, error.to_string()))?;

    if !human.contains("Daemon: running") || !human.contains("Wallet freshness: scanning") {
        return Err(BenchError::case_failed(
            STATUS_CASE_ID,
            "human status render omitted expected runtime summary fields",
        ));
    }
    if !json.contains("\"health_signals\"") || !json.contains("\"metrics\"") {
        return Err(BenchError::case_failed(
            STATUS_CASE_ID,
            "json status render omitted expected runtime sections",
        ));
    }

    Ok(())
}

fn run_dashboard_projection_case() -> Result<(), BenchError> {
    let snapshot = sample_status_snapshot();
    let state = DashboardState::from_snapshot(&snapshot);
    let rendered = render_dashboard_text_snapshot(&snapshot);

    if state.sections.is_empty() || state.charts.is_empty() || state.actions.is_empty() {
        return Err(BenchError::case_failed(
            DASHBOARD_CASE_ID,
            "dashboard projection did not produce the expected sections, charts, and actions",
        ));
    }
    if !rendered.contains("Open Bitcoin Dashboard") || !rendered.contains("## Charts") {
        return Err(BenchError::case_failed(
            DASHBOARD_CASE_ID,
            "dashboard text render omitted expected headings",
        ));
    }

    Ok(())
}

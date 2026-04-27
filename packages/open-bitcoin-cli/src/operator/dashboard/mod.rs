// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

//! Terminal dashboard runtime built on the shared status snapshot.

use std::{io::IsTerminal, path::PathBuf};

use super::{
    DashboardArgs, OperatorOutputFormat,
    runtime::OperatorCommandOutcome,
    service::{ServiceManager, platform_service_manager},
    status::{StatusCollectorInput, StatusRenderMode, StatusRpcClient, collect_status_snapshot},
};
use open_bitcoin_node::status::OpenBitcoinStatusSnapshot;

pub mod action;
mod app;
pub mod model;

/// Dashboard launch mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardMode {
    Interactive,
    Snapshot,
}

/// Runtime context supplied by the operator shell.
pub struct DashboardRuntimeContext {
    pub render_mode: StatusRenderMode,
    pub status_input: StatusCollectorInput,
    pub maybe_rpc_client: Option<Box<dyn StatusRpcClient>>,
    pub service: DashboardServiceRuntime,
}

/// Owned service execution dependencies for dashboard actions.
pub struct DashboardServiceRuntime {
    pub binary_path: PathBuf,
    pub data_dir: PathBuf,
    pub maybe_config_path: Option<PathBuf>,
    pub maybe_log_path: Option<PathBuf>,
    pub manager: Box<dyn ServiceManager>,
}

impl DashboardServiceRuntime {
    pub fn as_execution_context(&self) -> action::DashboardServiceContext<'_> {
        action::DashboardServiceContext {
            binary_path: self.binary_path.clone(),
            data_dir: self.data_dir.clone(),
            maybe_config_path: self.maybe_config_path.clone(),
            maybe_log_path: self.maybe_log_path.clone(),
            manager: self.manager.as_ref(),
        }
    }
}

/// Build the default platform-backed service runtime for dashboard actions.
pub fn platform_dashboard_service_runtime(
    binary_path: PathBuf,
    data_dir: PathBuf,
    maybe_config_path: Option<PathBuf>,
    maybe_log_path: Option<PathBuf>,
    home_dir: PathBuf,
) -> DashboardServiceRuntime {
    DashboardServiceRuntime {
        binary_path,
        data_dir,
        maybe_config_path,
        maybe_log_path,
        manager: platform_service_manager(home_dir),
    }
}

/// Execute the dashboard command.
pub fn run_dashboard(
    args: &DashboardArgs,
    context: DashboardRuntimeContext,
) -> OperatorCommandOutcome {
    let snapshot = collect_dashboard_snapshot(&context);
    if context.render_mode == StatusRenderMode::Json {
        return match serde_json::to_string_pretty(&snapshot) {
            Ok(rendered) => OperatorCommandOutcome::success(format!("{rendered}\n")),
            Err(error) => OperatorCommandOutcome::failure(error.to_string()),
        };
    }

    match select_dashboard_mode(&context) {
        DashboardMode::Snapshot => OperatorCommandOutcome::success(format!(
            "{}\n",
            render_dashboard_text_snapshot(&snapshot)
        )),
        DashboardMode::Interactive => match app::run_interactive_dashboard(args, &context) {
            Ok(()) => OperatorCommandOutcome::success("dashboard exited\n"),
            Err(_) => OperatorCommandOutcome::success(format!(
                "{}\n",
                render_dashboard_text_snapshot(&snapshot)
            )),
        },
    }
}

/// Collect exactly one shared status snapshot for a dashboard refresh.
pub fn collect_dashboard_snapshot(context: &DashboardRuntimeContext) -> OpenBitcoinStatusSnapshot {
    collect_status_snapshot(
        &context.status_input,
        context
            .maybe_rpc_client
            .as_ref()
            .map(|client| client.as_ref() as &dyn StatusRpcClient),
    )
}

/// Render deterministic sectioned text for non-interactive dashboard contexts.
pub fn render_dashboard_text_snapshot(snapshot: &OpenBitcoinStatusSnapshot) -> String {
    let state = model::DashboardState::from_snapshot(snapshot);
    let mut lines = vec!["Open Bitcoin Dashboard".to_string()];
    for section in state.sections {
        lines.push(String::new());
        lines.push(format!("## {}", section.title));
        for row in section.rows {
            lines.push(format!("{}: {}", row.label, row.value));
        }
    }
    lines.push(String::new());
    lines.push("## Charts".to_string());
    for chart in state.charts {
        let points = if chart.points.is_empty() {
            "Unavailable".to_string()
        } else {
            chart
                .points
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        lines.push(format!(
            "{}: {} ({})",
            chart.title, points, chart.availability
        ));
    }
    lines.push(String::new());
    lines.push("## Actions".to_string());
    lines.push(
        state
            .actions
            .iter()
            .map(|action| {
                if action.destructive {
                    format!("{} {} (confirm)", action.key, action.label)
                } else {
                    format!("{} {}", action.key, action.label)
                }
            })
            .collect::<Vec<_>>()
            .join(" | "),
    );
    lines.join("\n")
}

fn select_dashboard_mode(context: &DashboardRuntimeContext) -> DashboardMode {
    if context.render_mode == StatusRenderMode::Json || !std::io::stdout().is_terminal() {
        return DashboardMode::Snapshot;
    }
    DashboardMode::Interactive
}

impl From<OperatorOutputFormat> for StatusRenderMode {
    fn from(format: OperatorOutputFormat) -> Self {
        match format {
            OperatorOutputFormat::Human => Self::Human,
            OperatorOutputFormat::Json => Self::Json,
        }
    }
}

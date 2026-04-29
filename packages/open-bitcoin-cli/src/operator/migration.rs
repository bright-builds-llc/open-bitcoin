// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

//! Dry-run migration planning for existing Core or Knots installs.

use std::fmt;

use super::{
    MigrationArgs, MigrationCommand, MigrationPlanArgs, OperatorOutputFormat,
    config::OperatorConfigResolution,
    detect::DetectedInstallation,
    runtime::{OperatorCommandOutcome, OperatorRuntimeError},
};

mod planning;
mod service_evidence;
#[cfg(test)]
mod tests;
mod types;

use planning::{action_kind_name, display_optional_string, notice_level_name};

pub use types::{
    MigrationAction, MigrationActionGroup, MigrationActionKind, MigrationDeviationNotice,
    MigrationExplanation, MigrationInstallationSummary, MigrationNoticeLevel, MigrationPlan,
    MigrationServiceSummary, MigrationSourceSelection, MigrationTargetEnvironment,
    MigrationWalletSummary,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationError {
    message: String,
}

impl MigrationError {
    fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

impl fmt::Display for MigrationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::error::Error for MigrationError {}

pub fn execute_migration_command(
    args: &MigrationArgs,
    format: OperatorOutputFormat,
    config_resolution: &OperatorConfigResolution,
    detections: &[DetectedInstallation],
) -> Result<OperatorCommandOutcome, OperatorRuntimeError> {
    let rendered = match &args.command {
        MigrationCommand::Plan(request) => {
            let plan = plan_migration(config_resolution, detections, request);
            render_migration_plan(&plan, format).map_err(|error| {
                OperatorRuntimeError::InvalidRequest {
                    message: error.to_string(),
                }
            })?
        }
    };

    Ok(OperatorCommandOutcome::success(format!("{rendered}\n")))
}

pub fn plan_migration(
    config_resolution: &OperatorConfigResolution,
    detections: &[DetectedInstallation],
    request: &MigrationPlanArgs,
) -> MigrationPlan {
    planning::plan_migration(config_resolution, detections, request)
}

pub fn render_migration_plan(
    plan: &MigrationPlan,
    format: OperatorOutputFormat,
) -> Result<String, MigrationError> {
    if format == OperatorOutputFormat::Json {
        return serde_json::to_string_pretty(plan)
            .map_err(|error| MigrationError::new(error.to_string()));
    }

    Ok(render_human_plan(plan))
}

#[doc(hidden)]
pub fn migration_deviation_definitions() -> Vec<MigrationDeviationNotice> {
    planning::migration_deviation_definitions()
}

fn render_human_plan(plan: &MigrationPlan) -> String {
    let mut lines = vec![
        "Migration plan (dry run only)".to_string(),
        "Source data, services, and external wallets remain read-only in Phase 21.".to_string(),
        String::new(),
        "Source selection:".to_string(),
    ];

    match &plan.source_selection {
        MigrationSourceSelection::Selected { installation } => {
            lines.push(format!(
                "- product={} confidence={} datadir={}",
                installation.product_family,
                installation.confidence,
                display_optional_string(installation.maybe_data_dir.as_deref())
            ));
            if !installation.uncertainty.is_empty() {
                lines.push(format!(
                    "- uncertainty={}",
                    installation.uncertainty.join(", ")
                ));
            }
        }
        MigrationSourceSelection::ManualReviewRequired { reason, candidates } => {
            lines.push(format!("- manual review required: {reason}"));
            for candidate in candidates {
                lines.push(format!(
                    "- candidate: product={} confidence={} datadir={}",
                    candidate.product_family,
                    candidate.confidence,
                    display_optional_string(candidate.maybe_data_dir.as_deref())
                ));
            }
        }
    }

    lines.push(String::new());
    lines.push("Target Open Bitcoin environment:".to_string());
    lines.push(format!(
        "- config={}",
        display_optional_string(
            plan.target_environment
                .maybe_target_open_bitcoin_config_path
                .as_deref(),
        )
    ));
    lines.push(format!(
        "- datadir={}",
        display_optional_string(plan.target_environment.maybe_target_data_dir.as_deref())
    ));
    lines.push(format!(
        "- network={}",
        display_optional_string(plan.target_environment.maybe_target_network.as_deref())
    ));

    push_explanation_section(&mut lines, "Benefits", &plan.explanation.benefits);
    push_explanation_section(&mut lines, "Tradeoffs", &plan.explanation.tradeoffs);
    push_explanation_section(
        &mut lines,
        "Unsupported surfaces",
        &plan.explanation.unsupported_surfaces,
    );
    push_explanation_section(
        &mut lines,
        "Rollback expectations",
        &plan.explanation.rollback_expectations,
    );
    push_explanation_section(
        &mut lines,
        "Backup requirements",
        &plan.explanation.backup_requirements,
    );

    lines.push("Planned actions:".to_string());
    for group in &plan.action_groups {
        lines.push(format!("{}:", group.title));
        for action in &group.actions {
            match action.maybe_path.as_deref() {
                Some(path) => lines.push(format!(
                    "- [{}] {} ({path})",
                    action_kind_name(action.kind),
                    action.summary
                )),
                None => lines.push(format!(
                    "- [{}] {}",
                    action_kind_name(action.kind),
                    action.summary
                )),
            }
        }
    }

    if !plan.relevant_deviations.is_empty() {
        lines.push(String::new());
        lines.push("Intentional differences relevant to this migration:".to_string());
        for deviation in &plan.relevant_deviations {
            lines.push(format!(
                "- [{}] {}: {} ({})",
                notice_level_name(deviation.level),
                deviation.id,
                deviation.summary,
                deviation.docs_path
            ));
        }
    }

    lines.join("\n")
}

fn push_explanation_section(lines: &mut Vec<String>, title: &str, entries: &[String]) {
    lines.push(String::new());
    lines.push(format!("{title}:"));
    for entry in entries {
        lines.push(format!("- {entry}"));
    }
}

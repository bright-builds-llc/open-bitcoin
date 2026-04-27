// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use serde::Serialize;

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationPlan {
    pub source_selection: MigrationSourceSelection,
    pub target_environment: MigrationTargetEnvironment,
    pub explanation: MigrationExplanation,
    pub action_groups: Vec<MigrationActionGroup>,
    pub relevant_deviations: Vec<MigrationDeviationNotice>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MigrationSourceSelection {
    Selected {
        installation: MigrationInstallationSummary,
    },
    ManualReviewRequired {
        reason: String,
        candidates: Vec<MigrationInstallationSummary>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationTargetEnvironment {
    pub maybe_target_open_bitcoin_config_path: Option<String>,
    pub maybe_target_bitcoin_conf_path: Option<String>,
    pub maybe_target_data_dir: Option<String>,
    pub maybe_target_log_dir: Option<String>,
    pub maybe_target_metrics_store_path: Option<String>,
    pub maybe_target_network: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationExplanation {
    pub benefits: Vec<String>,
    pub tradeoffs: Vec<String>,
    pub unsupported_surfaces: Vec<String>,
    pub rollback_expectations: Vec<String>,
    pub backup_requirements: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationInstallationSummary {
    pub product_family: String,
    pub confidence: String,
    pub uncertainty: Vec<String>,
    pub maybe_data_dir: Option<String>,
    pub maybe_config_file: Option<String>,
    pub maybe_cookie_file: Option<String>,
    pub service_candidates: Vec<MigrationServiceSummary>,
    pub wallet_candidates: Vec<MigrationWalletSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationServiceSummary {
    pub manager: String,
    pub service_name: String,
    pub path: String,
    pub present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationWalletSummary {
    pub kind: String,
    pub maybe_name: Option<String>,
    pub chain_scope: String,
    pub product_family: String,
    pub product_confidence: String,
    pub path: String,
    pub present: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationActionGroup {
    pub title: String,
    pub actions: Vec<MigrationAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationAction {
    pub kind: MigrationActionKind,
    pub summary: String,
    pub maybe_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationActionKind {
    ReadOnlyCheck,
    TargetWritePreview,
    ManualStep,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub struct MigrationDeviationNotice {
    pub id: String,
    pub level: MigrationNoticeLevel,
    pub summary: String,
    pub docs_path: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MigrationNoticeLevel {
    Info,
    Warn,
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use std::{collections::BTreeSet, path::Path};

mod labels;

use self::labels::{
    detection_confidence_name, detection_uncertainty_name, network_name, product_family_name,
    render_path, service_manager_name, wallet_chain_scope_name, wallet_kind_name,
};
use super::{
    MigrationAction, MigrationActionGroup, MigrationActionKind, MigrationDeviationNotice,
    MigrationExplanation, MigrationInstallationSummary, MigrationNoticeLevel, MigrationPlan,
    MigrationServiceSummary, MigrationSourceSelection, MigrationTargetEnvironment,
    MigrationWalletSummary,
    service_evidence::{
        SERVICE_REVIEW_AMBIGUOUS, ServiceAssociation, associate_service_candidates,
        summary_service_review_is_ambiguous,
    },
};
use crate::operator::{
    MigrationPlanArgs,
    config::OperatorConfigResolution,
    detect::{DetectedInstallation, DetectionScan, ServiceCandidate, WalletCandidate},
};

const MIGRATION_AUDIT_DOCS_PATH: &str = "docs/parity/catalog/drop-in-audit-and-migration.md";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum MigrationSurface {
    Config,
    Service,
    Wallet,
    OperatorDocs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct MigrationDeviationDefinition {
    id: &'static str,
    level: MigrationNoticeLevel,
    surfaces: &'static [MigrationSurface],
    summary: &'static str,
    docs_path: &'static str,
}

const MIGRATION_DEVIATION_DEFINITIONS: [MigrationDeviationDefinition; 3] = [
    MigrationDeviationDefinition {
        id: "mig-jsonc-open-bitcoin-settings",
        level: MigrationNoticeLevel::Info,
        surfaces: &[MigrationSurface::Config, MigrationSurface::OperatorDocs],
        summary: "Open Bitcoin-only onboarding, service, dashboard, migration, metrics, logging, storage, and sync settings live in open-bitcoin.jsonc instead of bitcoin.conf.",
        docs_path: MIGRATION_AUDIT_DOCS_PATH,
    },
    MigrationDeviationDefinition {
        id: "mig-dry-run-only-switch-over",
        level: MigrationNoticeLevel::Warn,
        surfaces: &[MigrationSurface::Service, MigrationSurface::OperatorDocs],
        summary: "Phase 21 migration is dry-run only. It does not disable source services, rewrite source datadirs, or perform automatic switch-over.",
        docs_path: MIGRATION_AUDIT_DOCS_PATH,
    },
    MigrationDeviationDefinition {
        id: "mig-managed-wallet-backup-format",
        level: MigrationNoticeLevel::Warn,
        surfaces: &[MigrationSurface::Wallet, MigrationSurface::OperatorDocs],
        summary: "Open Bitcoin exports managed-wallet backups as repo-owned JSON and does not copy, restore, or rewrite Core or Knots wallet.dat files.",
        docs_path: MIGRATION_AUDIT_DOCS_PATH,
    },
];

pub(super) fn plan_migration(
    config_resolution: &OperatorConfigResolution,
    detections: &DetectionScan,
    request: &MigrationPlanArgs,
) -> MigrationPlan {
    let source_selection =
        select_source_installation(detections, request.maybe_source_data_dir.as_deref());
    let target_environment = summarize_target_environment(config_resolution);
    let explanation = migration_explanation();
    let action_groups = build_action_groups(&source_selection, &target_environment);
    let relevant_deviations = relevant_deviations(&source_selection);

    MigrationPlan {
        source_selection,
        target_environment,
        explanation,
        action_groups,
        relevant_deviations,
    }
}

pub(super) fn migration_deviation_definitions() -> Vec<MigrationDeviationNotice> {
    MIGRATION_DEVIATION_DEFINITIONS
        .iter()
        .map(deviation_notice)
        .collect()
}

fn select_source_installation(
    detections: &DetectionScan,
    maybe_source_data_dir: Option<&Path>,
) -> MigrationSourceSelection {
    let candidates = detections
        .installations
        .iter()
        .map(|installation| summarize_installation(installation, &detections.service_candidates))
        .collect::<Vec<_>>();

    let Some(source_data_dir) = maybe_source_data_dir else {
        if detections.installations.len() == 1 {
            let maybe_installation = detections
                .installations
                .first()
                .filter(|installation| installation.maybe_data_dir.is_some());
            if let Some(installation) = maybe_installation {
                return MigrationSourceSelection::Selected {
                    installation: summarize_installation(
                        installation,
                        &detections.service_candidates,
                    ),
                };
            }
        }

        let reason = if detections.installations.is_empty() {
            "No existing Core or Knots installation was detected. Pass --source-datadir <path> if your source install lives in a custom location.".to_string()
        } else {
            "Multiple or partial install candidates were detected. Pass --source-datadir <path> to choose the source install explicitly.".to_string()
        };
        return MigrationSourceSelection::ManualReviewRequired { reason, candidates };
    };

    let maybe_installation = detections.installations.iter().find(|installation| {
        installation
            .maybe_data_dir
            .as_deref()
            .is_some_and(|data_dir| data_dir == source_data_dir)
    });
    if let Some(installation) = maybe_installation {
        if !supports_explicit_source_selection(installation) {
            return MigrationSourceSelection::ManualReviewRequired {
                reason: format!(
                    "The explicit source path {} exists, but it does not yet expose source config, cookie, or wallet evidence. Confirm the path points at a supported Core or Knots datadir before rerunning the planner.",
                    source_data_dir.display()
                ),
                candidates,
            };
        }
        return MigrationSourceSelection::Selected {
            installation: summarize_installation(installation, &detections.service_candidates),
        };
    }

    MigrationSourceSelection::ManualReviewRequired {
        reason: format!(
            "No detected installation matched --source-datadir {}.",
            source_data_dir.display()
        ),
        candidates,
    }
}

fn supports_explicit_source_selection(installation: &DetectedInstallation) -> bool {
    installation.maybe_config_file.is_some()
        || installation.maybe_cookie_file.is_some()
        || !installation.wallet_candidates.is_empty()
}

fn summarize_target_environment(
    config_resolution: &OperatorConfigResolution,
) -> MigrationTargetEnvironment {
    MigrationTargetEnvironment {
        maybe_target_open_bitcoin_config_path: config_resolution
            .maybe_config_path
            .as_deref()
            .map(render_path),
        maybe_target_bitcoin_conf_path: config_resolution
            .maybe_bitcoin_conf_path
            .as_deref()
            .map(render_path),
        maybe_target_data_dir: config_resolution.maybe_data_dir.as_deref().map(render_path),
        maybe_target_log_dir: config_resolution.maybe_log_dir.as_deref().map(render_path),
        maybe_target_metrics_store_path: config_resolution
            .maybe_metrics_store_path
            .as_deref()
            .map(render_path),
        maybe_target_network: config_resolution.maybe_network.map(network_name),
    }
}

fn migration_explanation() -> MigrationExplanation {
    MigrationExplanation {
        benefits: vec![
            "The planner keeps the source install read-only while still surfacing the config, wallet, service, and parity evidence needed for a safe migration review.".to_string(),
            "The output shows migration-relevant intentional differences before any future switch-over decision.".to_string(),
        ],
        tradeoffs: vec![
            "Phase 21 does not execute automatic cutover, service replacement, or source-datadir mutation.".to_string(),
            "Ambiguous detections stay explicit and may require a manual source selection step via --source-datadir.".to_string(),
        ],
        unsupported_surfaces: vec![
            "No external-wallet import, restore, copy, or rewrite flow is implemented in this phase.".to_string(),
            "No automatic disable or uninstall of existing Core or Knots services is implemented in this phase.".to_string(),
            "This planner is evidence-scoped and does not claim full drop-in replacement parity for deferred sync, RPC, or operator surfaces.".to_string(),
        ],
        rollback_expectations: vec![
            "Keep the source datadir and source service untouched until the Open Bitcoin target has been validated with status, logs, and wallet freshness checks.".to_string(),
            "Use a separate Open Bitcoin datadir and config path instead of reusing source files in place.".to_string(),
        ],
        backup_requirements: vec![
            "Create verified upstream backups for every detected external wallet candidate before any future import or migration work.".to_string(),
            "Preserve source bitcoin.conf, source service definitions, and source cookie-file paths as review evidence during cutover planning.".to_string(),
        ],
    }
}

fn build_action_groups(
    source_selection: &MigrationSourceSelection,
    target_environment: &MigrationTargetEnvironment,
) -> Vec<MigrationActionGroup> {
    vec![
        config_action_group(source_selection, target_environment),
        file_action_group(source_selection, target_environment),
        service_action_group(source_selection),
        wallet_action_group(source_selection),
        follow_up_action_group(source_selection),
    ]
}

fn config_action_group(
    source_selection: &MigrationSourceSelection,
    target_environment: &MigrationTargetEnvironment,
) -> MigrationActionGroup {
    let mut actions = Vec::new();

    match source_selection {
        MigrationSourceSelection::Selected { installation } => {
            if let Some(source_config) = installation.maybe_config_file.as_deref() {
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ReadOnlyCheck,
                    summary: "Review the source bitcoin.conf and carry forward only the baseline-compatible settings that still apply to the target node.".to_string(),
                    maybe_path: Some(source_config.to_string()),
                });
            } else {
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ManualStep,
                    summary: "No source bitcoin.conf was detected. Confirm whether the source install relies on defaults or a custom config location before proceeding.".to_string(),
                    maybe_path: None,
                });
            }
        }
        MigrationSourceSelection::ManualReviewRequired { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Select a single source install first so the planner can map the correct source bitcoin.conf into the migration review.".to_string(),
            maybe_path: None,
        }),
    }

    actions.push(MigrationAction {
        kind: MigrationActionKind::TargetWritePreview,
        summary:
            "Prepare Open Bitcoin-only settings in open-bitcoin.jsonc instead of writing them into bitcoin.conf.".to_string(),
        maybe_path: target_environment.maybe_target_open_bitcoin_config_path.clone(),
    });
    actions.push(MigrationAction {
        kind: MigrationActionKind::Deferred,
        summary: "Phase 21 does not rewrite or delete the source bitcoin.conf. Any later target write should happen only in the Open Bitcoin target config path.".to_string(),
        maybe_path: target_environment.maybe_target_bitcoin_conf_path.clone(),
    });

    MigrationActionGroup {
        title: "Config".to_string(),
        actions,
    }
}

fn file_action_group(
    source_selection: &MigrationSourceSelection,
    target_environment: &MigrationTargetEnvironment,
) -> MigrationActionGroup {
    let mut actions = Vec::new();

    match source_selection {
        MigrationSourceSelection::Selected { installation } => {
            if let Some(source_data_dir) = installation.maybe_data_dir.as_deref() {
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ReadOnlyCheck,
                    summary: "Keep the detected source datadir read-only during migration planning and validation.".to_string(),
                    maybe_path: Some(source_data_dir.to_string()),
                });
            }
            if let Some(source_cookie) = installation.maybe_cookie_file.as_deref() {
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ReadOnlyCheck,
                    summary: "Inspect the source cookie-file path for operator review only; never copy or print the cookie value.".to_string(),
                    maybe_path: Some(source_cookie.to_string()),
                });
            }
        }
        MigrationSourceSelection::ManualReviewRequired { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Confirm the exact source datadir and auth file paths before any later target configuration or cutover work.".to_string(),
            maybe_path: None,
        }),
    }

    actions.push(MigrationAction {
        kind: MigrationActionKind::TargetWritePreview,
        summary:
            "Use a separate Open Bitcoin datadir rather than reusing or rewriting the source datadir in place.".to_string(),
        maybe_path: target_environment.maybe_target_data_dir.clone(),
    });

    MigrationActionGroup {
        title: "Files and Datadir".to_string(),
        actions,
    }
}

fn service_action_group(source_selection: &MigrationSourceSelection) -> MigrationActionGroup {
    let mut actions = Vec::new();

    match source_selection {
        MigrationSourceSelection::Selected { installation } => {
            if installation.service_candidates.is_empty() {
                let summary = if summary_service_review_is_ambiguous(installation) {
                    "Detected managed service definitions could not be confidently tied to the selected source install. Review the source supervisor manually before any future cutover."
                } else {
                    "No managed source service definition was detected. If the source node is supervised elsewhere, capture that startup path manually before cutover."
                };
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ManualStep,
                    summary: summary.to_string(),
                    maybe_path: None,
                });
            } else {
                for service in &installation.service_candidates {
                    actions.push(MigrationAction {
                        kind: MigrationActionKind::ReadOnlyCheck,
                        summary: format!(
                            "Inspect the existing {} service definition for restart policy, launch context, and operator ownership before any future cutover.",
                            service.manager
                        ),
                        maybe_path: Some(service.path.clone()),
                    });
                }
            }
        }
        MigrationSourceSelection::ManualReviewRequired { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Select the source install first so the planner can show the correct service cutover review path.".to_string(),
            maybe_path: None,
        }),
    }

    actions.push(MigrationAction {
        kind: MigrationActionKind::Deferred,
        summary: "Phase 21 does not disable, uninstall, or replace the source service automatically. Service cutover remains a manual follow-up after validation.".to_string(),
        maybe_path: None,
    });

    MigrationActionGroup {
        title: "Service".to_string(),
        actions,
    }
}

fn wallet_action_group(source_selection: &MigrationSourceSelection) -> MigrationActionGroup {
    let mut actions = Vec::new();

    match source_selection {
        MigrationSourceSelection::Selected { installation } => {
            if installation.wallet_candidates.is_empty() {
                actions.push(MigrationAction {
                    kind: MigrationActionKind::ManualStep,
                    summary: "No wallet candidates were detected. Confirm whether the source install keeps wallets in a custom location before proceeding.".to_string(),
                    maybe_path: None,
                });
            } else {
                for wallet in &installation.wallet_candidates {
                    let wallet_name = wallet
                        .maybe_name
                        .clone()
                        .unwrap_or_else(|| "(unnamed)".to_string());
                    actions.push(MigrationAction {
                        kind: MigrationActionKind::ReadOnlyCheck,
                        summary: format!(
                            "Inspect external wallet candidate {} (format={}, chain={}) and create an upstream backup before any later migration step.",
                            wallet_name, wallet.kind, wallet.chain_scope
                        ),
                        maybe_path: Some(wallet.path.clone()),
                    });
                }
            }
        }
        MigrationSourceSelection::ManualReviewRequired { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Select the source install first so the planner can enumerate the relevant external wallet candidates.".to_string(),
            maybe_path: None,
        }),
    }

    actions.push(MigrationAction {
        kind: MigrationActionKind::Deferred,
        summary: "Phase 21 does not import, restore, copy, or rewrite external wallet files. It remains a read-only planning surface.".to_string(),
        maybe_path: None,
    });

    MigrationActionGroup {
        title: "Wallet".to_string(),
        actions,
    }
}

fn follow_up_action_group(source_selection: &MigrationSourceSelection) -> MigrationActionGroup {
    let mut actions = Vec::new();

    match source_selection {
        MigrationSourceSelection::Selected { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Review the Phase 21 parity audit page and the intentional differences below before any later switch-over step.".to_string(),
            maybe_path: Some(MIGRATION_AUDIT_DOCS_PATH.to_string()),
        }),
        MigrationSourceSelection::ManualReviewRequired { .. } => actions.push(MigrationAction {
            kind: MigrationActionKind::ManualStep,
            summary: "Resolve source-install ambiguity first, then rerun the planner so the action list and deviations are scoped correctly.".to_string(),
            maybe_path: None,
        }),
    }

    actions.push(MigrationAction {
        kind: MigrationActionKind::ManualStep,
        summary: "Prepare the Open Bitcoin target config with `open-bitcoin onboard --detect-existing` only after this dry-run plan and backup review look correct.".to_string(),
        maybe_path: None,
    });

    MigrationActionGroup {
        title: "Operator Follow-Up".to_string(),
        actions,
    }
}

fn relevant_deviations(
    source_selection: &MigrationSourceSelection,
) -> Vec<MigrationDeviationNotice> {
    let surfaces = relevant_surfaces(source_selection);
    MIGRATION_DEVIATION_DEFINITIONS
        .iter()
        .filter(|definition| {
            definition
                .surfaces
                .iter()
                .any(|surface| surfaces.contains(surface))
        })
        .map(deviation_notice)
        .collect()
}

fn relevant_surfaces(source_selection: &MigrationSourceSelection) -> BTreeSet<MigrationSurface> {
    let mut surfaces = BTreeSet::from([MigrationSurface::Config, MigrationSurface::OperatorDocs]);

    if let MigrationSourceSelection::Selected { installation } = source_selection {
        if !installation.service_candidates.is_empty()
            || summary_service_review_is_ambiguous(installation)
        {
            surfaces.insert(MigrationSurface::Service);
        }
        if !installation.wallet_candidates.is_empty() {
            surfaces.insert(MigrationSurface::Wallet);
        }
    }

    surfaces
}

fn deviation_notice(definition: &MigrationDeviationDefinition) -> MigrationDeviationNotice {
    MigrationDeviationNotice {
        id: definition.id.to_string(),
        level: definition.level,
        summary: definition.summary.to_string(),
        docs_path: definition.docs_path.to_string(),
    }
}

fn summarize_installation(
    installation: &DetectedInstallation,
    service_candidates: &[ServiceCandidate],
) -> MigrationInstallationSummary {
    let mut uncertainty = installation
        .uncertainty
        .iter()
        .copied()
        .map(detection_uncertainty_name)
        .map(str::to_string)
        .collect::<Vec<_>>();
    let service_candidates = match associate_service_candidates(installation, service_candidates) {
        ServiceAssociation::Matched(service_candidates) => service_candidates
            .iter()
            .map(summarize_service_candidate)
            .collect(),
        ServiceAssociation::Ambiguous => {
            uncertainty.push(SERVICE_REVIEW_AMBIGUOUS.to_string());
            Vec::new()
        }
        ServiceAssociation::NoneDetected => Vec::new(),
    };

    MigrationInstallationSummary {
        product_family: product_family_name(installation.product_family).to_string(),
        confidence: detection_confidence_name(installation.confidence).to_string(),
        uncertainty,
        maybe_data_dir: installation.maybe_data_dir.as_deref().map(render_path),
        maybe_config_file: installation.maybe_config_file.as_deref().map(render_path),
        maybe_cookie_file: installation.maybe_cookie_file.as_deref().map(render_path),
        service_candidates,
        wallet_candidates: installation
            .wallet_candidates
            .iter()
            .map(summarize_wallet_candidate)
            .collect(),
    }
}

fn summarize_service_candidate(candidate: &ServiceCandidate) -> MigrationServiceSummary {
    MigrationServiceSummary {
        manager: service_manager_name(candidate.manager).to_string(),
        service_name: candidate.service_name.clone(),
        path: render_path(&candidate.path),
        present: candidate.present,
    }
}

fn summarize_wallet_candidate(candidate: &WalletCandidate) -> MigrationWalletSummary {
    MigrationWalletSummary {
        kind: wallet_kind_name(candidate.kind).to_string(),
        maybe_name: candidate.maybe_name.clone(),
        chain_scope: wallet_chain_scope_name(candidate.chain_scope).to_string(),
        product_family: product_family_name(candidate.product_family).to_string(),
        product_confidence: detection_confidence_name(candidate.product_confidence).to_string(),
        path: render_path(&candidate.path),
        present: candidate.present,
    }
}

pub(super) fn action_kind_name(kind: MigrationActionKind) -> &'static str {
    match kind {
        MigrationActionKind::ReadOnlyCheck => "read_only_check",
        MigrationActionKind::TargetWritePreview => "target_write_preview",
        MigrationActionKind::ManualStep => "manual_step",
        MigrationActionKind::Deferred => "deferred",
    }
}

pub(super) fn notice_level_name(level: MigrationNoticeLevel) -> &'static str {
    match level {
        MigrationNoticeLevel::Info => "info",
        MigrationNoticeLevel::Warn => "warn",
    }
}

pub(super) fn display_optional_string(maybe_value: Option<&str>) -> &str {
    maybe_value.unwrap_or("Unavailable")
}

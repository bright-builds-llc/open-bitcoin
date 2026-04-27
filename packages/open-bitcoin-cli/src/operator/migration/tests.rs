// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use std::path::PathBuf;

use serde_json::Value;

use super::{MigrationSourceSelection, plan_migration, render_migration_plan};
use crate::operator::{
    MigrationPlanArgs, NetworkSelection, OperatorOutputFormat,
    config::OperatorConfigResolution,
    detect::{
        DetectedInstallation, DetectionConfidence, DetectionSourcePath, DetectionSourcePathKind,
        DetectionUncertainty, ProductFamily, ServiceCandidate, ServiceManager, WalletCandidate,
        WalletCandidateKind, WalletChainScope,
    },
};

#[test]
fn planner_renders_explanation_and_action_groups_for_selected_installation() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![detected_installation("/tmp/core")];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: None,
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);
    let rendered =
        render_migration_plan(&plan, OperatorOutputFormat::Human).expect("migration plan");

    // Assert
    assert!(matches!(
        plan.source_selection,
        MigrationSourceSelection::Selected { .. }
    ));
    assert!(rendered.contains("Migration plan (dry run only)"));
    assert!(rendered.contains("Benefits:"));
    assert!(rendered.contains("Backup requirements:"));
    assert!(rendered.contains("Config:"));
    assert!(rendered.contains("Files and Datadir:"));
    assert!(rendered.contains("Service:"));
    assert!(rendered.contains("Wallet:"));
    assert!(rendered.contains("Operator Follow-Up:"));
    assert!(rendered.contains("Intentional differences relevant to this migration:"));
}

#[test]
fn planner_rendering_redacts_cookie_contents_and_raw_wallet_data() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![detected_installation("/tmp/core")];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: None,
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);
    let rendered =
        render_migration_plan(&plan, OperatorOutputFormat::Human).expect("migration plan");

    // Assert
    assert!(rendered.contains("/tmp/core/.cookie"));
    assert!(!rendered.contains("__cookie__:secret"));
    assert!(!rendered.contains("legacy wallet bytes"));
}

#[test]
fn planner_requires_manual_review_for_ambiguous_detections() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![
        detected_installation("/tmp/core"),
        detected_installation("/tmp/knots"),
    ];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: None,
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);
    let rendered =
        render_migration_plan(&plan, OperatorOutputFormat::Human).expect("migration plan");

    // Assert
    assert!(matches!(
        plan.source_selection,
        MigrationSourceSelection::ManualReviewRequired { .. }
    ));
    assert!(rendered.contains("manual review required"));
    assert!(rendered.contains("--source-datadir"));
}

#[test]
fn source_datadir_selects_matching_detection() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![
        detected_installation("/tmp/core"),
        detected_installation("/tmp/knots"),
    ];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: Some(PathBuf::from("/tmp/knots")),
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);

    // Assert
    let MigrationSourceSelection::Selected { installation } = plan.source_selection else {
        panic!("expected selected source installation");
    };
    assert_eq!(installation.maybe_data_dir, Some("/tmp/knots".to_string()));
    assert_eq!(installation.product_family, "bitcoin_knots");
}

#[test]
fn json_render_includes_relevant_deviations() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![detected_installation("/tmp/core")];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: None,
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);
    let rendered =
        render_migration_plan(&plan, OperatorOutputFormat::Json).expect("json migration plan");
    let decoded: Value = serde_json::from_str(&rendered).expect("plan json");

    // Assert
    assert_eq!(decoded["source_selection"]["kind"], "selected");
    assert!(decoded["relevant_deviations"].is_array());
    assert!(
        decoded["relevant_deviations"]
            .as_array()
            .is_some_and(|items| !items.is_empty())
    );
}

fn sample_resolution() -> OperatorConfigResolution {
    OperatorConfigResolution {
        maybe_config_path: Some(PathBuf::from("/tmp/open-bitcoin/open-bitcoin.jsonc")),
        maybe_bitcoin_conf_path: Some(PathBuf::from("/tmp/open-bitcoin/bitcoin.conf")),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
        maybe_log_dir: Some(PathBuf::from("/tmp/open-bitcoin/logs")),
        maybe_metrics_store_path: Some(PathBuf::from("/tmp/open-bitcoin/metrics")),
        ..OperatorConfigResolution::default()
    }
}

fn detected_installation(data_dir: &str) -> DetectedInstallation {
    let data_dir = PathBuf::from(data_dir);
    let product_family = if data_dir.to_string_lossy().contains("knots") {
        ProductFamily::BitcoinKnots
    } else {
        ProductFamily::BitcoinCore
    };
    DetectedInstallation {
        product_family,
        confidence: DetectionConfidence::Medium,
        uncertainty: vec![DetectionUncertainty::ProductAmbiguous],
        source_paths: vec![
            DetectionSourcePath {
                kind: DetectionSourcePathKind::DataDir,
                path: data_dir.clone(),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::ConfigFile,
                path: data_dir.join("bitcoin.conf"),
                present: true,
            },
            DetectionSourcePath {
                kind: DetectionSourcePathKind::CookieFile,
                path: data_dir.join(".cookie"),
                present: true,
            },
        ],
        maybe_data_dir: Some(data_dir.clone()),
        maybe_config_file: Some(data_dir.join("bitcoin.conf")),
        maybe_cookie_file: Some(data_dir.join(".cookie")),
        service_candidates: vec![ServiceCandidate {
            product_family,
            manager: ServiceManager::Systemd,
            service_name: "bitcoind".to_string(),
            path: PathBuf::from("/etc/systemd/system/bitcoind.service"),
            present: true,
        }],
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::LegacyWalletFile,
            path: data_dir.join("wallets/main/wallet.dat"),
            maybe_name: Some("main".to_string()),
            present: true,
            product_family,
            product_confidence: DetectionConfidence::Medium,
            chain_scope: WalletChainScope::Mainnet,
        }],
    }
}

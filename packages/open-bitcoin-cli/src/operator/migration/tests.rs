// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

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

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-migration-tests-{label}-{}",
            NEXT_TEST_DIRECTORY_ID.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(&directory).expect("test directory");
        Self { path: directory }
    }

    fn child(&self, relative: &str) -> PathBuf {
        self.path.join(relative)
    }
}

impl Drop for TestDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

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
fn explicit_source_datadir_without_source_evidence_stays_manual_review() {
    // Arrange
    let resolution = sample_resolution();
    let detections = vec![DetectedInstallation {
        product_family: ProductFamily::Unknown,
        confidence: DetectionConfidence::Low,
        uncertainty: vec![
            DetectionUncertainty::MissingConfig,
            DetectionUncertainty::MissingCookie,
            DetectionUncertainty::WalletFormatUnknown,
        ],
        source_paths: vec![DetectionSourcePath {
            kind: DetectionSourcePathKind::DataDir,
            path: PathBuf::from("/tmp/custom-core"),
            present: true,
        }],
        maybe_data_dir: Some(PathBuf::from("/tmp/custom-core")),
        maybe_config_file: None,
        maybe_cookie_file: None,
        service_candidates: Vec::new(),
        wallet_candidates: Vec::new(),
    }];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: Some(PathBuf::from("/tmp/custom-core")),
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);

    // Assert
    let MigrationSourceSelection::ManualReviewRequired { reason, .. } = plan.source_selection
    else {
        panic!("expected manual review for unsupported explicit source path");
    };
    assert!(reason.contains("does not yet expose source config, cookie, or wallet evidence"));
}

#[test]
fn planner_limits_service_review_to_selected_source_installation() {
    // Arrange
    let sandbox = TestDirectory::new("selected-service-review");
    let selected_data_dir = sandbox.child("selected/.bitcoin");
    let other_data_dir = sandbox.child("other/.bitcoin");
    fs::create_dir_all(&selected_data_dir).expect("selected datadir");
    fs::create_dir_all(&other_data_dir).expect("other datadir");

    let service_dir = sandbox.child("services");
    fs::create_dir_all(&service_dir).expect("service dir");
    let matched_service_path = service_dir.join("selected.service");
    let other_service_path = service_dir.join("other.service");
    fs::write(
        &matched_service_path,
        format!(
            "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
            selected_data_dir.join("bitcoin.conf").display(),
            selected_data_dir.display()
        ),
    )
    .expect("matched service");
    fs::write(
        &other_service_path,
        format!(
            "[Service]\nExecStart=/usr/bin/bitcoind -conf={} -datadir={}\n",
            other_data_dir.join("bitcoin.conf").display(),
            other_data_dir.display()
        ),
    )
    .expect("other service");

    let resolution = sample_resolution();
    let detections = vec![
        detected_installation_with_services(
            &selected_data_dir,
            vec![
                service_candidate(&matched_service_path, ServiceManager::Systemd),
                service_candidate(&other_service_path, ServiceManager::Systemd),
            ],
        ),
        detected_installation_with_services(&other_data_dir, Vec::new()),
    ];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: Some(selected_data_dir.clone()),
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);

    // Assert
    let MigrationSourceSelection::Selected { installation } = &plan.source_selection else {
        panic!("expected selected source installation");
    };
    let matched_service_path = matched_service_path.display().to_string();
    let other_service_path = other_service_path.display().to_string();
    assert_eq!(installation.service_candidates.len(), 1);
    assert_eq!(
        installation.service_candidates[0].path,
        matched_service_path
    );
    assert!(
        !installation
            .uncertainty
            .iter()
            .any(|uncertainty| uncertainty == "service_review_ambiguous")
    );

    let service_group = plan
        .action_groups
        .iter()
        .find(|group| group.title == "Service")
        .expect("service action group");
    assert!(
        service_group
            .actions
            .iter()
            .any(|action| { action.maybe_path.as_deref() == Some(matched_service_path.as_str()) })
    );
    assert!(
        !service_group
            .actions
            .iter()
            .any(|action| { action.maybe_path.as_deref() == Some(other_service_path.as_str()) })
    );
}

#[test]
fn planner_uses_manual_service_review_when_service_ownership_is_ambiguous() {
    // Arrange
    let sandbox = TestDirectory::new("ambiguous-service-review");
    let selected_data_dir = sandbox.child("selected/.bitcoin");
    fs::create_dir_all(&selected_data_dir).expect("selected datadir");

    let service_dir = sandbox.child("services");
    fs::create_dir_all(&service_dir).expect("service dir");
    let ambiguous_service_path = service_dir.join("ambiguous.service");
    fs::write(
        &ambiguous_service_path,
        "[Service]\nExecStart=/usr/bin/bitcoind\n",
    )
    .expect("ambiguous service");

    let resolution = sample_resolution();
    let detections = vec![detected_installation_with_services(
        &selected_data_dir,
        vec![service_candidate(
            &ambiguous_service_path,
            ServiceManager::Systemd,
        )],
    )];
    let request = MigrationPlanArgs {
        maybe_source_data_dir: Some(selected_data_dir.clone()),
    };

    // Act
    let plan = plan_migration(&resolution, &detections, &request);

    // Assert
    let MigrationSourceSelection::Selected { installation } = &plan.source_selection else {
        panic!("expected selected source installation");
    };
    assert!(installation.service_candidates.is_empty());
    assert!(
        installation
            .uncertainty
            .iter()
            .any(|uncertainty| uncertainty == "service_review_ambiguous")
    );
    assert!(
        plan.relevant_deviations
            .iter()
            .any(|notice| notice.id == "mig-dry-run-only-switch-over")
    );

    let service_group = plan
        .action_groups
        .iter()
        .find(|group| group.title == "Service")
        .expect("service action group");
    let ambiguous_service_path = ambiguous_service_path.display().to_string();
    assert!(service_group.actions.iter().any(|action| {
        action.kind == super::MigrationActionKind::ManualStep
            && action
                .summary
                .contains("could not be confidently tied to the selected source install")
    }));
    assert!(
        !service_group.actions.iter().any(|action| {
            action.maybe_path.as_deref() == Some(ambiguous_service_path.as_str())
        })
    );
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
    detected_installation_with_services(&data_dir, Vec::new())
}

fn detected_installation_with_services(
    data_dir: &Path,
    service_candidates: Vec<ServiceCandidate>,
) -> DetectedInstallation {
    let data_dir = data_dir.to_path_buf();
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
        service_candidates,
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

fn service_candidate(path: &Path, manager: ServiceManager) -> ServiceCandidate {
    ServiceCandidate {
        product_family: ProductFamily::BitcoinCore,
        manager,
        service_name: "bitcoind".to_string(),
        path: path.to_path_buf(),
        present: true,
    }
}

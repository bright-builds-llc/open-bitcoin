// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md

use std::path::PathBuf;

use super::{
    DetectedInstallation, DetectionConfidence, DetectionRoots, DetectionSourcePath,
    DetectionSourcePathKind, DetectionUncertainty, ProductFamily, ServiceCandidate, ServiceManager,
    WalletCandidate, WalletCandidateKind,
};

#[test]
fn detected_installation_records_sources_and_uncertainty() {
    // Arrange
    let data_dir = PathBuf::from("/Users/alice/Library/Application Support/Bitcoin");

    // Act
    let installation = DetectedInstallation {
        product_family: ProductFamily::BitcoinKnots,
        confidence: DetectionConfidence::High,
        uncertainty: vec![DetectionUncertainty::WalletFormatUnknown],
        source_paths: vec![DetectionSourcePath {
            kind: DetectionSourcePathKind::DataDir,
            path: data_dir.clone(),
            present: true,
        }],
        maybe_data_dir: Some(data_dir),
        maybe_config_file: Some(PathBuf::from("/tmp/bitcoin.conf")),
        maybe_cookie_file: Some(PathBuf::from("/tmp/.cookie")),
        service_candidates: vec![ServiceCandidate {
            product_family: ProductFamily::BitcoinKnots,
            manager: ServiceManager::Launchd,
            service_name: "org.bitcoin.bitcoind".to_string(),
            path: PathBuf::from("/Library/LaunchDaemons/org.bitcoin.bitcoind.plist"),
            present: true,
        }],
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::DescriptorWalletDirectory,
            path: PathBuf::from("/tmp/wallets/default"),
            maybe_name: Some("default".to_string()),
            present: true,
        }],
    };

    // Assert
    assert_eq!(installation.product_family, ProductFamily::BitcoinKnots);
    assert_eq!(installation.confidence, DetectionConfidence::High);
    assert_eq!(
        installation.uncertainty,
        vec![DetectionUncertainty::WalletFormatUnknown]
    );
}

#[test]
fn detection_roots_compare_without_filesystem_access() {
    // Arrange
    let roots = DetectionRoots {
        home_dir: PathBuf::from("/Users/alice"),
        data_dirs: vec![PathBuf::from("/Users/alice/.bitcoin")],
        config_dirs: vec![PathBuf::from("/Users/alice/.bitcoin")],
        service_dirs: vec![PathBuf::from("/Library/LaunchDaemons")],
    };

    // Act
    let same_roots = roots.clone();

    // Assert
    assert_eq!(roots, same_roots);
}

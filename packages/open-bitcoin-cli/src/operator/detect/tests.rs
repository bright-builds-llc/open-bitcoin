// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md

use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use super::{
    DetectedInstallation, DetectionConfidence, DetectionRoots, DetectionSourcePath,
    DetectionSourcePathKind, DetectionUncertainty, ProductFamily, ServiceManager, WalletCandidate,
    WalletCandidateKind, WalletChainScope, detect_existing_installations,
};

static NEXT_TEST_DIRECTORY_ID: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestDirectory {
    path: PathBuf,
}

impl TestDirectory {
    fn new(label: &str) -> Self {
        let directory = std::env::temp_dir().join(format!(
            "open-bitcoin-operator-detect-tests-{label}-{}",
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
        wallet_candidates: vec![WalletCandidate {
            kind: WalletCandidateKind::DescriptorWalletDirectory,
            path: PathBuf::from("/tmp/wallets/default"),
            maybe_name: Some("default".to_string()),
            present: true,
            product_family: ProductFamily::BitcoinKnots,
            product_confidence: DetectionConfidence::High,
            chain_scope: WalletChainScope::Mainnet,
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

#[test]
fn detects_linux_core_knots_candidates_read_only() {
    // Arrange
    let sandbox = TestDirectory::new("linux");
    let home = sandbox.child("home/alice");
    let data_dir = home.join(".bitcoin");
    let user_service_dir = home.join(".config/systemd/user");
    let system_service_dir = sandbox.child("etc/systemd/system");
    fs::create_dir_all(data_dir.join("wallets/main")).expect("wallet dir");
    fs::create_dir_all(data_dir.join("regtest/wallets/regwallet")).expect("regtest wallet dir");
    fs::create_dir_all(&user_service_dir).expect("user service dir");
    fs::create_dir_all(&system_service_dir).expect("system service dir");
    fs::write(data_dir.join("bitcoin.conf"), "regtest=1\n").expect("bitcoin.conf");
    fs::write(data_dir.join(".cookie"), "__cookie__:secret").expect("cookie");
    fs::write(data_dir.join("wallet.dat"), "legacy wallet").expect("legacy wallet");
    fs::write(data_dir.join("wallets/main/wallet.dat"), "nested wallet").expect("nested wallet");
    fs::write(
        user_service_dir.join("bitcoind.service"),
        "[Service]\nExecStart=bitcoind\n",
    )
    .expect("user service");
    fs::write(
        system_service_dir.join("bitcoind.service"),
        "[Service]\nExecStart=bitcoind\n",
    )
    .expect("system service");
    let before_conf = read_bytes(data_dir.join("bitcoin.conf"));
    let before_cookie = read_bytes(data_dir.join(".cookie"));
    let before_wallet = read_bytes(data_dir.join("wallets/main/wallet.dat"));
    let before_conf_modified = modified(data_dir.join("bitcoin.conf"));
    let before_wallet_readonly = readonly(data_dir.join("wallets/main/wallet.dat"));
    let roots = DetectionRoots {
        home_dir: home.clone(),
        data_dirs: Vec::new(),
        config_dirs: Vec::new(),
        service_dirs: vec![system_service_dir],
    };

    // Act
    let detections = detect_existing_installations(&roots);

    // Assert
    let installation = detections
        .installations
        .iter()
        .find(|candidate| candidate.maybe_data_dir.as_ref() == Some(&data_dir))
        .expect("linux bitcoin datadir");
    let source_paths = render_source_paths(installation);
    assert!(source_paths.iter().any(|path| path.ends_with(".bitcoin")));
    assert!(
        source_paths
            .iter()
            .any(|path| path.ends_with("bitcoin.conf"))
    );
    assert!(source_paths.iter().any(|path| path.ends_with(".cookie")));
    assert!(
        installation
            .wallet_candidates
            .iter()
            .any(|candidate| candidate.path.ends_with("wallet.dat"))
    );
    assert!(installation.wallet_candidates.iter().any(|candidate| {
        candidate.path.ends_with("wallets/main/wallet.dat")
            && candidate.kind == WalletCandidateKind::LegacyWalletFile
            && candidate.chain_scope == WalletChainScope::Mainnet
            && candidate.product_confidence == DetectionConfidence::Medium
    }));
    assert!(installation.wallet_candidates.iter().any(|candidate| {
        candidate.path.ends_with("regtest/wallets/regwallet")
            && candidate.kind == WalletCandidateKind::DescriptorWalletDirectory
            && candidate.chain_scope == WalletChainScope::Regtest
    }));
    assert!(
        installation
            .wallet_candidates
            .iter()
            .all(|candidate| !candidate.path.ends_with("/wallets"))
    );
    assert!(
        detections
            .service_candidates
            .iter()
            .any(|candidate| candidate.manager == ServiceManager::Systemd && candidate.present)
    );
    assert!(
        !source_paths
            .iter()
            .any(|path| path.ends_with("bitcoind.service"))
    );
    assert_eq!(installation.product_family, ProductFamily::Unknown);
    assert!(
        installation
            .uncertainty
            .contains(&DetectionUncertainty::ProductAmbiguous)
    );
    assert_eq!(read_bytes(data_dir.join("bitcoin.conf")), before_conf);
    assert_eq!(read_bytes(data_dir.join(".cookie")), before_cookie);
    assert_eq!(
        read_bytes(data_dir.join("wallets/main/wallet.dat")),
        before_wallet
    );
    assert_eq!(
        modified(data_dir.join("bitcoin.conf")),
        before_conf_modified
    );
    assert_eq!(
        readonly(data_dir.join("wallets/main/wallet.dat")),
        before_wallet_readonly
    );
}

#[test]
fn detects_macos_core_knots_candidates_read_only() {
    // Arrange
    let sandbox = TestDirectory::new("macos");
    let home = sandbox.child("Users/alice");
    let data_dir = home.join("Library/Application Support/Bitcoin");
    let launch_agent_dir = home.join("Library/LaunchAgents");
    let launch_daemon_dir = sandbox.child("Library/LaunchDaemons");
    fs::create_dir_all(data_dir.join("wallets/main")).expect("wallet dir");
    fs::create_dir_all(data_dir.join("signet/wallets/signet-wallet")).expect("signet wallet dir");
    fs::create_dir_all(&launch_agent_dir).expect("launch agent dir");
    fs::create_dir_all(&launch_daemon_dir).expect("launch daemon dir");
    fs::write(data_dir.join("bitcoin.conf"), "signet=1\n").expect("bitcoin.conf");
    fs::write(data_dir.join(".cookie"), "__cookie__:secret").expect("cookie");
    fs::write(data_dir.join("wallets/main/wallet.dat"), "wallet").expect("wallet");
    fs::write(
        launch_agent_dir.join("org.bitcoin.bitcoind.plist"),
        "<plist></plist>\n",
    )
    .expect("launch agent");
    fs::write(
        launch_daemon_dir.join("org.bitcoin.bitcoind.plist"),
        "<plist></plist>\n",
    )
    .expect("launch daemon");
    let before_conf = read_bytes(data_dir.join("bitcoin.conf"));
    let before_cookie = read_bytes(data_dir.join(".cookie"));
    let before_wallet = read_bytes(data_dir.join("wallets/main/wallet.dat"));
    let before_wallet_readonly = readonly(data_dir.join("wallets/main/wallet.dat"));
    let roots = DetectionRoots {
        home_dir: home,
        data_dirs: Vec::new(),
        config_dirs: Vec::new(),
        service_dirs: vec![launch_daemon_dir],
    };

    // Act
    let detections = detect_existing_installations(&roots);

    // Assert
    let installation = detections
        .installations
        .iter()
        .find(|candidate| candidate.maybe_data_dir.as_ref() == Some(&data_dir))
        .expect("macOS bitcoin datadir");
    let source_paths = render_source_paths(installation);
    assert!(
        source_paths
            .iter()
            .any(|path| path.contains("Library/Application Support/Bitcoin"))
    );
    assert!(
        source_paths
            .iter()
            .any(|path| path.ends_with("bitcoin.conf"))
    );
    assert!(source_paths.iter().any(|path| path.ends_with(".cookie")));
    assert!(installation.wallet_candidates.iter().any(|candidate| {
        candidate.path.ends_with("wallets/main/wallet.dat")
            && candidate.kind == WalletCandidateKind::LegacyWalletFile
            && candidate.chain_scope == WalletChainScope::Mainnet
    }));
    assert!(installation.wallet_candidates.iter().any(|candidate| {
        candidate.path.ends_with("signet/wallets/signet-wallet")
            && candidate.kind == WalletCandidateKind::DescriptorWalletDirectory
            && candidate.chain_scope == WalletChainScope::Signet
    }));
    assert!(
        detections
            .service_candidates
            .iter()
            .any(|candidate| candidate.manager == ServiceManager::Launchd && candidate.present)
    );
    assert!(
        !source_paths
            .iter()
            .any(|path| path.ends_with("org.bitcoin.bitcoind.plist"))
    );
    assert!(
        installation
            .uncertainty
            .contains(&DetectionUncertainty::ProductAmbiguous)
    );
    assert_eq!(read_bytes(data_dir.join("bitcoin.conf")), before_conf);
    assert_eq!(read_bytes(data_dir.join(".cookie")), before_cookie);
    assert_eq!(
        read_bytes(data_dir.join("wallets/main/wallet.dat")),
        before_wallet
    );
    assert_eq!(
        readonly(data_dir.join("wallets/main/wallet.dat")),
        before_wallet_readonly
    );
}

fn render_source_paths(installation: &DetectedInstallation) -> Vec<String> {
    installation
        .source_paths
        .iter()
        .filter(|source_path| source_path.present)
        .map(|source_path| source_path.path.display().to_string())
        .collect()
}

fn read_bytes(path: impl AsRef<Path>) -> Vec<u8> {
    fs::read(path).expect("read fixture")
}

fn modified(path: impl AsRef<Path>) -> std::time::SystemTime {
    fs::metadata(path)
        .expect("fixture metadata")
        .modified()
        .expect("fixture modified time")
}

fn readonly(path: impl AsRef<Path>) -> bool {
    fs::metadata(path)
        .expect("fixture metadata")
        .permissions()
        .readonly()
}

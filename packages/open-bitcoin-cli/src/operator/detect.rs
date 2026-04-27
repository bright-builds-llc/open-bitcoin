// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md

//! Read-only Core and Knots detection contract surface.

use std::{
    fs,
    path::{Path, PathBuf},
};

const BITCOIN_CONF_FILE_NAME: &str = "bitcoin.conf";
const COOKIE_FILE_NAME: &str = ".cookie";
const LEGACY_WALLET_FILE_NAME: &str = "wallet.dat";
const SERVICE_NAME: &str = "bitcoind";
const SYSTEMD_SERVICE_FILE_NAME: &str = "bitcoind.service";
const LAUNCHD_SERVICE_FILE_NAME: &str = "org.bitcoin.bitcoind.plist";
const CHAIN_WALLET_DIRS: [&str; 3] = ["regtest/wallets", "signet/wallets", "testnet3/wallets"];

/// Root paths the read-only detector may inspect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionRoots {
    /// User home directory used to derive default Core/Knots locations.
    pub home_dir: PathBuf,
    /// Candidate datadirs to inspect.
    pub data_dirs: Vec<PathBuf>,
    /// Candidate config directories to inspect.
    pub config_dirs: Vec<PathBuf>,
    /// Candidate service definition directories to inspect.
    pub service_dirs: Vec<PathBuf>,
}

/// Product family inferred from read-only evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ProductFamily {
    /// Bitcoin Core installation.
    BitcoinCore,
    /// Bitcoin Knots installation.
    BitcoinKnots,
    /// Open Bitcoin installation.
    OpenBitcoin,
    /// Evidence exists but cannot identify the product precisely.
    Unknown,
}

/// Detector confidence in a product-family classification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DetectionConfidence {
    /// Strong product-specific evidence.
    High,
    /// General Bitcoin evidence with some product signal.
    Medium,
    /// Weak evidence that should be treated as advisory.
    Low,
}

/// Explicit uncertainty attached to a detected installation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DetectionUncertainty {
    /// Product-specific evidence is ambiguous.
    ProductAmbiguous,
    /// Expected config file is absent.
    MissingConfig,
    /// Cookie path is absent or not yet created.
    MissingCookie,
    /// Service manager could not be identified from files.
    ServiceManagerUnknown,
    /// Wallet candidate format was not identified.
    WalletFormatUnknown,
}

/// Path evidence supporting a detection result.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DetectionSourcePath {
    /// Type of evidence represented by this path.
    pub kind: DetectionSourcePathKind,
    /// Filesystem path observed or expected.
    pub path: PathBuf,
    /// Whether the path existed when inspected.
    pub present: bool,
}

/// Kind of detection evidence represented by a source path.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DetectionSourcePathKind {
    /// Datadir candidate.
    DataDir,
    /// Config-file candidate.
    ConfigFile,
    /// Cookie-file candidate without cookie contents.
    CookieFile,
    /// Service definition candidate.
    ServiceDefinition,
    /// Wallet directory candidate.
    WalletDirectory,
    /// Wallet database or metadata file candidate.
    WalletFile,
}

/// Service definition discovered through read-only file evidence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ServiceCandidate {
    /// Product family the service appears to manage.
    pub product_family: ProductFamily,
    /// Service manager type inferred from the file path.
    pub manager: ServiceManager,
    /// Human-readable service name.
    pub service_name: String,
    /// Service definition path.
    pub path: PathBuf,
    /// Whether the definition exists.
    pub present: bool,
}

/// Service manager family inferred from candidate paths.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ServiceManager {
    /// macOS launchd.
    Launchd,
    /// Linux systemd.
    Systemd,
    /// Unknown or future service manager.
    Unknown,
}

/// Wallet candidate discovered through read-only path evidence.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WalletCandidate {
    /// Wallet storage shape inferred from paths.
    pub kind: WalletCandidateKind,
    /// Candidate wallet path.
    pub path: PathBuf,
    /// Optional wallet name inferred from directory structure.
    pub maybe_name: Option<String>,
    /// Whether the candidate path exists.
    pub present: bool,
}

/// Wallet storage shape inferred from read-only evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WalletCandidateKind {
    /// Descriptor wallet directory.
    DescriptorWalletDirectory,
    /// Legacy `wallet.dat` file.
    LegacyWalletFile,
    /// Unknown wallet-shaped path.
    Unknown,
}

/// Read-only detection result for a Core, Knots, or Open Bitcoin install.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectedInstallation {
    /// Detected product family.
    pub product_family: ProductFamily,
    /// Confidence in the product classification.
    pub confidence: DetectionConfidence,
    /// Ambiguities callers should show before any migration decision.
    pub uncertainty: Vec<DetectionUncertainty>,
    /// Source paths used as evidence.
    pub source_paths: Vec<DetectionSourcePath>,
    /// Candidate datadir, when known.
    pub maybe_data_dir: Option<PathBuf>,
    /// Candidate `bitcoin.conf`, when known.
    pub maybe_config_file: Option<PathBuf>,
    /// Candidate cookie path, when known.
    pub maybe_cookie_file: Option<PathBuf>,
    /// Read-only service definition candidates.
    pub service_candidates: Vec<ServiceCandidate>,
    /// Read-only wallet candidates.
    pub wallet_candidates: Vec<WalletCandidate>,
}

/// Detect existing Core/Knots-style installations from injected roots only.
pub fn detect_existing_installations(roots: &DetectionRoots) -> Vec<DetectedInstallation> {
    let service_candidates = collect_service_candidates(roots);
    let mut installations = Vec::new();

    for data_dir in candidate_data_dirs(roots) {
        let source_paths = collect_data_dir_source_paths(&data_dir);
        let present_source_paths: Vec<_> = source_paths
            .iter()
            .filter(|source_path| source_path.present)
            .cloned()
            .collect();
        let wallet_candidates = collect_wallet_candidates(&data_dir);

        if present_source_paths.is_empty() && wallet_candidates.is_empty() {
            continue;
        }

        let config_file = data_dir.join(BITCOIN_CONF_FILE_NAME);
        let cookie_file = data_dir.join(COOKIE_FILE_NAME);
        let mut uncertainty = Vec::new();
        uncertainty.push(DetectionUncertainty::ProductAmbiguous);
        if !config_file.exists() {
            uncertainty.push(DetectionUncertainty::MissingConfig);
        }
        if !cookie_file.exists() {
            uncertainty.push(DetectionUncertainty::MissingCookie);
        }
        if service_candidates
            .iter()
            .all(|candidate| !candidate.present)
        {
            uncertainty.push(DetectionUncertainty::ServiceManagerUnknown);
        }
        if wallet_candidates.is_empty() {
            uncertainty.push(DetectionUncertainty::WalletFormatUnknown);
        }

        let product_family = classify_product_family(&data_dir);
        let confidence = if config_file.exists() {
            DetectionConfidence::Medium
        } else {
            DetectionConfidence::Low
        };

        let mut all_source_paths = source_paths;
        all_source_paths.extend(
            service_candidates
                .iter()
                .map(|candidate| DetectionSourcePath {
                    kind: DetectionSourcePathKind::ServiceDefinition,
                    path: candidate.path.clone(),
                    present: candidate.present,
                }),
        );

        installations.push(DetectedInstallation {
            product_family,
            confidence,
            uncertainty,
            source_paths: all_source_paths,
            maybe_data_dir: Some(data_dir),
            maybe_config_file: config_file.exists().then_some(config_file),
            maybe_cookie_file: cookie_file.exists().then_some(cookie_file),
            service_candidates: service_candidates.clone(),
            wallet_candidates,
        });
    }

    if installations.is_empty() {
        let present_services: Vec<_> = service_candidates
            .into_iter()
            .filter(|candidate| candidate.present)
            .collect();
        if !present_services.is_empty() {
            let source_paths = present_services
                .iter()
                .map(|candidate| DetectionSourcePath {
                    kind: DetectionSourcePathKind::ServiceDefinition,
                    path: candidate.path.clone(),
                    present: true,
                })
                .collect();
            installations.push(DetectedInstallation {
                product_family: ProductFamily::Unknown,
                confidence: DetectionConfidence::Low,
                uncertainty: vec![
                    DetectionUncertainty::ProductAmbiguous,
                    DetectionUncertainty::MissingConfig,
                    DetectionUncertainty::MissingCookie,
                    DetectionUncertainty::WalletFormatUnknown,
                ],
                source_paths,
                maybe_data_dir: None,
                maybe_config_file: None,
                maybe_cookie_file: None,
                service_candidates: present_services,
                wallet_candidates: Vec::new(),
            });
        }
    }

    installations
}

fn candidate_data_dirs(roots: &DetectionRoots) -> Vec<PathBuf> {
    let mut paths = vec![
        roots.home_dir.join(".bitcoin"),
        roots.home_dir.join("Library/Application Support/Bitcoin"),
    ];
    paths.extend(roots.data_dirs.iter().cloned());
    paths.extend(roots.config_dirs.iter().cloned());
    deduplicate_paths(paths)
}

fn collect_data_dir_source_paths(data_dir: &Path) -> Vec<DetectionSourcePath> {
    [
        (
            DetectionSourcePathKind::DataDir,
            data_dir.to_path_buf(),
            data_dir.is_dir(),
        ),
        (
            DetectionSourcePathKind::ConfigFile,
            data_dir.join(BITCOIN_CONF_FILE_NAME),
            data_dir.join(BITCOIN_CONF_FILE_NAME).is_file(),
        ),
        (
            DetectionSourcePathKind::CookieFile,
            data_dir.join(COOKIE_FILE_NAME),
            data_dir.join(COOKIE_FILE_NAME).is_file(),
        ),
    ]
    .into_iter()
    .map(|(kind, path, present)| DetectionSourcePath {
        kind,
        path,
        present,
    })
    .collect()
}

fn collect_service_candidates(roots: &DetectionRoots) -> Vec<ServiceCandidate> {
    let mut paths = vec![
        roots
            .home_dir
            .join("Library/LaunchAgents")
            .join(LAUNCHD_SERVICE_FILE_NAME),
        roots
            .home_dir
            .join(".config/systemd/user")
            .join(SYSTEMD_SERVICE_FILE_NAME),
    ];
    for dir in &roots.service_dirs {
        paths.extend(service_paths_for_dir(dir));
    }

    deduplicate_paths(paths)
        .into_iter()
        .map(|path| {
            let manager = service_manager_for_path(&path);
            ServiceCandidate {
                product_family: ProductFamily::Unknown,
                manager,
                service_name: SERVICE_NAME.to_string(),
                present: path.is_file(),
                path,
            }
        })
        .collect()
}

fn service_paths_for_dir(dir: &Path) -> Vec<PathBuf> {
    let rendered = dir.to_string_lossy();
    if rendered.contains("LaunchAgents") || rendered.contains("LaunchDaemons") {
        return vec![dir.join(LAUNCHD_SERVICE_FILE_NAME)];
    }
    if rendered.contains("systemd") {
        return vec![dir.join(SYSTEMD_SERVICE_FILE_NAME)];
    }
    vec![
        dir.join(LAUNCHD_SERVICE_FILE_NAME),
        dir.join(SYSTEMD_SERVICE_FILE_NAME),
    ]
}

fn service_manager_for_path(path: &Path) -> ServiceManager {
    let rendered = path.to_string_lossy();
    if rendered.contains("LaunchAgents") || rendered.contains("LaunchDaemons") {
        return ServiceManager::Launchd;
    }
    if rendered.contains("systemd") || rendered.ends_with(SYSTEMD_SERVICE_FILE_NAME) {
        return ServiceManager::Systemd;
    }
    ServiceManager::Unknown
}

fn collect_wallet_candidates(data_dir: &Path) -> Vec<WalletCandidate> {
    let mut candidates = Vec::new();
    let legacy_wallet = data_dir.join(LEGACY_WALLET_FILE_NAME);
    if legacy_wallet.is_file() {
        candidates.push(WalletCandidate {
            kind: WalletCandidateKind::LegacyWalletFile,
            path: legacy_wallet,
            maybe_name: None,
            present: true,
        });
    }
    collect_wallet_directory_candidates(&data_dir.join("wallets"), &mut candidates);
    for relative in CHAIN_WALLET_DIRS {
        collect_wallet_directory_candidates(&data_dir.join(relative), &mut candidates);
    }
    candidates
}

fn collect_wallet_directory_candidates(wallets_dir: &Path, candidates: &mut Vec<WalletCandidate>) {
    if !wallets_dir.is_dir() {
        return;
    }
    candidates.push(WalletCandidate {
        kind: WalletCandidateKind::DescriptorWalletDirectory,
        path: wallets_dir.to_path_buf(),
        maybe_name: None,
        present: true,
    });

    let Ok(entries) = fs::read_dir(wallets_dir) else {
        return;
    };
    for entry_result in entries {
        let Ok(entry) = entry_result else {
            continue;
        };
        let path = entry.path();
        let maybe_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .map(str::to_string);
        if path.is_dir() {
            candidates.push(WalletCandidate {
                kind: WalletCandidateKind::DescriptorWalletDirectory,
                path: path.clone(),
                maybe_name: maybe_name.clone(),
                present: true,
            });
            let nested_legacy_wallet = path.join(LEGACY_WALLET_FILE_NAME);
            if nested_legacy_wallet.is_file() {
                candidates.push(WalletCandidate {
                    kind: WalletCandidateKind::LegacyWalletFile,
                    path: nested_legacy_wallet,
                    maybe_name,
                    present: true,
                });
            }
        } else if path.is_file() {
            let kind = if path.file_name().and_then(|name| name.to_str())
                == Some(LEGACY_WALLET_FILE_NAME)
            {
                WalletCandidateKind::LegacyWalletFile
            } else {
                WalletCandidateKind::Unknown
            };
            candidates.push(WalletCandidate {
                kind,
                path,
                maybe_name,
                present: true,
            });
        }
    }
}

fn classify_product_family(path: &Path) -> ProductFamily {
    let rendered = path.to_string_lossy().to_ascii_lowercase();
    if rendered.contains("knots") {
        return ProductFamily::BitcoinKnots;
    }
    if rendered.contains("core") {
        return ProductFamily::BitcoinCore;
    }
    ProductFamily::Unknown
}

fn deduplicate_paths(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut deduplicated = Vec::new();
    for path in paths {
        if !deduplicated.contains(&path) {
            deduplicated.push(path);
        }
    }
    deduplicated
}

#[cfg(test)]
mod tests;

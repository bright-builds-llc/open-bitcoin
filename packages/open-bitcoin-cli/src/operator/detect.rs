// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md

//! Read-only Core and Knots detection contract surface.

use std::path::PathBuf;

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

#[cfg(test)]
mod tests;

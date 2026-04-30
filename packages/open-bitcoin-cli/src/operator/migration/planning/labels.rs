// Parity breadcrumbs:
// - packages/bitcoin-knots/doc/files.md
// - packages/bitcoin-knots/doc/init.md
// - packages/bitcoin-knots/doc/managing-wallets.md
// - packages/bitcoin-knots/src/common/args.cpp
// - packages/bitcoin-knots/src/common/config.cpp

use std::path::Path;

use crate::operator::{
    NetworkSelection,
    detect::{
        DetectionConfidence, DetectionUncertainty, ProductFamily, ServiceManager,
        WalletCandidateKind, WalletChainScope,
    },
};

pub(super) fn product_family_name(product_family: ProductFamily) -> &'static str {
    match product_family {
        ProductFamily::BitcoinCore => "bitcoin_core",
        ProductFamily::BitcoinKnots => "bitcoin_knots",
        ProductFamily::OpenBitcoin => "open_bitcoin",
        ProductFamily::Unknown => "unknown",
    }
}

pub(super) fn detection_confidence_name(confidence: DetectionConfidence) -> &'static str {
    match confidence {
        DetectionConfidence::High => "high",
        DetectionConfidence::Medium => "medium",
        DetectionConfidence::Low => "low",
    }
}

pub(super) fn detection_uncertainty_name(uncertainty: DetectionUncertainty) -> &'static str {
    match uncertainty {
        DetectionUncertainty::ProductAmbiguous => "product_ambiguous",
        DetectionUncertainty::MissingConfig => "missing_config",
        DetectionUncertainty::MissingCookie => "missing_cookie",
        DetectionUncertainty::ServiceManagerUnknown => "service_manager_unknown",
        DetectionUncertainty::WalletFormatUnknown => "wallet_format_unknown",
    }
}

pub(super) fn service_manager_name(manager: ServiceManager) -> &'static str {
    match manager {
        ServiceManager::Launchd => "launchd",
        ServiceManager::Systemd => "systemd",
        ServiceManager::Unknown => "unknown",
    }
}

pub(super) fn wallet_kind_name(kind: WalletCandidateKind) -> &'static str {
    match kind {
        WalletCandidateKind::DescriptorWalletDirectory => "descriptor_wallet_directory",
        WalletCandidateKind::LegacyWalletFile => "legacy_wallet_dat",
        WalletCandidateKind::Unknown => "unknown",
    }
}

pub(super) fn wallet_chain_scope_name(chain_scope: WalletChainScope) -> &'static str {
    match chain_scope {
        WalletChainScope::Mainnet => "mainnet",
        WalletChainScope::Testnet => "testnet",
        WalletChainScope::Signet => "signet",
        WalletChainScope::Regtest => "regtest",
        WalletChainScope::Unknown => "unknown",
    }
}

pub(super) fn network_name(network: NetworkSelection) -> String {
    match network {
        NetworkSelection::Mainnet => "mainnet",
        NetworkSelection::Testnet => "testnet",
        NetworkSelection::Signet => "signet",
        NetworkSelection::Regtest => "regtest",
    }
    .to_string()
}

pub(super) fn render_path(path: &Path) -> String {
    path.display().to_string()
}

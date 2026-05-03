// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use open_bitcoin_node::status::{HealthSignal, HealthSignalLevel};

use crate::operator::detect::{
    DetectedInstallation, DetectionConfidence, DetectionSourcePathKind, DetectionUncertainty,
    ProductFamily, ServiceManager as DetectServiceManager,
};

use super::{StatusDetectionEvidence, StatusRpcAuthSource};

pub(super) fn detection_health_signals(evidence: &StatusDetectionEvidence) -> Vec<HealthSignal> {
    evidence
        .detected_installations
        .iter()
        .map(detection_health_signal)
        .collect()
}

pub(super) fn live_rpc_bootstrap_health_signal(auth_source: &StatusRpcAuthSource) -> HealthSignal {
    HealthSignal {
        level: HealthSignalLevel::Warn,
        source: "live_rpc_bootstrap".to_string(),
        message: live_rpc_bootstrap_message(auth_source),
    }
}

fn detection_health_signal(installation: &DetectedInstallation) -> HealthSignal {
    let present_paths = installation
        .source_paths
        .iter()
        .filter(|source_path| source_path.present)
        .map(|source_path| {
            format!(
                "{}:{}",
                source_path_kind_name(source_path.kind),
                source_path.path.display()
            )
        })
        .collect::<Vec<_>>();
    let uncertainty = installation
        .uncertainty
        .iter()
        .map(|value| uncertainty_name(*value))
        .collect::<Vec<_>>();
    let confidence = confidence_name(installation.confidence);
    let product = product_family_name(installation.product_family);
    let message = format!(
        "uncertain {product} candidate; confidence={confidence}; paths=[{}]; uncertainty=[{}]",
        present_paths.join(", "),
        uncertainty.join(", ")
    );

    HealthSignal {
        level: HealthSignalLevel::Info,
        source: "detection".to_string(),
        message,
    }
}

pub(super) fn service_manager_name(manager: DetectServiceManager) -> String {
    match manager {
        DetectServiceManager::Launchd => "launchd".to_string(),
        DetectServiceManager::Systemd => "systemd".to_string(),
        DetectServiceManager::Unknown => "unknown".to_string(),
    }
}

fn product_family_name(product_family: ProductFamily) -> &'static str {
    match product_family {
        ProductFamily::BitcoinCore => "bitcoin_core",
        ProductFamily::BitcoinKnots => "bitcoin_knots",
        ProductFamily::OpenBitcoin => "open_bitcoin",
        ProductFamily::Unknown => "unknown",
    }
}

fn confidence_name(confidence: DetectionConfidence) -> &'static str {
    match confidence {
        DetectionConfidence::High => "high",
        DetectionConfidence::Medium => "medium",
        DetectionConfidence::Low => "low",
    }
}

fn uncertainty_name(uncertainty: DetectionUncertainty) -> &'static str {
    match uncertainty {
        DetectionUncertainty::ProductAmbiguous => "product_ambiguous",
        DetectionUncertainty::MissingConfig => "missing_config",
        DetectionUncertainty::MissingCookie => "missing_cookie",
        DetectionUncertainty::ServiceManagerUnknown => "service_manager_unknown",
        DetectionUncertainty::WalletFormatUnknown => "wallet_format_unknown",
    }
}

fn source_path_kind_name(kind: DetectionSourcePathKind) -> &'static str {
    match kind {
        DetectionSourcePathKind::DataDir => "datadir",
        DetectionSourcePathKind::ConfigFile => "config",
        DetectionSourcePathKind::CookieFile => "cookie",
        DetectionSourcePathKind::ServiceDefinition => "service",
        DetectionSourcePathKind::WalletDirectory => "wallet_dir",
        DetectionSourcePathKind::WalletFile => "wallet_file",
    }
}

fn live_rpc_bootstrap_message(auth_source: &StatusRpcAuthSource) -> String {
    match auth_source {
        StatusRpcAuthSource::CookieFile { path } => format!(
            "live RPC was not attempted because no rediscoverable RPC credentials were found for the selected datadir; add RPC auth to the datadir-local bitcoin.conf or provide a discoverable cookie at {}.",
            path.display()
        ),
        StatusRpcAuthSource::UserCredentialsConfigured | StatusRpcAuthSource::None => {
            "live RPC was not attempted because no rediscoverable RPC credentials were found for the selected datadir; add RPC auth to the datadir-local bitcoin.conf or provide a discoverable .cookie.".to_string()
        }
    }
}

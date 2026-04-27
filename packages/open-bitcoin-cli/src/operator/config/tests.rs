// Parity breadcrumbs:
// - packages/bitcoin-knots/src/common/config.cpp
// - packages/bitcoin-knots/src/common/args.cpp

use std::path::PathBuf;

use super::{
    OperatorConfigPathKind, OperatorConfigPathReport, OperatorConfigResolution,
    OperatorConfigSource,
};
use crate::operator::NetworkSelection;

#[test]
fn config_sources_keep_documented_precedence_names() {
    // Arrange
    let sources = OperatorConfigSource::ordered();

    // Act
    let names: Vec<_> = sources.iter().map(|source| source.as_str()).collect();

    // Assert
    assert_eq!(
        names,
        vec![
            "cli_flags",
            "environment",
            "open_bitcoin_jsonc",
            "bitcoin_conf",
            "cookies",
            "defaults",
        ]
    );
}

#[test]
fn config_resolution_compares_deterministically() {
    // Arrange
    let report = OperatorConfigPathReport {
        source: OperatorConfigSource::OpenBitcoinJsonc,
        kind: OperatorConfigPathKind::ConfigFile,
        path: PathBuf::from("/tmp/open-bitcoin.jsonc"),
        present: true,
    };

    // Act
    let left = OperatorConfigResolution {
        ordered_sources: OperatorConfigSource::ordered().to_vec(),
        path_reports: vec![report.clone()],
        maybe_config_path: Some(report.path.clone()),
        maybe_data_dir: Some(PathBuf::from("/tmp/open-bitcoin")),
        maybe_network: Some(NetworkSelection::Regtest),
    };
    let right = left.clone();

    // Assert
    assert_eq!(left, right);
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/init.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/wallet/wallet.cpp

use super::*;
use std::{collections::BTreeMap, path::Path};

fn observe_existing(source: &Source, index: usize, stage: &str) -> String {
    // Non-authorizing observation only: current schema/complete coverage already
    // exist. Generic schema/history recovery is a no-op on this strict shape.
    assert!(source.0.is_dir());
    let reader = FjallNodeStore::open(&source.0).unwrap_or_else(|error| {
        let category = match error {
            open_bitcoin_node::StorageError::BackendFailure { message, .. }
                if message.contains("database locked by another process") =>
            {
                "store_locked"
            }
            _ => "storage_unavailable",
        };
        panic!(
            "logical observer selector={} stage={stage} category={category}",
            CHAINS[index].3
        );
    });
    logical(&reader)
}

fn rejected(index: usize, changed: RuntimeConfig, category: &str) {
    // Arrange: obtaining the capability already admitted the original engine.
    let source = Source::new();
    drop(
        FjallNodeStore::open_canonical_genesis(&source.0, CHAINS[index].0)
            .expect("initial canonical shape"),
    );
    let before = observe_existing(&source, index, "before_capability");
    let opened = FjallNodeStore::reopen_canonical_genesis_existing(&source.0, CHAINS[index].0)
        .expect("sealed input");
    INITIALIZATIONS.set(0);
    // Act
    let error = Context::from_canonical_genesis_runtime_config(&changed, opened)
        .err()
        .expect("factory refusal");
    // Assert: no application initialization; physical engine teardown is separate.
    assert_eq!(INITIALIZATIONS.get(), 0);
    assert_eq!(observe_existing(&source, index, "after_refusal"), before);
    match category {
        "mismatch" => assert!(matches!(error, CanonicalContextError::ChainMismatch)),
        "config" => assert!(matches!(
            error,
            CanonicalContextError::InvalidGenesisConfiguration
        )),
        "history" => assert!(matches!(
            error,
            CanonicalContextError::UnsupportedValidationScope(_)
        )),
        _ => panic!("invalid category"),
    }
    let message = error.to_string();
    assert!(
        message.len() < 1024
            && !message.contains("PRIVATE")
            && !message.contains(&source.0.to_string_lossy().to_string())
    );
}

#[test]
fn phase166_genesis_context_selector_and_capability_mismatches_refuse() {
    let _scenario = scenario();
    for index in 0..4 {
        for (other, profile) in CHAINS.iter().enumerate() {
            if index == other {
                continue;
            }
            // Arrange / Act / Assert
            let mut inconsistent = config(index, BasicFilterIndexSetting::Basic);
            inconsistent.sync.runtime.network = profile.2;
            rejected(index, inconsistent, "mismatch");
            rejected(
                index,
                config(other, BasicFilterIndexSetting::Basic),
                "mismatch",
            );
        }
    }
}

#[test]
fn phase166_genesis_context_invalid_peer_filters_refuse() {
    let _scenario = scenario();
    for index in 0..4 {
        // Arrange / Act / Assert
        let mut changed = config(index, BasicFilterIndexSetting::Disabled);
        changed.peer_block_filters = true;
        rejected(index, changed, "config");
    }
}

fn activation_refusal(kind: u8) {
    let _scenario = scenario();
    for index in 0..4 {
        // Arrange / Act / Assert
        let mut changed = config(index, BasicFilterIndexSetting::Basic);
        match kind {
            0 => changed.sync.mode = crate::config::DaemonSyncMode::MainnetIbd,
            1 => changed.inbound.enabled = true,
            2 => changed.relay.enabled = true,
            3 => changed.block_serving.block_serving.enabled = true,
            4 => changed.block_serving.compact_relay.enabled = true,
            _ => panic!("unknown activation"),
        }
        rejected(index, changed, "history");
    }
}
#[test]
fn phase166_genesis_context_sync_activation_refuses() {
    activation_refusal(0);
}
#[test]
fn phase166_genesis_context_inbound_activation_refuses() {
    activation_refusal(1);
}
#[test]
fn phase166_genesis_context_relay_activation_refuses() {
    activation_refusal(2);
}
#[test]
fn phase166_genesis_context_block_serving_activation_refuses() {
    activation_refusal(3);
}
#[test]
fn phase166_genesis_context_compact_relay_activation_refuses() {
    activation_refusal(4);
}
#[cfg(unix)]
fn physical(root: &Path) -> BTreeMap<PathBuf, (Vec<u8>, u32, std::time::SystemTime)> {
    use std::os::unix::fs::PermissionsExt;
    fn visit(
        root: &Path,
        entry: &Path,
        result: &mut BTreeMap<PathBuf, (Vec<u8>, u32, std::time::SystemTime)>,
    ) {
        let metadata = fs::symlink_metadata(entry).expect("metadata");
        assert!(!metadata.is_symlink());
        let bytes = if metadata.is_file() {
            fs::read(entry).expect("bytes")
        } else {
            vec![]
        };
        result.insert(
            entry.strip_prefix(root).expect("relative").to_owned(),
            (
                bytes,
                metadata.permissions().mode(),
                metadata.modified().expect("mtime"),
            ),
        );
        if metadata.is_dir() {
            for child in fs::read_dir(entry).expect("directory") {
                visit(root, &child.expect("entry").path(), result);
            }
        }
    }
    let mut result = BTreeMap::new();
    visit(root, root, &mut result);
    result
}

#[cfg(unix)]
#[test]
fn phase166_genesis_context_closed_foreign_and_unproved_sources_preserved() {
    let _scenario = scenario();
    for index in 0..4 {
        for shape in ["foreign", "raw", "forged-coins"] {
            // Arrange: all writers are closed before the strict physical snapshot.
            let source = Source::new();
            if shape == "foreign" {
                drop(
                    FjallNodeStore::open_canonical_genesis(&source.0, CHAINS[(index + 1) % 4].0)
                        .expect("foreign canonical store"),
                );
            } else {
                let store = FjallNodeStore::open(&source.0).expect("unproved raw fixture");
                if shape == "forged-coins" {
                    let header = CHAINS[index].0.spec().genesis_header();
                    let snapshot = open_bitcoin_node::core::chainstate::ChainstateSnapshot::new(vec![
                        open_bitcoin_node::core::chainstate::ChainPosition::new(header, 0,
                            open_bitcoin_node::core::chainstate::ChainPosition::untrusted_fixture_work(1), 0)
                    ], Default::default(), Default::default());
                    store
                        .seed_coins_from_snapshot(&snapshot)
                        .expect("unproved numerical binding");
                    store
                        .save_chainstate_snapshot(&snapshot, open_bitcoin_node::PersistMode::Sync)
                        .expect("forged snapshot");
                }
            }
            let before = physical(&source.0);
            // Act
            let error =
                FjallNodeStore::reopen_canonical_genesis_existing(&source.0, CHAINS[index].0)
                    .err()
                    .expect("unproved opener refusal");
            // Assert: no context factory is entered; atime is intentionally excluded.
            assert_eq!(physical(&source.0), before);
            let message = error.to_string();
            assert!(
                message.len() < 1024
                    && !message.contains("PRIVATE")
                    && !message.contains(&source.0.to_string_lossy().to_string())
            );
        }
    }
}

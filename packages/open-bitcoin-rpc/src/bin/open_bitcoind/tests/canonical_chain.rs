// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use open_bitcoin_node::{
    SyncNetwork,
    core::{chainstate::PruneMode, wallet::AddressNetwork},
};
use open_bitcoin_rpc::config::{BasicFilterIndexSetting, DaemonSyncMode};
use std::collections::BTreeMap;

static PROCESS_SCENARIO: std::sync::Mutex<()> = std::sync::Mutex::new(());

pub(super) fn scenario() -> std::sync::MutexGuard<'static, ()> {
    PROCESS_SCENARIO.lock().expect("canonical process scenario")
}

pub(super) const NETWORKS: [(AddressNetwork, SyncNetwork, &str); 4] = [
    (AddressNetwork::Mainnet, SyncNetwork::Mainnet, "mainnet"),
    (AddressNetwork::Testnet, SyncNetwork::Testnet, "testnet3"),
    (AddressNetwork::Signet, SyncNetwork::Signet, "signet"),
    (AddressNetwork::Regtest, SyncNetwork::Regtest, "regtest"),
];

pub(super) fn owned_path(label: &str) -> PathBuf {
    // macOS temp_dir can contain /var, a symlink deliberately refused by production.
    std::env::temp_dir()
        .canonicalize()
        .expect("physical temp parent")
        .join(temp_store_path(label).file_name().expect("fixture name"))
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PhysicalEntry {
    kind: fs::FileType,
    bytes: Vec<u8>,
    length: u64,
    modified: SystemTime,
    readonly: bool,
    maybe_link: Option<PathBuf>,
    #[cfg(unix)]
    mode: u32,
}

pub(super) fn manifest(root: &Path) -> BTreeMap<PathBuf, PhysicalEntry> {
    fn visit(root: &Path, path: &Path, entries: &mut BTreeMap<PathBuf, PhysicalEntry>) {
        let metadata = fs::symlink_metadata(path).expect("owned metadata");
        let kind = metadata.file_type();
        entries.insert(
            path.strip_prefix(root).expect("prefix").to_path_buf(),
            PhysicalEntry {
                kind,
                bytes: if kind.is_file() {
                    fs::read(path).expect("owned bytes")
                } else {
                    Vec::new()
                },
                length: metadata.len(),
                modified: metadata.modified().expect("mtime"),
                readonly: metadata.permissions().readonly(),
                maybe_link: kind
                    .is_symlink()
                    .then(|| fs::read_link(path).expect("owned link")),
                #[cfg(unix)]
                mode: {
                    use std::os::unix::fs::PermissionsExt;
                    metadata.permissions().mode()
                },
            },
        );
        if kind.is_dir() {
            for entry in fs::read_dir(path).expect("owned directory") {
                visit(root, &entry.expect("owned entry").path(), entries);
            }
        }
    }
    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

pub(super) fn config(
    path: &Path,
    address: AddressNetwork,
    sync: SyncNetwork,
    triggers: u8,
) -> RuntimeConfig {
    let mut runtime = RuntimeConfig {
        chain: address,
        maybe_data_dir: Some(path.to_path_buf()),
        ..RuntimeConfig::default()
    };
    runtime.sync.runtime.network = sync;
    if triggers & 1 != 0 {
        runtime.sync.mode = DaemonSyncMode::MainnetIbd;
    }
    runtime.inbound.enabled = triggers & 2 != 0;
    if triggers & 4 != 0 {
        runtime.prune_mode = PruneMode::ManualOnly;
    }
    if triggers & 8 != 0 {
        runtime.block_filter_index = BasicFilterIndexSetting::Basic;
    } else if triggers & 16 != 0 {
        runtime.block_filter_index = BasicFilterIndexSetting::Disabled;
    }
    runtime
}

fn assert_refusal(message: &str, selected: &str) {
    assert!(message.contains(selected), "{message}");
    assert!(message.contains("canonical_bootstrap"), "{message}");
    assert!(message.contains("exact_work"), "{message}");
    assert!(message.contains("candidate_history"), "{message}");
    assert!(message.len() < 1024);
    assert!(!message.contains("PRIVATE"));
}

#[test]
fn canonical_chain_daemon_every_durable_trigger_preserves_closed_store() {
    let _scenario = super::canonical_chain::scenario();
    // Arrange
    let path = owned_path("PRIVATE\nquoted-store");
    let store = FjallNodeStore::open(&path).expect("owned fixture store");
    drop(store);
    fs::write(path.join("private-sentinel"), b"private data").expect("sentinel");
    let before = manifest(&path);
    for (address, sync, selected) in NETWORKS {
        for triggers in 1..32 {
            let runtime = config(&path, address, sync, triggers);
            // Act
            let result = crate::open_runtime_store(&runtime);
            // Assert
            let message = result.err().expect("ordinary refusal").to_string();
            assert_refusal(&message, selected);
            assert!(message.contains("existing_state_unproved"), "{message}");
            assert_eq!(manifest(&path), before);
        }
    }
    remove_dir_if_exists(&path);
}

#[test]
fn canonical_chain_daemon_missing_and_empty_paths_keep_shape_for_all_triggers() {
    let _scenario = super::canonical_chain::scenario();
    for (address, sync, selected) in NETWORKS {
        for empty in [false, true] {
            // Arrange
            let path = owned_path("missing-empty");
            if empty {
                fs::create_dir(&path).expect("empty fixture");
            }
            let maybe_before = empty.then(|| manifest(&path));
            for triggers in (1..32).filter(|triggers| triggers & 3 != 0) {
                let runtime = config(&path, address, sync, triggers);
                // Act
                let result = crate::open_runtime_store(&runtime);
                // Assert
                assert_refusal(
                    &result.err().expect("unsupported scope").to_string(),
                    selected,
                );
                assert_eq!(path.exists(), empty);
                if let Some(before) = maybe_before.as_ref() {
                    assert_eq!(&manifest(&path), before);
                }
            }
            remove_dir_if_exists(&path);
        }
    }
}

#[test]
fn canonical_chain_daemon_transient_and_supplied_raw_store_never_grant_authority() {
    let _scenario = super::canonical_chain::scenario();
    for (address, sync, selected) in NETWORKS {
        // Arrange
        let path = owned_path("supplied");
        let runtime = config(&path, address, sync, 0);
        let store = FjallNodeStore::open(&path).expect("owned fixture");
        let before = store.load_chainstate_snapshot().expect("active chain");
        let _sealed_signature: fn(
            &RuntimeConfig,
            Option<open_bitcoin_node::storage::OpenedCanonicalGenesis>,
        ) -> Result<
            crate::OpenedAuthoritativeRuntime,
            crate::DaemonSyncPreflightError,
        > = crate::open_authoritative_network_runtime;
        for _ in 0..2 {
            assert!(
                crate::open_runtime_store(&runtime)
                    .expect("storage-free selection")
                    .is_none()
            );
            // Act
            let result = crate::open_authoritative_network_runtime(&runtime, None);
            // Assert
            assert_refusal(
                &result.err().expect("ordinary scope refusal").to_string(),
                selected,
            );
            assert_eq!(
                store.load_chainstate_snapshot().expect("unchanged chain"),
                before
            );
        }
        drop(store);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn canonical_chain_daemon_metrics_fallback_preserves_closed_store_and_missing_shape() {
    let _scenario = super::canonical_chain::scenario();
    for (address, sync, selected) in NETWORKS {
        for shape in ["missing", "empty_directory", "existing_unproved"] {
            // Arrange
            let path = owned_path("metrics-PRIVATE");
            let existing = shape != "missing";
            if shape == "empty_directory" {
                fs::create_dir(&path).expect("owned empty fixture");
            } else if existing {
                drop(FjallNodeStore::open(&path).expect("owned fixture"));
            }
            let maybe_before = existing.then(|| manifest(&path));
            let runtime = config(&path, address, sync, 2);
            let shared = Arc::new(tokio::sync::Mutex::new(
                ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
            ));
            // Act
            let result =
                crate::inbound_metrics::start_inbound_metrics_worker(&runtime, shared, None);
            // Assert
            assert_refusal(
                &result.err().expect("metrics refusal").to_string(),
                selected,
            );
            assert_eq!(path.exists(), existing);
            if let Some(before) = maybe_before {
                assert_eq!(manifest(&path), before);
            }
            remove_dir_if_exists(&path);
        }
    }
}

#[test]
fn canonical_chain_daemon_supplied_metrics_handle_does_not_authorize_worker_or_writes() {
    let _scenario = super::canonical_chain::scenario();
    for (address, sync, selected) in NETWORKS {
        // Arrange
        let path = owned_path("supplied-metrics");
        let runtime = config(&path, address, sync, 2);
        let store = FjallNodeStore::open(&path).expect("owned raw fixture");
        let before_metrics = store.load_metrics_snapshot().expect("metrics snapshot");
        let before_runtime = store.load_runtime_metadata().expect("runtime snapshot");
        let shared = Arc::new(tokio::sync::Mutex::new(
            ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
        ));
        // Act
        let result = crate::inbound_metrics::start_inbound_metrics_worker(
            &runtime,
            shared,
            Some(store.clone()),
        );
        // Assert
        assert_refusal(
            &result
                .err()
                .expect("supplied metrics scope refusal")
                .to_string(),
            selected,
        );
        assert_eq!(
            store.load_metrics_snapshot().expect("unchanged metrics"),
            before_metrics
        );
        assert_eq!(
            store.load_runtime_metadata().expect("unchanged runtime"),
            before_runtime
        );
        drop(store);
        remove_dir_if_exists(&path);
    }
}

#[test]
fn canonical_chain_daemon_absent_datadir_never_silently_succeeds_for_durable_triggers() {
    let _scenario = super::canonical_chain::scenario();
    for (address, sync, selected) in NETWORKS {
        for triggers in 1..32 {
            // Arrange
            let mut runtime = config(&owned_path("absent"), address, sync, triggers);
            runtime.maybe_data_dir = None;
            // Act
            let result = crate::open_runtime_store(&runtime);
            // Assert
            let message = result.err().expect("absent datadir refusal").to_string();
            assert_refusal(&message, selected);
            assert!(message.contains("observation=missing"));
        }
    }
}

#[test]
fn canonical_chain_daemon_inconsistent_selectors_refuse_before_any_store_or_owner() {
    let _scenario = super::canonical_chain::scenario();
    // Arrange
    let path = owned_path("mismatch");
    for (address, _, _) in NETWORKS {
        for (other, sync, _) in NETWORKS {
            if address == other {
                continue;
            }
            let runtime = config(&path, address, sync, 2);
            let shared = Arc::new(tokio::sync::Mutex::new(
                ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
            ));
            // Act
            let store = crate::open_runtime_store(&runtime);
            let owner = crate::open_authoritative_network_runtime(&runtime, None);
            let metrics =
                crate::inbound_metrics::start_inbound_metrics_worker(&runtime, shared, None);
            // Assert
            for message in [
                store.err().expect("store mismatch"),
                owner.err().expect("owner mismatch"),
                metrics.err().expect("metrics mismatch"),
            ] {
                assert!(
                    message
                        .to_string()
                        .starts_with("inconsistent_network_selectors:")
                );
            }
            assert!(!path.exists());
        }
    }
}

#[cfg(unix)]
#[test]
fn canonical_chain_daemon_unsafe_alias_and_non_directory_preserve_physical_sources() {
    let _scenario = super::canonical_chain::scenario();
    // Arrange
    let source = owned_path("unsafe-source");
    drop(FjallNodeStore::open(&source).expect("owned closed source"));
    let before = manifest(&source);
    let alias = owned_path("PRIVATE-alias");
    std::os::unix::fs::symlink(&source, &alias).expect("owned alias");
    let file = owned_path("PRIVATE-file");
    fs::write(&file, b"malformed private backend bytes").expect("owned file");
    let file_before = manifest(&file);
    for (address, sync, selected) in NETWORKS {
        for target in [&alias, &file] {
            let runtime = config(target, address, sync, 2);
            let shared = Arc::new(tokio::sync::Mutex::new(
                ManagedRpcContext::from_fixture_runtime_config(&RuntimeConfig::default()),
            ));
            // Act
            let store = crate::open_runtime_store(&runtime);
            let metrics =
                crate::inbound_metrics::start_inbound_metrics_worker(&runtime, shared, None);
            // Assert
            for message in [
                store.err().expect("store unsafe refusal"),
                metrics.err().expect("metrics unsafe refusal"),
            ] {
                assert_refusal(&message.to_string(), selected);
                assert!(message.to_string().starts_with("unsafe_path:"));
            }
            assert_eq!(manifest(&source), before);
            assert_eq!(manifest(&file), file_before);
            assert_eq!(fs::read_link(&alias).expect("unchanged alias"), source);
        }
    }
    fs::remove_file(alias).expect("remove owned alias");
    fs::remove_file(file).expect("remove owned file");
    remove_dir_if_exists(&source);
}

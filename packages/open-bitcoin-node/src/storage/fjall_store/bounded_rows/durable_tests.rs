// Parity breadcrumbs:
// - packages/bitcoin-knots/src/txdb.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::BasicFilterStartupMode;
use crate::chainstate::validation_history::tests::phase167 as genuine;
use open_bitcoin_core::consensus::ScriptVerifyFlags;
#[cfg(unix)]
use std::os::unix::fs::MetadataExt;
use std::{
    path::{Path, PathBuf},
    time::{Instant, SystemTime},
};

#[derive(Debug)]
pub(crate) struct DurableRowsMeasurement {
    pub(crate) logical_entries: usize,
    pub(crate) logical_bytes: usize,
    pub(crate) authority_max: usize,
    pub(crate) basic_max: usize,
    pub(crate) body_max: usize,
    pub(crate) undo_max: usize,
    pub(crate) physical_entries: usize,
    pub(crate) physical_bytes: u64,
    pub(crate) copy_bytes: u64,
    pub(crate) elapsed: std::time::Duration,
    pub(crate) manifest_equal: bool,
}

#[derive(Debug, PartialEq, Eq)]
struct PhysicalEntry {
    directory: bool,
    length: u64,
    modified: SystemTime,
    mode: u32,
    digest: [u8; 32],
}

fn physical_manifest(root: &Path) -> BTreeMap<PathBuf, PhysicalEntry> {
    fn visit(root: &Path, relative: &Path, rows: &mut BTreeMap<PathBuf, PhysicalEntry>) {
        let path = root.join(relative);
        let metadata = std::fs::symlink_metadata(&path).expect("physical metadata");
        assert!(!metadata.file_type().is_symlink());
        #[cfg(unix)]
        let mode = metadata.mode();
        #[cfg(not(unix))]
        let mode = 0;
        let digest = if metadata.is_dir() {
            [0; 32]
        } else {
            open_bitcoin_core::consensus::crypto::Sha256::digest(
                &std::fs::read(&path).expect("physical bytes"),
            )
        };
        rows.insert(
            relative.to_path_buf(),
            PhysicalEntry {
                directory: metadata.is_dir(),
                length: metadata.len(),
                modified: metadata.modified().expect("mtime"),
                mode,
                digest,
            },
        );
        if metadata.is_dir() {
            for entry in std::fs::read_dir(path).expect("physical directory") {
                visit(
                    root,
                    &relative.join(entry.expect("physical entry").file_name()),
                    rows,
                );
            }
        }
    }
    let mut rows = BTreeMap::new();
    visit(root, Path::new(""), &mut rows);
    rows
}

// Prevent competing test copies from inflating the physical 30-second measurement.
static DETACHED_MEASUREMENT: std::sync::Mutex<()> = std::sync::Mutex::new(());
/// Detached test measurements only. No records, lease, engine or proof escapes.
pub(crate) fn measure_detached_rows_for_test(
    root: &Path,
) -> Result<DurableRowsMeasurement, StorageError> {
    let _serial = match DETACHED_MEASUREMENT.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };
    let lease = super::super::open_lease::OperationLease::acquire(root)?;
    let before = physical_manifest(root);
    let physical_bytes = before
        .values()
        .filter(|e| !e.directory)
        .map(|e| e.length)
        .sum();
    assert!(before.len() <= 4096 && physical_bytes <= 536870912);
    let started = Instant::now();
    let mut measured = super::super::inspection::with_inspection(&lease, |records| {
        let mut measured = DurableRowsMeasurement {
            logical_entries: 0,
            logical_bytes: 0,
            authority_max: 0,
            basic_max: 0,
            body_max: 0,
            undo_max: 0,
            physical_entries: before.len(),
            physical_bytes,
            copy_bytes: physical_bytes,
            elapsed: started.elapsed(),
            manifest_equal: false,
        };
        for name in records.namespaces() {
            measured.logical_entries += 1;
            measured.logical_bytes += name.len();
            let rows = records.maybe_records(name).expect("enumerated namespace");
            for (key, value) in rows {
                measured.logical_entries += 1;
                measured.logical_bytes += key.len() + value.len();
                if matches!(
                    name,
                    "schema" | "headers" | "block_index" | "chainstate" | "coins"
                ) {
                    measured.authority_max = measured.authority_max.max(value.len());
                }
                if key.starts_with(b"basic_filter:") {
                    measured.basic_max = measured.basic_max.max(value.len());
                }
                if name == "block_index" && key.starts_with(b"block:") {
                    measured.body_max = measured.body_max.max(value.len());
                }
                if name == "chainstate" && key.starts_with(b"undo:") {
                    measured.undo_max = measured.undo_max.max(value.len());
                }
            }
            let maybe_family = match name {
                "headers" => Some(RowFamily::Headers),
                "block_index" => Some(RowFamily::BlockIndex),
                "chainstate" => Some(RowFamily::ChainMeta),
                _ => None,
            };
            if let Some(family) = maybe_family {
                let bytes = decode_detached_rows(family, rows, true)?.expect("complete family");
                if name == "headers" && rows.contains_key(family.manifest_key().as_bytes()) {
                    for mutation in super::tests::TAMPERS {
                        let mut changed = rows.clone();
                        super::tests::corrupt(&mut changed, mutation);
                        assert!(decode_detached_rows(family, &changed, true).is_err());
                    }
                }
                // Refuse a copied malformed map without touching the original.
                if let Some(key) = rows
                    .keys()
                    .find(|key| key.as_slice() == family.manifest_key().as_bytes())
                {
                    let mut changed = rows.clone();
                    changed.get_mut(key).expect("manifest").push(0);
                    assert!(decode_detached_rows(family, &changed, true).is_err());
                    assert!(!bytes.is_empty());
                }
            }
        }
        assert!(measured.logical_entries <= 4096 && measured.logical_bytes <= 536870912);
        assert!(measured.authority_max <= 65536);
        Ok(measured)
    })?;
    measured.manifest_equal = before == physical_manifest(root);
    assert!(measured.manifest_equal);
    assert!(measured.physical_entries <= 4096 && measured.physical_bytes <= 536870912);
    assert_eq!(measured.copy_bytes, measured.physical_bytes);
    assert!(
        measured.elapsed.as_secs_f64() < 30.0,
        "actual isolated copy/recovery elapsed {:?}",
        measured.elapsed
    );
    Ok(measured)
}

fn basic_rows(store: &FjallNodeStore) -> BTreeMap<Vec<u8>, Vec<u8>> {
    store
        .block_index
        .prefix("basic_filter:")
        .map(|guard| {
            let (key, value) = guard.into_inner().expect("complete BASIC rows");
            (key.to_vec(), value.to_vec())
        })
        .collect()
}

fn ordinary_301(mode: BasicFilterStartupMode) {
    genuine::with_mode(mode, |manager, store, params| {
        // Arrange: real proved genesis, actual held network admission, real absorption.
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let mut blocks = vec![
            open_bitcoin_core::consensus::CanonicalChain::Regtest
                .spec()
                .genesis_block(),
        ];
        let mut parent = network.maybe_chain_tip().expect("genesis").header;
        for height in 1..=301 {
            let block = genuine::child(&parent, height);
            genuine::admit(&mut network, std::slice::from_ref(&block), params);
            store
                .save_block(&block, PersistMode::Sync)
                .expect("actual received body retention");
            network
                .connect_local_block(&block, ScriptVerifyFlags::NONE, params)
                .expect("actual managed absorption");
            parent = block.header.clone();
            blocks.push(block);
        }
        network
            .persist_validation_header_snapshot()
            .expect("trusted paired checkpoint");
        network
            .chainstate_mut()
            .flush_with_mode(
                open_bitcoin_core::chainstate::FlushMode::Always,
                open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(50_001),
                u64::MAX,
            )
            .expect("full coins checkpoint");
        let network = crate::ManagedNetworkHandle::new(network);
        if mode == BasicFilterStartupMode::Enabled {
            network
                .initialize_basic_filter_index_owner()
                .expect("real ordered BASIC owner from validated flush");
            let mut maybe_previous_height = None;
            for _ in 0..302 {
                let turn = network
                    .drive_basic_filter_index_turn()
                    .expect("actual bounded BASIC turn");
                let height = turn
                    .maybe_progress
                    .expect("enabled progress")
                    .maybe_processed_endpoint()
                    .expect("catch-up endpoint")
                    .height();
                assert!(
                    maybe_previous_height.is_none_or(|previous| height > previous),
                    "BASIC catch-up stalled"
                );
                maybe_previous_height = Some(height);
                if turn.maybe_accepted_lag == Some(0) {
                    break;
                }
            }
        } else {
            assert!(
                network
                    .drive_basic_filter_index_turn()
                    .expect("disabled turn")
                    .maybe_progress
                    .is_none()
            );
        }
        let snapshot = network.chainstate_snapshot().expect("live complete state");
        let entries = network.header_entries().expect("held headers");
        let basic = store.maybe_basic_filter_state().expect("BASIC state");
        let progress = network.maybe_basic_index_progress().expect("progress");
        assert_eq!(progress.is_some(), mode == BasicFilterStartupMode::Enabled);
        if let Some(progress) = progress {
            let proof = store
                .maybe_basic_filter_append_proof()
                .expect("live proof")
                .expect("live owner");
            assert_eq!(proof.branch_identity(), progress.branch_identity());
            assert!(
                snapshot
                    .active_chain
                    .iter()
                    .any(|p| p.block_hash == proof.branch_identity())
            );
            assert_eq!(
                progress.maybe_processed_endpoint().map(|p| p.height()),
                Some(301)
            );
            assert_eq!(
                progress.maybe_safe_durable_endpoint().map(|p| p.height()),
                Some(301)
            );
        }
        let basic_rows = basic_rows(&store);
        if mode == BasicFilterStartupMode::Enabled {
            let mut maybe_previous = None;
            for position in &snapshot.active_chain {
                let key = crate::storage::filter_index::record_key(position.block_hash);
                let record = crate::storage::filter_index::decode_record(
                    &key,
                    basic_rows
                        .get(key.as_bytes())
                        .expect("every complete BASIC record"),
                    maybe_previous.as_ref(),
                )
                .expect("complete linked filter/header commitment");
                assert_eq!(record.identity().height(), position.height);
                assert_eq!(record.identity().block_hash(), position.block_hash);
                assert_eq!(
                    store
                        .basic_filter_projection(position.height)
                        .expect("active branch"),
                    position.block_hash
                );
                maybe_previous = Some(record.identity());
            }
            assert_eq!(
                basic_rows
                    .keys()
                    .filter(|key| key
                        .starts_with(crate::storage::filter_index::RECORD_PREFIX.as_bytes()))
                    .count(),
                302
            );
        } else {
            assert!(basic_rows.is_empty());
        }
        let statuses: Vec<_> = snapshot
            .active_chain
            .iter()
            .map(|p| {
                (
                    store
                        .maybe_validation_history_record(p.block_hash)
                        .expect("record"),
                    store
                        .validation_provenance(p.block_hash)
                        .expect("provenance"),
                )
            })
            .collect();
        assert_eq!(snapshot.active_chain.len(), 302);
        assert_eq!(snapshot.utxos.len(), 301);
        assert_eq!(snapshot.undo_by_block.len(), 301);
        let stored = store
            .wallet_scan_chainstate_snapshot()
            .expect("wallet loader")
            .expect("data");
        assert_eq!(stored.active_chain, snapshot.active_chain);
        assert_eq!(stored.utxos, snapshot.utxos);
        assert_eq!(stored.undo_by_block, snapshot.undo_by_block);
        assert_eq!(
            stored.maybe_confirmed_txid_counts,
            snapshot.maybe_confirmed_txid_counts
        );
        assert!(
            snapshot
                .maybe_confirmed_txid_counts
                .as_ref()
                .is_some_and(|counts| counts.len() == 301)
        );
        // Act: every owner closes before isolated recovery and actual storage reopen.
        drop(network);
        drop(store);
        let measurements: super::super::DurableRowsMeasurement =
            super::super::measure_detached_rows_for_test(&root).expect("actual copied worker");
        eprintln!("DURABLE_301 mode={mode:?} {measurements:?} counts_policy=Some_exact_live_map");
        let reopened = FjallNodeStore::open(&root).expect("all-handle physical reopen");
        for block in &blocks {
            assert_eq!(
                reopened
                    .load_block(block_hash(&block.header))
                    .expect("complete body"),
                Some(block.clone())
            );
        }
        // Assert: full typed equality includes exact full work, MTP and complete undo fields.
        assert_eq!(
            reopened
                .load_header_entries()
                .expect("headers")
                .expect("present")
                .entries,
            entries
        );
        assert_eq!(
            reopened
                .load_block_index_entries()
                .expect("index")
                .expect("present")
                .entries,
            entries
        );
        assert_eq!(
            reopened
                .wallet_scan_chainstate_snapshot()
                .expect("wallet")
                .expect("present"),
            stored
        );
        assert_eq!(
            reopened
                .hydrate_chainstate_for_open()
                .expect("hydrate")
                .expect("present"),
            stored
        );
        assert_eq!(
            reopened.load_chain_meta_for_open().expect("metadata"),
            (
                snapshot.active_chain.clone(),
                snapshot.maybe_confirmed_txid_counts.clone()
            )
        );
        assert_eq!(reopened.maybe_basic_filter_state().expect("BASIC"), basic);
        assert_eq!(
            self::basic_rows(&reopened),
            basic_rows,
            "every BASIC byte/header/active branch survives reopen"
        );
        if let Some(progress) = progress {
            assert!(
                reopened
                    .maybe_basic_filter_append_proof()
                    .expect("data-only reopen")
                    .is_none()
            );
            reopened
                .configure_basic_filter_index_before_prune(
                    Some(
                        stored
                            .active_chain
                            .last()
                            .expect("actual stored tip")
                            .block_hash,
                    ),
                    BasicFilterStartupMode::Enabled,
                )
                .expect("explicit configured BASIC recovery on data-only storage");
            assert_eq!(
                reopened
                    .maybe_basic_filter_state()
                    .expect("unchanged configured state"),
                basic
            );
            assert_eq!(
                self::basic_rows(&reopened),
                basic_rows,
                "configuration preserves all BASIC bytes"
            );
            assert_eq!(
                reopened
                    .wallet_scan_chainstate_snapshot()
                    .expect("configured snapshot")
                    .expect("present"),
                stored
            );
            let proof = reopened
                .maybe_basic_filter_append_proof()
                .expect("recovered BASIC proof")
                .expect("BASIC owner");
            assert_eq!(proof.generation(), progress.generation());
            assert_eq!(
                proof.branch_identity(),
                snapshot
                    .active_chain
                    .last()
                    .expect("captured live tip")
                    .block_hash
            );
            assert_eq!(proof.protection(), progress.protection());
            let endpoint =
                |checkpoint: open_bitcoin_core::chainstate::FilterCheckpoint| match checkpoint
                    .prefix()
                {
                    open_bitcoin_core::chainstate::IndexPrefix::Empty => None,
                    open_bitcoin_core::chainstate::IndexPrefix::Committed(identity) => {
                        Some(identity)
                    }
                };
            assert_eq!(
                endpoint(proof.processed()),
                progress.maybe_processed_endpoint()
            );
            assert_eq!(
                endpoint(proof.safe_checkpoint()),
                progress.maybe_safe_durable_endpoint()
            );
        }
        for (position, expected) in stored.active_chain.iter().zip(statuses) {
            assert_eq!(
                (
                    reopened
                        .maybe_validation_history_record(position.block_hash)
                        .expect("status"),
                    reopened
                        .validation_provenance(position.block_hash)
                        .expect("provenance")
                ),
                expected
            );
        }
        drop(reopened);
    });
}

#[test]
fn phase167_durable_rows_reopen_actual_301_basic_disabled() {
    ordinary_301(BasicFilterStartupMode::Disabled);
}

#[test]
fn phase167_durable_rows_reopen_actual_301_basic_enabled() {
    ordinary_301(BasicFilterStartupMode::Enabled);
}

fn trusted_header_fault(after: bool) {
    genuine::with_manager(|manager, store, params| {
        // Arrange
        let root = store.datadir().to_path_buf();
        let mut network = genuine::network(manager);
        let old = store
            .load_header_entries()
            .expect("old")
            .expect("genesis")
            .entries;
        let mut parent = old[0].header.clone();
        let mut blocks = Vec::new();
        for height in 1..=301 {
            let block = genuine::child(&parent, height);
            parent = block.header.clone();
            blocks.push(block);
        }
        genuine::admit(&mut network, &blocks, params);
        let next = network.header_entries();
        use super::super::validation_history::HistoryPublicationFault;
        store.set_validation_history_fault(if after {
            HistoryPublicationFault::SnapshotAfterCommitFailure
        } else {
            HistoryPublicationFault::SnapshotBeforeCommitFailure
        });
        // Act
        assert!(network.persist_validation_header_snapshot().is_err());
        // Assert
        let expected = if after { next } else { old };
        assert_eq!(
            store
                .load_header_entries()
                .expect("full headers")
                .expect("present")
                .entries,
            expected
        );
        assert_eq!(
            store
                .load_block_index_entries()
                .expect("full index")
                .expect("present")
                .entries,
            expected
        );
        if after {
            assert!(
                store.recovered_validation_history().is_err(),
                "uncertain live owner remains poisoned"
            );
        }
        drop(network);
        drop(store);
        let reopened = FjallNodeStore::open(root).expect("reopen full committed family");
        assert_eq!(
            reopened
                .load_header_entries()
                .expect("headers")
                .expect("present")
                .entries,
            expected
        );
        assert_eq!(
            reopened
                .load_block_index_entries()
                .expect("index")
                .expect("present")
                .entries,
            expected
        );
        drop(reopened);
    });
}

#[test]
fn phase167_durable_rows_reopen_trusted_before_commit_preserves_old_pair() {
    trusted_header_fault(false);
}

#[test]
fn phase167_durable_rows_reopen_trusted_after_commit_poison_reopens_full_pair() {
    trusted_header_fault(true);
}

fn metadata_fault(after: bool) {
    genuine::with_manager(|manager, store, _| {
        // Arrange: explicit data writer, never canonical reproof.
        let root = store.datadir().to_path_buf();
        let old = store.load_chain_meta_for_open().expect("old");
        let large = vec![old.0[0].clone(); 401];
        super::METADATA_FAULT.set(Some(after));
        // Act
        assert!(store.save_chain_meta(&large, PersistMode::Sync).is_err());
        // Assert
        let expected = if after { (large, None) } else { old };
        assert_eq!(
            store.load_chain_meta_for_open().expect("complete metadata"),
            expected
        );
        drop(manager);
        drop(store);
        let reopened = FjallNodeStore::open(root).expect("physical reopen");
        assert_eq!(
            reopened
                .load_chain_meta_for_open()
                .expect("complete metadata"),
            expected
        );
        drop(reopened);
    });
}

#[test]
fn phase167_durable_rows_reopen_metadata_before_commit_preserves_old_family() {
    metadata_fault(false);
}

#[test]
fn phase167_durable_rows_reopen_metadata_after_commit_has_complete_new_family() {
    metadata_fault(true);
}

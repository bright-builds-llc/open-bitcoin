// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp
// - packages/bitcoin-knots/src/txdb.cpp

use super::*;
use crate::storage::{canonical_bootstrap::records, validation_history as history};
use std::fs;

#[cfg(unix)]
mod basic;

const CHAINS: [CanonicalChain; 4] = [
    CanonicalChain::Mainnet,
    CanonicalChain::Testnet3,
    CanonicalChain::Signet,
    CanonicalChain::Regtest,
];

fn source() -> PathBuf {
    let mut nonce = [0_u8; 16];
    getrandom::fill(&mut nonce).expect("fixture nonce");
    std::fs::canonicalize(std::env::temp_dir())
        .expect("canonical temp parent")
        .join(format!(
            "open-bitcoin-phase166-genesis-{:x}",
            u128::from_le_bytes(nonce)
        ))
}

#[test]
fn phase166_atomic_publishes_complete_genesis() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        let expected = bootstrap::prepare_genesis_rows(chain).expect("prepared rows");
        // Act
        let opened = open(&source, chain);
        let actual =
            inspection::InspectedRecords::read_existing(&opened.store.db).expect("live records");
        // Assert
        assert_eq!(opened.genesis().chain(), chain);
        assert_eq!(opened.position().height, 0);
        assert_eq!(opened.position().chain_work, opened.genesis().work());
        assert!(opened.recovered.check(&opened.store).is_ok());
        for name in actual.namespaces() {
            let rows = actual.maybe_records(name).expect("namespace");
            if let Some(expected) = expected.get(name) {
                assert_eq!(rows, expected, "every persisted authority byte");
            } else {
                assert!(rows.is_empty(), "no implicit unrelated state");
            }
        }
        let coins = actual.maybe_records("coins").expect("coins");
        assert_eq!(coins.len(), 1, "B only, no C/H or genesis outputs");
        assert_eq!(
            coins.get(b"B".as_slice()).expect("B"),
            chain.spec().genesis_hash().as_bytes()
        );
        assert_eq!(
            actual
                .maybe_records("chainstate")
                .expect("chainstate")
                .len(),
            1,
            "no undo/snapshot"
        );
        assert!(
            !actual
                .maybe_records("block_index")
                .expect("index")
                .keys()
                .any(|key| key.starts_with(b"basic_filter:"))
        );
        drop(opened);
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[test]
fn phase166_reopen_releases_every_handle_before_same_datadir_proof() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        let fresh = open(&source, chain);
        let genesis = fresh.genesis().clone();
        let position = fresh.position().clone();
        let old_instance = Arc::downgrade(&fresh.store.filter_publication);
        drop(fresh);
        assert!(
            old_instance.upgrade().is_none(),
            "all original owners released"
        );
        // Act
        let reopened = reopen(&source, chain);
        // Assert
        assert_eq!(reopened.genesis(), &genesis);
        assert_eq!(reopened.position(), &position);
        assert!(reopened.recovered.check(&reopened.store).is_ok());
        drop(reopened);
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

fn open(source: &Path, chain: CanonicalChain) -> OpenedCanonicalGenesis {
    FjallNodeStore::open_canonical_genesis(source, chain).expect("canonical fresh or existing")
}

fn reopen(source: &Path, chain: CanonicalChain) -> OpenedCanonicalGenesis {
    FjallNodeStore::reopen_canonical_genesis_existing(source, chain).expect("canonical existing")
}

#[test]
fn phase166_atomic_before_commit_leaves_no_authority_and_refuses_partial_next_open() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        MAYBE_FAULT.set(Some(Fault::BeforeCommit));
        // Act
        let result = FjallNodeStore::open_canonical_genesis(&source, chain);
        // Assert
        assert!(matches!(result, Err(Refusal::StorageUnavailable)));
        let raw = FjallNodeStore::open_unrecovered(&source).expect("error released handles");
        let rows = inspection::InspectedRecords::read_existing(&raw.db).expect("records");
        assert!(
            rows.namespaces()
                .all(|name| rows.maybe_records(name).expect("namespace").is_empty())
        );
        drop(raw);
        assert!(matches!(
            FjallNodeStore::open_canonical_genesis(&source, chain),
            Err(Refusal::MissingRecord)
        ));
        assert!(matches!(
            FjallNodeStore::reopen_canonical_genesis_existing(&source, chain),
            Err(Refusal::MissingRecord)
        ));
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[test]
fn phase166_atomic_uncertain_commit_never_returns_owner_but_later_proves_complete() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        MAYBE_FAULT.set(Some(Fault::AfterCommit));
        // Act
        let result = FjallNodeStore::open_canonical_genesis(&source, chain);
        // Assert
        assert!(matches!(result, Err(Refusal::CommitUncertain)));
        let reopened = reopen(&source, chain);
        assert_eq!(reopened.genesis().hash(), chain.spec().genesis_hash());
        assert!(reopened.recovered.check(&reopened.store).is_ok());
        drop(reopened);
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[test]
fn phase166_atomic_one_batch_advances_one_database_sequence() {
    // Arrange
    let source = source();
    // Act
    let opened = open(&source, CanonicalChain::Regtest);
    // Assert
    let (before, after) = LAST_BATCH_SEQUENCES
        .take()
        .expect("actual authority commit");
    assert_eq!(
        after,
        before + 1,
        "one authority batch after keyspace creation"
    );
    assert_eq!(
        opened.store.db.seqno(),
        after,
        "postproof recovery adds no writes"
    );
    drop(opened);
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_observation_never_initializes_missing_or_empty_storage() {
    // Arrange
    let source = source();
    // Act / Assert
    assert!(matches!(
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest),
        Err(Refusal::MissingRecord)
    ));
    assert!(!source.exists());
    fs::create_dir(&source).expect("empty source");
    assert!(matches!(
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest),
        Err(Refusal::MissingRecord)
    ));
    assert_eq!(fs::read_dir(&source).expect("empty source").count(), 0);
    let opened = open(&source, CanonicalChain::Regtest);
    drop(opened);
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_sealed_owner_excludes_all_supported_openers_until_drop() {
    // Arrange
    let source = source();
    let opened = open(&source, CanonicalChain::Regtest);
    // Act / Assert
    assert!(FjallNodeStore::open(&source).is_err());
    assert!(FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest).is_err());
    assert!(
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest)
            .is_err()
    );
    drop(opened);
    let reopened = reopen(&source, CanonicalChain::Regtest);
    drop(reopened);
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_capability_rejects_foreign_live_instance() {
    // Arrange
    let first = source();
    let second = source();
    let opened = open(&first, CanonicalChain::Regtest);
    let other = open(&second, CanonicalChain::Regtest);
    // Act / Assert
    assert!(matches!(
        opened.recovered.check(&other.store),
        Err(Refusal::ChangedStore)
    ));
    assert!(opened.into_parts().is_ok());
    drop(other);
    fs::remove_dir_all(first).expect("fixture cleanup");
    fs::remove_dir_all(second).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_generation_refuses_even_identical_rewritten_authority() {
    // Arrange
    let source = source();
    let opened = open(&source, CanonicalChain::Regtest);
    let identity = records::encode_identity(CanonicalChain::Regtest);
    // Act
    opened
        .store
        .schema
        .insert(records::IDENTITY_KEY, identity)
        .expect("identical raw rewrite");
    // Assert
    assert!(
        prove_live(&opened.store, CanonicalChain::Regtest).is_ok(),
        "facts still identical"
    );
    assert!(
        matches!(opened.into_parts(), Err(Refusal::ChangedStore)),
        "generation still invalidates"
    );
    let reopened = reopen(&source, CanonicalChain::Regtest);
    drop(reopened);
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_postproof_coverage_owner_invalidates_raw_authority_writes() {
    // Arrange
    let source = source();
    let opened = open(&source, CanonicalChain::Regtest);
    let entries = opened
        .store
        .load_header_entries()
        .expect("headers")
        .expect("present")
        .entries;
    assert_eq!(
        opened
            .store
            .validation_provenance(opened.genesis().hash())
            .expect("history"),
        history::ValidationProvenance::ScriptsValid
    );
    // Act
    opened
        .store
        .save_header_entries(&entries, PersistMode::Sync)
        .expect("raw public snapshot sink");
    // Assert
    let coverage = opened
        .store
        .get_bytes(StorageNamespace::BlockIndex, history::COVERAGE_KEY)
        .expect("coverage")
        .expect("present");
    assert_eq!(
        history::decode_coverage(&coverage).expect("decode"),
        history::ValidationCoverage::UnknownLegacy
    );
    assert!(opened.into_parts().is_err());
    assert!(
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest)
            .is_err()
    );
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_original_is_immediately_reproved_before_authority() {
    for fault in [Fault::OriginalProof, Fault::DifferentProof] {
        // Arrange
        let source = source();
        drop(open(&source, CanonicalChain::Regtest));
        MAYBE_FAULT.set(Some(fault));
        // Act
        let result =
            FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest);
        // Assert
        if fault == Fault::DifferentProof {
            assert!(
                matches!(result, Err(Refusal::ChangedStore)),
                "independently valid changed proof must refuse"
            );
        } else {
            assert!(matches!(result, Err(Refusal::MalformedRecord)));
        }
        let raw = FjallNodeStore::open_unrecovered(&source).expect("error released handles");
        drop(raw);
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[cfg(unix)]
type SourceSnapshot = std::collections::BTreeMap<PathBuf, (u32, i64, i64, Vec<u8>)>;

#[cfg(unix)]
fn snapshot(root: &Path) -> SourceSnapshot {
    use std::os::unix::fs::MetadataExt;
    fn visit(root: &Path, current: &Path, out: &mut SourceSnapshot) {
        let metadata = fs::symlink_metadata(current).expect("fixture metadata");
        let bytes = if metadata.is_file() {
            fs::read(current).expect("fixture bytes")
        } else {
            vec![]
        };
        out.insert(
            current
                .strip_prefix(root)
                .expect("relative fixture")
                .to_path_buf(),
            (
                metadata.mode(),
                metadata.mtime(),
                metadata.mtime_nsec(),
                bytes,
            ),
        );
        if metadata.is_dir() {
            for entry in fs::read_dir(current).expect("fixture entries") {
                visit(root, &entry.expect("fixture entry").path(), out);
            }
        }
    }
    let mut out = std::collections::BTreeMap::new();
    visit(root, root, &mut out);
    out
}

#[cfg(unix)]
#[test]
fn phase166_reopen_wrong_network_preserves_every_original_byte_mode_and_mtime() {
    for stored in CHAINS {
        // Arrange
        let source = source();
        drop(open(&source, stored));
        for selected in CHAINS.into_iter().filter(|selected| *selected != stored) {
            let before = snapshot(&source);
            let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
            // Act
            let result = FjallNodeStore::open_canonical_genesis(&source, selected);
            // Assert
            assert!(matches!(result, Err(Refusal::InconsistentGenesis)));
            assert_eq!(
                builders,
                crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get(),
                "original backend never reached"
            );
            assert_eq!(before, snapshot(&source));
        }
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[cfg(unix)]
fn assert_corrupt_preserved(mutate: impl FnOnce(&FjallNodeStore)) {
    let source = source();
    drop(open(&source, CanonicalChain::Regtest));
    let raw = FjallNodeStore::open_unrecovered(&source).expect("fixture raw store");
    mutate(&raw);
    raw.db
        .persist(FjallPersistMode::SyncAll)
        .expect("fixture durable corruption");
    drop(raw);
    let before = snapshot(&source);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    let result =
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest);
    assert!(result.is_err(), "corrupt existing authority must refuse");
    let error = result.err().expect("fixed refusal");
    assert!(error.to_string().len() < 100);
    assert!(
        !error
            .to_string()
            .contains(source.to_str().expect("fixture path"))
    );
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get(),
        "scratch refusal precedes original open"
    );
    assert_eq!(before, snapshot(&source));
    let raw = FjallNodeStore::open_unrecovered(&source)
        .expect("refusal released application and engine locks");
    drop(raw);
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[cfg(unix)]
#[test]
fn phase166_reopen_missing_authority_rows_refuse_without_source_effects() {
    for (namespace, key) in [
        (StorageNamespace::Schema, "schema_version".to_owned()),
        (StorageNamespace::Schema, records::IDENTITY_KEY.to_owned()),
        (StorageNamespace::Headers, "snapshot".to_owned()),
        (StorageNamespace::BlockIndex, "snapshot".to_owned()),
        (
            StorageNamespace::BlockIndex,
            block_key(CanonicalChain::Regtest.spec().genesis_hash()),
        ),
        (
            StorageNamespace::BlockIndex,
            history::record_key(CanonicalChain::Regtest.spec().genesis_hash()),
        ),
        (
            StorageNamespace::BlockIndex,
            history::COVERAGE_KEY.to_owned(),
        ),
        (StorageNamespace::Chainstate, "chain_meta".to_owned()),
        (StorageNamespace::Coins, "B".to_owned()),
    ] {
        // Arrange / Act / Assert
        assert_corrupt_preserved(|raw| {
            raw.keyspace(namespace)
                .remove(key)
                .expect("remove required row");
        });
    }
}

#[cfg(unix)]
#[test]
fn phase166_reopen_legacy_and_high_work_corruptions_preserve_source() {
    for namespace in [
        StorageNamespace::Headers,
        StorageNamespace::BlockIndex,
        StorageNamespace::Chainstate,
    ] {
        for replacement in [
            serde_json::json!(42),
            serde_json::json!({"version":1,"kind":"exact_number","bytes_le":[0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,128]}),
        ] {
            // Arrange / Act / Assert
            assert_corrupt_preserved(|raw| {
                let (key, pointer) = if namespace == StorageNamespace::Chainstate {
                    ("chain_meta", "/payload/active_chain/0/chain_work")
                } else {
                    ("snapshot", "/payload/entries/0/chain_work")
                };
                let bytes = raw
                    .get_bytes(namespace, key)
                    .expect("read")
                    .expect("present");
                let mut json: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
                *json.pointer_mut(pointer).expect("work") = replacement;
                raw.keyspace(namespace)
                    .insert(key, serde_json::to_vec(&json).expect("json bytes"))
                    .expect("corrupt work");
            });
        }
    }
}

#[cfg(unix)]
#[test]
fn phase166_reopen_interrupted_foreign_or_extra_rows_preserve_source() {
    for (namespace, key, value) in [
        (StorageNamespace::Coins, "H", vec![0]),
        (StorageNamespace::Coins, "C", vec![0]),
        (StorageNamespace::Coins, "B", vec![7; 32]),
        (StorageNamespace::Schema, "schema_version", b"1".to_vec()),
        (StorageNamespace::Schema, records::IDENTITY_KEY, vec![0; 34]),
        (StorageNamespace::Chainstate, "undo:untrusted", vec![0]),
        (StorageNamespace::Runtime, "recovery_marker", vec![0]),
        (
            StorageNamespace::BlockIndex,
            "basic_filter:unknown",
            vec![0],
        ),
    ] {
        // Arrange / Act / Assert
        assert_corrupt_preserved(|raw| {
            raw.keyspace(namespace)
                .insert(key, value)
                .expect("insert bad row");
        });
    }
}

#[cfg(unix)]
#[test]
fn phase166_reopen_corrupted_body_preserves_source() {
    // Arrange / Act / Assert
    assert_corrupt_preserved(|raw| {
        let key = block_key(CanonicalChain::Regtest.spec().genesis_hash());
        let mut bytes = raw
            .get_bytes(StorageNamespace::BlockIndex, &key)
            .expect("body")
            .expect("present");
        *bytes.last_mut().expect("body byte") ^= 1;
        raw.block_index.insert(key, bytes).expect("mutated body");
    });
}

#[cfg(unix)]
#[test]
fn phase166_reopen_generic_store_keeps_its_noncanonical_provenance() {
    // Arrange
    let source = source();
    let raw = FjallNodeStore::open(&source).expect("existing generic contract");
    drop(raw);
    let before = snapshot(&source);
    // Act
    let result = FjallNodeStore::open_canonical_genesis(&source, CanonicalChain::Regtest);
    // Assert
    assert!(matches!(result, Err(Refusal::MissingRecord)));
    assert_eq!(before, snapshot(&source));
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[test]
fn phase166_reopen_ordinary_remains_refused_for_every_proved_genesis() {
    for chain in CHAINS {
        // Arrange
        let source = source();
        drop(open(&source, chain));
        // Act / Assert
        assert!(FjallNodeStore::open_ordinary(&source, chain).is_err());
        let reopened = reopen(&source, chain);
        assert_eq!(reopened.genesis().chain(), chain);
        drop(reopened);
        fs::remove_dir_all(source).expect("fixture cleanup");
    }
}

#[cfg(unix)]
#[test]
fn phase166_reopen_symlink_path_refuses_without_target_effects() {
    // Arrange
    let source = source();
    let alias = source.with_extension("symlink");
    drop(open(&source, CanonicalChain::Regtest));
    std::os::unix::fs::symlink(&source, &alias).expect("fixture symlink");
    let before = snapshot(&source);
    // Act
    let result = FjallNodeStore::open_canonical_genesis(&alias, CanonicalChain::Regtest);
    // Assert
    assert!(matches!(result, Err(Refusal::StorageUnavailable)));
    assert_eq!(before, snapshot(&source));
    fs::remove_file(alias).expect("alias cleanup");
    fs::remove_dir_all(source).expect("fixture cleanup");
}

#[cfg(unix)]
#[test]
fn phase166_reopen_existing_engine_lock_refuses_before_original_builder() {
    // Arrange
    let source = source();
    drop(open(&source, CanonicalChain::Regtest));
    let engine = Database::builder(&source)
        .open()
        .expect("direct fixture engine");
    let before = snapshot(&source);
    let builders = crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get();
    // Act
    let result =
        FjallNodeStore::reopen_canonical_genesis_existing(&source, CanonicalChain::Regtest);
    // Assert
    assert!(matches!(result, Err(Refusal::StorageUnavailable)));
    assert_eq!(
        builders,
        crate::storage::chain_compatibility::BACKEND_BUILDER_CALLS.get()
    );
    assert_eq!(before, snapshot(&source));
    drop(engine);
    drop(reopen(&source, CanonicalChain::Regtest));
    fs::remove_dir_all(source).expect("fixture cleanup");
}

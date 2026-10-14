// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::tests::{CHAINS, Fixture, ordinary_refusal};
use super::*;
use crate::storage::{FjallCoinsView, FjallNodeStore, PersistMode, StorageNamespace};
use crate::storage::{coins_codec, filter_index, validation_history as history};
use open_bitcoin_core::{
    chainstate::{
        BasicFilterInputs, BlockUndo, ChainPosition, ChainstateSnapshot, Coin, CoinsView,
        IndexInputProtection,
    },
    primitives::{Amount, Block, BlockHash, OutPoint, ScriptBuf, TransactionOutput, Txid},
};
use std::{
    collections::{BTreeMap, HashMap},
    path::PathBuf,
    time::SystemTime,
};

#[derive(Debug, PartialEq, Eq)]
struct PhysicalEntry {
    kind: &'static str,
    length: u64,
    bytes: Vec<u8>,
    maybe_link: Option<PathBuf>,
    modified: SystemTime,
    readonly: bool,
    #[cfg(unix)]
    mode: u32,
}

fn physical_manifest(root: &Path) -> BTreeMap<PathBuf, PhysicalEntry> {
    fn visit(root: &Path, path: &Path, entries: &mut BTreeMap<PathBuf, PhysicalEntry>) {
        let metadata = fs::symlink_metadata(path).expect("owned fixture metadata");
        let file_type = metadata.file_type();
        let (kind, bytes, maybe_link) = if file_type.is_file() {
            ("file", fs::read(path).expect("owned fixture bytes"), None)
        } else if file_type.is_dir() {
            ("directory", Vec::new(), None)
        } else if file_type.is_symlink() {
            (
                "symlink",
                Vec::new(),
                Some(fs::read_link(path).expect("owned fixture link")),
            )
        } else {
            ("special", Vec::new(), None)
        };
        let entry = PhysicalEntry {
            kind,
            length: metadata.len(),
            bytes,
            maybe_link,
            modified: metadata.modified().expect("fixture modification timestamp"),
            readonly: metadata.permissions().readonly(),
            #[cfg(unix)]
            mode: {
                use std::os::unix::fs::PermissionsExt;
                metadata.permissions().mode()
            },
        };
        entries.insert(
            path.strip_prefix(root)
                .expect("fixture prefix")
                .to_path_buf(),
            entry,
        );
        if file_type.is_dir() {
            for entry in fs::read_dir(path).expect("owned fixture directory") {
                visit(root, &entry.expect("owned fixture entry").path(), entries);
            }
        }
    }
    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

fn assert_physical_preservation(source: &Path) {
    // Every Fjall owner has been dropped by the caller before this recursive,
    // test-only snapshot. Access times are deliberately excluded from write claims.
    let before = physical_manifest(source);
    assert!(before.values().any(|entry| entry.kind == "file"));
    for chain in CHAINS {
        // Act
        let refusal = ordinary_refusal(source, chain);
        let after = physical_manifest(source);
        // Assert
        assert_eq!(refusal.observation(), DatadirObservation::ExistingUnproved);
        assert_eq!(refusal.reason(), CompatibilityReason::ExistingStateUnproved);
        assert_eq!(
            refusal.observed_identity(),
            ObservedChainIdentity::Unavailable
        );
        assert_eq!(refusal.work_profile(), WorkProfileObservation::Unavailable);
        assert_eq!(before, after, "physical source changed for {chain}");
    }
}

fn write_sentinels(store: &FjallNodeStore) {
    for namespace in [
        StorageNamespace::Wallet,
        StorageNamespace::Metrics,
        StorageNamespace::Runtime,
    ] {
        store
            .write_raw_for_test(
                namespace,
                "private-sentinel",
                b"private-wallet-secret".to_vec(),
            )
            .expect("namespace sentinel");
    }
    fs::write(
        store.datadir().join("unrelated-private-file"),
        b"unrelated bytes\x00\xff",
    )
    .expect("unrelated file");
    fs::create_dir(store.datadir().join("unrelated-directory")).expect("unrelated directory");
    #[cfg(unix)]
    std::os::unix::fs::symlink(
        "unrelated-private-file",
        store.datadir().join("unrelated-link"),
    )
    .expect("unrelated link");
}

fn position(block: &Block, height: u32) -> ChainPosition {
    // Synthetic chainwork exists only to arrange explicitly owned infrastructure fixtures.
    ChainPosition::new(block.header.clone(), height, u128::from(height) + 1, 1)
}

fn arrange_schema_one(source: &Path) -> (BlockHash, OutPoint) {
    let store = FjallNodeStore::open(source).expect("owned schema fixture");
    let block = CanonicalChain::Regtest.spec().genesis_block();
    store
        .save_block(&block, PersistMode::Sync)
        .expect("body required by legacy confirmation migration");
    let root = position(&block, 0);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([0x42; 32]),
        vout: 1,
    };
    let coin = Coin {
        output: TransactionOutput {
            value: Amount::from_sats(5_000).expect("fixture amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("fixture script"),
        },
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let snapshot = ChainstateSnapshot::new(
        vec![root],
        HashMap::from([(outpoint.clone(), coin)]),
        HashMap::new(),
    );
    store
        .save_chainstate_snapshot(&snapshot, PersistMode::Sync)
        .expect("legacy snapshot");
    store.write_schema_version_for_test(1).expect("schema one");
    write_sentinels(&store);
    assert_eq!(
        store.coins_view().best_block().expect("no migrated B"),
        None
    );
    (block_hash(&block), outpoint)
}

fn block_hash(block: &Block) -> BlockHash {
    open_bitcoin_core::consensus::block_hash(&block.header)
}

fn arrange_pending_prune(source: &Path) -> BlockHash {
    let store = FjallNodeStore::open(source).expect("owned prune fixture");
    let mut positions = Vec::new();
    let mut block = CanonicalChain::Regtest.spec().genesis_block();
    let mut maybe_prune_hash = None;
    for height in 0..=300 {
        if height != 0 {
            block.header.previous_block_hash = positions
                .last()
                .map(|position: &ChainPosition| position.block_hash)
                .expect("preceding fixture position");
            block.header.nonce = height;
        }
        let current = position(&block, height);
        if height == 1 {
            store
                .save_block(&block, PersistMode::Sync)
                .expect("prune mate body");
            store
                .save_undo(
                    current.block_hash,
                    &BlockUndo {
                        transactions: Vec::new(),
                    },
                    PersistMode::Sync,
                )
                .expect("prune mate undo");
            maybe_prune_hash = Some(current.block_hash);
        }
        positions.push(current);
    }
    store
        .save_chain_meta(&positions, PersistMode::Sync)
        .expect("fixture ancestry");
    let tip = positions.last().expect("fixture tip").block_hash;
    store
        .coins_view()
        .write_raw_bytes(
            &coins_codec::encode_best_block_key(),
            coins_codec::encode_best_block_value(tip),
        )
        .expect("fixture best block");
    let prune_hash = maybe_prune_hash.expect("height one mate");
    store
        .sync_prune_intent(crate::storage::fjall_store::PruneIntent {
            height: 1,
            block_hash: prune_hash,
        })
        .expect("resumable prune intent");
    write_sentinels(&store);
    prune_hash
}

#[test]
fn chain_compatibility_preserves_closed_schema_two_fixture() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("store");
    {
        let store = FjallNodeStore::open(&source).expect("owned raw fixture store");
        store
            .save_block(
                &CanonicalChain::Mainnet.spec().genesis_block(),
                PersistMode::Sync,
            )
            .expect("fixture root");
    }
    // Act / Assert
    assert_physical_preservation(&source);
}

#[test]
fn chain_compatibility_preserves_missing_and_empty_paths_physically() {
    // Arrange
    let fixture = Fixture::new();
    let missing = fixture.path().join("missing/child");
    let before = physical_manifest(fixture.path());
    for chain in CHAINS {
        // Act
        let absent_refusal = ordinary_refusal(&missing, chain);
        let empty_refusal = ordinary_refusal(fixture.path(), chain);
        // Assert
        assert_eq!(absent_refusal.observation(), DatadirObservation::Missing);
        assert_eq!(
            empty_refusal.observation(),
            DatadirObservation::EmptyDirectory
        );
        assert_eq!(physical_manifest(fixture.path()), before);
        assert_eq!(
            fs::symlink_metadata(&missing)
                .expect_err("path remains absent")
                .kind(),
            io::ErrorKind::NotFound
        );
    }
}

#[cfg(unix)]
#[test]
fn chain_compatibility_preserves_unsafe_paths_and_their_closed_store_targets() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("store");
    {
        let store = FjallNodeStore::open(&source).expect("owned target store");
        write_sentinels(&store);
    }
    let link = fixture.path().join("link");
    std::os::unix::fs::symlink("store", &link).expect("relative source link");
    let file = source.join("unrelated-private-file");
    let socket = fixture.path().join("sock");
    let _socket = std::os::unix::net::UnixListener::bind(&socket).expect("special path");
    let before = physical_manifest(fixture.path());
    for chain in CHAINS {
        for path in [
            &file,
            &file.join("child"),
            &link,
            &link.join("child"),
            &socket,
        ] {
            // Act
            let refusal = ordinary_refusal(path, chain);
            // Assert
            assert_eq!(refusal.observation(), DatadirObservation::UnsafePath);
            assert_eq!(physical_manifest(fixture.path()), before);
        }
    }
}

#[test]
fn chain_compatibility_preserves_closed_store_on_injected_inspection_io_failure() {
    // Arrange: injection is reliable even if mode-bit denial is bypassed by privilege.
    let fixture = Fixture::new();
    let source = fixture.path().join("store");
    {
        let store = FjallNodeStore::open(&source).expect("owned I/O fixture");
        write_sentinels(&store);
    }
    let before = physical_manifest(&source);
    for chain in CHAINS {
        for (kind, expected) in [
            (
                io::ErrorKind::PermissionDenied,
                InspectionIoKind::PermissionDenied,
            ),
            (io::ErrorKind::Other, InspectionIoKind::Other),
        ] {
            ORDINARY_BACKEND_OPEN_SPY.set(true);
            // Act: the same production inspector and refusal mapping, with only I/O injected.
            let observation = inspect_with(
                &source,
                |path| fs::symlink_metadata(path),
                |_| Err(io::Error::new(kind, "private source error")),
            );
            let refusal = CompatibilityRefusal::from_observation(chain, observation);
            ORDINARY_BACKEND_OPEN_SPY.set(false);
            // Assert
            assert_eq!(
                refusal.observation(),
                DatadirObservation::InspectionIoFailure(expected)
            );
            assert_eq!(refusal.reason(), CompatibilityReason::InspectionIoFailure);
            assert!(!refusal.to_string().contains("private source error"));
            assert_eq!(physical_manifest(&source), before);
        }
    }
}

#[test]
fn chain_compatibility_preserves_schema_one_before_any_coins_migration() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("schema-one");
    arrange_schema_one(&source);
    // Act / Assert
    assert_physical_preservation(&source);
}

#[test]
fn chain_compatibility_preserves_schema_one_control_proves_raw_migration() {
    // Arrange: a distinct fixture, never the preserved source from another test.
    let fixture = Fixture::new();
    let source = fixture.path().join("migration-control");
    let (tip, outpoint) = arrange_schema_one(&source);
    let before = physical_manifest(&source);
    // Act
    let migrated = FjallNodeStore::open(&source).expect("raw control migrates schema one");
    let view = FjallCoinsView::from_store(&migrated);
    // Assert
    assert_eq!(view.best_block().expect("migrated B"), Some(tip));
    assert!(
        view.contains_raw_key(&coins_codec::encode_coin_key(&outpoint))
            .expect("migrated C")
    );
    drop(view);
    drop(migrated);
    assert_ne!(physical_manifest(&source), before);
}

#[test]
fn chain_compatibility_preserves_foreign_canonical_and_synthetic_fixture_roots() {
    for chain in CHAINS {
        for synthetic in [false, true] {
            // Arrange
            let fixture = Fixture::new();
            let source = fixture.path().join("root");
            {
                let store = FjallNodeStore::open(&source).expect("owned root fixture");
                let mut block = chain.spec().genesis_block();
                if synthetic {
                    block.header.nonce ^= 1;
                }
                store
                    .save_block(&block, PersistMode::Sync)
                    .expect("fixture body");
                store
                    .save_chain_meta(&[position(&block, 0)], PersistMode::Sync)
                    .expect("fixture root metadata");
                write_sentinels(&store);
            }
            // Act / Assert: four selectors, including three foreign pairs per real root.
            assert_physical_preservation(&source);
        }
    }
}

#[test]
fn chain_compatibility_preserves_interrupted_coins_and_recovery_markers() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("interrupted");
    {
        let store = FjallNodeStore::open(&source).expect("owned interrupted fixture");
        let heads = coins_codec::encode_head_blocks_value(&[
            BlockHash::from_byte_array([1; 32]),
            BlockHash::from_byte_array([2; 32]),
        ])
        .expect("two heads");
        store
            .coins_view()
            .write_raw_bytes(&coins_codec::encode_head_blocks_key(), heads)
            .expect("interrupted H");
        store
            .write_raw_for_test(
                StorageNamespace::Runtime,
                "recovery_marker",
                vec![0xff, 0, 1],
            )
            .expect("malformed recovery marker");
        write_sentinels(&store);
    }
    // Act / Assert
    assert_physical_preservation(&source);
}

#[test]
fn chain_compatibility_preserves_valid_malformed_and_interrupted_validation_history() {
    for state in ["valid", "malformed", "missing-coverage"] {
        // Arrange
        let fixture = Fixture::new();
        let source = fixture.path().join(state);
        {
            let store = FjallNodeStore::open_without_ensure_schema_for_test(&source)
                .expect("raw owned history fixture");
            store.write_schema_version_for_test(2).expect("schema two");
            let hash = CanonicalChain::Mainnet.spec().genesis_hash();
            let identity = history::BlockValidationIdentity::new(hash, BlockHash::default(), 0)
                .expect("fixture record identity");
            let record = history::ValidationHistoryRecord::new(
                identity,
                history::StoredValidationStatus::KnownOnly,
            );
            let bytes = if state == "malformed" {
                vec![0xff, 0]
            } else {
                history::encode_record(record)
            };
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    &history::record_key(hash),
                    bytes,
                )
                .expect("history artifact");
            if state != "missing-coverage" {
                let bytes = if state == "malformed" {
                    vec![0xff]
                } else {
                    history::encode_coverage(history::ValidationCoverage::Complete).to_vec()
                };
                store
                    .write_raw_for_test(StorageNamespace::BlockIndex, history::COVERAGE_KEY, bytes)
                    .expect("coverage artifact");
            }
            write_sentinels(&store);
        }
        // Act / Assert
        assert_physical_preservation(&source);
    }
}

#[test]
fn chain_compatibility_preserves_pending_prune_before_any_resume() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("pending-prune");
    arrange_pending_prune(&source);
    // Act / Assert
    assert_physical_preservation(&source);
}

#[test]
fn chain_compatibility_preserves_prune_control_proves_resume_deletes_both_mates() {
    // Arrange: an independent owned control, never the preserved source.
    let fixture = Fixture::new();
    let source = fixture.path().join("prune-control");
    let hash = arrange_pending_prune(&source);
    let store = FjallNodeStore::open(&source).expect("raw control reopen");
    assert!(store.has_block(hash).expect("body present"));
    assert!(store.has_undo(hash).expect("undo present"));
    // Act
    crate::storage::fjall_store::resume_prune_intent(&store, &[])
        .expect("resume owned prune control");
    // Assert
    assert!(!store.has_block(hash).expect("body removed"));
    assert!(!store.has_undo(hash).expect("undo removed"));
    assert_eq!(store.maybe_prune_intent().expect("intent cleared"), None);
    assert!(store.load_have_pruned().expect("pruned recorded"));
}

#[test]
fn chain_compatibility_preserves_valid_and_malformed_basic_artifacts() {
    for malformed in [false, true] {
        // Arrange
        let fixture = Fixture::new();
        let source = fixture.path().join("basic");
        {
            let store = FjallNodeStore::open(&source).expect("owned BASIC fixture");
            let block = CanonicalChain::Mainnet.spec().genesis_block();
            let root = position(&block, 0);
            let inputs = BasicFilterInputs::from_historical(&block, &root, None)
                .expect("fixture historical inputs");
            let record = filter_index::StoredFilterRecord::generate(&inputs, &root, None)
                .expect("valid fixture record");
            let record_bytes = if malformed {
                vec![0xff]
            } else {
                filter_index::encode_record(&record)
            };
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    &filter_index::record_key(root.block_hash),
                    record_bytes,
                )
                .expect("BASIC record");
            let state = filter_index::StoredFilterState {
                maybe_endpoint: Some((0, root.block_hash)),
                fence_height: 0,
                fence_hash: root.block_hash,
                protection: IndexInputProtection::FromHeight(1),
            };
            let state_bytes = if malformed {
                vec![0xff, 0]
            } else {
                filter_index::encode_state(state)
            };
            store
                .write_raw_for_test(
                    StorageNamespace::BlockIndex,
                    filter_index::STATE_KEY,
                    state_bytes,
                )
                .expect("BASIC state");
            write_sentinels(&store);
        }
        // Act / Assert
        assert_physical_preservation(&source);
    }
}

#[test]
fn chain_compatibility_preserves_malformed_backend_bytes_without_opening_them() {
    // Arrange
    let fixture = Fixture::new();
    let source = fixture.path().join("malformed-backend");
    {
        let store = FjallNodeStore::open(&source).expect("owned initial backend");
        write_sentinels(&store);
    }
    fs::write(source.join("version"), b"invalid-version\0\xff").expect("malformed backend version");
    // Act / Assert
    assert_physical_preservation(&source);
}

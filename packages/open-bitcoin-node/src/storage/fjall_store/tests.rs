// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::{
    collections::HashMap,
    fs, io,
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, TxUndo},
    consensus::{block_hash, transaction_txid, transaction_wtxid},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness,
        Transaction, TransactionInput, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot, WalletUtxo},
};
use open_bitcoin_network::HeaderEntry;

use super::{
    FjallNodeStore, RECOVERY_MARKER_KEY, RuntimeMetadata, SNAPSHOT_KEY, StorageError,
    StorageNamespace, StorageRecoveryAction,
};
use crate::metrics::MetricsAvailability;
use crate::recovery::{
    LockEvidenceKind, RecoveryActionClass, RecoveryCause, RecoveryClassifierInput,
    RecoveryEvidenceSnapshot, classify_recovery,
};
use crate::status::{FieldAvailability, SyncRecoveryCategory};
use crate::storage::{FJALL_LOCK_FILE_NAME, probe_fjall_lock};
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{
    MetricKind, MetricRetentionPolicy, MetricSample, MetricsStorageSnapshot, PersistMode,
    SchemaVersion, SelectedWalletRecord, WalletRegistrySnapshot, WalletRescanFreshness,
    WalletRescanJob, WalletRescanJobState,
};

fn temp_store_path(test_name: &str) -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time after unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "open-bitcoin-fjall-{test_name}-{}-{timestamp}",
        std::process::id()
    ))
}

fn remove_dir_if_exists(path: &Path) {
    match fs::remove_dir_all(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn remove_file_if_exists(path: &Path) {
    match fs::remove_file(path) {
        Ok(()) => {}
        Err(error) if error.kind() == io::ErrorKind::NotFound => {}
        Err(error) => panic!("failed to remove {}: {error}", path.display()),
    }
}

fn lock_path(datadir: &Path) -> PathBuf {
    datadir.join(FJALL_LOCK_FILE_NAME)
}

fn base_recovery_classifier_input() -> RecoveryClassifierInput {
    RecoveryClassifierInput {
        maybe_storage_error: None,
        maybe_recovery_marker: None,
        lock_evidence: FieldAvailability::unavailable("lock evidence unavailable"),
        service_same_datadir: FieldAvailability::unavailable("service evidence unavailable"),
        live_rpc_available: FieldAvailability::unavailable("live RPC unavailable"),
        resource_bounds: FieldAvailability::unavailable("resource bounds unavailable"),
        unavailable_reason: "no recovery signal recorded".to_string(),
    }
}

fn available_recovery_evidence(
    evidence: FieldAvailability<RecoveryEvidenceSnapshot>,
) -> RecoveryEvidenceSnapshot {
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("recovery evidence should be available");
    };
    evidence
}

fn header(previous_block_hash: BlockHash, nonce: u32) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash,
        merkle_root: MerkleRoot::from_byte_array([nonce as u8; 32]),
        time: 1_700_000_000 + nonce,
        bits: 0x207f_ffff,
        nonce,
    }
}

fn header_entries() -> Vec<HeaderEntry> {
    let genesis_header = header(BlockHash::from_byte_array([0_u8; 32]), 1);
    let genesis_hash = block_hash(&genesis_header);
    let child_header = header(genesis_hash, 2);
    let child_hash = block_hash(&child_header);

    vec![
        HeaderEntry {
            block_hash: genesis_hash,
            header: genesis_header,
            height: 0,
            chain_work: 1,
        },
        HeaderEntry {
            block_hash: child_hash,
            header: child_header,
            height: 1,
            chain_work: 2,
        },
    ]
}

fn block(previous_block_hash: BlockHash, nonce: u32) -> Block {
    Block {
        header: header(previous_block_hash, nonce),
        transactions: Vec::new(),
    }
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(&[0x51]),
    }
}

fn mempool_transaction(seed: u8) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([seed; 32]),
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![output(10_000)],
        lock_time: 0,
    }
}

fn mempool_snapshot() -> MempoolSnapshot {
    let transaction = mempool_transaction(42);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");

    MempoolSnapshot {
        records: vec![MempoolSnapshotRecord::with_fail_closed_legacy_metadata(
            txid,
            wtxid,
            transaction,
            1_000,
            100,
        )],
    }
}

fn chainstate_snapshot() -> ChainstateSnapshot {
    let position = ChainPosition::new(header(BlockHash::from_byte_array([0_u8; 32]), 1), 0, 1, 1);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([9; 32]),
        vout: 0,
    };
    let coin = Coin {
        output: output(5_000),
        is_coinbase: false,
        created_height: 0,
        created_median_time_past: 1,
    };
    let mut utxos = HashMap::new();
    utxos.insert(outpoint, coin.clone());
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(
        position.block_hash,
        BlockUndo {
            transactions: vec![TxUndo {
                restored_inputs: vec![coin],
            }],
        },
    );

    ChainstateSnapshot::new(vec![position], utxos, undo_by_block)
}

fn wallet_snapshot() -> WalletSnapshot {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    let descriptor_id = wallet
        .import_descriptor(
            "receive",
            DescriptorRole::External,
            "wpkh(cMec2DGaTXkYJYfi7x3ZGjRXkeqmAvYAoWzMAcWj5fdLaqudWsNi)",
        )
        .expect("descriptor import");
    let mut snapshot = wallet.snapshot();
    snapshot.utxos.push(WalletUtxo {
        descriptor_id,
        outpoint: OutPoint {
            txid: Txid::from_byte_array([4; 32]),
            vout: 1,
        },
        output: output(10_000),
        created_height: 2,
        created_median_time_past: 3,
        is_coinbase: false,
    });
    snapshot
}

#[test]
fn fjall_store_reopens_saved_snapshots_and_metadata() {
    // Arrange
    let path = temp_store_path("reopen-snapshots");
    remove_dir_if_exists(&path);
    let chainstate = chainstate_snapshot();
    let wallet = wallet_snapshot();
    let headers = header_entries();
    let block = block(headers[0].block_hash, 3);
    let block_hash = block_hash(&block.header);
    let metrics = MetricsStorageSnapshot {
        samples: vec![MetricSample::new(MetricKind::SyncHeight, 1.0, 2)],
    };
    let metadata = RuntimeMetadata {
        last_clean_shutdown: true,
        ..RuntimeMetadata::default()
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_chainstate_snapshot(&chainstate, PersistMode::Sync)
            .expect("save chainstate");
        store
            .save_wallet_snapshot(&wallet, PersistMode::Sync)
            .expect("save wallet");
        store
            .save_header_entries(&headers, PersistMode::Sync)
            .expect("save headers");
        store
            .save_block(&block, PersistMode::Sync)
            .expect("save block");
        store
            .save_metrics_snapshot(&metrics, PersistMode::Sync)
            .expect("save metrics");
        store
            .save_runtime_metadata(&metadata, PersistMode::Sync)
            .expect("save runtime metadata");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_chainstate_snapshot()
            .expect("load chainstate"),
        Some(chainstate)
    );
    assert_eq!(
        reopened.load_wallet_snapshot().expect("load wallet"),
        Some(wallet)
    );
    assert_eq!(
        reopened
            .load_header_entries()
            .expect("load headers")
            .expect("headers")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_block_index_entries()
            .expect("load block index")
            .expect("block index")
            .entries,
        headers
    );
    assert_eq!(
        reopened
            .load_header_store()
            .expect("load header store")
            .expect("header store")
            .best_height(),
        1
    );
    assert_eq!(
        reopened.load_block(block_hash).expect("load block"),
        Some(block)
    );
    assert_eq!(
        reopened
            .load_block(BlockHash::from_byte_array([99_u8; 32]))
            .expect("load missing block"),
        None
    );
    assert_eq!(
        reopened.load_metrics_snapshot().expect("load metrics"),
        Some(metrics)
    );
    assert_eq!(
        reopened.load_runtime_metadata().expect("load metadata"),
        Some(metadata)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_round_trips_after_reopen() {
    // Arrange
    let path = temp_store_path("mempool-reopen");
    remove_dir_if_exists(&path);
    let snapshot = mempool_snapshot();

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_mempool_snapshot(&snapshot, PersistMode::Sync)
            .expect("save mempool snapshot");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_mempool_snapshot()
            .expect("load mempool snapshot"),
        Some(snapshot)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_remove_clears_persisted_state() {
    // Arrange
    let path = temp_store_path("mempool-clear");
    remove_dir_if_exists(&path);
    let snapshot = mempool_snapshot();
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .save_mempool_snapshot(&snapshot, PersistMode::Sync)
        .expect("save mempool snapshot");

    // Act
    store
        .clear_mempool_snapshot(PersistMode::Sync)
        .expect("clear mempool snapshot");

    // Assert
    assert_eq!(
        store
            .load_mempool_snapshot()
            .expect("load cleared mempool snapshot"),
        None
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_mempool_snapshot_reports_corruption() {
    // Arrange
    let path = temp_store_path("mempool-corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Mempool,
            SNAPSHOT_KEY,
            b"{not-json".to_vec(),
        )
        .expect("write corrupt mempool snapshot");

    // Act
    let error = store
        .load_mempool_snapshot()
        .expect_err("corrupt mempool snapshot should fail");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Mempool,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn named_wallet_registry_and_selection_survive_reopen() {
    // Arrange
    let path = temp_store_path("named-wallet-registry");
    remove_dir_if_exists(&path);
    let alpha = wallet_snapshot();
    let mut beta = wallet_snapshot();
    beta.next_descriptor_id = 99;
    let registry = WalletRegistrySnapshot::new(["alpha".to_string(), "beta".to_string()]);
    let selected = SelectedWalletRecord {
        wallet_name: "beta".to_string(),
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_named_wallet_snapshot("alpha", &alpha, PersistMode::Sync)
            .expect("save alpha");
        store
            .save_named_wallet_snapshot("beta", &beta, PersistMode::Sync)
            .expect("save beta");
        store
            .save_wallet_registry(&registry, PersistMode::Sync)
            .expect("save registry");
        store
            .save_selected_wallet(&selected, PersistMode::Sync)
            .expect("save selected wallet");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_wallet_registry()
            .expect("load registry")
            .expect("registry"),
        registry
    );
    assert_eq!(
        reopened
            .load_selected_wallet()
            .expect("load selected")
            .expect("selected"),
        selected
    );
    assert_eq!(
        reopened
            .load_named_wallet_snapshot("alpha")
            .expect("load alpha"),
        Some(alpha)
    );
    assert_eq!(
        reopened
            .load_named_wallet_snapshot("beta")
            .expect("load beta"),
        Some(beta)
    );

    remove_dir_if_exists(&path);
}

#[test]
fn wallet_rescan_jobs_survive_reopen_with_checkpoint_state() {
    // Arrange
    let path = temp_store_path("wallet-rescan-job");
    remove_dir_if_exists(&path);
    let job = WalletRescanJob {
        wallet_name: "alpha".to_string(),
        target_tip_hash: BlockHash::from_byte_array([8_u8; 32]),
        target_tip_height: 144,
        next_height: 121,
        maybe_scanned_through_height: Some(120),
        maybe_tip_median_time_past: Some(1_700_000_120),
        freshness: WalletRescanFreshness::Partial,
        state: WalletRescanJobState::Scanning,
        maybe_error: None,
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .save_wallet_rescan_job(&job, PersistMode::Sync)
            .expect("save rescan job");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_wallet_rescan_job("alpha")
            .expect("load job")
            .expect("job"),
        job
    );
    assert_eq!(
        reopened.load_wallet_rescan_jobs().expect("load all jobs"),
        vec![job]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn metrics_history_appends_across_reopen() {
    // Arrange
    let path = temp_store_path("metrics-history-reopen");
    remove_dir_if_exists(&path);
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 4,
        max_age_seconds: 1_000,
    };

    // Act
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .append_metric_samples(
                &[MetricSample::new(MetricKind::SyncHeight, 10.0, 10)],
                retention,
                10,
                PersistMode::Sync,
            )
            .expect("append first metrics");
        store
            .append_metric_samples(
                &[MetricSample::new(MetricKind::SyncHeight, 20.0, 40)],
                retention,
                40,
                PersistMode::Sync,
            )
            .expect("append second metrics");
    }
    let reopened = FjallNodeStore::open(&path).expect("reopen store");

    // Assert
    assert_eq!(
        reopened
            .load_metrics_snapshot()
            .expect("load metrics")
            .expect("metrics snapshot")
            .samples,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 10.0, 10),
            MetricSample::new(MetricKind::SyncHeight, 20.0, 40),
        ]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn metrics_history_prunes_per_series_cap() {
    // Arrange
    let path = temp_store_path("metrics-history-series-cap");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 2,
        max_age_seconds: 1_000,
    };
    store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::SyncHeight, 5.0, 5)],
            retention,
            5,
            PersistMode::Sync,
        )
        .expect("append sync metrics");

    // Act
    let snapshot = store
        .append_metric_samples(
            &[
                MetricSample::new(MetricKind::HeaderHeight, 10.0, 10),
                MetricSample::new(MetricKind::HeaderHeight, 20.0, 20),
                MetricSample::new(MetricKind::HeaderHeight, 30.0, 30),
            ],
            retention,
            30,
            PersistMode::Sync,
        )
        .expect("append header metrics");

    // Assert
    assert_eq!(
        snapshot.samples,
        vec![
            MetricSample::new(MetricKind::SyncHeight, 5.0, 5),
            MetricSample::new(MetricKind::HeaderHeight, 20.0, 20),
            MetricSample::new(MetricKind::HeaderHeight, 30.0, 30),
        ]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn metrics_history_prunes_expired_samples() {
    // Arrange
    let path = temp_store_path("metrics-history-expired");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 30,
        max_samples_per_series: 4,
        max_age_seconds: 10,
    };
    store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::PeerCount, 1.0, 10)],
            retention,
            10,
            PersistMode::Sync,
        )
        .expect("append old metrics");

    // Act
    let snapshot = store
        .append_metric_samples(
            &[MetricSample::new(MetricKind::PeerCount, 2.0, 25)],
            retention,
            25,
            PersistMode::Sync,
        )
        .expect("append fresh metrics");

    // Assert
    assert_eq!(
        snapshot.samples,
        vec![MetricSample::new(MetricKind::PeerCount, 2.0, 25)]
    );

    remove_dir_if_exists(&path);
}

#[test]
fn missing_metrics_snapshot_reports_unavailable_status() {
    // Arrange
    let path = temp_store_path("metrics-status");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let retention = MetricRetentionPolicy {
        sample_interval_seconds: 15,
        max_samples_per_series: 2,
        max_age_seconds: 60,
    };

    // Act
    let missing_status = store
        .load_metrics_status(retention)
        .expect("load missing metrics status");
    store
        .save_metrics_snapshot(
            &MetricsStorageSnapshot {
                samples: Vec::new(),
            },
            PersistMode::Sync,
        )
        .expect("save empty metrics snapshot");
    let available_status = store
        .load_metrics_status(retention)
        .expect("load available metrics status");

    // Assert
    assert_eq!(missing_status.retention, retention);
    assert_eq!(missing_status.enabled_series, MetricKind::ALL.to_vec());
    assert_eq!(
        missing_status.availability,
        MetricsAvailability::Unavailable {
            reason: "metrics history unavailable: no metrics snapshot recorded".to_string()
        }
    );
    assert_eq!(available_status.retention, retention);
    assert_eq!(available_status.enabled_series, MetricKind::ALL.to_vec());
    assert_eq!(
        available_status.availability,
        MetricsAvailability::Available
    );

    remove_dir_if_exists(&path);
}

#[test]
fn lock_probe_missing_datadir_reports_unavailable_reason() {
    // Arrange
    let path = temp_store_path("lock-probe-missing");
    remove_dir_if_exists(&path);

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    assert_eq!(
        evidence,
        FieldAvailability::unavailable("lock probe unavailable: datadir does not exist")
    );
}

#[test]
fn lock_probe_datadir_without_lock_reports_no_artifact_without_creating_file() {
    // Arrange
    let path = temp_store_path("lock-probe-no-artifact");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("create temp datadir");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::NoLockArtifact);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(evidence.detail, "no Fjall lock artifact found");
    assert!(!lock_path(&path).exists());

    remove_dir_if_exists(&path);
}

#[test]
fn lock_probe_present_unheld_lock_reports_stale_evidence_and_keeps_file() {
    // Arrange
    let path = temp_store_path("lock-probe-stale");
    remove_dir_if_exists(&path);
    fs::create_dir_all(&path).expect("create temp datadir");
    fs::File::create(lock_path(&path)).expect("create lock artifact");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::StaleLockEvidence);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(
        evidence.detail,
        "Fjall lock artifact is present but not currently held"
    );
    assert!(lock_path(&path).exists());

    remove_dir_if_exists(&path);
}

#[test]
fn lock_probe_held_fjall_store_reports_active_contention_without_opening_store() {
    // Arrange
    let path = temp_store_path("lock-probe-active");
    remove_dir_if_exists(&path);
    let store_guard = FjallNodeStore::open(&path).expect("open store guard");

    // Act
    let evidence = probe_fjall_lock(&path);

    // Assert
    let FieldAvailability::Available(evidence) = evidence else {
        panic!("lock evidence should be available");
    };
    assert_eq!(evidence.kind, LockEvidenceKind::ActiveContention);
    assert_eq!(evidence.lock_path, lock_path(&path).display().to_string());
    assert_eq!(
        evidence.detail,
        "Fjall lock is currently held by another opener"
    );

    drop(store_guard);
    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_lock_contention_maps_typed_backend_failure() {
    // Arrange
    let path = temp_store_path("recovery-lock-contention");
    remove_dir_if_exists(&path);
    let store_guard = FjallNodeStore::open(&path).expect("open store guard");

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("second open should hit lock contention"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            action: StorageRecoveryAction::Restart,
            ..
        }
    ));
    assert_eq!(
        error.recovery_category(),
        SyncRecoveryCategory::StorageLockContention
    );
    if let StorageError::BackendFailure { message, .. } = &error {
        assert_eq!(message, "database locked by another process");
    }

    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(
        evidence.category,
        SyncRecoveryCategory::StorageLockContention
    );
    assert_eq!(evidence.cause, RecoveryCause::ActiveLock);
    assert_eq!(evidence.action_class, RecoveryActionClass::SafeRetry);

    drop(store_guard);
    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_path_as_file_maps_backend_open_failure() {
    // Arrange
    let path = temp_store_path("recovery-path-as-file");
    remove_file_if_exists(&path);
    remove_dir_if_exists(&path);
    fs::File::create(&path).expect("create path-as-file fixture");

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("file path should fail store open"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::BackendFailure {
            namespace: StorageNamespace::Runtime,
            ..
        }
    ));
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(
        evidence.category,
        SyncRecoveryCategory::StorageBackendFailure
    );
    assert_eq!(evidence.cause, RecoveryCause::BackendOpenFailure);
    assert_eq!(evidence.action_class, RecoveryActionClass::StopAndEscalate);

    remove_file_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_schema_mismatch_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-schema-mismatch");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(SchemaVersion::CURRENT.get() + 1)
            .expect("write schema version");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("expected schema mismatch"),
        Err(error) => error,
    };

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.category, SyncRecoveryCategory::IncompatibleSchema);
    assert_eq!(evidence.cause, RecoveryCause::SchemaMismatch);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_corruption_marker_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-corruption-marker");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Runtime,
            RECOVERY_MARKER_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed marker");

    // Act
    let error = store
        .load_recovery_marker()
        .expect_err("malformed recovery marker");

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_storage_error = Some(error);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.category, SyncRecoveryCategory::StoreCorruption);
    assert_eq!(evidence.cause, RecoveryCause::CorruptionMarker);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );

    remove_dir_if_exists(&path);
}

#[test]
fn fjall_recovery_evidence_partial_write_maps_classifier_cause() {
    // Arrange
    let path = temp_store_path("recovery-partial-write");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");

    // Act
    let marker = store
        .mark_interrupted_write(
            StorageNamespace::BlockIndex,
            StorageRecoveryAction::Reindex,
            "block index write interrupted",
            PersistMode::Sync,
        )
        .expect("write recovery marker");

    // Assert
    let mut input = base_recovery_classifier_input();
    input.maybe_recovery_marker = Some(marker);
    let evidence = available_recovery_evidence(classify_recovery(input));
    assert_eq!(evidence.cause, RecoveryCause::PartialWrite);
    assert_eq!(
        evidence.action_class,
        RecoveryActionClass::BackupThenRebuild
    );
    assert_eq!(
        evidence.maybe_affected_namespace,
        Some("block_index".to_string())
    );

    remove_dir_if_exists(&path);
}

#[test]
fn incompatible_schema_version_returns_schema_mismatch() {
    // Arrange
    let path = temp_store_path("schema-mismatch");
    remove_dir_if_exists(&path);
    {
        let store = FjallNodeStore::open(&path).expect("open store");
        store
            .write_schema_version_for_test(SchemaVersion::CURRENT.get() + 1)
            .expect("write schema version");
    }

    // Act
    let error = match FjallNodeStore::open(&path) {
        Ok(_) => panic!("expected schema mismatch"),
        Err(error) => error,
    };

    // Assert
    assert!(matches!(
        error,
        StorageError::SchemaMismatch {
            expected: SchemaVersion::CURRENT,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_snapshot_maps_to_corruption() {
    // Arrange
    let path = temp_store_path("corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Chainstate,
            SNAPSHOT_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed record");

    // Act
    let error = store
        .load_chainstate_snapshot()
        .expect_err("malformed chainstate");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            action: StorageRecoveryAction::Repair,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

#[test]
fn recovery_marker_round_trips_and_clean_shutdown_clears_it() {
    // Arrange
    let path = temp_store_path("recovery-marker");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");

    // Act
    let marker = store
        .mark_interrupted_write(
            StorageNamespace::BlockIndex,
            StorageRecoveryAction::Reindex,
            "block index write interrupted",
            PersistMode::Sync,
        )
        .expect("write recovery marker");
    let loaded = store
        .load_recovery_marker()
        .expect("load recovery marker")
        .expect("recovery marker");
    store
        .mark_clean_shutdown(PersistMode::Sync)
        .expect("mark clean shutdown");

    // Assert
    assert_eq!(loaded, marker);
    assert_eq!(store.load_recovery_marker().expect("reload marker"), None);
    assert!(
        store
            .load_runtime_metadata()
            .expect("load runtime metadata")
            .expect("runtime metadata")
            .last_clean_shutdown
    );

    remove_dir_if_exists(&path);
}

#[test]
fn malformed_recovery_marker_maps_to_runtime_corruption() {
    // Arrange
    let path = temp_store_path("recovery-marker-corruption");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    store
        .write_raw_for_test(
            StorageNamespace::Runtime,
            RECOVERY_MARKER_KEY,
            b"{bad-json".to_vec(),
        )
        .expect("write malformed marker");

    // Act
    let error = store
        .load_recovery_marker()
        .expect_err("malformed recovery marker");

    // Assert
    assert!(matches!(
        error,
        StorageError::RecoveryMarkerCorruption {
            namespace: StorageNamespace::Runtime,
            ..
        }
    ));

    remove_dir_if_exists(&path);
}

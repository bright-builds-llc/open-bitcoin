// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use super::*;
use open_bitcoin_core::consensus::crypto::hash160;
use open_bitcoin_core::consensus::{
    ConsensusParams, ScriptVerifyFlags, block_merkle_root, check_block_header,
};
use open_bitcoin_mempool::{PolicyConfig, PolicyTime};
use open_bitcoin_network::LocalPeerConfig;

use crate::MemoryChainstateStore;
use crate::network::{
    CheckpointTrigger, EffectCompletion, ManagedNetworkHandle, ManagedPeerNetwork,
};
use crate::storage::fjall_store::{MempoolSnapshotDecodeLimits, SnapshotWriteExecutionError};
use crate::storage::mempool_snapshot::{CapturedMempoolGeneration, MempoolSnapshotFormatVersion};

fn empty_network_handle() -> ManagedNetworkHandle {
    ManagedNetworkHandle::from_network_fixture(ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    ))
}

fn snapshot_decode_limits() -> MempoolSnapshotDecodeLimits {
    MempoolSnapshotDecodeLimits::new(2 * 1024 * 1024, 128, 128, 1024 * 1024, 2 * 1024 * 1024)
}

mod load_limits;

fn checkpoint_p2sh_script() -> ScriptBuf {
    let redeem_script = script(&[0x51]);
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&hash160(redeem_script.as_bytes()));
    bytes.push(0x87);
    script(&bytes)
}

fn checkpoint_test_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    let script_sig = if height == 0 {
        vec![0, 0x51]
    } else {
        vec![1, height as u8, 0x51]
    };
    let coinbase = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid coinbase value"),
            script_pubkey: checkpoint_p2sh_script(),
        }],
        lock_time: 0,
    };
    let transactions = vec![coinbase];
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy target should have a valid nonce");
    block
}

fn checkpoint_network_handle() -> (ManagedNetworkHandle, Txid) {
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        LocalPeerConfig::default(),
        PolicyConfig::default(),
    );
    let consensus = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    let flags = ScriptVerifyFlags::P2SH
        | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
        | ScriptVerifyFlags::CHECKSEQUENCEVERIFY;
    let genesis = checkpoint_test_block(BlockHash::from_byte_array([0; 32]), 0, 500_000_000);
    let spendable = checkpoint_test_block(block_hash(&genesis.header), 1, 500_000_000);
    network
        .connect_local_block(&genesis, flags, consensus)
        .expect("connect genesis");
    network
        .connect_local_block(&spendable, flags, consensus)
        .expect("connect spendable block");
    let coinbase_txid = transaction_txid(&spendable.transactions[0]).expect("coinbase txid");
    (
        ManagedNetworkHandle::from_network_fixture(network),
        coinbase_txid,
    )
}

fn checkpoint_spend(previous_txid: Txid, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: previous_txid,
                vout: 0,
            },
            script_sig: script(&[0x01, 0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid spend value"),
            script_pubkey: checkpoint_p2sh_script(),
        }],
        lock_time: 0,
    }
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
fn prepared_mempool_snapshot_executor_persists_and_completes_exactly_once() {
    // Arrange
    let path = temp_store_path("prepared-mempool-success");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_040), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");

    // Act
    let completion = store
        .execute_prepared_mempool_snapshot_write(&handle, prepared, PersistMode::Sync)
        .expect("snapshot executor should persist");

    // Assert
    assert_eq!(completion, EffectCompletion::Applied);
    let persisted = store
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect("load persisted snapshot")
        .expect("prepared snapshot should persist");
    assert_eq!(
        persisted.format_version(),
        Some(MempoolSnapshotFormatVersion::CURRENT)
    );
    assert_eq!(
        persisted.captured_generation(),
        Some(CapturedMempoolGeneration::new(0))
    );
    assert_eq!(persisted.captured_at(), Some(PolicyTime::new(135_040)));
    assert!(
        handle
            .prepare_mempool_snapshot_write(PolicyTime::new(135_041), CheckpointTrigger::Periodic,)
            .is_ok(),
        "successful completion should release the pending slot"
    );

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_aborts_save_failure_and_allows_retry() {
    // Arrange
    let path = temp_store_path("prepared-mempool-write-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let persisted_before_failure = mempool_snapshot();
    store
        .save_mempool_snapshot(&persisted_before_failure, PersistMode::Sync)
        .expect("save pre-existing mempool snapshot");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_040), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let expected = StorageError::BackendFailure {
        namespace: StorageNamespace::Mempool,
        message: "injected mempool snapshot write failure".to_string(),
        action: StorageRecoveryAction::Restart,
    };

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        PersistMode::Sync,
        crate::storage::snapshot_codec::encode_mempool_snapshot,
        |_, _| Err(expected.clone()),
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Storage(error)) if error == expected
    ));
    assert_eq!(
        store
            .load_mempool_snapshot_with_limits(snapshot_decode_limits())
            .expect("load after write failure"),
        Some(persisted_before_failure)
    );
    let retry = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_041), CheckpointTrigger::Periodic)
        .expect("save failure abort should restore pending capacity");
    let retry_completion = store
        .execute_prepared_mempool_snapshot_write(&handle, retry, PersistMode::Sync)
        .expect("retry should persist and complete");
    assert_eq!(retry_completion, EffectCompletion::Applied);
    let persisted = store
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect("load after successful retry")
        .expect("retry snapshot should persist");
    assert_eq!(
        persisted.format_version(),
        Some(MempoolSnapshotFormatVersion::CURRENT)
    );
    assert_eq!(
        persisted.captured_generation(),
        Some(CapturedMempoolGeneration::new(0))
    );
    assert_eq!(persisted.captured_at(), Some(PolicyTime::new(135_041)));

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_aborts_encode_failure_and_allows_retry() {
    // Arrange
    let path = temp_store_path("prepared-mempool-encode-failure");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let persisted_before_failure = mempool_snapshot();
    store
        .save_mempool_snapshot(&persisted_before_failure, PersistMode::Sync)
        .expect("save pre-existing mempool snapshot");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_040), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let expected = StorageError::Corruption {
        namespace: StorageNamespace::Mempool,
        detail: "injected mempool snapshot encoding failure".to_string(),
        action: StorageRecoveryAction::Repair,
    };

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        PersistMode::Sync,
        |_| Err(expected.clone()),
        |_, _| panic!("save must not run after encoding fails"),
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Storage(error)) if error == expected
    ));
    assert_eq!(
        store
            .load_mempool_snapshot_with_limits(snapshot_decode_limits())
            .expect("load after encoding failure"),
        Some(persisted_before_failure)
    );
    let retry = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_041), CheckpointTrigger::Periodic)
        .expect("encoding failure abort should restore pending capacity");
    let retry_completion = store
        .execute_prepared_mempool_snapshot_write(&handle, retry, PersistMode::Sync)
        .expect("retry should persist and complete");
    assert_eq!(retry_completion, EffectCompletion::Applied);
    let persisted = store
        .load_mempool_snapshot_with_limits(snapshot_decode_limits())
        .expect("load after successful retry")
        .expect("retry snapshot should persist");
    assert_eq!(
        persisted.format_version(),
        Some(MempoolSnapshotFormatVersion::CURRENT)
    );
    assert_eq!(
        persisted.captured_generation(),
        Some(CapturedMempoolGeneration::new(0))
    );
    assert_eq!(persisted.captured_at(), Some(PolicyTime::new(135_041)));

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_preserves_newer_authority_after_stale_persistence() {
    // Arrange
    let path = temp_store_path("stale-prepared-mempool");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let (handle, coinbase_txid) = checkpoint_network_handle();
    let prepared_old = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_000), CheckpointTrigger::Periodic)
        .expect("old snapshot should prepare");
    store
        .save_mempool_snapshot(prepared_old.snapshot(), PersistMode::Sync)
        .expect("old snapshot should persist");
    let old_receipt = prepared_old.into_parts().1.acknowledge_write();
    let old_duplicate = old_receipt.duplicate_for_test();
    handle
        .submit_local_transaction_outcome_at(
            checkpoint_spend(coinbase_txid, 499_999_000),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            200_002,
            RelayIntent::Requested,
        )
        .expect("newer transaction should apply");

    // Act
    let stale_completion = handle
        .complete_snapshot_write(old_receipt)
        .expect("stale completion should dispatch");
    let prepared_current = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_003), CheckpointTrigger::Periodic)
        .expect("current snapshot should prepare");
    let current_completion = store
        .execute_prepared_mempool_snapshot_write(&handle, prepared_current, PersistMode::Sync)
        .expect("current snapshot should persist and complete");
    let state_before_duplicate = handle.mempool_info().expect("mempool info");
    let duplicate_completion = handle
        .complete_snapshot_write(old_duplicate)
        .expect("duplicate completion should dispatch");

    // Assert
    assert_eq!(stale_completion, EffectCompletion::AchievedButStale);
    assert_eq!(current_completion, EffectCompletion::Applied);
    assert_eq!(duplicate_completion, EffectCompletion::AlreadyApplied);
    assert_eq!(
        handle.mempool_info().expect("mempool info after duplicate"),
        state_before_duplicate
    );
    assert_eq!(state_before_duplicate.transaction_count, 1);
    assert_eq!(
        store
            .load_mempool_snapshot_with_limits(snapshot_decode_limits())
            .expect("load current persisted snapshot")
            .expect("current snapshot should exist")
            .records
            .len(),
        1
    );

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_encode_failure_preserves_newer_dirty_state() {
    // Arrange
    let path = temp_store_path("prepared-mempool-newer-dirty");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let (handle, coinbase_txid) = checkpoint_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_000), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    handle
        .submit_local_transaction_outcome_at(
            checkpoint_spend(coinbase_txid, 499_999_000),
            ScriptVerifyFlags::P2SH
                | ScriptVerifyFlags::CHECKLOCKTIMEVERIFY
                | ScriptVerifyFlags::CHECKSEQUENCEVERIFY,
            ConsensusParams {
                coinbase_maturity: 1,
                ..ConsensusParams::default()
            },
            200_002,
            RelayIntent::Requested,
        )
        .expect("newer transaction should make the snapshot dirty");
    let expected = StorageError::Corruption {
        namespace: StorageNamespace::Mempool,
        detail: "injected snapshot encoding failure".to_string(),
        action: StorageRecoveryAction::Repair,
    };

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        PersistMode::Sync,
        |_| Err(expected.clone()),
        |_, _| panic!("save must not run after encoding fails"),
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Storage(error)) if error == expected
    ));
    let retry = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_003), CheckpointTrigger::Periodic)
        .expect("encoding failure abort should restore pending capacity");
    assert_eq!(retry.snapshot().records.len(), 1);
    store
        .execute_prepared_mempool_snapshot_write(&handle, retry, PersistMode::Sync)
        .expect("retry should persist and complete");

    remove_dir_if_exists(&path);
}

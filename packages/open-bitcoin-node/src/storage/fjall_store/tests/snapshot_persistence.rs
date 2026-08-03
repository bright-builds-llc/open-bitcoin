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
    CheckpointPersistenceStrength, CheckpointTrigger, EffectCompletion, ManagedNetworkHandle,
    ManagedPeerNetwork, SnapshotWriteFailure,
};
use crate::storage::MempoolRecoveryStatus;
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

mod confirmation_migration;
mod gap_closure;
mod load_limits;
mod reopen;
mod write_execution_failures;

fn checkpoint_p2sh_script() -> ScriptBuf {
    let redeem_script = script(&[0x51]);
    let mut bytes = vec![0xa9, 20];
    bytes.extend_from_slice(&hash160(redeem_script.as_bytes()));
    bytes.push(0x87);
    script(&bytes)
}

fn checkpoint_test_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    checkpoint_test_block_with_transactions(previous_block_hash, height, value, Vec::new())
}

fn checkpoint_test_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    value: i64,
    mut additional_transactions: Vec<Transaction>,
) -> Block {
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
    let mut transactions = vec![coinbase];
    transactions.append(&mut additional_transactions);
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
fn prepared_mempool_snapshot_executor_persists_and_completes_exactly_once() {
    // Arrange
    let path = temp_store_path("prepared-mempool-success");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(135_040), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let mut samples = 0_u8;

    // Act
    let receipt = store
        .execute_prepared_mempool_snapshot_write(&handle, prepared, || {
            samples += 1;
            PolicyTime::new(135_050)
        })
        .expect("snapshot executor should persist");
    assert_eq!(receipt.completed_at(), Some(PolicyTime::new(135_050)));
    assert_eq!(
        receipt.persistence_strength(),
        Some(CheckpointPersistenceStrength::Sync)
    );
    let completion = handle
        .complete_snapshot_write(receipt)
        .expect("snapshot completion should dispatch");
    drop(store);
    let reopened = FjallNodeStore::open(&path).expect("reopen store after Sync write");

    // Assert
    assert_eq!(samples, 1);
    assert_eq!(completion, EffectCompletion::Applied);
    let persisted = reopened
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
        crate::storage::snapshot_codec::encode_mempool_snapshot,
        |_, mode| {
            assert_eq!(mode, PersistMode::Sync);
            Err(expected.clone())
        },
        || PolicyTime::new(135_050),
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
        .execute_prepared_mempool_snapshot_write(&handle, retry, || PolicyTime::new(135_051))
        .expect("retry should persist");
    let retry_completion = handle
        .complete_snapshot_write(retry_completion)
        .expect("retry completion should dispatch");
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
        |_| Err(expected.clone()),
        |_, _| panic!("save must not run after encoding fails"),
        || PolicyTime::new(135_050),
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Encode(error)) if error == expected
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
        .execute_prepared_mempool_snapshot_write(&handle, retry, || PolicyTime::new(135_051))
        .expect("retry should persist");
    let retry_completion = handle
        .complete_snapshot_write(retry_completion)
        .expect("retry completion should dispatch");
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
    let old_receipt = prepared_old.into_parts().1.acknowledge_write(
        PolicyTime::new(200_001),
        CheckpointPersistenceStrength::Sync,
    );
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
        .execute_prepared_mempool_snapshot_write(&handle, prepared_current, || {
            PolicyTime::new(200_004)
        })
        .expect("current snapshot should persist");
    let current_completion = handle
        .complete_snapshot_write(current_completion)
        .expect("current snapshot completion should dispatch");
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
        |_| Err(expected.clone()),
        |_, _| panic!("save must not run after encoding fails"),
        || PolicyTime::new(200_003),
    );

    // Assert
    assert!(matches!(
        result,
        Err(SnapshotWriteExecutionError::Encode(error)) if error == expected
    ));
    let retry = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(200_003), CheckpointTrigger::Periodic)
        .expect("encoding failure abort should restore pending capacity");
    assert_eq!(retry.snapshot().records.len(), 1);
    store
        .execute_prepared_mempool_snapshot_write(&handle, retry, || PolicyTime::new(200_004))
        .expect("retry should persist");

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_retains_receipt_across_completion_dispatch_failure() {
    // Arrange
    let path = temp_store_path("prepared-mempool-completion-retry");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(300_000), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let receipt = store
        .execute_prepared_mempool_snapshot_write(&handle, prepared, || PolicyTime::new(300_001))
        .expect("Sync write should return an achieved receipt");
    handle.fail_next_checkpoint_completion_dispatch_for_test();

    // Act
    let error = handle
        .complete_snapshot_write(receipt)
        .expect_err("injected completion dispatch should fail");
    let retained = error.into_receipt();
    let completion = handle
        .complete_snapshot_write(retained)
        .expect("retained receipt should complete on retry");

    // Assert
    assert_eq!(completion, EffectCompletion::Applied);
    let evidence = handle
        .checkpoint_evidence(PolicyTime::new(300_002), 60)
        .expect("checkpoint evidence");
    assert_eq!(evidence.maybe_last_durable_generation, Some(0));

    remove_dir_if_exists(&path);
}

#[test]
fn prepared_mempool_snapshot_executor_samples_time_once_on_storage_failure() {
    // Arrange
    let handle = empty_network_handle();
    let prepared = handle
        .prepare_mempool_snapshot_write(PolicyTime::new(400_000), CheckpointTrigger::Periodic)
        .expect("snapshot should prepare");
    let expected = StorageError::BackendFailure {
        namespace: StorageNamespace::Mempool,
        message: "injected failure".to_string(),
        action: StorageRecoveryAction::Restart,
    };
    let mut samples = 0_u8;

    // Act
    let result = FjallNodeStore::execute_prepared_mempool_snapshot_write_with(
        &handle,
        prepared,
        crate::storage::snapshot_codec::encode_mempool_snapshot,
        |_, _| Err(expected.clone()),
        || {
            samples += 1;
            PolicyTime::new(400_001)
        },
    );

    // Assert
    assert!(matches!(
        result,
        Err(ref error) if error.failure() == SnapshotWriteFailure::Storage
    ));
    assert_eq!(samples, 1);
}

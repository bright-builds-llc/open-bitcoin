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
use open_bitcoin_mempool::MempoolEntryMetadata;

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
        records: vec![MempoolSnapshotRecord {
            txid,
            wtxid,
            transaction,
            fee_sats: 1_000,
            virtual_size: 100,
            metadata: MempoolEntryMetadata::legacy_unknown(),
        }],
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

mod corruption_and_markers;
mod lock_probe;
mod metrics_persistence;
mod recovery_classification;
mod snapshot_persistence;
mod wallet_persistence;

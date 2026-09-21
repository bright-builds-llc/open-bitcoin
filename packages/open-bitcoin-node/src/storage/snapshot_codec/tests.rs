// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::collections::{BTreeSet, HashMap};

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, TxUndo},
    consensus::{transaction_txid, transaction_wtxid},
    primitives::{
        BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot, WalletUtxo},
};
use open_bitcoin_network::HeaderEntry;

use super::{
    MempoolSnapshotDecodeLimits, MetricsStorageSnapshot, decode_block_undo,
    decode_chainstate_snapshot, decode_header_entries, decode_mempool_snapshot,
    decode_mempool_snapshot_with_limits, decode_metrics_snapshot, decode_selected_wallet,
    decode_wallet_registry_snapshot, decode_wallet_rescan_job, decode_wallet_snapshot,
    encode_block_undo, encode_chainstate_snapshot, encode_header_entries, encode_mempool_snapshot,
    encode_metrics_snapshot, encode_selected_wallet, encode_wallet_registry_snapshot,
    encode_wallet_rescan_job, encode_wallet_snapshot,
};
use open_bitcoin_mempool::{
    MempoolAcceptanceTime, MempoolEntryMetadata, MempoolMemberIdentity, MempoolOrigin, PolicyTime,
    RelayIntent, transaction_weight_and_virtual_size,
};

use crate::storage::mempool_snapshot::CapturedMempoolGeneration;
use crate::storage::{MempoolSnapshot, MempoolSnapshotRecord};
use crate::{
    MetricKind, MetricSample, SchemaVersion, SelectedWalletRecord, StorageError, StorageNamespace,
    WalletRegistrySnapshot, WalletRescanFreshness, WalletRescanJob, WalletRescanJobState,
};

pub(super) fn header(seed: u8) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash: BlockHash::from_byte_array([seed.saturating_sub(1); 32]),
        merkle_root: MerkleRoot::from_byte_array([seed; 32]),
        time: u32::from(seed),
        bits: 0x207f_ffff,
        nonce: u32::from(seed),
    }
}

pub(super) fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

pub(super) fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: open_bitcoin_core::primitives::Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(&[0x51]),
    }
}

pub(super) fn mempool_transaction(seed: u8) -> Transaction {
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

pub(super) fn mempool_snapshot() -> MempoolSnapshot {
    let transaction = mempool_transaction(24);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let (_, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    let record = MempoolSnapshotRecord::try_from_compatibility(
        transaction,
        txid,
        wtxid,
        1_000,
        virtual_size,
        MempoolEntryMetadata::new(
            MempoolAcceptanceTime::Known(PolicyTime::from_unix_seconds(90)),
            MempoolOrigin::Local,
            RelayIntent::Requested,
        ),
    )
    .expect("valid mempool record");
    let member = record.member_identity().expect("canonical member identity");
    MempoolSnapshot::try_new_current(
        CapturedMempoolGeneration::new(42),
        PolicyTime::from_unix_seconds(120),
        vec![record],
        BTreeSet::from([member]),
    )
    .expect("valid current mempool snapshot")
}

pub(super) fn legacy_mempool_snapshot() -> MempoolSnapshot {
    let transaction = mempool_transaction(23);
    let txid = transaction_txid(&transaction).expect("txid");
    let wtxid = transaction_wtxid(&transaction).expect("wtxid");
    let (_, virtual_size) =
        transaction_weight_and_virtual_size(&transaction).expect("transaction size");
    let record = MempoolSnapshotRecord::try_from_compatibility(
        transaction,
        txid,
        wtxid,
        4_321,
        virtual_size,
        MempoolEntryMetadata::legacy_unknown(),
    )
    .expect("valid legacy mempool record");
    MempoolSnapshot::from_legacy_v1(vec![record])
}

pub(super) fn chainstate_snapshot() -> ChainstateSnapshot {
    let position = ChainPosition::new(header(1), 0, 1, 1);
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

pub(super) fn wallet_snapshot() -> WalletSnapshot {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    let descriptor_id = wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        )
        .expect("descriptor import");
    let _ = wallet
        .allocate_receive_address()
        .expect("first ranged address");
    let _ = wallet
        .allocate_receive_address()
        .expect("second ranged address");
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
fn chainstate_snapshot_round_trips_through_storage_dto() {
    // Arrange
    let snapshot = chainstate_snapshot();

    // Act
    let encoded = encode_chainstate_snapshot(&snapshot).expect("encode chainstate");
    let decoded = decode_chainstate_snapshot(&encoded).expect("decode chainstate");

    // Assert
    assert_eq!(decoded, snapshot);
}

mod chainstate_wallet_and_headers;
mod mempool_codec;

#[path = "tests/mempool_limits.rs"]
mod mempool_limits;

#[path = "tests/terminal_generation.rs"]
mod terminal_generation;

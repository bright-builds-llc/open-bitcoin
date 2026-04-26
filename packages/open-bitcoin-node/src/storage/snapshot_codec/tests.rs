// Parity breadcrumbs:
// - none: Open Bitcoin-only support/infrastructure; no direct Bitcoin Knots source anchor identified.

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin, TxUndo},
    primitives::{
        BlockHash, BlockHeader, MerkleRoot, OutPoint, ScriptBuf, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot, WalletUtxo},
};
use open_bitcoin_network::HeaderEntry;

use super::{
    MetricsStorageSnapshot, decode_chainstate_snapshot, decode_header_entries,
    decode_metrics_snapshot, decode_wallet_snapshot, encode_chainstate_snapshot,
    encode_header_entries, encode_metrics_snapshot, encode_wallet_snapshot,
};
use crate::{MetricKind, MetricSample, StorageError, StorageNamespace};

fn header(seed: u8) -> BlockHeader {
    BlockHeader {
        version: 1,
        previous_block_hash: BlockHash::from_byte_array([seed.saturating_sub(1); 32]),
        merkle_root: MerkleRoot::from_byte_array([seed; 32]),
        time: u32::from(seed),
        bits: 0x207f_ffff,
        nonce: u32::from(seed),
    }
}

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn output(value: i64) -> TransactionOutput {
    TransactionOutput {
        value: open_bitcoin_core::primitives::Amount::from_sats(value).expect("valid amount"),
        script_pubkey: script(&[0x51]),
    }
}

fn chainstate_snapshot() -> ChainstateSnapshot {
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
fn chainstate_snapshot_round_trips_through_storage_dto() {
    // Arrange
    let snapshot = chainstate_snapshot();

    // Act
    let encoded = encode_chainstate_snapshot(&snapshot).expect("encode chainstate");
    let decoded = decode_chainstate_snapshot(&encoded).expect("decode chainstate");

    // Assert
    assert_eq!(decoded, snapshot);
}

#[test]
fn wallet_snapshot_round_trips_through_original_descriptors() {
    // Arrange
    let snapshot = wallet_snapshot();

    // Act
    let encoded = encode_wallet_snapshot(&snapshot).expect("encode wallet");
    let decoded = decode_wallet_snapshot(&encoded).expect("decode wallet");

    // Assert
    assert_eq!(decoded, snapshot);
}

#[test]
fn header_entries_round_trip_and_validate_header_hashes() {
    // Arrange
    let header = header(2);
    let entry = HeaderEntry {
        block_hash: open_bitcoin_core::consensus::block_hash(&header),
        header,
        height: 1,
        chain_work: 2,
    };

    // Act
    let encoded = encode_header_entries(std::slice::from_ref(&entry)).expect("encode headers");
    let decoded = decode_header_entries(&encoded).expect("decode headers");

    // Assert
    assert_eq!(decoded.entries, vec![entry]);
}

#[test]
fn metrics_snapshot_round_trips_samples() {
    // Arrange
    let snapshot = MetricsStorageSnapshot {
        samples: vec![MetricSample::new(MetricKind::HeaderHeight, 1.0, 2)],
    };

    // Act
    let encoded = encode_metrics_snapshot(&snapshot).expect("encode metrics");
    let decoded = decode_metrics_snapshot(&encoded).expect("decode metrics");

    // Assert
    assert_eq!(decoded, snapshot);
}

#[test]
fn malformed_json_maps_to_corruption() {
    // Arrange
    let malformed = b"{not-json";

    // Act
    let error = decode_chainstate_snapshot(malformed).expect_err("malformed json");

    // Assert
    assert!(matches!(
        error,
        StorageError::Corruption {
            namespace: StorageNamespace::Chainstate,
            ..
        }
    ));
}

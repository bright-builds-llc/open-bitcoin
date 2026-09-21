// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateSnapshot, Coin},
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet},
};

use super::{PersistMode, remove_dir_if_exists, temp_store_path};
use crate::{
    FjallNodeStore, WalletRegistry, WalletRescanFreshness, WalletRescanJobState,
    sync::WalletRescanRuntime,
};

fn tip(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: if height == 0 {
                BlockHash::from_byte_array([0_u8; 32])
            } else {
                BlockHash::from_byte_array([height as u8 - 1; 32])
            },
            merkle_root: Default::default(),
            time: 1_700_000_000 + height,
            bits: 0x207f_ffff,
            nonce: height,
        },
        height,
        u128::from(height) + 1,
        i64::from(1_700_000_000 + height),
    )
}

fn wallet_with_ranged_descriptor() -> Wallet {
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)",
        )
        .expect("descriptor");
    wallet
}

fn funded_chainstate(wallet: &Wallet) -> ChainstateSnapshot {
    let receive_script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(25_000).expect("amount"),
                script_pubkey: receive_script.clone(),
            },
            is_coinbase: false,
            created_height: 1,
            created_median_time_past: 1_700_000_001,
        },
    );
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([2_u8; 32]),
            vout: 1,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(35_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 3,
            created_median_time_past: 1_700_000_003,
        },
    );

    ChainstateSnapshot::new(
        vec![tip(0), tip(1), tip(2), tip(3)],
        utxos,
        Default::default(),
    )
}

fn block_matching_position(position: &ChainPosition) -> Block {
    Block {
        header: position.header.clone(),
        transactions: Vec::new(),
    }
}

fn plant_coins_truth_with_payloads(
    store: &FjallNodeStore,
    coins_truth: &ChainstateSnapshot,
    payload_heights: &[u32],
) {
    store
        .seed_coins_from_snapshot(coins_truth)
        .expect("seed coins");
    for position in &coins_truth.active_chain {
        if !payload_heights.contains(&position.height) {
            continue;
        }
        store
            .save_block(&block_matching_position(position), PersistMode::Sync)
            .expect("save block payload");
    }
}

fn poison_leftover_snapshot() -> ChainstateSnapshot {
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([9_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(999_999).expect("amount"),
                script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("script"),
            },
            is_coinbase: false,
            created_height: 99,
            created_median_time_past: 1_700_000_099,
        },
    );
    ChainstateSnapshot::new(vec![tip(99)], utxos, Default::default())
}

#[test]
fn restart_resume_advances_pending_rescan_in_bounded_chunks() {
    // Arrange
    let path = temp_store_path("resume-chunks");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let wallet = wallet_with_ranged_descriptor();
    let coins_truth = funded_chainstate(&wallet);
    plant_coins_truth_with_payloads(&store, &coins_truth, &[0, 1, 2, 3]);
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("save wallet");

    {
        let runtime = WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 2)
            .expect("runtime");
        let first_job = runtime.enqueue_rescan("alpha").expect("enqueue");
        assert_eq!(first_job.state, WalletRescanJobState::Scanning);
        assert_eq!(first_job.freshness, WalletRescanFreshness::Partial);
        assert_eq!(first_job.maybe_scanned_through_height, Some(1));
    }

    // Act
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let reopened_runtime =
        WalletRescanRuntime::open_with_chunk_size(reopened_store, PersistMode::Sync, 2)
            .expect("reopened runtime");
    let resumed_job = reopened_runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load job")
        .expect("job");
    let resumed_registry = WalletRegistry::load(reopened_runtime.store()).expect("registry");
    let resumed_wallet = resumed_registry
        .wallet_snapshot("alpha")
        .expect("wallet snapshot");

    // Assert
    assert_eq!(resumed_job.state, WalletRescanJobState::Complete);
    assert_eq!(resumed_job.freshness, WalletRescanFreshness::Fresh);
    assert_eq!(resumed_job.maybe_scanned_through_height, Some(3));
    assert_eq!(resumed_wallet.maybe_tip_height, Some(3));
    assert_eq!(resumed_wallet.utxos.len(), 2);

    remove_dir_if_exists(&path);
}

#[test]
fn disagreeing_leftover_snapshot_does_not_change_wallet_balances() {
    // Arrange
    let path = temp_store_path("disagreeing-leftover");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let wallet = wallet_with_ranged_descriptor();
    let coins_truth = funded_chainstate(&wallet);
    plant_coins_truth_with_payloads(&store, &coins_truth, &[0, 1, 2, 3]);
    store
        .save_chainstate_snapshot(&poison_leftover_snapshot(), PersistMode::Sync)
        .expect("plant poison leftover");
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("save wallet");
    drop(store);

    // Act
    let reopened_store = FjallNodeStore::open(&path).expect("reopen store");
    let runtime = WalletRescanRuntime::open_with_chunk_size(reopened_store, PersistMode::Sync, 4)
        .expect("runtime");
    let job = runtime.enqueue_rescan("alpha").expect("enqueue");
    let registry = WalletRegistry::load(runtime.store()).expect("registry");
    let wallet_snapshot = registry
        .wallet_snapshot("alpha")
        .expect("wallet snapshot")
        .clone();
    let leftover_present = runtime
        .store()
        .load_chainstate_snapshot()
        .expect("load leftover")
        .is_some();

    // Assert
    assert_eq!(job.state, WalletRescanJobState::Complete);
    assert_eq!(wallet_snapshot.maybe_tip_height, Some(3));
    assert_eq!(wallet_snapshot.utxos.len(), 2);
    let total_sats: i64 = wallet_snapshot
        .utxos
        .iter()
        .map(|utxo| utxo.output.value.to_sats())
        .sum();
    assert_eq!(total_sats, 60_000);
    assert!(leftover_present);
    for position in &coins_truth.active_chain {
        assert!(
            runtime
                .store()
                .has_block(position.block_hash)
                .expect("has_block"),
            "payload must remain for height {}",
            position.height
        );
    }

    remove_dir_if_exists(&path);
}

#[test]
fn missing_block_payload_fails_chunk_closed() {
    // Arrange
    let path = temp_store_path("missing-payload");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let wallet = wallet_with_ranged_descriptor();
    let receive_script = wallet
        .default_receive_address()
        .expect("receive")
        .script_pubkey;
    let mut utxos = HashMap::new();
    utxos.insert(
        OutPoint {
            txid: Txid::from_byte_array([1_u8; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(25_000).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: 1,
            created_median_time_past: 1_700_000_001,
        },
    );
    let coins_truth =
        ChainstateSnapshot::new(vec![tip(0), tip(1), tip(2)], utxos, Default::default());
    plant_coins_truth_with_payloads(&store, &coins_truth, &[0, 1]);
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("save wallet");

    // Act
    let runtime =
        WalletRescanRuntime::open_with_chunk_size(store, PersistMode::Sync, 3).expect("runtime");
    let enqueue_result = runtime.enqueue_rescan("alpha");
    let job = runtime
        .store()
        .load_wallet_rescan_job("alpha")
        .expect("load job")
        .expect("job");

    // Assert
    assert!(enqueue_result.is_err(), "missing payload must fail closed");
    assert_eq!(job.state, WalletRescanJobState::Failed);
    assert_ne!(job.freshness, WalletRescanFreshness::Fresh);
    let error = job.maybe_error.expect("failed job error");
    assert!(error.contains("missing block payload"), "error was {error}");

    remove_dir_if_exists(&path);
}

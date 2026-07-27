// Parity breadcrumbs:
// - packages/bitcoin-knots/src/net_processing.cpp
// - packages/bitcoin-knots/test/functional/p2p_initial_headers_sync.py

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{ChainPosition, ChainstateSnapshot, Coin},
    primitives::{BlockHash, BlockHeader, OutPoint, TransactionOutput, Txid},
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
                value: open_bitcoin_core::primitives::Amount::from_sats(25_000).expect("amount"),
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
                value: open_bitcoin_core::primitives::Amount::from_sats(35_000).expect("amount"),
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

#[test]
fn restart_resume_advances_pending_rescan_in_bounded_chunks() {
    // Arrange
    let path = temp_store_path("resume-chunks");
    remove_dir_if_exists(&path);
    let store = FjallNodeStore::open(&path).expect("open store");
    let wallet = wallet_with_ranged_descriptor();
    store
        .save_chainstate_snapshot(&funded_chainstate(&wallet), PersistMode::Sync)
        .expect("save chainstate");
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

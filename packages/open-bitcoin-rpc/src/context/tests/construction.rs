// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/rpc/protocol.h
// - packages/bitcoin-knots/src/rpc/request.cpp
// - packages/bitcoin-knots/src/rpc/server.cpp
// - packages/bitcoin-knots/src/rpc/blockchain.cpp
// - packages/bitcoin-knots/src/rpc/mempool.cpp
// - packages/bitcoin-knots/src/rpc/net.cpp
// - packages/bitcoin-knots/src/rpc/rawtransaction.cpp
// - packages/bitcoin-knots/test/functional/interface_rpc.py

use super::*;

#[test]
fn authoritative_network_sync_mutation_is_visible_to_rpc() {
    // Arrange
    let data_dir = test_data_dir("authoritative-network");
    let store = FjallNodeStore::open(&data_dir).expect("open authoritative store");
    let sync_runtime = DurableSyncRuntime::open(
        store.clone(),
        SyncRuntimeConfig {
            network: SyncNetwork::Regtest,
            dns_seeds: Vec::new(),
            ..SyncRuntimeConfig::default()
        },
    )
    .expect("open authoritative sync runtime");
    let sync_network = sync_runtime.network_handle();
    let context = ManagedRpcContext::from_runtime_config_with_network_handle(
        &RuntimeConfig {
            chain: AddressNetwork::Regtest,
            ..RuntimeConfig::default()
        },
        sync_runtime.network_handle(),
        Some(store),
    )
    .expect("construct RPC from authoritative handle");

    // Act
    sync_network
        .connect_outbound_peer(42, 1_777_225_210)
        .expect("sync mutation should succeed");
    let network_info = context
        .network_info()
        .expect("RPC should snapshot the shared authority");

    // Assert
    assert_eq!(network_info.outbound_peers, 1);
    fs::remove_dir_all(data_dir).expect("remove authoritative store");
}

#[test]
fn managed_rpc_context_builds_from_runtime_config() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);
    let network_info = context.network_info().expect("authoritative network info");
    let wallet_info = context.wallet_info();
    let snapshot = context
        .blockchain_snapshot()
        .expect("authoritative chainstate snapshot");

    // Assert
    assert_eq!(context.chain(), AddressNetwork::Regtest);
    assert_eq!(network_info.connected_peers, 0);
    assert!(
        !context
            .network_info()
            .expect("authoritative network info")
            .relay
    );
    assert_eq!(wallet_info.network, AddressNetwork::Regtest);
    assert!(snapshot.active_chain.is_empty());
}

#[test]
fn managed_rpc_context_builds_from_runtime_config_with_enabled_relay_activation() {
    // Arrange
    let runtime = RuntimeConfig {
        chain: AddressNetwork::Regtest,
        relay: RelayActivationConfig { enabled: true },
        inbound: open_bitcoin_network::InboundListenerConfig {
            enabled: true,
            ..Default::default()
        },
        ..RuntimeConfig::default()
    };

    // Act
    let context = ManagedRpcContext::from_runtime_config(&runtime);

    // Assert
    assert!(
        context
            .network_info()
            .expect("authoritative network info")
            .relay
    );
}

const RANGED_RECEIVE_DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)";

fn tip_position(height: u32) -> open_bitcoin_node::core::chainstate::ChainPosition {
    use open_bitcoin_node::core::chainstate::ChainPosition;
    use open_bitcoin_node::core::primitives::{BlockHash, BlockHeader};

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

fn wallet_with_ranged_receive() -> open_bitcoin_node::core::wallet::Wallet {
    use open_bitcoin_node::core::wallet::{AddressNetwork, DescriptorRole, Wallet};

    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor(
            "receive-ranged",
            DescriptorRole::External,
            RANGED_RECEIVE_DESCRIPTOR,
        )
        .expect("descriptor");
    wallet
}

fn coins_truth_with_wallet_utxo(
    wallet: &open_bitcoin_node::core::wallet::Wallet,
    tip_height: u32,
    value_sats: i64,
) -> open_bitcoin_node::core::chainstate::ChainstateSnapshot {
    use std::collections::HashMap;

    use open_bitcoin_node::core::chainstate::{ChainstateSnapshot, Coin};
    use open_bitcoin_node::core::primitives::{Amount, OutPoint, TransactionOutput, Txid};

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
                value: Amount::from_sats(value_sats).expect("amount"),
                script_pubkey: receive_script,
            },
            is_coinbase: false,
            created_height: tip_height.min(1),
            created_median_time_past: 1_700_000_001,
        },
    );
    let active_chain = (0..=tip_height).map(tip_position).collect();
    ChainstateSnapshot::new(active_chain, utxos, Default::default())
}

fn poison_leftover_snapshot() -> open_bitcoin_node::core::chainstate::ChainstateSnapshot {
    use std::collections::HashMap;

    use open_bitcoin_node::core::chainstate::{ChainstateSnapshot, Coin};
    use open_bitcoin_node::core::primitives::{
        Amount, OutPoint, ScriptBuf, TransactionOutput, Txid,
    };

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
    ChainstateSnapshot::new(vec![tip_position(99)], utxos, Default::default())
}

fn block_matching_position(
    position: &open_bitcoin_node::core::chainstate::ChainPosition,
) -> open_bitcoin_node::core::primitives::Block {
    use open_bitcoin_node::core::primitives::Block;

    Block {
        header: position.header.clone(),
        transactions: Vec::new(),
    }
}

#[test]
fn durable_rescan_ignores_disagreeing_leftover_snapshot() {
    use open_bitcoin_node::core::wallet::AddressNetwork;
    use open_bitcoin_node::{PersistMode, WalletRegistry};

    use crate::config::WalletRuntimeConfig;

    // Arrange
    let data_dir = test_data_dir("durable-rescan-leftover");
    let store = FjallNodeStore::open(&data_dir).expect("open store");
    let wallet = wallet_with_ranged_receive();
    let coins_truth = coins_truth_with_wallet_utxo(&wallet, 1, 25_000);
    store
        .seed_coins_from_snapshot(&coins_truth)
        .expect("seed coins");
    for position in &coins_truth.active_chain {
        store
            .save_block(&block_matching_position(position), PersistMode::Sync)
            .expect("save block payload");
    }
    store
        .save_chainstate_snapshot(&poison_leftover_snapshot(), PersistMode::Sync)
        .expect("plant poison leftover");
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("create wallet");
    drop(store);

    // Act
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    });
    context.set_request_wallet_name(Some("alpha".to_string()));
    let execution = context
        .rescan_wallet_range(Some(0), None)
        .expect("rescan from coins truth");
    let wallet_snapshot = context.wallet_snapshot().expect("wallet snapshot");
    assert_eq!(execution.stop_height, 1);
    assert_eq!(wallet_snapshot.maybe_tip_height, Some(1));
    let total_sats: i64 = wallet_snapshot
        .utxos
        .iter()
        .map(|utxo| utxo.output.value.to_sats())
        .sum();
    assert_eq!(total_sats, 25_000);
    drop(context);

    // Assert leftover remains non-authoritative on disk (D-05).
    let leftover_present = FjallNodeStore::open(&data_dir)
        .expect("reopen store")
        .load_chainstate_snapshot()
        .expect("load leftover")
        .is_some();
    assert!(leftover_present);
    fs::remove_dir_all(data_dir).expect("remove durable store");
}

#[test]
fn durable_rescan_missing_block_payload_fails_closed() {
    use open_bitcoin_node::core::wallet::AddressNetwork;
    use open_bitcoin_node::{PersistMode, WalletRegistry};

    use crate::config::WalletRuntimeConfig;

    // Arrange
    let data_dir = test_data_dir("durable-rescan-missing-payload");
    let store = FjallNodeStore::open(&data_dir).expect("open store");
    let wallet = wallet_with_ranged_receive();
    let coins_truth = coins_truth_with_wallet_utxo(&wallet, 2, 25_000);
    store
        .seed_coins_from_snapshot(&coins_truth)
        .expect("seed coins");
    for position in &coins_truth.active_chain {
        if position.height == 2 {
            continue;
        }
        store
            .save_block(&block_matching_position(position), PersistMode::Sync)
            .expect("save block payload");
    }
    let mut registry = WalletRegistry::default();
    registry
        .create_wallet(&store, "alpha", wallet, PersistMode::Sync)
        .expect("create wallet");
    drop(store);

    // Act
    let mut context = ManagedRpcContext::from_runtime_config(&RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(data_dir.clone()),
        wallet: WalletRuntimeConfig {
            coinbase_maturity: 1,
            ..WalletRuntimeConfig::default()
        },
        ..RuntimeConfig::default()
    });
    context.set_request_wallet_name(Some("alpha".to_string()));
    let error = context
        .rescan_wallet_range(Some(0), None)
        .expect_err("missing payload must fail closed");

    // Assert
    let message = error
        .maybe_detail
        .as_ref()
        .map(|detail| detail.message.as_str())
        .unwrap_or_default();
    assert!(
        message.contains("missing block payload"),
        "error detail was {message:?}"
    );
    fs::remove_dir_all(data_dir).expect("remove durable store");
}

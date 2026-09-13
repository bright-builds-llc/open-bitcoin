// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use std::collections::HashMap;

use open_bitcoin_core::{
    chainstate::{BlockUndo, ChainstateSnapshot, Coin, CoinsBatch, CoinsCacheEntry, TxUndo},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
        transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, NetworkAddress, NetworkMagic, OutPoint, ScriptBuf,
        ScriptWitness, Transaction, TransactionInput, TransactionOutput, Txid,
    },
};

use crate::ManagedPeerNetwork;
use crate::chainstate::{
    ChainstateStore, FlushLifecycle, ManagedChainstate, ManagerReadiness, MemoryChainstateStore,
};
use open_bitcoin_mempool::PolicyConfig;
use open_bitcoin_network::{LocalPeerConfig, ServiceFlags};

const EASY_BITS: u32 = 0x207f_ffff;

fn script(bytes: &[u8]) -> ScriptBuf {
    ScriptBuf::from_bytes(bytes.to_vec()).expect("valid script")
}

fn serialized_script_num(value: i64) -> Vec<u8> {
    if value == 0 {
        return vec![0x00];
    }

    let mut magnitude = value as u64;
    let mut encoded = Vec::new();
    while magnitude > 0 {
        encoded.push((magnitude & 0xff) as u8);
        magnitude >>= 8;
    }

    let mut script = Vec::with_capacity(encoded.len() + 2);
    script.push(encoded.len() as u8);
    script.extend(encoded);
    script.push(0x51);
    script
}

fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut script_sig = serialized_script_num(i64::from(height));
    script_sig.push(0x51);
    Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: script(&script_sig),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn spend_transaction(
    previous_txid: Txid,
    previous_vout: u32,
    value: i64,
    extra_missing_input: bool,
) -> Transaction {
    let mut inputs = vec![TransactionInput {
        previous_output: OutPoint {
            txid: previous_txid,
            vout: previous_vout,
        },
        script_sig: script(&[0x51]),
        sequence: TransactionInput::SEQUENCE_FINAL,
        witness: ScriptWitness::default(),
    }];
    if extra_missing_input {
        inputs.push(TransactionInput {
            previous_output: OutPoint {
                txid: Txid::from_byte_array([9_u8; 32]),
                vout: 0,
            },
            script_sig: script(&[0x51]),
            sequence: TransactionInput::SEQUENCE_FINAL,
            witness: ScriptWitness::default(),
        });
    }
    Transaction {
        version: 2,
        inputs,
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    }
}

fn mine_header(block: &mut Block) {
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("expected nonce at easy target");
}

fn build_block(previous_block_hash: BlockHash, height: u32, value: i64) -> Block {
    build_block_with_transactions(
        previous_block_hash,
        height,
        vec![coinbase_transaction(height, value)],
    )
}

fn build_block_with_transactions(
    previous_block_hash: BlockHash,
    height: u32,
    transactions: Vec<Transaction>,
) -> Block {
    let (merkle_root, maybe_mutated) = block_merkle_root(&transactions).expect("merkle root");
    assert!(!maybe_mutated);

    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash,
            merkle_root,
            time: 1_231_006_500 + height,
            bits: EASY_BITS,
            nonce: 0,
        },
        transactions,
    };
    mine_header(&mut block);
    block
}

fn mature_params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

fn sample_coin(created_height: u32, created_median_time_past: i64) -> Coin {
    Coin {
        output: TransactionOutput {
            value: Amount::from_sats(50).expect("valid amount"),
            script_pubkey: script(&[0x51]),
        },
        is_coinbase: true,
        created_height,
        created_median_time_past,
    }
}

fn connect_genesis(managed: &mut ManagedChainstate<MemoryChainstateStore>) -> (Block, OutPoint) {
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);
    managed
        .connect_block(&genesis, 1, ScriptVerifyFlags::P2SH, mature_params())
        .expect("genesis should connect");
    let genesis_txid = transaction_txid(&genesis.transactions[0]).expect("txid");
    (
        genesis,
        OutPoint {
            txid: genesis_txid,
            vout: 0,
        },
    )
}

#[test]
fn memory_store_round_trips_saved_snapshots() {
    // Arrange
    let mut store = MemoryChainstateStore::default();
    let snapshot = open_bitcoin_core::chainstate::ChainstateSnapshot::new(
        Vec::new(),
        Default::default(),
        Default::default(),
    );

    // Act
    store.save_snapshot(snapshot.clone());

    // Assert
    assert_eq!(store.load_snapshot(), Some(snapshot));
}

#[test]
fn memory_store_from_snapshot_serves_coins_from_view_not_later_snapshot_utxos() {
    // Arrange
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1; 32]),
        vout: 0,
    };
    let coin = sample_coin(4, 9);
    let undo_hash = BlockHash::from_byte_array([2; 32]);
    let undo = BlockUndo {
        transactions: vec![TxUndo {
            restored_inputs: vec![coin.clone()],
        }],
    };
    let mut utxos = HashMap::new();
    utxos.insert(outpoint.clone(), coin.clone());
    let mut undo_by_block = HashMap::new();
    undo_by_block.insert(undo_hash, undo.clone());
    let snapshot = ChainstateSnapshot::new(Vec::new(), utxos, undo_by_block);
    let mut store = MemoryChainstateStore::from_snapshot(snapshot);
    let leftover = ChainstateSnapshot::new(Vec::new(), HashMap::new(), HashMap::new());

    // Act
    store.save_snapshot(leftover.clone());

    // Assert
    assert_eq!(
        store.get_coin(&outpoint).expect("view-backed get_coin"),
        Some(coin)
    );
    assert_eq!(
        store.load_undo(undo_hash).expect("view-backed load_undo"),
        Some(undo)
    );
    assert_eq!(store.load_snapshot(), Some(leftover));
}

#[test]
fn memory_store_batch_write_updates_view_and_best_block() {
    // Arrange
    let mut store = MemoryChainstateStore::default();
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1; 32]),
        vout: 0,
    };
    let coin = sample_coin(4, 9);
    let mut entries = HashMap::new();
    entries.insert(
        outpoint.clone(),
        CoinsCacheEntry::unspent_dirty(coin.clone()),
    );
    let best_block = BlockHash::from_byte_array([3; 32]);

    // Act
    store
        .batch_write(CoinsBatch { entries }, Some(best_block))
        .expect("batch_write updates the memory view");

    // Assert
    assert_eq!(
        store
            .get_coin(&outpoint)
            .expect("get_coin after batch_write"),
        Some(coin)
    );
    assert_eq!(
        store.best_block().expect("best_block after batch_write"),
        Some(best_block)
    );
    assert_eq!(
        store.head_blocks().expect("memory heads stay empty"),
        Vec::<BlockHash>::new()
    );
    assert_eq!(store.load_snapshot(), None);
}

#[test]
fn memory_store_save_undo_round_trips_without_snapshot() {
    // Arrange
    let mut store = MemoryChainstateStore::default();
    let block_hash = BlockHash::from_byte_array([4; 32]);
    let undo = BlockUndo {
        transactions: vec![TxUndo {
            restored_inputs: vec![sample_coin(4, 9)],
        }],
    };

    // Act
    store
        .save_undo(block_hash, undo.clone())
        .expect("save_undo stores the record");

    // Assert
    assert_eq!(
        store.load_undo(block_hash).expect("load_undo round-trip"),
        Some(undo)
    );
    assert_eq!(
        store
            .load_undo(BlockHash::from_byte_array([9; 32]))
            .expect("missing undo is a successful miss"),
        None
    );
}

#[test]
fn managed_chainstate_persists_after_connect() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);

    // Act
    let position = managed
        .connect_block(&genesis, 1, ScriptVerifyFlags::P2SH, mature_params())
        .expect("managed chainstate should persist connected snapshot");

    // Assert
    assert_eq!(position.height, 0);
    assert_eq!(managed.store().load_snapshot(), None);
    let _ = managed.store().best_block().expect("coins best_block");
    assert_eq!(managed.chainstate().tip(), Some(&position));
    assert!(!managed.chainstate().utxos().is_empty());
}

#[test]
fn prepare_connect_does_not_change_live_utxos_or_best_block() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let (_genesis, genesis_outpoint) = connect_genesis(&mut managed);
    let tip_before = managed.chainstate().tip().cloned();
    let utxos_before = managed.chainstate().utxos();
    let in_cache_before = managed.chainstate().have_coin_in_cache(&genesis_outpoint);
    let orphan = build_block(BlockHash::from_byte_array([7_u8; 32]), 1, 50);

    // Act
    let result =
        managed.prepare_connect_block(&orphan, 2, ScriptVerifyFlags::P2SH, mature_params());

    // Assert
    let Err(error) = result else {
        panic!("wrong prev hash must fail prepare");
    };
    assert!(matches!(
        error,
        open_bitcoin_core::chainstate::ChainstateError::InvalidTipExtension { .. }
    ));
    assert_eq!(managed.chainstate().tip(), tip_before.as_ref());
    assert_eq!(managed.chainstate().utxos(), utxos_before);
    assert_eq!(
        managed.chainstate().have_coin_in_cache(&genesis_outpoint),
        in_cache_before
    );
}

#[test]
fn prepare_connect_then_drop_leaves_parent_unpopulated() {
    // Arrange
    let mut seeded = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let (genesis, genesis_outpoint) = connect_genesis(&mut seeded);
    let snapshot = seeded.chainstate().snapshot();
    let rematerialized =
        ManagedChainstate::from_store(MemoryChainstateStore::from_snapshot(snapshot));
    assert!(
        !rematerialized
            .chainstate()
            .have_coin_in_cache(&genesis_outpoint),
        "from_snapshot hydrates parent-only coins"
    );
    let spend = spend_transaction(genesis_outpoint.txid, genesis_outpoint.vout, 40, true);
    let failing = build_block_with_transactions(
        block_hash(&genesis.header),
        1,
        vec![coinbase_transaction(1, 50), spend],
    );

    // Act
    let result =
        rematerialized.prepare_connect_block(&failing, 2, ScriptVerifyFlags::P2SH, mature_params());

    // Assert
    let Err(_) = result else {
        panic!("missing second input must fail after peeking the parent coin");
    };

    // Assert
    assert!(
        !rematerialized
            .chainstate()
            .have_coin_in_cache(&genesis_outpoint),
        "failed prepare must not FetchCoin into the live cache"
    );
    assert!(
        rematerialized
            .chainstate()
            .have_coin(&genesis_outpoint)
            .expect("parent-only genesis coin stays spendable")
    );
}

#[test]
fn commit_prepared_connect_flushes_and_persists_snapshot_blob() {
    // Arrange
    let mut managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);
    let prepared = managed
        .prepare_connect_block(&genesis, 1, ScriptVerifyFlags::P2SH, mature_params())
        .expect("genesis should prepare");

    // Act
    let position = managed
        .commit_prepared_connect(prepared)
        .expect("prepared connect persist");

    // Assert
    assert_eq!(position.height, 0);
    assert_eq!(managed.store().load_snapshot(), None);
    let _ = managed.store().get_coin(&OutPoint {
        txid: transaction_txid(&genesis.transactions[0]).expect("txid"),
        vout: 0,
    });
    let _ = managed.store().best_block().expect("coins best_block");
    assert_eq!(managed.chainstate().tip(), Some(&position));
    assert!(!managed.chainstate().utxos().is_empty());
}

#[test]
fn managed_chainstate_owns_required_flush_lifecycle() {
    // Arrange
    let managed = ManagedChainstate::from_store(MemoryChainstateStore::default());
    let source = include_str!("../chainstate.rs");

    // Act / Assert
    assert_eq!(
        managed.flush_lifecycle.readiness(),
        ManagerReadiness::ReadyToFlush
    );
    assert!(source.contains("flush_lifecycle: FlushLifecycle"));
    assert!(!source.contains("Option<FlushLifecycle>"));
    assert!(!source.contains("maybe_flush_lifecycle"));
    let _ = FlushLifecycle::ready(
        open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
        open_bitcoin_core::chainstate::FlushPolicyTime::from_unix_seconds(0),
        0,
        false,
    );
}

#[test]
fn connect_block_does_not_call_flush_mode_always() {
    // Arrange
    let source = include_str!("../chainstate.rs");
    let names = [
        "fn persist",
        "fn connect_block",
        "fn commit_prepared_connect",
        "fn commit_prepared_reorg",
        "fn disconnect_tip",
        "fn reorg",
    ];

    // Act / Assert
    for name in names {
        let body = function_body(source, name);
        assert!(
            !body.contains("FlushMode::Always") && !body.contains("FlushMode::Periodic"),
            "{name} must not call Always or Periodic"
        );
    }
}

fn function_body(source: &str, marker: &str) -> String {
    let start = source.find(marker).expect(marker);
    let after = &source[start..];
    let open = after.find('{').expect("body open");
    let mut depth = 0_i32;
    for (idx, ch) in after[open..].char_indices() {
        match ch {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return after[open..=open + idx].to_string();
                }
            }
            _ => {}
        }
    }
    panic!("unbalanced {marker}");
}

fn network_local_config() -> LocalPeerConfig {
    LocalPeerConfig {
        magic: NetworkMagic::MAINNET,
        services: ServiceFlags::NETWORK | ServiceFlags::WITNESS,
        address: NetworkAddress {
            services: 0,
            address_bytes: [0_u8; 16],
            port: 8333,
        },
        nonce: 13904,
        relay: true,
        user_agent: "/open-bitcoin:test/".to_string(),
    }
}

#[test]
fn prepare_chainstate_failure_does_not_warm_live_cache_before_mempool_prepare() {
    // Arrange
    let mut network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        network_local_config(),
        PolicyConfig::default(),
    );
    let orphan = build_block(BlockHash::from_byte_array([7_u8; 32]), 1, 50);
    let tip_before = network.chainstate().chainstate().tip().cloned();
    let utxos_before = network.chainstate().chainstate().utxos();

    // Act
    let result = network.chainstate().prepare_connect_block(
        &orphan,
        2,
        ScriptVerifyFlags::P2SH,
        mature_params(),
    );

    // Assert
    let Err(_) = result else {
        panic!("non-extending header must fail prepare before mempool prepare");
    };
    assert_eq!(network.chainstate().chainstate().tip(), tip_before.as_ref());
    assert_eq!(network.chainstate().chainstate().utxos(), utxos_before);

    let genesis = build_block(BlockHash::from_byte_array([0_u8; 32]), 0, 50);
    let position = network
        .connect_local_block(&genesis, ScriptVerifyFlags::P2SH, mature_params())
        .expect("failed prepare must not corrupt the live handle");
    assert_eq!(position.height, 0);
}

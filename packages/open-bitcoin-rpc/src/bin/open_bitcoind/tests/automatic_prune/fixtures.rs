// Parity breadcrumbs:
// - packages/bitcoin-knots/src/bitcoind.cpp
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use open_bitcoin_node::core::chainstate::{BlockUndo, ChainPosition, ChainstateSnapshot, Coin};
use open_bitcoin_node::core::{
    consensus::{ConsensusParams, ScriptVerifyFlags},
    primitives::{
        Amount, Block, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput, Txid,
    },
    wallet::{AddressNetwork, DescriptorRole, Wallet, WalletSnapshot},
};
use open_bitcoin_node::{
    FjallChainstateStore, ManagedNetworkHandle, PersistMode, SyncNetwork, WalletRegistry,
};
use std::collections::HashMap;

pub(super) const TARGET: u64 = 550 * 1024 * 1024;
const DESCRIPTOR: &str = "wpkh(tprv8ZgxMBicQKsPd7Uf69XL1XwhmjHopUGep8GuEiJDZmbQz6o58LninorQAfcKZWARbtRtfnLcJ5MQ2AtHcQJCCRUcMRvmDUjyEmNUWwx8UbK/1/1/*)";

pub(super) struct TempDir(pub(super) PathBuf);

impl TempDir {
    pub(super) fn new(label: &str) -> Self {
        Self(temp_store_path(label))
    }
}

impl Drop for TempDir {
    fn drop(&mut self) {
        remove_dir_if_exists(&self.0);
    }
}

pub(super) fn config(temp: &TempDir) -> RuntimeConfig {
    RuntimeConfig {
        chain: AddressNetwork::Regtest,
        maybe_data_dir: Some(temp.0.clone()),
        prune_mode: PruneMode::Automatic { target_mib: 550 },
        sync: DaemonSyncConfig {
            runtime: SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                dns_seeds: Vec::new(),
                manual_peers: Vec::new(),
                ..SyncRuntimeConfig::default()
            },
            ..DaemonSyncConfig::default()
        },
        ..RuntimeConfig::default()
    }
}

pub(super) fn position(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            time: 1_700_000_000 + height,
            nonce: height,
            bits: 0x207f_ffff,
            ..BlockHeader::default()
        },
        height,
        u128::from(height) + 1,
        1_700_000_000 + i64::from(height),
    )
}

pub(super) fn seed(store: &FjallNodeStore) -> ChainstateSnapshot {
    let positions: Vec<_> = [1, 500, 510, 713, 714, 1_001]
        .into_iter()
        .map(position)
        .collect();
    let mut wallet = Wallet::new(AddressNetwork::Regtest);
    wallet
        .import_descriptor("receive", DescriptorRole::External, DESCRIPTOR)
        .expect("descriptor");
    let script = wallet
        .default_receive_address()
        .expect("address")
        .script_pubkey;
    let utxos = HashMap::from([(
        OutPoint {
            txid: Txid::from_byte_array([1; 32]),
            vout: 0,
        },
        Coin {
            output: TransactionOutput {
                value: Amount::from_sats(25_000).expect("amount"),
                script_pubkey: script,
            },
            is_coinbase: false,
            created_height: 1,
            created_median_time_past: 1_700_000_001,
        },
    )]);
    let undo = positions
        .iter()
        .map(|p| (p.block_hash, BlockUndo::default()))
        .collect();
    let truth = ChainstateSnapshot::new(positions, utxos, undo);
    store
        .seed_coins_from_snapshot(&truth)
        .expect("seed consistent coins and chain meta");
    for p in &truth.active_chain {
        save_small(store, p);
    }
    let mut prior = truth.clone();
    prior.active_chain.retain(|p| p.height <= 1);
    wallet.rescan_chainstate(&prior).expect("fund prior wallet");
    WalletRegistry::default()
        .create_wallet(store, "alpha", wallet, PersistMode::Sync)
        .expect("wallet");
    truth
}

pub(super) fn small_block(p: &ChainPosition) -> Block {
    Block {
        header: p.header.clone(),
        transactions: Vec::new(),
    }
}

pub(super) fn save_small(store: &FjallNodeStore, p: &ChainPosition) {
    store
        .save_block(&small_block(p), PersistMode::Sync)
        .expect("small block");
    store
        .save_undo(p.block_hash, &BlockUndo::default(), PersistMode::Sync)
        .expect("undo");
}

pub(super) fn cache_small(
    handle: &ManagedNetworkHandle<
        impl open_bitcoin_node::ChainstateStore,
        impl open_bitcoin_node::core::chainstate::CoinsView,
    >,
    p: &ChainPosition,
) {
    handle
        .connect_stored_block(
            &small_block(p),
            p.chain_work,
            p.median_time_past,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        )
        .expect("cache existing active block");
    assert!(handle.cached_block_present(p.block_hash).expect("cache"));
}

/// One reused ~2.5 MiB body; distinct canonical headers identify nonactive values.
/// These are codec-valid sparse fixtures, not a consensus-validated chain.
pub(super) fn populate_threshold(store: &FjallNodeStore, truth: &ChainstateSnapshot) -> u64 {
    let initial = store
        .retained_payload_usage(&truth.active_chain)
        .expect("initial usage");
    let before = initial.current_usage_bytes;
    let script = ScriptBuf::from_bytes(vec![0x51; 8_192]).expect("bounded valid script");
    let mut block = Block {
        header: position(10_000).header,
        transactions: vec![Transaction {
            version: 2,
            inputs: vec![TransactionInput {
                previous_output: OutPoint {
                    txid: Txid::from_byte_array([7; 32]),
                    vout: 0,
                },
                script_sig: ScriptBuf::default(),
                sequence: TransactionInput::SEQUENCE_FINAL,
                witness: ScriptWitness::default(),
            }],
            outputs: (0..300)
                .map(|_| TransactionOutput {
                    value: Amount::from_sats(1).expect("amount"),
                    script_pubkey: script.clone(),
                })
                .collect(),
            lock_time: 0,
        }],
    };
    let body_len = u64::try_from(
        open_bitcoin_codec::encode_block(&block)
            .expect("codec")
            .len(),
    )
    .expect("length");
    let undo = BlockUndo::default();
    let small_body_len = u64::try_from(
        open_bitcoin_codec::encode_block(&small_block(&truth.active_chain[0]))
            .expect("small codec")
            .len(),
    )
    .expect("length");
    let undo_len = initial.height_sizes[&1]
        .checked_sub(small_body_len)
        .expect("measured encoded undo mate");
    let mut encoded_sum = 0;
    let mut count = 0;
    while encoded_sum <= TARGET {
        block.header.nonce += 1;
        let hash = store
            .save_block(&block, PersistMode::Buffered)
            .expect("real block value");
        store
            .save_undo(hash, &undo, PersistMode::Buffered)
            .expect("real undo value");
        encoded_sum += body_len + undo_len;
        count += 1;
    }
    let measured = store
        .retained_payload_usage(&truth.active_chain)
        .expect("exact measured usage")
        .current_usage_bytes;
    assert_eq!(measured, before + encoded_sum);
    assert!(measured > TARGET);
    eprintln!(
        "genuine retention: target={TARGET} logical_bytes={measured} nonactive_bytes={encoded_sum} blocks={count} body_bytes={body_len} undo_bytes={undo_len}"
    );
    measured
}

pub(super) fn saved_wallet(store: &FjallNodeStore) -> WalletSnapshot {
    WalletRegistry::load(store)
        .expect("registry")
        .wallet_snapshot("alpha")
        .expect("wallet")
        .clone()
}

pub(super) struct MetadataFaultStore {
    inner: FjallChainstateStore,
    snapshot: ChainstateSnapshot,
    fail: Arc<std::sync::atomic::AtomicBool>,
}

pub(super) fn metadata_fault_owner(
    store: &FjallNodeStore,
) -> (
    ManagedNetworkHandle<MetadataFaultStore>,
    Arc<std::sync::atomic::AtomicBool>,
) {
    let snapshot = store
        .wallet_scan_chainstate_snapshot()
        .expect("durable truth")
        .expect("coins truth");
    let fail = Arc::new(std::sync::atomic::AtomicBool::new(true));
    let sink = MetadataFaultStore {
        inner: FjallChainstateStore::from_store(store.clone()),
        snapshot,
        fail: Arc::clone(&fail),
    };
    let network =
        open_bitcoin_node::ManagedPeerNetwork::new(sink, Default::default(), Default::default());
    let mut handle = ManagedNetworkHandle::from_network_fixture(network);
    handle
        .set_prune_mode(PruneMode::Automatic { target_mib: 550 })
        .expect("legal mode");
    handle
        .set_prune_network(SyncNetwork::Regtest)
        .expect("network threshold");
    handle
        .set_coins_next_write(open_bitcoin_node::core::chainstate::FlushPolicyTime::new(
            10_000,
        ))
        .expect("future deadline");
    (handle, fail)
}

use open_bitcoin_node::chainstate::FlushPersistSink;
use open_bitcoin_node::core::{
    chainstate::{ChainstateError, CoinsBatch},
    primitives::BlockHash,
};
use open_bitcoin_node::storage::fjall_store::{
    PairedDeleteOutcome, PayloadUsageRevision, RetainedPayloadUsage,
};
use open_bitcoin_node::{ChainstateStore, StorageError};

impl ChainstateStore for MetadataFaultStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        Some(self.snapshot.clone())
    }
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.snapshot = snapshot;
    }
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.inner.get_coin(outpoint)
    }
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.inner.have_coin(outpoint)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.inner.best_block()
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.inner.head_blocks()
    }
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.inner.batch_write(writes, maybe_best)
    }
    fn load_undo(&self, hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        self.inner.load_undo(hash)
    }
    fn save_undo(&mut self, hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.inner.save_undo(hash, undo)
    }
}

impl FlushPersistSink for MetadataFaultStore {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        self.inner.persist_block(block)
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        self.inner.persist_undo(hash, undo)
    }
    fn persist_header_entries(
        &mut self,
        entries: &[open_bitcoin_network::HeaderEntry],
    ) -> Result<(), StorageError> {
        self.inner.persist_header_entries(entries)
    }
    fn persist_chain_meta(&mut self, chain: &[ChainPosition]) -> Result<(), StorageError> {
        if self.fail.load(std::sync::atomic::Ordering::SeqCst) {
            return Err(StorageError::UnavailableNamespace {
                namespace: open_bitcoin_node::StorageNamespace::Chainstate,
            });
        }
        self.inner.persist_chain_meta(chain)
    }
    fn retained_payload_usage(
        &self,
        chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        self.inner.retained_payload_usage(chain)
    }
    fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        self.inner.payload_usage_revision()
    }
    fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        self.inner.load_prune_locks()
    }
    fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        self.inner.sync_prune_locks(locks)
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        self.inner.commit_paired_unlink(height, hash)
    }
    fn record_successful_prune_batch(&mut self, heights: &[u32]) -> Result<(), StorageError> {
        self.inner.record_successful_prune_batch(heights)
    }
}

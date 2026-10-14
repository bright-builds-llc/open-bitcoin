// Parity breadcrumbs:
// - packages/bitcoin-knots/src/kernel/chainparams.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use open_bitcoin_node::{FjallChainstateStore, PersistMode, core::wallet::Wallet};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
    time::SystemTime,
};

static NEXT_DIR: AtomicU64 = AtomicU64::new(0);

use open_bitcoin_node::{
    chainstate::FlushPersistSink,
    core::{
        chainstate::{
            BlockUndo, ChainPosition, ChainstateError, ChainstateSnapshot, Coin, CoinsBatch,
        },
        primitives::{Block, BlockHash, OutPoint},
    },
};
use std::sync::Arc;

#[derive(Default)]
pub(super) struct ObservedStore {
    inner: MemoryChainstateStore,
    pub(super) calls: Arc<AtomicU64>,
}

impl ObservedStore {
    fn observe(&self) {
        self.calls.fetch_add(1, Ordering::Relaxed);
    }
}

impl FlushPersistSink for ObservedStore {
    fn persist_block(&mut self, block: &Block) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_block(block)
    }
    fn persist_undo(&mut self, hash: BlockHash, undo: &BlockUndo) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_undo(hash, undo)
    }
    fn persist_header_entries(
        &mut self,
        entries: &[open_bitcoin_network::HeaderEntry],
    ) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_header_entries(entries)
    }
    fn persist_chain_meta(&mut self, active_chain: &[ChainPosition]) -> Result<(), StorageError> {
        self.observe();
        self.inner.persist_chain_meta(active_chain)
    }
}

impl ChainstateStore for ObservedStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        self.observe();
        self.inner.load_snapshot()
    }
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.observe();
        self.inner.save_snapshot(snapshot);
    }
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.observe();
        self.inner.get_coin(outpoint)
    }
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.observe();
        self.inner.have_coin(outpoint)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.observe();
        self.inner.best_block()
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.observe();
        self.inner.head_blocks()
    }
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.observe();
        self.inner.batch_write(writes, maybe_best_block)
    }
    fn load_undo(&self, hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        self.observe();
        self.inner.load_undo(hash)
    }
    fn save_undo(&mut self, hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.observe();
        self.inner.save_undo(hash, undo)
    }
}

fn owned_path(label: &str) -> PathBuf {
    std::env::temp_dir()
        .canonicalize()
        .expect("physical temp parent")
        .join(format!(
            "phase164-rpc-{label}-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ))
}

pub(super) fn memory_handle(maybe_chain: Option<CanonicalChain>) -> ManagedNetworkHandle {
    let network = ManagedPeerNetwork::new(
        MemoryChainstateStore::default(),
        Default::default(),
        PolicyConfig::default(),
    );
    match maybe_chain {
        Some(chain) => ManagedNetworkHandle::from_canonical_network(network, chain),
        None => ManagedNetworkHandle::from_network_fixture(network),
    }
}

pub(super) fn owner_snapshot<S: ChainstateStore, V: CoinsView>(
    handle: &ManagedNetworkHandle<S, V>,
) -> String {
    format!(
        "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
        handle.operator_snapshot().expect("operator snapshot"),
        handle.chainstate_snapshot().expect("coins and tip"),
        handle.prune_mode().expect("prune mode"),
        handle.list_prune_locks().expect("prune locks"),
        handle
            .latest_mempool_recovery_summary()
            .expect("mempool recovery"),
        handle
            .latest_mempool_recovery_storage_error()
            .expect("recovery failure"),
        handle.maybe_canonical_chain().expect("binding"),
        handle
            .checkpoint_evidence(open_bitcoin_node::core::mempool::PolicyTime::new(0), 300)
            .expect("checkpoint state")
    )
}

pub(super) struct OwnedStore {
    pub(super) dir: PathBuf,
    pub(super) store: FjallNodeStore,
}

impl OwnedStore {
    pub(super) fn new() -> Self {
        let dir = owned_path("PRIVATE-durable");
        let store = FjallNodeStore::open(&dir).expect("owned raw fixture");
        store
            .save_wallet_snapshot(
                &Wallet::new(AddressNetwork::Regtest).snapshot(),
                PersistMode::Sync,
            )
            .expect("wallet sentinel");
        store
            .save_wallet_registry(
                &open_bitcoin_node::WalletRegistrySnapshot::new(["retained".to_string()]),
                PersistMode::Sync,
            )
            .expect("registry sentinel");
        store
            .save_selected_wallet(
                &open_bitcoin_node::SelectedWalletRecord {
                    wallet_name: "retained".to_string(),
                },
                PersistMode::Sync,
            )
            .expect("selected wallet sentinel");
        Self { dir, store }
    }

    pub(super) fn handle(
        &self,
        maybe_chain: Option<CanonicalChain>,
    ) -> ManagedNetworkHandle<FjallChainstateStore> {
        let network = ManagedPeerNetwork::new(
            FjallChainstateStore::from_store(self.store.clone()),
            Default::default(),
            PolicyConfig::default(),
        );
        match maybe_chain {
            Some(chain) => ManagedNetworkHandle::from_canonical_network(network, chain),
            None => ManagedNetworkHandle::from_network_fixture(network),
        }
    }

    pub(super) fn records(&self) -> String {
        format!(
            "{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}|{:?}",
            self.store.load_wallet_snapshot().expect("wallet"),
            self.store.load_wallet_registry().expect("registry"),
            self.store.load_selected_wallet().expect("selection"),
            self.store.load_chainstate_snapshot().expect("chain"),
            self.store.load_header_entries().expect("headers"),
            self.store.load_block_index_entries().expect("index"),
            self.store.load_runtime_metadata().expect("runtime"),
            self.store.load_recovery_marker().expect("recovery"),
            self.store.coins_view().best_block().expect("best block"),
            self.store.coins_view().head_blocks().expect("heads"),
            self.store.load_prune_protection().expect("prune protection"),
            self.store.load_mempool_snapshot_with_limits(open_bitcoin_node::storage::fjall_store::MempoolSnapshotDecodeLimits::for_persisted_input().expect("limits")).expect("mempool")
        )
    }

    pub(super) fn cleanup(self) {
        drop(self.store);
        fs::remove_dir_all(self.dir).expect("owned store cleanup");
    }
}

pub(super) fn closed_source(shape: &str) -> PathBuf {
    let dir = owned_path("PRIVATE\nclosed");
    match shape {
        "missing" => {}
        "empty" => fs::create_dir(&dir).expect("empty source"),
        "hostile" => {
            fs::create_dir(&dir).expect("hostile source");
            fs::write(dir.join("schema"), b"PRIVATE invalid schema\0\xff").expect("hostile bytes");
        }
        "synthetic" | "foreign" => {
            let store = FjallNodeStore::open(&dir).expect("closed raw source");
            store
                .save_wallet_snapshot(
                    &Wallet::new(AddressNetwork::Regtest).snapshot(),
                    PersistMode::Sync,
                )
                .expect("wallet bytes");
            if shape == "foreign" {
                store
                    .save_block(
                        &CanonicalChain::Mainnet.spec().genesis_block(),
                        PersistMode::Sync,
                    )
                    .expect("foreign root fixture");
            }
            drop(store);
        }
        _ => panic!("unknown owned source shape"),
    }
    dir
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct PhysicalEntry {
    kind: fs::FileType,
    bytes: Vec<u8>,
    length: u64,
    modified: SystemTime,
    readonly: bool,
    maybe_link: Option<PathBuf>,
    #[cfg(unix)]
    mode: u32,
}

pub(super) fn manifest(root: &Path) -> BTreeMap<PathBuf, PhysicalEntry> {
    fn visit(root: &Path, path: &Path, entries: &mut BTreeMap<PathBuf, PhysicalEntry>) {
        let metadata = fs::symlink_metadata(path).expect("owned metadata");
        let kind = metadata.file_type();
        entries.insert(
            path.strip_prefix(root).expect("prefix").to_path_buf(),
            PhysicalEntry {
                kind,
                bytes: if kind.is_file() {
                    fs::read(path).expect("owned bytes")
                } else {
                    Vec::new()
                },
                length: metadata.len(),
                modified: metadata.modified().expect("mtime"),
                readonly: metadata.permissions().readonly(),
                maybe_link: kind
                    .is_symlink()
                    .then(|| fs::read_link(path).expect("link")),
                #[cfg(unix)]
                mode: {
                    use std::os::unix::fs::PermissionsExt;
                    metadata.permissions().mode()
                },
            },
        );
        if kind.is_dir() {
            for entry in fs::read_dir(path).expect("directory") {
                visit(root, &entry.expect("entry").path(), entries);
            }
        }
    }
    let mut entries = BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

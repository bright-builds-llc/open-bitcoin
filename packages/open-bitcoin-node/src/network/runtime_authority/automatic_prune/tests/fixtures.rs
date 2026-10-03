// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

pub(super) const TARGET: u64 = 550 * 1024 * 1024;

#[derive(Default)]
pub(super) struct Facts {
    pub(super) scans: usize,
    pub(super) usage: u64,
    pub(super) sizes: BTreeMap<u32, u64>,
    pub(super) generation: u64,
    pub(super) locks: Vec<PruneLockInfo>,
    pub(super) deletes: Vec<u32>,
    pub(super) lock_writes: usize,
    pub(super) fail_accounting: bool,
    pub(super) fail_metadata: bool,
    pub(super) change_during_scan: bool,
    pub(super) invalid_revision: bool,
    pub(super) maybe_real_store: Option<crate::FjallNodeStore>,
    pub(super) maybe_revision_started: Option<std::sync::mpsc::Sender<()>>,
}

pub(super) struct TestStore {
    pub(super) memory: MemoryChainstateStore,
    pub(super) facts: Arc<Mutex<Facts>>,
}

pub(super) fn error() -> StorageError {
    StorageError::Corruption {
        namespace: StorageNamespace::BlockIndex,
        detail: "injected retention error".into(),
        action: StorageRecoveryAction::Repair,
    }
}
pub(super) fn revision(generation: u64) -> PayloadUsageRevision {
    PayloadUsageRevision::Current {
        store_id: 42,
        generation,
    }
}

impl FlushPersistSink for TestStore {
    fn persist_block(&mut self, _block: &Block) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_undo(&mut self, _hash: BlockHash, _undo: &BlockUndo) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_header_entries(&mut self, _entries: &[HeaderEntry]) -> Result<(), StorageError> {
        Ok(())
    }
    fn persist_chain_meta(&mut self, _chain: &[ChainPosition]) -> Result<(), StorageError> {
        if self.facts.lock().expect("facts").fail_metadata {
            Err(error())
        } else {
            Ok(())
        }
    }
    fn retained_payload_usage(
        &self,
        _chain: &[ChainPosition],
    ) -> Result<RetainedPayloadUsage, StorageError> {
        let mut facts = self.facts.lock().expect("facts");
        facts.scans += 1;
        if facts.fail_accounting {
            return Err(error());
        }
        if let Some(store) = facts.maybe_real_store.clone() {
            drop(facts);
            return store.retained_payload_usage(_chain);
        }
        let captured = if facts.invalid_revision {
            PayloadUsageRevision::Invalid
        } else {
            revision(facts.generation)
        };
        let usage = RetainedPayloadUsage {
            current_usage_bytes: facts.usage,
            height_sizes: facts.sizes.clone(),
            revision: captured,
        };
        if facts.change_during_scan {
            facts.generation += 1;
        }
        Ok(usage)
    }
    fn payload_usage_revision(&self) -> Result<PayloadUsageRevision, StorageError> {
        let mut facts = self.facts.lock().expect("facts");
        if let Some(sender) = facts.maybe_revision_started.take() {
            sender.send(()).expect("revision started");
        }
        if let Some(store) = facts.maybe_real_store.clone() {
            drop(facts);
            return store.payload_usage_revision();
        }
        Ok(if facts.invalid_revision {
            PayloadUsageRevision::Invalid
        } else {
            revision(facts.generation)
        })
    }
    fn load_prune_locks(&self) -> Result<Vec<PruneLockInfo>, StorageError> {
        Ok(self.facts.lock().expect("facts").locks.clone())
    }
    fn sync_prune_locks(&self, locks: &[PruneLockInfo]) -> Result<(), StorageError> {
        let mut facts = self.facts.lock().expect("facts");
        facts.lock_writes += 1;
        facts.locks = locks.to_vec();
        Ok(())
    }
    fn commit_paired_unlink(
        &mut self,
        height: u32,
        _hash: BlockHash,
    ) -> Result<PairedDeleteOutcome, StorageError> {
        let mut facts = self.facts.lock().expect("facts");
        let Some(size) = facts.sizes.remove(&height) else {
            return Ok(PairedDeleteOutcome::AlreadyAbsent);
        };
        facts.deletes.push(height);
        facts.usage -= size;
        facts.generation += 1;
        Ok(PairedDeleteOutcome::DeletedLiveMate)
    }
}

impl ChainstateStore for TestStore {
    fn load_snapshot(&self) -> Option<ChainstateSnapshot> {
        self.memory.load_snapshot()
    }
    fn save_snapshot(&mut self, snapshot: ChainstateSnapshot) {
        self.memory.save_snapshot(snapshot);
    }
    fn get_coin(&self, outpoint: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.memory.get_coin(outpoint)
    }
    fn have_coin(&self, outpoint: &OutPoint) -> Result<bool, ChainstateError> {
        self.memory.have_coin(outpoint)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.memory.best_block()
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.memory.head_blocks()
    }
    fn batch_write(
        &mut self,
        writes: CoinsBatch,
        maybe_best: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.memory.batch_write(writes, maybe_best)
    }
    fn load_undo(&self, hash: BlockHash) -> Result<Option<BlockUndo>, ChainstateError> {
        self.memory.load_undo(hash)
    }
    fn save_undo(&mut self, hash: BlockHash, undo: BlockUndo) -> Result<(), ChainstateError> {
        self.memory.save_undo(hash, undo)
    }
}

pub(super) fn position(height: u32) -> ChainPosition {
    ChainPosition::new(
        BlockHeader {
            version: 1,
            previous_block_hash: BlockHash::from_byte_array([0; 32]),
            merkle_root: open_bitcoin_core::primitives::MerkleRoot::from_byte_array([0; 32]),
            time: 1_700_000_000,
            bits: 0x207fffff,
            nonce: height,
        },
        height,
        u128::from(height),
        1_700_000_000,
    )
}

pub(super) fn fixture(
    mode: PruneMode,
    usage: u64,
) -> (ManagedNetworkHandle<TestStore>, Arc<Mutex<Facts>>) {
    let chain = vec![
        position(10),
        position(712),
        position(713),
        position(714),
        position(1_001),
    ];
    let snapshot = ChainstateSnapshot::new(chain, HashMap::new(), HashMap::new());
    let facts = Arc::new(Mutex::new(Facts {
        usage,
        sizes: BTreeMap::from([(10, 100), (712, 100), (713, 100), (714, 100)]),
        ..Default::default()
    }));
    let store = TestStore {
        memory: MemoryChainstateStore::from_snapshot(snapshot),
        facts: Arc::clone(&facts),
    };
    let network = ManagedPeerNetwork::new(store, Default::default(), Default::default());
    let mut handle = ManagedNetworkHandle::from_network_fixture(network);
    handle.set_prune_mode(mode).expect("mode");
    handle
        .set_prune_network(crate::SyncNetwork::Regtest)
        .expect("prune network");
    handle
        .set_coins_next_write(FlushPolicyTime::new(10_000))
        .expect("next write");
    (handle, facts)
}

pub(super) fn tick(
    handle: &ManagedNetworkHandle<TestStore>,
    mode: FlushMode,
    seconds: u64,
) -> Result<crate::chainstate::FlushExecution, crate::ManagedNetworkAuthorityError> {
    handle.flush_coins(mode, FlushPolicyTime::new(seconds), u64::MAX)
}

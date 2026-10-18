// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Continuous consensus-staged history with historical and same-block spends.

use super::*;
use crate::tests::fixture_runtime::{
    fixture_open_authoritative_network_runtime, fixture_open_runtime_store,
};
use open_bitcoin_node::core::{
    chainstate::{
        Chainstate, ChainstateError, ChainstateSnapshot, CoinsView, FlushMode, FlushPolicyTime,
    },
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, chain_proof::ProofWork,
        check_block_header, transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_node::{FjallChainstateStore, ManagedChainstate, chainstate::FlushLifecycle};

pub(crate) struct History {
    pub path: std::path::PathBuf,
    pub blocks: Vec<Block>,
    pub snapshot: ChainstateSnapshot,
}

static NEXT_FIXTURE: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl History {
    pub fn new(count: u32) -> Self {
        let id = NEXT_FIXTURE.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = crate::tests::temp_store_path(&format!("phase157-daemon-{id}"));
        std::fs::create_dir(&path).expect("reserve unique fixture");
        let mut state = Chainstate::default();
        let mut blocks: Vec<Block> = Vec::new();
        for height in 0..count {
            let maybe_previous = blocks.last();
            let block = next_block(maybe_previous, height);
            let staged = state
                .stage_connect_block_with_current_time(
                    &block,
                    open_bitcoin_node::core::chainstate::ChainPosition::untrusted_fixture_work(
                        u128::from(height) + 1,
                    ),
                    i64::from(block.header.time) + 1,
                    ScriptVerifyFlags::P2SH,
                    params(),
                )
                .expect("genuine continuous consensus stage");
            if height > 1 {
                assert_eq!(staged.undo().transactions.len(), 2);
                assert_eq!(
                    staged
                        .basic_filter_inputs(&block)
                        .expect("complete facts")
                        .spent_scripts()
                        .count(),
                    2
                );
            }
            state
                .commit_staged_connect(staged)
                .expect("validated commit");
            blocks.push(block);
        }
        Self {
            path,
            blocks,
            snapshot: state.snapshot(),
        }
    }

    pub fn seed(&self) -> FjallNodeStore {
        self.seed_with_missing(None, None)
    }

    pub fn seed_with_missing(
        &self,
        maybe_body: Option<u32>,
        maybe_undo: Option<u32>,
    ) -> FjallNodeStore {
        let store = FjallNodeStore::open(&self.path).expect("real store");
        for (height, block) in self.blocks.iter().enumerate() {
            if maybe_body == Some(height as u32) {
                continue;
            }
            store
                .save_block(block, open_bitcoin_node::PersistMode::Sync)
                .expect("body");
        }
        let mut snapshot = self.snapshot.clone();
        if let Some(height) = maybe_undo {
            snapshot
                .undo_by_block
                .remove(&self.snapshot.active_chain[height as usize].block_hash);
        }
        store
            .seed_coins_from_snapshot(&snapshot)
            .expect("real coins and metadata");
        store
    }

    pub fn open_genuine(
        &self,
        config: &RuntimeConfig,
    ) -> (
        FjallNodeStore,
        crate::AuthoritativeNetworkRuntime<
            open_bitcoin_node::FjallChainstateStore,
            open_bitcoin_node::FjallCoinsView,
        >,
    ) {
        let store = FjallNodeStore::open(&self.path).expect("fresh complete fixture store");
        let runtime = genuine_runtime(store.clone(), &self.blocks, config);
        let opened = crate::assemble_durable_runtime(config, runtime)
            .expect("same genuine configured owner")
            .expect_durable();
        (store, opened)
    }

    pub fn config(&self, maybe_option: Option<&str>) -> RuntimeConfig {
        let mut args = vec![std::ffi::OsString::from(format!(
            "-datadir={}",
            self.path.display()
        ))];
        if let Some(option) = maybe_option {
            args.push(option.into());
        }
        open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &self.path)
            .expect("actual config parser")
    }
}

impl Drop for History {
    fn drop(&mut self) {
        crate::tests::remove_dir_if_exists(&self.path);
    }
}

pub(crate) fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    }
}

pub(crate) fn next_block(maybe_previous: Option<&Block>, height: u32) -> Block {
    let mut number = height.to_le_bytes().to_vec();
    while number.last() == Some(&0) {
        number.pop();
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut script = vec![number.len() as u8];
    script.extend(number);
    script.push(0x51);
    let reward = Transaction {
        version: 1,
        inputs: vec![TransactionInput {
            previous_output: OutPoint::null(),
            script_sig: ScriptBuf::from_bytes(script).expect("height"),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(5_000_000_000).expect("reward"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
        lock_time: 0,
    };
    let mut transactions = vec![reward];
    if height > 1 {
        let previous = maybe_previous.expect("funding block");
        let first = spend(&previous.transactions[0], 4_999_999_000);
        let second = spend(&first, 4_999_998_000);
        transactions.extend([first, second]);
    }
    let mut block = Block {
        header: BlockHeader {
            version: 1,
            previous_block_hash: maybe_previous
                .map_or(BlockHash::default(), |block| block_hash(&block.header)),
            merkle_root: block_merkle_root(&transactions).expect("merkle").0,
            time: 1_000 + height * 100,
            bits: 0x207f_ffff,
            nonce: 0,
        },
        transactions,
    };
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy proof of work");
    block
}

fn spend(previous: &Transaction, value: i64) -> Transaction {
    Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: transaction_txid(previous).expect("txid"),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(value).expect("amount"),
            script_pubkey: ScriptBuf::from_bytes(vec![0x51]).expect("true"),
        }],
        lock_time: 0,
    }
}

/// Real accepted history; the older raw-seeded History remains a legacy control.
pub(crate) struct DaemonFixture {
    pub path: std::path::PathBuf,
    pub blocks: Vec<Block>,
    pub config: RuntimeConfig,
    pub store: FjallNodeStore,
    pub opened: crate::AuthoritativeNetworkRuntime<
        open_bitcoin_node::FjallChainstateStore,
        open_bitcoin_node::FjallCoinsView,
    >,
    pub state: open_bitcoin_rpc::http::RpcHttpState<
        open_bitcoin_node::FjallChainstateStore,
        open_bitcoin_node::FjallCoinsView,
    >,
    pub shared: Arc<
        tokio::sync::Mutex<
            ManagedRpcContext<
                open_bitcoin_node::FjallChainstateStore,
                open_bitcoin_node::FjallCoinsView,
            >,
        >,
    >,
}

impl DaemonFixture {
    pub fn new(count: u32) -> Self {
        let id = NEXT_FIXTURE
            .fetch_update(
                std::sync::atomic::Ordering::Relaxed,
                std::sync::atomic::Ordering::Relaxed,
                |id| id.checked_add(1),
            )
            .expect("checked unique fixture ID");
        let path = crate::tests::temp_store_path(&format!("phase159-daemon-{id}"));
        std::fs::create_dir(&path).expect("reserve unique fixture");
        let mut blocks = Vec::new();
        for height in 0..count {
            blocks.push(daemon_block(blocks.last(), height));
        }
        Self::open_policy(path, blocks, true, false, &["in"], false, true)
    }

    pub fn open(path: std::path::PathBuf, blocks: Vec<Block>, enabled: bool) -> Self {
        Self::open_policy(path, blocks, enabled, false, &["in"], false, false)
    }

    pub fn open_serving(
        path: std::path::PathBuf,
        blocks: Vec<Block>,
        enabled: bool,
        general: bool,
        permissions: &[&str],
    ) -> Self {
        Self::open_policy(path, blocks, enabled, general, permissions, true, false)
    }

    fn open_policy(
        path: std::path::PathBuf,
        blocks: Vec<Block>,
        enabled: bool,
        general: bool,
        permissions: &[&str],
        listener: bool,
        genuine: bool,
    ) -> Self {
        std::fs::write(path.join("open-bitcoin.jsonc"), "{\"prune\":1}")
            .expect("actual manual-prune config");
        let mut args = vec![
            format!("-datadir={}", path.display()).into(),
            "-regtest".into(),
            format!("-blockfilterindex={}", if enabled { "basic" } else { "0" }).into(),
            "-rpcuser=phase159".into(),
            "-rpcpassword=private-password".into(),
            format!("-peerblockfilters={}", u8::from(general)).into(),
            format!("-openbitcoininbound={}", u8::from(listener)).into(),
            "-openbitcoinlisten=127.0.0.1:0".into(),
            "-openbitcoinmaxinbound=2".into(),
            "-openbitcoinreservedslots=1".into(),
        ];
        if permissions != ["in"] {
            args.push(
                format!(
                    "-openbitcoininboundpermissionclass=evidence@127.0.0.1={}",
                    permissions.join(",")
                )
                .into(),
            );
        }
        let config = open_bitcoin_rpc::config::load_runtime_config_for_args(&args, &path)
            .expect("production config parser");
        assert!(!config.sync.is_enabled());
        assert_eq!(
            config.prune_mode,
            open_bitcoin_node::core::chainstate::PruneMode::ManualOnly
        );
        let store = fixture_open_runtime_store(&config)
            .expect("production store selection")
            .expect("configured durable store");
        let opened = if genuine {
            crate::assemble_durable_runtime(
                &config,
                genuine_runtime(store.clone(), &blocks, &config),
            )
            .expect("configured genuine fixture assembly")
            .expect_durable()
        } else {
            fixture_open_authoritative_network_runtime(&config, Some(store.clone()))
                .expect("actual raw configured observation")
                .expect_durable()
        };
        Self::from_opened(path, blocks, config, store, opened)
    }

    fn from_opened(
        path: std::path::PathBuf,
        blocks: Vec<Block>,
        config: RuntimeConfig,
        store: FjallNodeStore,
        opened: crate::AuthoritativeNetworkRuntime<
            open_bitcoin_node::FjallChainstateStore,
            open_bitcoin_node::FjallCoinsView,
        >,
    ) -> Self {
        let context = ManagedRpcContext::from_fixture_runtime_config_with_network_handle(
            &config,
            opened.network.clone(),
            Some(store.clone()),
        )
        .expect("actual daemon shared context");
        let shared = Arc::new(tokio::sync::Mutex::new(context));
        let state = open_bitcoin_rpc::http::build_http_state_with_shared_context(
            config.rpc_server.auth.clone(),
            Arc::clone(&shared),
        )
        .expect("real authenticated HTTP state");
        Self {
            path,
            blocks,
            config,
            store,
            opened,
            state,
            shared,
        }
    }

    /// Requalify a complete actual reopened prefix before any required body is pruned.
    pub fn requalify(self) -> Self {
        let config = self.config.clone();
        let (path, blocks) = self.close();
        let store = FjallNodeStore::open(&path).expect("fresh physical source after raw close");
        let saved = store
            .hydrate_chainstate_for_open()
            .expect("raw saved prefix")
            .expect("populated prefix");
        let physical = store
            .coins_view()
            .complete_snapshot()
            .expect("actual complete physical prefix");
        assert!(saved.maybe_confirmed_txid_counts.is_none());
        assert_eq!(saved.active_chain.len(), blocks.len());
        let raw = Chainstate::from_parent(
            store.coins_view(),
            saved.active_chain.clone(),
            saved.undo_by_block.clone(),
            saved.maybe_confirmed_txid_counts.clone(),
        );
        assert!(!raw.has_earned_work());
        assert_eq!(
            raw.validate_live_origin(),
            Err(ChainstateError::UntrustedWorkLineage)
        );
        drop(raw);
        let mut memory = Chainstate::new();
        for (index, position) in saved.active_chain.iter().enumerate() {
            let block = store
                .load_block(position.block_hash)
                .expect("actual retained body")
                .expect("required prefix body before prune");
            assert_eq!(block, blocks[index]);
            let receipt = memory
                .connect_block_with_current_time(
                    &block,
                    ProofWork::ZERO,
                    i64::from(block.header.time),
                    ScriptVerifyFlags::P2SH,
                    params(),
                )
                .expect("complete actual prefix validation");
            assert_eq!(&receipt, position);
            assert_eq!(
                store
                    .load_undo(position.block_hash)
                    .expect("actual retained undo"),
                memory.undo_by_block().get(&position.block_hash).cloned()
            );
        }
        let derived = memory.snapshot();
        assert_eq!(derived.active_chain, saved.active_chain);
        assert_eq!(derived.undo_by_block, saved.undo_by_block);
        assert!(derived.maybe_confirmed_txid_counts.is_some());
        assert_eq!(
            memory
                .coins()
                .complete_snapshot()
                .expect("genuine effective coins"),
            physical
        );
        let mut target = Chainstate::from_parent(
            store.coins_view(),
            derived.active_chain,
            derived.undo_by_block,
            derived.maybe_confirmed_txid_counts,
        );
        memory
            .rebind_into(&mut target)
            .expect("strict same physical prefix handoff");
        let now = FlushPolicyTime::new(0);
        let runtime = configured_source(
            ManagedChainstate::from_chainstate(
                FjallChainstateStore::from_store(store.clone()),
                target,
                FlushLifecycle::ready(now, now, 0, false),
            ),
            &config,
        )
        .expect("requalified same-config public source");
        assert_eq!(
            store
                .hydrate_chainstate_for_open()
                .expect("unchanged saved metadata"),
            Some(saved.clone())
        );
        assert_eq!(
            store
                .coins_view()
                .complete_snapshot()
                .expect("unchanged complete physical identity"),
            physical
        );
        for position in &saved.active_chain {
            let facts = runtime
                .network_handle()
                .maybe_body_eligibility(position.block_hash)
                .expect("prefix body facts")
                .expect("checked retained body");
            assert_eq!(facts.chain_work, position.chain_work);
            assert!(facts.available && !facts.failed);
            if position.height > 0 {
                assert_eq!(facts.maybe_sequence, None);
            }
        }
        let opened = crate::assemble_durable_runtime(&config, runtime)
            .expect("requalified configured assembly")
            .expect_durable();
        Self::from_opened(path, blocks, config, store, opened)
    }

    pub fn finish(&self) {
        for _ in 0..256 {
            let outcome = self
                .opened
                .network
                .drive_basic_filter_index_turn()
                .expect("bounded ordinary owner turn");
            let progress = outcome.maybe_progress.expect("BASIC owner");
            if progress.maybe_processed_endpoint().is_some_and(|endpoint| {
                endpoint.block_hash() == block_hash(&self.blocks.last().expect("tip").header)
            }) {
                return;
            }
        }
        panic!("bounded fixture owner did not reach tip");
    }

    pub fn accept_next(&mut self) -> BlockHash {
        let block = daemon_block(self.blocks.last(), self.blocks.len() as u32);
        let hash = block_hash(&block.header);
        self.opened
            .network
            .connect_local_block(&block, ScriptVerifyFlags::P2SH, params())
            .expect("genuine accepted later lag");
        self.store
            .save_block(&block, open_bitcoin_node::PersistMode::Sync)
            .expect("retained body");
        self.blocks.push(block);
        hash
    }

    /// Consume every database/authority/context clone before the caller reopens.
    pub fn close(self) -> (std::path::PathBuf, Vec<Block>) {
        let Self {
            path,
            blocks,
            config,
            store,
            opened,
            state,
            shared,
        } = self;
        drop((state, shared, opened, store, config));
        (path, blocks)
    }

    pub fn cleanup(self) {
        let (path, _) = self.close();
        std::fs::remove_dir_all(path).expect("closed fixture cleanup");
    }
}

fn genuine_runtime(
    store: FjallNodeStore,
    blocks: &[Block],
    config: &RuntimeConfig,
) -> DurableSyncRuntime {
    let mut core = Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
    Chainstate::new()
        .rebind_into(&mut core)
        .expect("genuine empty source");
    let now = FlushPolicyTime::new(0);
    let mut source = ManagedChainstate::from_chainstate(
        FjallChainstateStore::from_store(store.clone()),
        core,
        FlushLifecycle::ready(now, now, 0, false),
    );
    for block in blocks {
        source
            .connect_block(block, ProofWork::ZERO, ScriptVerifyFlags::P2SH, params())
            .expect("actual complete managed acceptance");
        store
            .save_block(block, open_bitcoin_node::PersistMode::Sync)
            .expect("retained actual body");
    }
    let (mut sink, mut core) = source.into_parts();
    let undo = core
        .undo_by_block()
        .iter()
        .map(|(hash, undo)| (*hash, undo.clone()))
        .collect::<Vec<_>>();
    let positions = core.active_chain().to_vec();
    let mut lifecycle = FlushLifecycle::ready(now, now, 0, false);
    lifecycle
        .execute_flush(
            &mut sink,
            &mut core,
            FlushMode::Always,
            FlushPolicyTime::new(1),
            u64::MAX,
            &undo,
            &[],
            blocks,
            &positions,
        )
        .expect("genuine accepted durable fence");
    configured_source(
        ManagedChainstate::from_chainstate(sink, core, lifecycle),
        config,
    )
    .expect("same genuine source and one startup turn")
}

fn configured_source(
    source: ManagedChainstate<FjallChainstateStore, open_bitcoin_node::FjallCoinsView>,
    config: &RuntimeConfig,
) -> Result<DurableSyncRuntime, open_bitcoin_node::SyncRuntimeError> {
    DurableSyncRuntime::from_live_fixture_source(
        source,
        config.sync.runtime.clone(),
        params(),
        crate::basic_filter_startup_mode(config),
        config.relay,
        config.block_serving,
        config.inbound.enabled,
    )
}

/// Three distinct always-true scripts expose historical and same-block inputs.
pub(crate) fn daemon_block(maybe_previous: Option<&Block>, height: u32) -> Block {
    let mut block = next_block(maybe_previous, height);
    block.transactions[0].outputs[0].script_pubkey =
        ScriptBuf::from_bytes(vec![0x01, height as u8, 0x75, 0x51])
            .expect("distinct historical coinbase true");
    if height > 1 {
        block.transactions[1].outputs[0].script_pubkey =
            ScriptBuf::from_bytes(vec![0x52, 0x75, 0x51]).expect("first true");
        block.transactions[2].inputs[0].previous_output.txid =
            transaction_txid(&block.transactions[1]).expect("same-block input after script change");
        block.transactions[2].outputs[0].script_pubkey =
            ScriptBuf::from_bytes(vec![0x53, 0x75, 0x51]).expect("second true");
    }
    block.header.merkle_root = block_merkle_root(&block.transactions)
        .expect("actual changed merkle")
        .0;
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("easy proof of work");
    block
}

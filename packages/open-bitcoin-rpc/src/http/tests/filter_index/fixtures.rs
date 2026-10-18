// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Real same-store acceptance fixtures, reusable by daemon integration tests.

use open_bitcoin_node::core::{
    chainstate::{Chainstate, FlushMode, FlushPolicyTime},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
        transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};
use open_bitcoin_node::{
    DurableSyncRuntime, FjallNodeStore, PersistMode, SyncNetwork, SyncRuntimeConfig,
};

pub(crate) struct History {
    pub path: std::path::PathBuf,
    pub blocks: Vec<Block>,
    pub runtime: DurableSyncRuntime,
}

static NEXT_HISTORY: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

impl History {
    pub fn new(count: u32) -> Self {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("time")
            .as_nanos();
        let id = NEXT_HISTORY.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let path = std::env::temp_dir().join(format!(
            "phase159-rpc-history-{}-{unique}-{id}",
            std::process::id()
        ));
        let store = FjallNodeStore::open(&path).expect("fresh store");
        let mut core =
            Chainstate::from_parent(store.coins_view(), vec![], Default::default(), None);
        Chainstate::new()
            .rebind_into(&mut core)
            .expect("genuine empty source");
        let now = FlushPolicyTime::new(0);
        let mut source = open_bitcoin_node::ManagedChainstate::from_chainstate(
            open_bitcoin_node::FjallChainstateStore::from_store(store.clone()),
            core,
            open_bitcoin_node::chainstate::FlushLifecycle::ready(now, now, 0, false),
        );
        let mut blocks = Vec::new();
        for height in 0..count {
            let block = next_block(blocks.last(), height);
            source
                .connect_block(
                    &block,
                    open_bitcoin_node::core::consensus::chain_proof::ProofWork::ZERO,
                    ScriptVerifyFlags::P2SH,
                    params(),
                )
                .expect("genuine same-authority acceptance");
            store
                .save_block(&block, PersistMode::Sync)
                .expect("ordinary retained body");
            blocks.push(block);
        }
        let (mut sink, mut core) = source.into_parts();
        let undo = core
            .undo_by_block()
            .iter()
            .map(|(hash, undo)| (*hash, undo.clone()))
            .collect::<Vec<_>>();
        let positions = core.active_chain().to_vec();
        let mut lifecycle =
            open_bitcoin_node::chainstate::FlushLifecycle::ready(now, now, 0, false);
        lifecycle
            .execute_flush(
                &mut sink,
                &mut core,
                FlushMode::Always,
                FlushPolicyTime::new(1),
                u64::MAX,
                &undo,
                &[],
                &blocks,
                &positions,
            )
            .expect("genuine durable fence");
        let runtime = DurableSyncRuntime::from_live_fixture_source(
            open_bitcoin_node::ManagedChainstate::from_chainstate(sink, core, lifecycle),
            SyncRuntimeConfig {
                network: SyncNetwork::Regtest,
                ..Default::default()
            },
            params(),
            open_bitcoin_node::chainstate::BasicFilterStartupMode::Enabled,
            Default::default(),
            Default::default(),
            false,
        )
        .expect("same genuine BASIC owner and initial turn");
        Self {
            path,
            blocks,
            runtime,
        }
    }

    pub fn accept_next(&mut self) -> BlockHash {
        let block = next_block(self.blocks.last(), self.blocks.len() as u32);
        let hash = block_hash(&block.header);
        self.runtime
            .network_handle()
            .connect_local_block(&block, ScriptVerifyFlags::P2SH, params())
            .expect("genuine accepted unflushed block");
        self.runtime
            .store()
            .save_block(&block, PersistMode::Sync)
            .expect("ordinary body persistence");
        self.blocks.push(block);
        hash
    }

    pub fn cleanup(self) {
        let Self { path, runtime, .. } = self;
        drop(runtime);
        std::fs::remove_dir_all(path).expect("closed fixture cleanup");
    }
}

pub(crate) fn params() -> ConsensusParams {
    ConsensusParams {
        coinbase_maturity: 1,
        ..Default::default()
    }
}

pub(crate) fn next_block(maybe_previous: Option<&Block>, height: u32) -> Block {
    let script = if height == 0 {
        vec![0, 0x51]
    } else {
        vec![1, height as u8, 0x51]
    };
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

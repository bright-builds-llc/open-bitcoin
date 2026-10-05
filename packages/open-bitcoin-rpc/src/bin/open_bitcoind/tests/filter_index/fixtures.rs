// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Continuous consensus-staged history with historical and same-block spends.

use super::*;
use open_bitcoin_node::core::{
    chainstate::{Chainstate, ChainstateSnapshot},
    consensus::{
        ConsensusParams, ScriptVerifyFlags, block_hash, block_merkle_root, check_block_header,
        transaction_txid,
    },
    primitives::{
        Amount, Block, BlockHash, BlockHeader, OutPoint, ScriptBuf, ScriptWitness, Transaction,
        TransactionInput, TransactionOutput,
    },
};

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
                    u128::from(height) + 1,
                    i64::from(block.header.time) + 1,
                    ScriptVerifyFlags::P2SH,
                    params(),
                )
                .expect("genuine continuous consensus stage");
            if height > 1 {
                assert_eq!(staged.undo.transactions.len(), 2);
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

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Continuous genuinely accepted easy-header history; maturity one is test-only.

use super::*;
use open_bitcoin_core::{
    chainstate::Chainstate,
    consensus::{ConsensusParams, ScriptVerifyFlags, transaction_txid},
};

pub(in crate::sync::tests::filter_index) struct TurnHistory {
    pub history: ValidatedHistory,
}

impl TurnHistory {
    pub fn new(prefix: usize, suffix: usize, script_len: usize) -> Self {
        Self::with_shape(prefix, suffix, script_len, false)
    }

    pub fn legal_large_singleton() -> Self {
        Self::with_shape(16, 1, 1, true)
    }

    fn with_shape(prefix: usize, suffix: usize, script_len: usize, large_reward: bool) -> Self {
        let mut history = ValidatedHistory::new("turn-continuous", false);
        history.path = reserved_filter_path("turn-continuous");
        let mut state = Chainstate::from_snapshot(history.full.clone());
        while history.blocks.len() < prefix + suffix {
            let height = history.blocks.len() as u32;
            let previous = history.blocks.last().expect("previous");
            let funding = &previous.transactions[0];
            let spend = |previous: &Transaction, value: i64, tag: u8| {
                let mut bytes = vec![0x51; script_len.max(1)];
                if script_len > 4 {
                    bytes[..4].copy_from_slice(&height.to_le_bytes());
                    // Unspent output script is data for indexing; no execute claim.
                    bytes[0] = tag;
                }
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
                        value: Amount::from_sats(value).expect("value"),
                        script_pubkey: ScriptBuf::from_bytes(bytes).expect("bounded script"),
                    }],
                    lock_time: 0,
                }
            };
            let first = spend(funding, 4_999_999_000, 0x51);
            // Keep same-block spend executable with OP_TRUE; larger scripts are
            // added to the last unspent output, preserving validated provenance.
            let mut first = first;
            first.outputs[0].script_pubkey = ScriptBuf::from_bytes(vec![0x51]).expect("true");
            let second = spend(&first, 4_999_998_000, 0x52);
            let mut block = fixture_block(block_hash(&previous.header), height);
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
            block.transactions[0].inputs[0].script_sig =
                ScriptBuf::from_bytes(script).expect("height");
            block.transactions.extend([first, second]);
            if large_reward && height as usize + 1 == prefix + suffix {
                let mut script = vec![0x61; 9_984];
                script.push(0x51);
                let output = TransactionOutput {
                    value: Amount::ZERO,
                    script_pubkey: ScriptBuf::from_bytes(script).expect("legal script"),
                };
                block.transactions[0]
                    .outputs
                    .extend(std::iter::repeat_n(output, 99));
            }
            block.header.time = 1_000 + height * 100;
            block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
            block.header.nonce = (0..=u32::MAX)
                .find(|nonce| {
                    block.header.nonce = *nonce;
                    check_block_header(&block.header).is_ok()
                })
                .expect("easy header");
            let staged = state
                .stage_connect_block_with_current_time(
                    &block,
                    u128::from(height) + 1,
                    i64::from(block.header.time) + 1,
                    ScriptVerifyFlags::P2SH,
                    ConsensusParams {
                        coinbase_maturity: 1,
                        ..ConsensusParams::default()
                    },
                )
                .expect("genuine continuous validation");
            state.commit_staged_connect(staged).expect("commit");
            let position = state.tip().expect("tip");
            let inputs = BasicFilterInputs::from_historical(
                &block,
                position,
                Some(HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo: &state.undo_by_block()[&position.block_hash],
                }),
            )
            .expect("complete history");
            history.records.push(
                StoredFilterRecord::generate(
                    &inputs,
                    position,
                    history
                        .records
                        .last()
                        .map(StoredFilterRecord::identity)
                        .as_ref(),
                )
                .expect("record"),
            );
            history.blocks.push(block);
        }
        history.full = state.snapshot();
        Self { history }
    }

    pub fn seed(&self, prefix: usize) -> FjallNodeStore {
        let store = FjallNodeStore::open(&self.history.path).expect("store");
        self.history.persist_payloads(&store);
        store
            .seed_coins_from_snapshot(&self.history.full)
            .expect("validated authority");
        store
            .initialize_basic_filter_state(&fence(&self.history.full.active_chain))
            .expect("empty");
        for records in self.history.records[..prefix].chunks(128) {
            let cp = checkpoint(records.last().expect("chunk"));
            publish_current(
                &store,
                &fence(&self.history.full.active_chain),
                cp,
                cp.input_protection(),
                records,
            )
            .expect("fixture prefix");
        }
        store
    }
}

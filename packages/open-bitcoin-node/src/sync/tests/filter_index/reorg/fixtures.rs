// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Test recipes carry bodies/expected commitments, never production authority.

use super::*;

pub(in crate::sync::tests) struct ForkFixture {
    pub history: ValidatedHistory,
    pub runtime: DurableSyncRuntime,
    standard_maturity: bool,
}

pub(in crate::sync::tests) struct ForkBranch {
    pub disconnect: Vec<Block>,
    pub connected: Vec<AnchoredBlock>,
    pub records: Vec<StoredFilterRecord>,
}

impl ForkFixture {
    pub fn compact(prefix: usize, suffix: usize, indexed: usize) -> Self {
        let fixture = TurnHistory::new(prefix, suffix, 1);
        let mut runtime = configured(fixture.seed(indexed));
        // Compact cases deliberately retain the existing maturity-one contract.
        runtime.consensus_params.coinbase_maturity = 1;
        Self {
            history: fixture.history,
            runtime,
            standard_maturity: false,
        }
    }

    pub fn standard() -> Self {
        let path = reserved_filter_path("phase158-standard-maturity");
        // Read actual configured parameters before building any recipe; no override.
        let observer = DurableSyncRuntime::open_configured(
            FjallNodeStore::open(&path).expect("empty real store"),
            sync_config(),
            BasicFilterStartupMode::PreserveSaved,
        )
        .expect("actual runtime parameter observer");
        let params = observer.consensus_params;
        let flags = observer.verify_flags;
        assert_eq!(params.coinbase_maturity, 100);
        drop(observer);
        let mut state = Chainstate::default();
        let mut blocks: Vec<Block> = Vec::new();
        let mut records = Vec::new();
        for height in 0..=104 {
            let maybe_parent = blocks.last();
            let maybe_funding = if height == 101 {
                Some(&blocks[1].transactions[0])
            } else if height > 101 {
                Some(
                    blocks
                        .last()
                        .expect("previous")
                        .transactions
                        .last()
                        .expect("final output"),
                )
            } else {
                None
            };
            let block = validated_child(maybe_parent, height, 0x51, maybe_funding);
            let staged = state
                .stage_connect_block_with_current_time(
                    &block,
                    u128::from(height) + 1,
                    i64::from(block.header.time) + 1,
                    flags,
                    params,
                )
                .expect("standard maturity genuine validation");
            state
                .commit_staged_connect(staged)
                .expect("standard commit");
            records.push(record_from_history(&state, &block, records.last()));
            blocks.push(block);
        }
        let full = state.snapshot();
        let fixture = TurnHistory {
            history: ValidatedHistory {
                path,
                blocks,
                records,
                old: full.clone(),
                full,
            },
        };
        let runtime = configured(fixture.seed(105));
        assert_eq!(runtime.consensus_params, params);
        assert_eq!(runtime.verify_flags, flags);
        Self {
            history: fixture.history,
            runtime,
            standard_maturity: true,
        }
    }

    pub fn fork(&self, ancestor: usize, count: usize, tag: u8) -> ForkBranch {
        let snapshot = self
            .runtime
            .network
            .chainstate_snapshot()
            .expect("genuine current chain");
        let state = Chainstate::from_snapshot(snapshot.clone());
        let disconnect: Vec<_> = snapshot.active_chain[ancestor + 1..]
            .iter()
            .rev()
            .map(|p| {
                self.runtime
                    .store()
                    .load_block(p.block_hash)
                    .expect("body read")
                    .expect("retained consensus body")
            })
            .collect();
        let mut parent = self
            .runtime
            .store()
            .load_block(snapshot.active_chain[ancestor].block_hash)
            .expect("ancestor body")
            .expect("retained ancestor");
        let mut connected = Vec::new();
        let mut expected_spent_scripts = Vec::new();
        for offset in 1..=count {
            let height = (ancestor + offset) as u32;
            let funding = if self.standard_maturity && height == 101 {
                &self.history.blocks[1].transactions[0]
            } else if self.standard_maturity {
                parent
                    .transactions
                    .last()
                    .expect("previous non-coinbase final output")
            } else {
                &parent.transactions[0]
            };
            let block = validated_child(Some(&parent), height, tag, Some(funding));
            expected_spent_scripts.push(vec![
                funding.outputs[0].script_pubkey.as_bytes().to_vec(),
                block.transactions[1].outputs[0]
                    .script_pubkey
                    .as_bytes()
                    .to_vec(),
            ]);
            connected.push(AnchoredBlock {
                block: block.clone(),
                chain_work: u128::from(height) + 1_000,
            });
            parent = block;
        }
        // Separate genuine staging is solely the oracle; it is dropped before reorg.
        let staged = state
            .stage_reorg(
                &disconnect,
                &connected,
                self.runtime.verify_flags,
                self.runtime.consensus_params,
            )
            .expect("oracle genuine fork stage");
        let mut previous = self.history.records[ancestor].identity();
        let mut records = Vec::new();
        for (anchored, expected_scripts) in connected.iter().zip(&expected_spent_scripts) {
            let hash = block_hash(&anchored.block.header);
            let position = staged
                .transition()
                .connected
                .iter()
                .find(|p| p.block_hash == hash)
                .expect("staged position");
            let undo = staged
                .maybe_replacement_undo(hash)
                .expect("genuine staged undo");
            let inputs = BasicFilterInputs::from_historical(
                &anchored.block,
                position,
                Some(HistoricalBlockUndo {
                    block_hash: hash,
                    undo,
                }),
            )
            .expect("complete historical and same-block scripts");
            assert_eq!(
                inputs
                    .spent_scripts()
                    .map(<[u8]>::to_vec)
                    .collect::<Vec<_>>(),
                *expected_scripts
            );
            let record = StoredFilterRecord::generate(&inputs, position, Some(&previous))
                .expect("independent expected commitment");
            previous = record.identity();
            records.push(record);
        }
        drop(staged);
        ForkBranch {
            disconnect,
            connected,
            records,
        }
    }

    pub fn apply(&self, branch: &ForkBranch) {
        // Sync stores received bodies before asking this existing reorg caller.
        for anchored in &branch.connected {
            self.runtime
                .store()
                .save_block(&anchored.block, PersistMode::Sync)
                .expect("actual retained replacement body");
        }
        self.runtime
            .network
            .reorg_to_branch(
                &branch.disconnect,
                &branch.connected,
                ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
                self.runtime.verify_flags,
                self.runtime.consensus_params,
            )
            .expect("production network reorg");
    }

    pub fn finish(&self) {
        for _ in 0..256 {
            let turn = self
                .runtime
                .network
                .drive_basic_filter_index_turn()
                .expect("ordinary scheduled turn");
            if turn.maybe_accepted_lag == Some(0) {
                let progress = turn.maybe_progress.expect("enabled owner");
                assert_eq!(progress.current_lag(), 0);
                return;
            }
        }
        panic!("bounded fixture catch-up did not finish");
    }

    pub fn flush(&self) {
        let flushed = self
            .runtime
            .network
            .flush_coins(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(50_001),
                u64::MAX,
            )
            .expect("ordinary own coins and metadata flush");
        assert!(flushed.wrote_coins);
        self.runtime
            .network
            .drive_basic_filter_index_turn()
            .expect("earned safe checkpoint promotion");
    }

    pub fn reopen(self) -> Self {
        self.reopen_inspecting(|_| {})
    }

    pub fn reopen_inspecting(self, inspect: impl FnOnce(&std::path::Path)) -> Self {
        let Self {
            history,
            runtime,
            standard_maturity,
        } = self;
        // This is the only store/manager owner; no clone/proof is retained here.
        drop(runtime);
        inspect(&history.path);
        let mut runtime =
            configured(FjallNodeStore::open(&history.path).expect("actual Fjall close/reopen"));
        if !standard_maturity {
            runtime.consensus_params.coinbase_maturity = 1;
        }
        Self {
            history,
            runtime,
            standard_maturity,
        }
    }

    pub fn cleanup(self) {
        let Self {
            history, runtime, ..
        } = self;
        drop(runtime);
        history.cleanup();
    }
}

fn configured(store: FjallNodeStore) -> DurableSyncRuntime {
    DurableSyncRuntime::open_configured(store, sync_config(), BasicFilterStartupMode::Enabled)
        .expect("configured genuine recovered owner")
}

fn record_from_history(
    state: &Chainstate,
    block: &Block,
    maybe_previous: Option<&StoredFilterRecord>,
) -> StoredFilterRecord {
    let position = state.tip().expect("validated tip");
    let maybe_history = (position.height != 0).then(|| HistoricalBlockUndo {
        block_hash: position.block_hash,
        undo: &state.undo_by_block()[&position.block_hash],
    });
    let inputs = BasicFilterInputs::from_historical(block, position, maybe_history)
        .expect("complete history");
    StoredFilterRecord::generate(
        &inputs,
        position,
        maybe_previous.map(StoredFilterRecord::identity).as_ref(),
    )
    .expect("record")
}

pub(super) fn validated_child(
    maybe_parent: Option<&Block>,
    height: u32,
    tag: u8,
    maybe_funding: Option<&Transaction>,
) -> Block {
    let parent = maybe_parent
        .map(|b| block_hash(&b.header))
        .unwrap_or_default();
    let mut block = fixture_block(parent, height);
    let mut number = height.to_le_bytes().to_vec();
    while number.last() == Some(&0) {
        number.pop();
    }
    if number.last().is_some_and(|byte| byte & 0x80 != 0) {
        number.push(0);
    }
    let mut height_script = vec![number.len() as u8];
    height_script.extend(number);
    height_script.extend([1, tag]);
    block.transactions[0].inputs[0].script_sig =
        ScriptBuf::from_bytes(height_script).expect("valid height script");
    block.transactions[0].outputs[0].script_pubkey = executable_tag(tag);
    if let Some(funding) = maybe_funding {
        let first = spend(funding, tag);
        let second = spend(&first, tag.wrapping_add(1));
        block.transactions.extend([first, second]);
    }
    block.header.time = 1_000 + height * 100 + u32::from(tag);
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    block.header.nonce = (0..=u32::MAX)
        .find(|nonce| {
            block.header.nonce = *nonce;
            check_block_header(&block.header).is_ok()
        })
        .expect("mined easy regtest POW");
    block
}

fn executable_tag(tag: u8) -> ScriptBuf {
    ScriptBuf::from_bytes(vec![1, tag, 0x75, 0x51]).expect("push tag, drop, true")
}

fn spend(previous: &Transaction, tag: u8) -> Transaction {
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
            value: Amount::from_sats(previous.outputs[0].value.to_sats() - 1_000)
                .expect("fee deducted"),
            script_pubkey: executable_tag(tag),
        }],
        lock_time: 0,
    }
}

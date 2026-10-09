// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

/// Real configured recovery and manager absorption shared by storage behavior tests.
pub(crate) struct ReorgFixture {
    path: std::path::PathBuf,
    pub(crate) store: FjallNodeStore,
    pub(crate) manager: ManagedChainstate<FjallChainstateStore, FjallCoinsView>,
    blocks: Vec<Block>,
    pub(crate) records: Vec<crate::storage::filter_index::StoredFilterRecord>,
}

impl ReorgFixture {
    pub(crate) fn budget() -> open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
        open_bitcoin_core::chainstate::filter_index::catch_up::TurnWork {
            blocks: 128,
            encoded_bytes: 40 * 1024 * 1024,
            cloned_bytes: 512 * 1024 * 1024,
            record_operations: 10_000,
            projection_operations: 10_000,
            checkpoint_operations: 1_000_000,
            ..Default::default()
        }
    }

    pub(crate) fn new(name: &str, indexed: usize) -> Self {
        let (path, store, mut manager, genesis) = managed_fixture(name);
        let mut blocks = vec![genesis];
        for height in 1..3 {
            let block = Self::child(blocks.last().expect("parent"), height, false);
            manager
                .connect_block_with_current_time(
                    &block,
                    u128::from(height) + 1,
                    i64::from(block.header.time) + 1,
                    open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                    Self::params(),
                )
                .expect("genuine spend acceptance");
            store
                .save_block(&block, PersistMode::Sync)
                .expect("retained body");
            blocks.push(block);
        }
        flush(&mut manager);
        let mut records: Vec<crate::storage::filter_index::StoredFilterRecord> = Vec::new();
        for (height, block) in blocks.iter().enumerate() {
            let position = &manager.chainstate.active_chain()[height];
            let maybe_history = manager
                .chainstate
                .undo_by_block()
                .get(&position.block_hash)
                .map(|undo| open_bitcoin_core::chainstate::HistoricalBlockUndo {
                    block_hash: position.block_hash,
                    undo,
                });
            let inputs = open_bitcoin_core::chainstate::BasicFilterInputs::from_historical(
                block,
                position,
                (height != 0).then_some(maybe_history).flatten(),
            )
            .expect("complete spent history");
            let record = crate::storage::filter_index::StoredFilterRecord::generate(
                &inputs,
                position,
                records.last().map(|r| r.identity()).as_ref(),
            )
            .expect("filter");
            records.push(record);
        }
        let proof = store
            .maybe_basic_filter_append_proof_with_budget(Self::budget())
            .expect("proof")
            .expect("live");
        let append = store
            .prepare_basic_filter_append(proof, &records[..indexed])
            .expect("index old branch");
        store
            .complete_basic_filter_append(append)
            .expect("publish old branch");
        Self {
            path,
            store,
            manager,
            blocks,
            records,
        }
    }

    fn params() -> open_bitcoin_core::consensus::ConsensusParams {
        open_bitcoin_core::consensus::ConsensusParams {
            coinbase_maturity: 1,
            ..Default::default()
        }
    }

    fn child(parent: &Block, height: u32, fork: bool) -> Block {
        use open_bitcoin_core::consensus::{
            block_hash, block_merkle_root, check_block_header, transaction_txid,
        };
        use open_bitcoin_core::primitives::{
            Amount, ScriptBuf, ScriptWitness, Transaction, TransactionInput, TransactionOutput,
        };
        let mut reward = parent.transactions[0].clone();
        reward.inputs[0].script_sig =
            ScriptBuf::from_bytes(vec![1, height as u8, if fork { 0x52 } else { 0x51 }])
                .expect("height script");
        reward.outputs[0].value = Amount::from_sats(5_000_000_000).expect("reward");
        let spend = |previous: &Transaction, value| Transaction {
            version: 2,
            lock_time: 0,
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
        };
        let first = spend(&parent.transactions[0], 4_999_999_000);
        let second = spend(&first, 4_999_998_000);
        let transactions = vec![reward, first, second];
        let mut block = Block {
            header: parent.header.clone(),
            transactions,
        };
        block.header.previous_block_hash = block_hash(&parent.header);
        block.header.time = 1_000 + height * 100 + u32::from(fork);
        block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
        block.header.nonce = (0..=u32::MAX)
            .find(|nonce| {
                block.header.nonce = *nonce;
                check_block_header(&block.header).is_ok()
            })
            .expect("easy POW");
        block
    }

    pub(crate) fn stage(
        &self,
        count: usize,
    ) -> (
        open_bitcoin_core::chainstate::StagedChainstateReorg,
        Vec<crate::storage::filter_index::StoredFilterRecord>,
    ) {
        self.stage_from(0, count)
    }

    pub(crate) fn stage_from(
        &self,
        ancestor: usize,
        count: usize,
    ) -> (
        open_bitcoin_core::chainstate::StagedChainstateReorg,
        Vec<crate::storage::filter_index::StoredFilterRecord>,
    ) {
        let mut replacement = Vec::new();
        let mut parent = self.blocks[ancestor].clone();
        for height in (ancestor + 1) as u32..=(ancestor + count) as u32 {
            let block = Self::child(&parent, height, true);
            self.store
                .save_block(&block, PersistMode::Sync)
                .expect("retained replacement");
            replacement.push(open_bitcoin_core::chainstate::AnchoredBlock {
                block: block.clone(),
                chain_work: u128::from(height) + 10,
            });
            parent = block;
        }
        let disconnect = self.blocks[ancestor + 1..]
            .iter()
            .rev()
            .cloned()
            .collect::<Vec<_>>();
        let staged = self
            .manager
            .chainstate
            .stage_reorg(
                &disconnect,
                &replacement,
                open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                Self::params(),
            )
            .expect("genuine fork with spends");
        let mut records = Vec::new();
        let mut previous = self.records[ancestor].identity();
        for anchored in replacement {
            let hash = open_bitcoin_core::consensus::block_hash(&anchored.block.header);
            let position = staged
                .transition()
                .connected
                .iter()
                .find(|position| position.block_hash == hash)
                .expect("staged position");
            let undo = staged
                .maybe_replacement_undo(hash)
                .expect("genuine staged undo");
            let inputs = open_bitcoin_core::chainstate::BasicFilterInputs::from_historical(
                &anchored.block,
                position,
                Some(open_bitcoin_core::chainstate::HistoricalBlockUndo {
                    block_hash: hash,
                    undo,
                }),
            )
            .expect("replacement history");
            let record = crate::storage::filter_index::StoredFilterRecord::generate(
                &inputs,
                position,
                Some(&previous),
            )
            .expect("replacement header");
            previous = record.identity();
            records.push(record);
        }
        (staged, records)
    }

    pub(crate) fn authorize(
        &mut self,
        staged: open_bitcoin_core::chainstate::StagedChainstateReorg,
        prepared: &PreparedBasicFilterReorg,
    ) -> Result<ValidatedBasicFilterReorg, StorageError> {
        let (_, accepted) = self
            .manager
            .chainstate
            .absorb_staged_reorg_with_receipt(staged);
        self.manager
            .maybe_validated_lineage
            .as_mut()
            .expect("real recovered lineage")
            .authorize_reorg(&accepted, prepared)
    }

    pub(crate) fn publish(
        &mut self,
        staged: open_bitcoin_core::chainstate::StagedChainstateReorg,
        prepared: PreparedBasicFilterReorg,
    ) -> Result<CompletedBasicFilterReorg, StorageError> {
        let authorization = self.authorize(staged, &prepared)?;
        let achieved = self
            .store
            .complete_basic_filter_reorg(prepared, authorization)?;
        self.manager
            .maybe_validated_lineage
            .as_mut()
            .expect("tracked")
            .confirm_reorg(&achieved)?;
        Ok(achieved)
    }

    pub(crate) fn append_replacement(
        &self,
        records: &[crate::storage::filter_index::StoredFilterRecord],
    ) -> Result<crate::storage::fjall_store::filters::BasicFilterAppendOutcome, StorageError> {
        let proof = self
            .store
            .maybe_basic_filter_append_proof_with_budget(Self::budget())?
            .expect("live");
        let positions = self
            .manager
            .authorize_basic_filter_append_positions(&proof, records)?;
        let prepared = self
            .store
            .prepare_basic_filter_replacement_append(proof, positions, records)?;
        self.store.complete_basic_filter_append(prepared)
    }

    pub(crate) fn flush(&mut self) {
        flush(&mut self.manager);
    }
    pub(crate) fn path(&self) -> &std::path::Path {
        &self.path
    }

    pub(crate) fn reopen(self) -> Self {
        let Self {
            path,
            store,
            manager,
            blocks,
            records,
        } = self;
        drop(manager);
        drop(store);
        let store = FjallNodeStore::open(&path).expect("actual Fjall reopen");
        let now = FlushPolicyTime::from_unix_seconds(0);
        let (lifecycle, _, cache) = super::super::super::initialize_configured(
            &store,
            now,
            now,
            0,
            false,
            u64::MAX,
            super::super::super::BasicFilterStartupMode::Enabled,
        )
        .expect("configured recovered authority");
        let (positions, counts) = store.load_chain_meta_for_open().expect("metadata");
        let state = Chainstate::from_coins_cache(
            cache,
            positions,
            store.load_all_undo_records().expect("undo"),
            counts,
        );
        let manager = ManagedChainstate::from_recovered_chainstate(
            FjallChainstateStore::from_store(store.clone()),
            state,
            lifecycle,
        )
        .expect("genuine recovered manager");
        Self {
            path,
            store,
            manager,
            blocks,
            records,
        }
    }
}

#[test]
fn phase158_storage_reorg_recovered_old_coins_rebinds_hidden_replacement_projection() {
    // Arrange
    let mut fixture = ReorgFixture::new("recovery-rebind-old", 3);
    let path = fixture.path().to_owned();
    let old = fixture.records.clone();
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    fixture
        .append_replacement(&replacement[..1])
        .expect("partial replacement");
    drop(proof);
    let fixture = fixture.reopen();
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    assert!(
        fixture
            .store
            .prepare_basic_filter_append(proof, &old[1..])
            .is_err()
    );
    // Act
    fixture
        .append_replacement(&old[1..])
        .expect("authenticated recovered projection rebind");
    // Assert
    assert_eq!(
        fixture.store.maybe_basic_filter_checkpoint().expect("safe"),
        Some(open_bitcoin_core::chainstate::FilterCheckpoint::new(
            open_bitcoin_core::chainstate::IndexPrefix::Committed(old[2].identity())
        ))
    );
    assert_eq!(
        fixture
            .store
            .load_basic_filter_record(replacement[0].identity().block_hash())
            .expect("retained fork"),
        Some(replacement[0].clone())
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_recovered_replacement_coins_rebinds_partial_suffix() {
    // Arrange
    let mut fixture = ReorgFixture::new("recovery-rebind-new", 3);
    let path = fixture.path().to_owned();
    let old = fixture.records.clone();
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    fixture
        .append_replacement(&replacement[..1])
        .expect("partial replacement");
    fixture.flush();
    drop(proof);
    let fixture = fixture.reopen();
    assert!(fixture.append_replacement(&old[1..]).is_err());
    let raw = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    assert!(
        fixture
            .store
            .prepare_basic_filter_append(raw, &old[1..1 + 1])
            .is_err()
    );
    // Act
    fixture
        .append_replacement(&replacement)
        .expect("authenticated recovered partial suffix");
    // Assert
    assert_eq!(
        fixture.store.maybe_basic_filter_checkpoint().expect("safe"),
        Some(open_bitcoin_core::chainstate::FilterCheckpoint::new(
            open_bitcoin_core::chainstate::IndexPrefix::Committed(replacement[1].identity())
        ))
    );
    for record in &old {
        assert_eq!(
            fixture
                .store
                .load_basic_filter_record(record.identity().block_hash())
                .expect("retained old"),
            Some(record.clone())
        );
    }
    let fixture = fixture.reopen();
    assert_eq!(
        fixture
            .store
            .maybe_active_basic_filter_record(2)
            .expect("active after second reopen"),
        Some(replacement[1].clone())
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_recovered_rewind_refuses_same_projection_old_branch_replay() {
    // Arrange
    let mut fixture = ReorgFixture::new("recovered-stale-same-projection", 3);
    let path = fixture.path().to_owned();
    let old = fixture.records.clone();
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    fixture.flush();
    drop(proof);
    let fixture = fixture.reopen();
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("live");
    assert_eq!(
        fixture
            .store
            .basic_filter_projection(1)
            .expect("retained physical old row"),
        old[1].identity().block_hash()
    );
    // Act
    let raw = fixture.store.prepare_basic_filter_append(proof, &old[1..2]);
    // Assert
    assert!(
        raw.is_err(),
        "a same-projection stale branch must require accepted-position authority"
    );
    assert!(fixture.append_replacement(&old[1..2]).is_err());
    fixture
        .append_replacement(&replacement)
        .expect("genuine recovered replacement");
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_storage_reorg_sealed_positions_refuse_matching_hash_with_wrong_parent() {
    // Arrange
    use crate::storage::filter_index as codec;
    let mut fixture = ReorgFixture::new("replacement-wrong-parent", 3);
    let path = fixture.path().to_owned();
    let (staged, replacement) = fixture.stage(2);
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof()
        .expect("proof")
        .expect("live");
    let prepared = fixture
        .store
        .prepare_basic_filter_reorg(&proof, &staged)
        .expect("prepare");
    fixture.publish(staged, prepared).expect("rewind");
    fixture
        .append_replacement(&replacement[..1])
        .expect("first row");
    let old_parent = fixture.records[1].identity();
    let mut bytes = codec::encode_record(&replacement[1]);
    bytes[38..70].copy_from_slice(old_parent.block_hash().as_bytes());
    bytes[70..102].copy_from_slice(old_parent.filter_header().as_bytes());
    let header = open_bitcoin_core::consensus::compute_filter_header(
        replacement[1].identity().filter_hash(),
        old_parent.filter_header(),
    );
    bytes[134..166].copy_from_slice(header.as_bytes());
    let wrong = codec::decode_record(
        &codec::record_key(replacement[1].identity().block_hash()),
        &bytes,
        Some(&old_parent),
    )
    .expect("self-consistent wrong ancestry with matching block hash");
    // Act
    let result = fixture.append_replacement(&[wrong]);
    // Assert
    assert!(
        result
            .expect_err("wrong accepted parent")
            .to_string()
            .contains("accepted position")
    );
    assert_eq!(
        fixture
            .store
            .basic_filter_projection(2)
            .expect("old row unchanged"),
        fixture.records[2].identity().block_hash()
    );
    drop(proof);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

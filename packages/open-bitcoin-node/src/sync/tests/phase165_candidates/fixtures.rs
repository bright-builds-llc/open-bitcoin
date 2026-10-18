// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/net_processing.cpp

use super::*;
use open_bitcoin_core::consensus::chain_proof::ProofWork;

#[test]
fn phase165_candidates_raw_unproved_header_never_schedules_a_body() {
    // Arrange
    let path = temp_store_path("165-unproved-request");
    let root = build_block(BlockHash::default(), 0);
    let hash = block_hash(&root.header);
    {
        let store = FjallNodeStore::open(&path).expect("raw fixture store");
        store
            .save_header_entries(
                &[HeaderEntry {
                    block_hash: hash,
                    header: root.header.clone(),
                    height: 0,
                    chain_work: ProofWork::from_le_bytes([255; 32]),
                }],
                PersistMode::Sync,
            )
            .expect("raw untrusted header bytes");
    }
    let mut runtime = DurableSyncRuntime::open(
        FjallNodeStore::open(&path).expect("raw reopen"),
        sync_config(),
    )
    .expect("runtime");
    connect_runtime_peer(&mut runtime, 165, 1);
    // Act
    let raw = crate::sync::block_reconcile::request_missing_blocks(&mut runtime, 165)
        .expect("raw-header request turn");
    // Assert
    assert!(getdata_block_hashes(&raw).is_empty());
    assert!(runtime.inflight_blocks.is_empty());
    super::super::support_runtime::assert_fixture_work_unearned(&runtime);
    // Act: actual checked header admission changes protocol facts only.
    super::super::support_runtime::admit_fixture_runtime_headers_only(&runtime);
    let checked = crate::sync::block_reconcile::request_missing_blocks(&mut runtime, 165)
        .expect("checked-header request turn");
    // Assert
    assert_eq!(getdata_block_hashes(&checked), vec![hash]);
    super::super::support_runtime::assert_fixture_work_unearned(&runtime);
    drop(runtime);
    fs::remove_dir_all(path).expect("cleanup");
}

pub(super) fn missing_coin_body(mut block: Block) -> Block {
    block.transactions.push(Transaction {
        version: 2,
        inputs: vec![TransactionInput {
            previous_output: OutPoint {
                txid: open_bitcoin_core::primitives::Txid::from_byte_array([0x77; 32]),
                vout: 0,
            },
            script_sig: ScriptBuf::default(),
            sequence: u32::MAX,
            witness: ScriptWitness::default(),
        }],
        outputs: vec![TransactionOutput {
            value: Amount::from_sats(1).expect("amount"),
            script_pubkey: script(&[0x51]),
        }],
        lock_time: 0,
    });
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    mine_header(&mut block);
    block
}

#[test]
fn phase165_candidates_early_local_coins_corruption_never_becomes_peer_invalidity() {
    // Arrange
    let mut fixture = CandidateFixture::new("165-fatal-after-prefix");
    let mut branch = fixture.branch(3, 20);
    branch[1] = missing_coin_body(branch[1].clone());
    branch[2].header.previous_block_hash = block_hash(&branch[1].header);
    mine_header(&mut branch[2]);
    let sibling = build_branch_block(block_hash(&branch[0].header), 2, 30);
    fixture.headers(&branch);
    fixture.headers(std::slice::from_ref(&sibling));
    for block in branch.iter().chain(std::iter::once(&sibling)) {
        fixture.cache(block);
    }
    let outpoint = branch[1].transactions[1].inputs[0].previous_output.clone();
    crate::storage::coins_view::FjallCoinsView::from_store(fixture.runtime.store())
        .write_raw_bytes(
            &crate::storage::coins_codec::encode_coin_key(&outpoint),
            vec![0],
        )
        .expect("actual malformed physical coin read, not a missing peer input");
    let captured = durable_tip_capture(&mut fixture.runtime);
    // Act
    let result = crate::sync::block_reconcile::reconcile_best_chain_for_live_session(
        &mut fixture.runtime,
        2_000_000_000,
    );
    // Assert
    assert!(
        matches!(result, Err(SyncRuntimeError::Network { .. })),
        "{result:?}"
    );
    assert_eq!(fixture.tip(), block_hash(&fixture.root.header));
    assert!(fixture.runtime.maybe_pending_durable_tip.is_none());
    assert!(captured.lock().expect("capture").is_empty());
    assert!(
        !fixture
            .runtime
            .network
            .maybe_body_eligibility(block_hash(&branch[1].header))
            .expect("read")
            .expect("checked actual body")
            .failed
    );
    fixture.cleanup();
}

pub(super) struct CandidateFixture {
    pub runtime: DurableSyncRuntime,
    pub root: Block,
    path: PathBuf,
}

impl CandidateFixture {
    pub fn new(name: &str) -> Self {
        Self::with_params(
            name,
            open_bitcoin_core::consensus::ConsensusParams::default(),
        )
    }

    pub fn with_params(name: &str, params: open_bitcoin_core::consensus::ConsensusParams) -> Self {
        let path = temp_store_path(name);
        let store = FjallNodeStore::open(&path).expect("real empty store");
        let mut runtime = DurableSyncRuntime::open(store, sync_config()).expect("runtime");
        runtime.consensus_params = params;
        let root = build_block(BlockHash::default(), 0);
        super::super::support_runtime::initialize_fixture_runtime_genesis(&runtime, &root);
        connect_runtime_peer(&mut runtime, 165, 100);
        Self {
            runtime,
            root,
            path,
        }
    }

    pub fn branch(&self, len: u32, offset: u32) -> Vec<Block> {
        let mut parent = block_hash(&self.root.header);
        (1..=len)
            .map(|height| {
                let block = build_branch_block(parent, height, offset);
                parent = block_hash(&block.header);
                block
            })
            .collect()
    }

    pub fn headers(&self, blocks: &[Block]) {
        self.runtime
            .network
            .receive_sync_message(
                165,
                WireNetworkMessage::Headers(HeadersMessage {
                    headers: blocks.iter().map(|block| block.header.clone()).collect(),
                }),
                2_000_000_000,
                self.runtime.verify_flags,
                self.runtime.consensus_params,
            )
            .expect("actual selected-context header intake");
    }

    pub fn body(&self, block: &Block) {
        self.runtime
            .store
            .save_block(block, PersistMode::Sync)
            .expect("durable received bytes");
        self.runtime
            .network
            .connect_stored_block(
                block,
                ProofWork::from_le_bytes([255; 32]),
                2_000_000_000,
                self.runtime.verify_flags,
                self.runtime.consensus_params,
            )
            .expect("actual body intake; supplied numeric work grants nothing");
    }

    pub fn reconcile(&mut self) -> SyncReconcileProgress {
        crate::sync::block_reconcile::reconcile_and_persist_best_chain(
            &mut self.runtime,
            2_000_000_000,
        )
        .expect("actual reconciliation")
    }

    pub fn cache(&self, block: &Block) {
        self.runtime
            .store
            .save_block(block, PersistMode::Sync)
            .expect("raw cached bytes only");
    }

    pub fn tip(&self) -> BlockHash {
        self.runtime
            .network
            .maybe_chain_tip()
            .expect("read")
            .expect("tip")
            .block_hash
    }

    pub fn cleanup(self) {
        drop(self.runtime);
        fs::remove_dir_all(self.path).expect("cleanup");
    }
}

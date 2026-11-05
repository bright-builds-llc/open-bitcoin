// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.h

use super::*;
use crate::coins::{CoinsBatch, CoinsView, CompleteCoinsSnapshot};
use open_bitcoin_consensus::CanonicalChain;
use open_bitcoin_consensus::chain_proof::{ProofWork, block_proof};
use open_bitcoin_consensus::chain_spec::CanonicalGenesisProof;
use std::cell::Cell;

pub(super) struct SecondReadFault {
    inner: MemoryCoinsView,
    target: OutPoint,
    reads: Cell<usize>,
}

impl SecondReadFault {
    pub(super) fn new(inner: MemoryCoinsView, target: OutPoint) -> Self {
        Self {
            inner,
            target,
            reads: Cell::new(0),
        }
    }

    pub(super) fn reads(&self) -> usize {
        self.reads.get()
    }
}

impl CoinsView for SecondReadFault {
    fn complete_snapshot(&self) -> Result<CompleteCoinsSnapshot, ChainstateError> {
        self.inner.complete_snapshot()
    }

    fn get_coin(&self, key: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        if *key == self.target {
            let reads = self.reads.get() + 1;
            self.reads.set(reads);
            if reads == 2 {
                return Err(ChainstateError::CoinsStorage {
                    detail: "controlled second coin read".to_owned(),
                });
            }
        }
        self.inner.get_coin(key)
    }

    fn have_coin(&self, key: &OutPoint) -> Result<bool, ChainstateError> {
        self.inner.have_coin(key)
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
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.inner.batch_write(writes, maybe_best_block)
    }
}

pub(super) fn coinbase_transaction(height: u32, value: i64) -> Transaction {
    let mut transaction = super::coinbase_transaction(height, value);
    if (1..=16).contains(&height) {
        transaction.inputs[0].script_sig = script(&[0x50 + height as u8, 0x51]);
    }
    transaction
}

pub(super) fn canonical() -> Chainstate {
    let spec = CanonicalChain::Regtest.spec();
    let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proof");
    Chainstate::from_canonical_genesis(
        &proof,
        CoinsCache::from_parent(MemoryCoinsView::from_coins(
            HashMap::new(),
            Some(proof.hash()),
        )),
    )
    .expect("canonical root")
}

pub(super) fn child(chain: &Chainstate, transactions: Vec<Transaction>) -> Block {
    let tip = chain.tip().expect("tip");
    let mut block = Block {
        header: BlockHeader {
            version: 4,
            previous_block_hash: tip.block_hash,
            time: tip.header.time + 600,
            bits: tip.header.bits,
            ..BlockHeader::default()
        },
        transactions,
    };
    block.header.merkle_root = block_merkle_root(&block.transactions).expect("merkle").0;
    while check_block_header(&block.header).is_err() {
        block.header.nonce += 1;
    }
    block
}

pub(super) fn work(chain: &Chainstate, block: &Block) -> ProofWork {
    chain
        .tip()
        .expect("tip")
        .chain_work
        .wrapping_add(block_proof(block.header.bits).expect("proof"))
}

pub(super) fn stage(
    chain: &Chainstate,
    block: &Block,
) -> Result<super::super::StagedChainstateConnect, ChainstateError> {
    chain.stage_connect_block_with_current_time(
        block,
        work(chain, block),
        -1,
        ScriptVerifyFlags::NONE,
        ConsensusParams::default(),
    )
}

pub(super) fn connect(chain: &mut Chainstate, block: &Block) {
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    let staged = stage(chain, block).expect("stage");
    chain.absorb_staged_connect(staged).expect("absorb");
}

#[test]
fn phase167_context_only_proof_initializes_selection_and_invalidation_removes_it() {
    // Arrange
    let earned = canonical();
    let mut synthetic = Chainstate::new();
    synthetic.active_chain = earned.active_chain.clone();
    let raw = Chainstate::from_snapshot(earned.snapshot());
    let fixture = Chainstate::from_snapshot_for_mechanical_fixture(earned.snapshot());
    let header = child(&earned, vec![coinbase_transaction(1, 5_000_000_000)]).header;
    // Act / Assert
    assert_eq!(
        earned.maybe_canonical_chain(),
        Some(CanonicalChain::Regtest)
    );
    for mut unproved in [synthetic, raw, fixture] {
        assert_eq!(unproved.maybe_canonical_chain(), None);
        assert!(
            unproved
                .admit_canonical_header(&header, i64::from(header.time))
                .is_err()
        );
    }
    let mut changed = earned;
    changed.coins_mut();
    assert_eq!(changed.maybe_canonical_chain(), None);
    assert!(
        changed
            .admit_canonical_header(&header, i64::from(header.time))
            .is_err()
    );
}

#[test]
fn phase167_context_original_shell_clock_boundary_and_no_caller_override() {
    // Arrange
    let mut chain = canonical();
    let block = child(&chain, vec![coinbase_transaction(1, 5_000_000_000)]);
    let before = chain.snapshot();
    // Act / Assert
    let too_early = i64::from(block.header.time) - 7201;
    assert!(
        chain
            .admit_canonical_header(&block.header, too_early)
            .is_err()
    );
    assert!(stage(&chain, &block).is_err());
    assert_eq!(chain.snapshot(), before);
    chain
        .admit_canonical_header(&block.header, too_early + 1)
        .expect("equality is valid");
    chain
        .admit_canonical_header(&block.header, i64::MAX)
        .expect("repeat retains first now");
    let selected = chain
        .selected_candidate_context(&chain.active_chain, &block.header, &[], 0)
        .expect("held")
        .expect("canonical");
    assert_eq!(selected.1, too_early + 1);
    let staged = stage(&chain, &block).expect("body uses original clock despite current=-1");
    chain.absorb_staged_connect(staged).expect("accepted");
    assert_eq!(chain.tip().expect("tip").height, 1);
    assert!(stage(&chain, &block).is_err(), "acceptance clears receipts");
}

#[test]
fn phase167_context_clone_and_rebind_preserve_selection_but_never_receipts() {
    // Arrange
    let mut owner = canonical();
    let block = child(&owner, vec![coinbase_transaction(1, 5_000_000_000)]);
    owner
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    let staged = stage(&owner, &block).expect("owned stage");
    let mut clone = owner.clone();
    // Act / Assert
    assert_eq!(clone.maybe_canonical_chain(), Some(CanonicalChain::Regtest));
    assert!(stage(&clone, &block).is_err());
    assert!(matches!(
        clone.absorb_staged_connect(staged),
        Err(ChainstateError::StageOwnerMismatch)
    ));
    let staged = stage(&owner, &block).expect("source stage");
    let mut destination = Chainstate::from_snapshot(owner.snapshot());
    owner
        .rebind_into(&mut destination)
        .expect("exact effective snapshot");
    assert_eq!(
        destination.maybe_canonical_chain(),
        Some(CanonicalChain::Regtest)
    );
    assert!(stage(&destination, &block).is_err());
    assert!(matches!(
        destination.absorb_staged_connect(staged),
        Err(ChainstateError::StageOwnerMismatch)
    ));
    connect(&mut destination, &block);
    assert_eq!(destination.tip().expect("tip").height, 1);
}

#[test]
fn phase167_context_header_branch_work_and_generation_mismatches_are_atomic() {
    // Arrange
    let mut chain = canonical();
    let block = child(&chain, vec![coinbase_transaction(1, 5_000_000_000)]);
    let before = chain.snapshot();
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    // Act / Assert
    let mut other = block.clone();
    other.header.time += 1;
    assert!(stage(&chain, &other).is_err());
    let wrong_work = chain.stage_connect_block_with_current_time(
        &block,
        ProofWork::ZERO,
        i64::MAX,
        ScriptVerifyFlags::NONE,
        ConsensusParams::default(),
    );
    assert!(wrong_work.is_err());
    let staged = stage(&chain, &block).expect("stage");
    chain.forget_undo(BlockHash::default());
    assert!(matches!(
        chain.absorb_staged_connect(staged),
        Err(ChainstateError::StaleStage)
    ));
    assert!(stage(&chain, &block).is_err());
    let mut wrong_parent = block.header.clone();
    wrong_parent.previous_block_hash = BlockHash::default();
    assert!(
        chain
            .admit_canonical_header(&wrong_parent, i64::MAX)
            .is_err()
    );
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase167_context_replacement_batch_derives_each_earned_branch_and_is_bounded() {
    // Arrange
    let mut chain = canonical();
    let old = child(&chain, vec![coinbase_transaction(1, 1)]);
    let mut replacement_owner = chain.clone();
    let first = child(&replacement_owner, vec![coinbase_transaction(1, 2)]);
    connect(&mut replacement_owner, &first);
    let second = child(&replacement_owner, vec![coinbase_transaction(2, 3)]);
    let second_work = work(&replacement_owner, &second);
    connect(&mut chain, &old);
    let before = chain.snapshot();
    let replacements = [
        AnchoredBlock {
            block: first.clone(),
            chain_work: replacement_owner.tip().expect("tip").chain_work,
        },
        AnchoredBlock {
            block: second.clone(),
            chain_work: second_work,
        },
    ];
    // Act / Assert
    assert!(
        chain
            .admit_canonical_reorg_headers(
                std::slice::from_ref(&first.header),
                &[(second.header.clone(), i64::MAX)]
            )
            .is_err()
    );
    let observations = [
        (first.header.clone(), i64::from(first.header.time)),
        (second.header.clone(), i64::from(second.header.time)),
    ];
    chain
        .admit_canonical_reorg_headers(std::slice::from_ref(&old.header), &observations)
        .expect("batch");
    assert!(
        chain
            .stage_reorg(
                &[],
                &replacements,
                ScriptVerifyFlags::NONE,
                ConsensusParams::default()
            )
            .is_err()
    );
    let staged = chain
        .stage_reorg(
            std::slice::from_ref(&old),
            &replacements,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        )
        .expect("branch-specific histories");
    assert_eq!(chain.snapshot(), before);
    chain.absorb_staged_reorg(staged).expect("accept");
    assert_eq!(
        chain.tip().expect("tip").block_hash,
        open_bitcoin_consensus::block_hash(&second.header)
    );
    assert!(
        chain
            .admit_canonical_reorg_headers(&[], &vec![(second.header.clone(), 0); 4097])
            .is_err()
    );
    assert!(
        chain
            .admit_canonical_reorg_headers(
                &[
                    second.header,
                    first.header,
                    CanonicalChain::Regtest.spec().genesis_header()
                ],
                &observations
            )
            .is_err()
    );
}

#[test]
fn phase167_context_seeded_fixture_executes_same_kernel_without_product_authority() {
    // Arrange
    let canonical = canonical();
    let block = child(&canonical, vec![coinbase_transaction(1, 5_000_000_000)]);
    let tip = canonical.tip().expect("root");
    let history = open_bitcoin_consensus::CandidateHistory::check_seeded(
        CanonicalChain::Regtest.spec().rules(),
        block.header.clone(),
        open_bitcoin_consensus::HistoryHeader::new(CanonicalChain::Regtest, 0, tip.header.clone()),
        vec![],
        None,
    )
    .expect("seeded");
    let mut fixture = Chainstate::from_snapshot_for_mechanical_fixture(canonical.snapshot());
    let before = fixture.snapshot();
    // Act / Assert
    assert!(
        canonical
            .stage_seeded_historical_block(&block, &history, i64::MAX)
            .is_err()
    );
    let mut altered = block.clone();
    altered.transactions[0].outputs[0].value = Amount::from_sats(1).expect("value");
    assert!(
        fixture
            .stage_seeded_historical_block(&altered, &history, i64::MAX)
            .is_err()
    );
    assert_eq!(fixture.snapshot(), before);
    let staged = fixture
        .stage_seeded_historical_block(&block, &history, i64::from(block.header.time))
        .expect("conditional stage");
    assert!(!staged.has_earned_work());
    fixture
        .absorb_staged_connect(staged)
        .expect("fixture accept");
    assert_eq!(fixture.maybe_canonical_chain(), None);
    assert!(!fixture.has_earned_work());
    assert!(fixture.validate_live_origin().is_err());
}

#[test]
fn phase167_context_preview_suspends_selection_and_only_original_stage_can_continue() {
    // Arrange
    let mut chain = canonical();
    let block = child(&chain, vec![coinbase_transaction(1, 1)]);
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("admit");
    let replacement = [AnchoredBlock {
        block: block.clone(),
        chain_work: work(&chain, &block),
    }];
    let staged = chain
        .stage_reorg(
            &[],
            &replacement,
            ScriptVerifyFlags::NONE,
            ConsensusParams::default(),
        )
        .expect("stage");
    // Act
    chain
        .install_staged_reorg_preview(&staged)
        .expect("preview");
    let mut clone = chain.clone();
    // Assert
    assert_eq!(chain.maybe_canonical_chain(), None);
    assert_eq!(clone.maybe_canonical_chain(), None);
    assert!(
        clone
            .admit_canonical_header(&block.header, i64::MAX)
            .is_err()
    );
    assert!(
        chain
            .admit_canonical_header(&block.header, i64::MAX)
            .is_err()
    );
    assert!(clone.check_staged_reorg(&staged).is_err());
    chain
        .absorb_staged_reorg(staged)
        .expect("single original preview continuation");
    assert_eq!(chain.maybe_canonical_chain(), Some(CanonicalChain::Regtest));
    assert!(stage(&chain, &block).is_err());
}

#[test]
fn phase167_context_missing_mtp_window_and_unsupported_networks_refuse() {
    // Arrange
    let mut chain = canonical();
    for height in 1..=10 {
        let block = child(&chain, vec![coinbase_transaction(height, 1)]);
        connect(&mut chain, &block);
    }
    let block = child(&chain, vec![coinbase_transaction(11, 1)]);
    chain.active_chain.remove(0);
    let before = chain.snapshot();
    // Act / Assert
    assert!(
        chain
            .admit_canonical_header(&block.header, i64::MAX)
            .is_err(),
        "no shortened MTP fallback"
    );
    assert_eq!(chain.snapshot(), before);
    for selector in [
        CanonicalChain::Mainnet,
        CanonicalChain::Testnet3,
        CanonicalChain::Signet,
    ] {
        let spec = selector.spec();
        let proof = CanonicalGenesisProof::prove(spec, &spec.genesis_block()).expect("proof");
        let mut owner = Chainstate::from_canonical_genesis(
            &proof,
            CoinsCache::from_parent(MemoryCoinsView::from_coins(
                HashMap::new(),
                Some(proof.hash()),
            )),
        )
        .expect("root");
        assert_eq!(owner.maybe_canonical_chain(), Some(selector));
        assert!(
            owner
                .admit_canonical_header(&block.header, i64::MAX)
                .is_err()
        );
    }
}

#[test]
fn phase167_context_canonical_body_cannot_substitute_candidate_time_for_admission() {
    // Arrange
    let chain = canonical();
    let block = child(&chain, vec![coinbase_transaction(1, 5_000_000_000)]);
    let proof = CanonicalGenesisProof::prove(
        CanonicalChain::Regtest.spec(),
        &CanonicalChain::Regtest.spec().genesis_block(),
    )
    .expect("proof");
    let before = chain.snapshot();
    // Act
    let result = chain.stage_connect_block_with_current_time(
        &block,
        work(&chain, &block),
        i64::from(block.header.time),
        ScriptVerifyFlags::NONE,
        super::super::canonical_genesis::consensus_params(&proof),
    );
    // Assert
    assert!(
        result.is_err(),
        "canonical body needs private trusted-clock admission"
    );
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase167_context_mtp_includes_current_and_only_ten_parents() {
    // Arrange
    let rows: Vec<_> = (0..11)
        .map(|h| {
            ChainPosition::new(
                BlockHeader {
                    time: if h == 0 { 55 } else { (h - 1) * 10 },
                    ..BlockHeader::default()
                },
                h,
                ProofWork::ZERO,
                0,
            )
        })
        .collect();
    // Act
    let mtp = compute_median_time_past(&rows, Some(55));
    // Assert
    assert_eq!(mtp, 50);
}

#[test]
fn second_coin_read_fault_propagates_after_preflight_without_publishing_overlay() {
    // Arrange
    let mut chain = canonical();
    let block = child(&chain, vec![coinbase_transaction(1, 5_000_000_000)]);
    chain
        .admit_canonical_header(&block.header, i64::from(block.header.time))
        .expect("selected header");
    let (history, now) = chain
        .selected_candidate_context(&chain.active_chain, &block.header, &[], 0)
        .expect("selected context")
        .expect("canonical owner");
    let context = history.block_validation_context(now);
    let target = OutPoint {
        txid: open_bitcoin_consensus::transaction_txid(&block.transactions[0]).expect("txid"),
        vout: 0,
    };
    let parent = SecondReadFault::new(
        MemoryCoinsView::from_coins(HashMap::new(), Some(chain.tip().expect("tip").block_hash)),
        target.clone(),
    );
    let mut overlay = CoinsOverlay::new();
    let before = chain.snapshot();

    // Act
    let error = super::super::apply::apply_connect_transactions(
        &mut overlay,
        &parent,
        &block,
        ScriptVerifyFlags::NONE,
        &context,
        Some(&history),
        &chain.active_chain,
    )
    .expect_err("second target read must propagate storage failure");

    // Assert
    assert!(matches!(error, ChainstateError::CoinsStorage { detail }
        if detail == "controlled second coin read"));
    assert_eq!(parent.reads(), 2);
    assert_eq!(chain.snapshot(), before);
    assert_eq!(
        overlay
            .peek_coin(&parent.inner, &target)
            .expect("overlay read"),
        None
    );
}

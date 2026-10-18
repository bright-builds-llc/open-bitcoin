// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

use super::*;
use open_bitcoin_consensus::chain_proof::{ProofWork, block_proof};

fn cache_facts(
    chain: &Chainstate,
) -> (
    u64,
    u64,
    Option<BlockHash>,
    HashMap<OutPoint, Option<crate::CoinsCacheEntry>>,
) {
    (
        chain.coins().cache_entry_count(),
        chain.coins().estimated_cache_bytes(),
        chain.coins_best_block().expect("coins B"),
        chain
            .utxos()
            .keys()
            .map(|key| (key.clone(), chain.coins().cached_entry(key).cloned()))
            .collect(),
    )
}

#[test]
fn phase165_connected_work_ignores_maximum_caller_score() {
    // Arrange
    let mut chain = Chainstate::new();
    let block = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let maximum = ProofWork::from_le_bytes([255; 32]);
    // Act
    let position = chain
        .connect_block(
            &block,
            maximum,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("valid block");
    // Assert
    assert_eq!(
        position.chain_work,
        block_proof(block.header.bits).expect("proof")
    );
}

#[test]
fn phase165_cross_owner_connect_refuses_without_effects() {
    // Arrange
    let source = Chainstate::new();
    let mut destination = Chainstate::new();
    let before = destination.snapshot();
    let block = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let staged = source
        .stage_connect_block_with_current_time(
            &block,
            ProofWork::ZERO,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("valid stage");
    // Act
    let result = destination.commit_staged_connect(staged);
    // Assert
    assert!(
        result.is_err(),
        "identical visible tips do not share an owner"
    );
    assert_eq!(destination.snapshot(), before);
}

#[test]
fn phase165_raw_snapshot_cannot_earn_parent_work() {
    // Arrange
    let mut source = Chainstate::new();
    let genesis = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    connect_block(&mut source, &genesis, 1);
    let raw = Chainstate::from_snapshot(source.snapshot());
    let before = raw.snapshot();
    let child = build_block(
        before.tip().expect("tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    );
    // Act
    let result = raw.stage_connect_block_with_current_time(
        &child,
        ProofWork::ZERO,
        i64::from(child.header.time),
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(result.is_err(), "snapshot bytes do not certify work");
    assert_eq!(raw.snapshot(), before);
}

fn stage_child(chain: &Chainstate, block: &Block) -> super::super::StagedChainstateConnect {
    chain
        .stage_connect_block_with_current_time(
            block,
            ProofWork::ZERO,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("valid stage")
}

pub(super) fn live_parent() -> (Chainstate, Block) {
    let mut chain = Chainstate::new();
    let genesis = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    chain
        .connect_block(
            &genesis,
            ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("valid genesis");
    (chain, genesis)
}

pub(super) fn child(chain: &Chainstate) -> Block {
    build_block(
        chain.tip().expect("tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    )
}

#[test]
fn phase165_independent_live_clone_rejects_source_stage() {
    // Arrange
    let (source, _) = live_parent();
    let mut independent = source.clone();
    let before = independent.snapshot();
    let staged = stage_child(&source, &child(&source));
    // Act
    let result = independent.absorb_staged_connect(staged);
    // Assert
    assert_eq!(result, Err(ChainstateError::StageOwnerMismatch));
    assert_eq!(independent.snapshot(), before);
    assert!(independent.has_earned_work());
}

#[test]
fn phase165_generation_change_refuses_connect_without_mutation() {
    // Arrange
    let (mut chain, _) = live_parent();
    let staged = stage_child(&chain, &child(&chain));
    chain.coins_mut();
    let before = chain.snapshot();
    let coins_before = cache_facts(&chain);
    // Act
    let result = chain.absorb_staged_connect(staged);
    // Assert
    assert_eq!(result, Err(ChainstateError::StaleStage));
    assert_eq!(chain.snapshot(), before);
    assert_eq!(cache_facts(&chain), coins_before);
}

#[test]
fn phase165_second_prepared_connect_is_stale_after_first_acceptance() {
    // Arrange
    let (mut chain, _) = live_parent();
    let block = child(&chain);
    let first = stage_child(&chain, &block);
    let duplicate = stage_child(&chain, &block);
    let expected = chain
        .tip()
        .expect("tip")
        .chain_work
        .wrapping_add(block_proof(block.header.bits).expect("proof"));
    chain
        .validate_staged_connect(first)
        .expect("owner-bound")
        .absorb();
    let before = chain.snapshot();
    // Act
    let result = chain.commit_staged_connect(duplicate);
    // Assert
    assert_eq!(result, Err(ChainstateError::StaleStage));
    assert_eq!(chain.snapshot(), before);
    assert_eq!(chain.tip().expect("tip").chain_work, expected);
}

#[test]
fn phase165_raw_parent_and_coins_best_block_do_not_authorize_work() {
    // Arrange
    let (source, _) = live_parent();
    let snapshot = source.snapshot();
    let parent = MemoryCoinsView::from_coins(
        snapshot.utxos.clone(),
        snapshot.tip().map(|tip| tip.block_hash),
    );
    let imported = Chainstate::from_parent(
        parent.clone(),
        snapshot.active_chain.clone(),
        snapshot.undo_by_block.clone(),
        snapshot.maybe_confirmed_txid_counts.clone(),
    );
    let cached = Chainstate::from_coins_cache(
        CoinsCache::from_parent(parent),
        snapshot.active_chain,
        snapshot.undo_by_block,
        snapshot.maybe_confirmed_txid_counts,
    );
    // Act / Assert
    for raw in [imported, cached] {
        let before = raw.snapshot();
        assert!(!raw.has_earned_work());
        assert!(matches!(
            raw.stage_connect_block_with_current_time(
                &child(&raw),
                ProofWork::ZERO,
                1_231_006_600,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            ),
            Err(ChainstateError::UntrustedWorkLineage)
        ));
        assert_eq!(raw.snapshot(), before);
    }
}

pub(super) fn stage_replacement(chain: &Chainstate) -> super::super::StagedChainstateReorg {
    chain
        .stage_reorg(
            &[],
            &[AnchoredBlock {
                block: child(chain),
                chain_work: ProofWork::from_le_bytes([255; 32]),
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("validated replacement")
}

#[test]
fn phase165_foreign_reorg_refuses_preview_commit_and_absorb() {
    // Arrange
    let (source, _) = live_parent();
    // Act / Assert
    for action in 0..3 {
        let mut independent = source.clone();
        let staged = stage_replacement(&source);
        let before = independent.snapshot();
        let coins_before = cache_facts(&independent);
        let result = match action {
            0 => independent.install_staged_reorg_preview(&staged),
            1 => independent.commit_staged_reorg(staged).map(|_| ()),
            _ => independent.absorb_staged_reorg(staged).map(|_| ()),
        };
        assert_eq!(result, Err(ChainstateError::StageOwnerMismatch));
        assert_eq!(independent.snapshot(), before);
        assert_eq!(cache_facts(&independent), coins_before);
    }
}

#[test]
fn phase165_preview_is_once_and_absorption_consumes_generation() {
    // Arrange
    let (mut chain, _) = live_parent();
    let staged = stage_replacement(&chain);
    let duplicate = stage_replacement(&chain);
    chain
        .install_staged_reorg_preview(&staged)
        .expect("first preview");
    let before = chain.snapshot();
    // Act
    let duplicate_preview = chain.install_staged_reorg_preview(&staged);
    let (_, receipt) = chain
        .validate_staged_reorg(staged)
        .expect("same preview owner")
        .absorb();
    let replay = chain.absorb_staged_reorg(duplicate);
    // Assert
    assert_eq!(duplicate_preview, Err(ChainstateError::StaleStage));
    assert_eq!(replay, Err(ChainstateError::StaleStage));
    assert_eq!(chain.snapshot(), before);
    assert_eq!(
        receipt.maybe_new_endpoint(),
        chain.tip().map(|tip| (tip.height, tip.block_hash))
    );
    assert_eq!(
        chain.tip().expect("tip").chain_work,
        block_proof(EASY_BITS)
            .expect("proof")
            .wrapping_add(block_proof(EASY_BITS).expect("proof"))
    );
}

#[test]
fn phase165_preview_then_cache_mutation_refuses_absorption() {
    // Arrange
    let (mut chain, _) = live_parent();
    let staged = stage_replacement(&chain);
    chain
        .install_staged_reorg_preview(&staged)
        .expect("preview");
    chain.coins_mut();
    let before = chain.snapshot();
    // Act
    let result = chain.absorb_staged_reorg(staged);
    // Assert
    assert_eq!(result, Err(ChainstateError::StaleStage));
    assert_eq!(chain.snapshot(), before);
    assert!(!chain.has_earned_work());
}

#[test]
fn phase165_fixture_origin_remains_distinct_after_connection_and_clone() {
    // Arrange
    let (source, _) = live_parent();
    let mut fixture = Chainstate::from_snapshot_for_mechanical_fixture(source.snapshot());
    let staged = stage_child(&fixture, &child(&fixture));
    // Act
    let stage_is_earned = staged.has_earned_work();
    fixture
        .absorb_staged_connect(staged)
        .expect("explicit mechanical connection");
    let independent = fixture.clone();
    // Assert
    assert!(!stage_is_earned);
    assert!(!fixture.has_earned_work());
    assert!(!independent.has_earned_work());
}

#[test]
fn phase165_live_parent_hash_height_and_header_are_bound() {
    // Arrange
    let (source, _) = live_parent();
    let block = child(&source);
    // Act / Assert
    for alteration in 0..3 {
        let mut forged = source.clone();
        let tip = forged.active_chain.last_mut().expect("tip");
        match alteration {
            0 => tip.block_hash = BlockHash::from_byte_array([8; 32]),
            1 => tip.height = 9,
            _ => tip.header.previous_block_hash = BlockHash::from_byte_array([9; 32]),
        }
        let before = forged.snapshot();
        assert!(matches!(
            forged.stage_connect_block_with_current_time(
                &block,
                ProofWork::ZERO,
                i64::from(block.header.time),
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            ),
            Err(ChainstateError::UntrustedWorkLineage)
        ));
        assert_eq!(forged.snapshot(), before);
    }
}

#[test]
fn phase165_selected_parameters_cannot_change_after_live_acceptance() {
    // Arrange
    let (chain, _) = live_parent();
    let params = ConsensusParams {
        pow_limit_bits: 0x205f_ffff,
        ..ConsensusParams::default()
    };
    let before = chain.snapshot();
    // Act
    let result = chain.stage_connect_block_with_current_time(
        &child(&chain),
        ProofWork::ZERO,
        1_231_006_600,
        ScriptVerifyFlags::P2SH,
        params,
    );
    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::WorkParametersMismatch)
    ));
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase165_preview_cannot_mint_live_acceptance_or_clone_authority() {
    // Arrange
    let (mut chain, _) = live_parent();
    let staged = stage_replacement(&chain);
    assert!(staged.has_earned_work());
    chain
        .install_staged_reorg_preview(&staged)
        .expect("preview");
    // Act
    let clone = chain.clone();
    let result = chain.stage_reorg(
        &[],
        &[],
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(!chain.has_earned_work());
    assert!(!clone.has_earned_work());
    assert!(matches!(result, Err(ChainstateError::StaleStage)));
    assert!(matches!(
        clone.stage_reorg(
            &[],
            &[],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default()
        ),
        Err(ChainstateError::UntrustedWorkLineage)
    ));
}

#[test]
fn phase165_forgetting_undo_invalidates_prepared_generation() {
    // Arrange
    let (mut chain, _) = live_parent();
    let staged = stage_replacement(&chain);
    chain.forget_undo(BlockHash::from_byte_array([7; 32]));
    let before = chain.snapshot();
    // Act
    let result = chain.commit_staged_reorg(staged);
    // Assert
    assert_eq!(result, Err(ChainstateError::StaleStage));
    assert_eq!(chain.snapshot(), before);
}

#[test]
fn phase165_named_parent_cache_fixtures_never_become_earned() {
    // Arrange
    let root = Chainstate::new_for_mechanical_fixture();
    let parent = MemoryCoinsView::default();
    // Act
    let imported = Chainstate::from_parent_for_mechanical_fixture(
        parent.clone(),
        Vec::new(),
        HashMap::new(),
        None,
    );
    let cached = Chainstate::from_coins_cache_for_mechanical_fixture(
        CoinsCache::from_parent(parent),
        Vec::new(),
        HashMap::new(),
        None,
    );
    // Assert
    for fixture in [root, imported, cached] {
        let stage = fixture
            .stage_reorg(
                &[],
                &[],
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default(),
            )
            .expect("mechanical staging");
        assert!(!fixture.has_earned_work());
        assert!(!stage.has_earned_work());
    }
}

#[test]
fn phase165_authority_refusals_have_specific_display_text() {
    // Arrange
    let errors = [
        ChainstateError::UntrustedWorkLineage,
        ChainstateError::WorkParametersMismatch,
        ChainstateError::StageOwnerMismatch,
        ChainstateError::StaleStage,
    ];
    // Act
    let messages = errors.map(|error| error.to_string());
    // Assert
    assert_eq!(
        messages,
        [
            "untrusted work lineage",
            "work authority parameters mismatch",
            "stage belongs to a different chainstate owner",
            "stage source tip or generation is stale"
        ]
    );
}

#[test]
fn phase165_numeric_child_derivation_refuses_invalid_compact_without_authority() {
    // Arrange
    let parent_work = ProofWork::from_le_bytes([255; 32]);
    // Act
    let result = super::super::work_authority::derive_child_work(parent_work, 0);
    // Assert
    assert!(matches!(
        result,
        Err(ChainstateError::Serialization {
            context: "block proof",
            ..
        })
    ));
}

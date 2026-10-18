// Parity breadcrumbs:
// - packages/bitcoin-knots/src/validation.cpp
// - packages/bitcoin-knots/src/chain.cpp

use super::*;
use crate::coins::{CoinsFlush, CoinsView, CompleteCoinsSnapshot};
use open_bitcoin_consensus::chain_proof::ProofWork;

fn live() -> Chainstate {
    let mut source = Chainstate::new();
    let block = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    source
        .connect_block(
            &block,
            ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("actual genesis");
    source
}

fn next_block(source: &Chainstate) -> Block {
    build_block(
        source.tip().expect("tip").block_hash,
        1_231_006_600,
        vec![coinbase_transaction(1, 50)],
    )
}

struct HintOnly(MemoryCoinsView);
impl CoinsView for HintOnly {
    fn get_coin(&self, key: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        self.0.get_coin(key)
    }
    fn have_coin(&self, key: &OutPoint) -> Result<bool, ChainstateError> {
        self.0.have_coin(key)
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        self.0.best_block()
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        self.0.head_blocks()
    }
    fn batch_write(
        &mut self,
        writes: crate::CoinsBatch,
        maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        self.0.batch_write(writes, maybe_best_block)
    }
}

struct SnapshotView(Result<CompleteCoinsSnapshot, ChainstateError>);
impl CoinsView for SnapshotView {
    fn complete_snapshot(&self) -> Result<CompleteCoinsSnapshot, ChainstateError> {
        self.0.clone()
    }
    fn get_coin(&self, key: &OutPoint) -> Result<Option<Coin>, ChainstateError> {
        Ok(self.complete_snapshot()?.coins.get(key).cloned())
    }
    fn have_coin(&self, key: &OutPoint) -> Result<bool, ChainstateError> {
        Ok(self.get_coin(key)?.is_some())
    }
    fn best_block(&self) -> Result<Option<BlockHash>, ChainstateError> {
        Ok(self.complete_snapshot()?.maybe_best_block)
    }
    fn head_blocks(&self) -> Result<Vec<BlockHash>, ChainstateError> {
        Ok(self.complete_snapshot()?.head_blocks)
    }
    fn batch_write(
        &mut self,
        _writes: crate::CoinsBatch,
        _maybe_best_block: Option<BlockHash>,
    ) -> Result<(), ChainstateError> {
        Err(ChainstateError::CoinsStorage {
            detail: "read-only snapshot fixture".to_owned(),
        })
    }
}

#[test]
fn phase165_rebind_genuine_empty_owner_transfers_to_matching_complete_view() {
    // Arrange
    let source = Chainstate::new();
    let mut target =
        Chainstate::from_parent(MemoryCoinsView::default(), Vec::new(), HashMap::new(), None);
    // Act
    source
        .rebind_into(&mut target)
        .expect("genuine empty authority transfer");
    let genesis = build_block(
        BlockHash::default(),
        1_231_006_500,
        vec![coinbase_transaction(0, 50)],
    );
    let accepted = target.connect_block(
        &genesis,
        open_bitcoin_consensus::chain_proof::ProofWork::ZERO,
        ScriptVerifyFlags::P2SH,
        ConsensusParams::default(),
    );
    // Assert
    assert!(accepted.is_ok());
    assert!(target.has_earned_work());
}

#[test]
fn phase165_rebind_raw_source_cannot_upgrade_matching_target() {
    // Arrange
    let source = Chainstate::from_snapshot(Chainstate::new().snapshot());
    let mut target = Chainstate::new();
    let before = target.snapshot();
    // Act
    let result = source.rebind_into(&mut target);
    // Assert
    assert_eq!(result, Err(ChainstateError::UntrustedWorkLineage));
    assert_eq!(target.snapshot(), before);
}

#[test]
fn phase165_rebind_mechanical_source_never_upgrades() {
    // Arrange
    let source = Chainstate::new_for_mechanical_fixture();
    let mut target = Chainstate::new();
    let before = target.snapshot();
    // Act
    let result = source.rebind_into(&mut target);
    // Assert
    assert_eq!(result, Err(ChainstateError::UntrustedWorkLineage));
    assert_eq!(target.snapshot(), before);
}

#[test]
fn phase165_rebind_public_coin_mutation_cannot_forge_a_live_source() {
    // Arrange
    let mut source = Chainstate::new();
    let accepted = live().snapshot();
    let (key, coin) = accepted.utxos.into_iter().next().expect("coin");
    source
        .coins_mut()
        .add_coin(key, coin, true)
        .expect("raw insertion");
    let mut target = Chainstate::from_snapshot(source.snapshot());
    let before = target.snapshot();
    // Act
    let result = source.rebind_into(&mut target);
    // Assert
    assert_eq!(result, Err(ChainstateError::UntrustedWorkLineage));
    assert_eq!(target.snapshot(), before);
    assert!(!target.has_earned_work());
}

#[test]
fn phase165_rebind_default_hint_is_not_complete_empty_evidence() {
    // Arrange
    let source = Chainstate::new();
    let mut target = Chainstate::from_parent(
        HintOnly(MemoryCoinsView::default()),
        Vec::new(),
        HashMap::new(),
        None,
    );
    // Act
    let result = source.rebind_into(&mut target);
    // Assert
    assert_eq!(
        result,
        Err(ChainstateError::CompleteCoinsSnapshotUnavailable)
    );
    assert!(target.active_chain().is_empty());
    assert!(!target.has_earned_work());
}

#[test]
fn phase165_rebind_backend_snapshot_error_refuses_without_target_changes() {
    // Arrange
    let expected = ChainstateError::CoinsStorage {
        detail: "snapshot cursor failed".to_owned(),
    };
    let mut target = Chainstate::from_parent(
        SnapshotView(Err(expected.clone())),
        Vec::new(),
        HashMap::new(),
        None,
    );
    // Act
    let result = Chainstate::new().rebind_into(&mut target);
    // Assert
    assert_eq!(result, Err(expected));
    assert!(target.active_chain().is_empty());
    assert_eq!(target.coins().cache_entry_count(), 0);
}

#[test]
fn phase165_rebind_uncached_parent_coin_cannot_hide_behind_empty_overlay() {
    // Arrange
    let coins = live().utxos();
    let mut target = Chainstate::from_parent(
        MemoryCoinsView::from_coins(coins, None),
        Vec::new(),
        HashMap::new(),
        None,
    );
    let before = target.snapshot();
    // Act
    let result = Chainstate::new().rebind_into(&mut target);
    // Assert
    assert_eq!(result, Err(ChainstateError::WorkRebindMismatch));
    assert_eq!(target.snapshot(), before);
    assert_eq!(target.coins().cache_entry_count(), 0);
}

#[test]
fn phase165_rebind_complete_snapshot_honors_overlay_tombstones() {
    // Arrange
    let coins = live().utxos();
    let key = coins.keys().next().expect("coin").clone();
    let mut cache = CoinsCache::from_parent(MemoryCoinsView::from_coins(coins, None));
    cache.spend_coin(&key).expect("tombstone");
    let mut target = Chainstate::from_coins_cache(cache, Vec::new(), HashMap::new(), None);
    let entry = target.coins().cached_entry(&key).cloned();
    // Act
    Chainstate::new()
        .rebind_into(&mut target)
        .expect("effective complete empty state");
    // Assert
    assert_eq!(target.coins().cached_entry(&key).cloned(), entry);
    assert!(
        target
            .coins()
            .complete_snapshot()
            .expect("full view")
            .coins
            .is_empty()
    );
    assert!(
        target
            .work_authority
            .check_parent(&[], ConsensusParams::default())
            .is_ok()
    );
}

#[test]
fn phase165_rebind_all_coin_fields_are_identity_facts() {
    // Arrange
    let source = live();
    // Act / Assert
    for alteration in 0..5 {
        let mut snapshot = source.snapshot();
        let coin = snapshot.utxos.values_mut().next().expect("coin");
        match alteration {
            0 => coin.output.value = Amount::from_sats(49).expect("amount"),
            1 => coin.output.script_pubkey = script(&[0x52]),
            2 => coin.is_coinbase = false,
            3 => coin.created_height = 9,
            _ => coin.created_median_time_past = 99,
        }
        let mut target = Chainstate::from_snapshot(snapshot);
        let before = target.snapshot();
        assert_eq!(
            source.clone().rebind_into(&mut target),
            Err(ChainstateError::WorkRebindMismatch)
        );
        assert_eq!(target.snapshot(), before);
    }
}

#[test]
fn phase165_rebind_b_and_heads_are_complete_identity_facts() {
    // Arrange
    let source = live();
    let snapshot = source.snapshot();
    // Act / Assert
    for interrupted in [false, true] {
        let parent = SnapshotView(Ok(CompleteCoinsSnapshot {
            coins: snapshot.utxos.clone(),
            maybe_best_block: if interrupted {
                Some(snapshot.tip().expect("tip").block_hash)
            } else {
                Some(BlockHash::from_byte_array([9; 32]))
            },
            head_blocks: if interrupted {
                vec![BlockHash::default(), BlockHash::default()]
            } else {
                Vec::new()
            },
        }));
        let mut target = Chainstate::from_parent(
            parent,
            snapshot.active_chain.clone(),
            snapshot.undo_by_block.clone(),
            snapshot.maybe_confirmed_txid_counts.clone(),
        );
        assert_eq!(
            source.clone().rebind_into(&mut target),
            Err(ChainstateError::WorkRebindMismatch)
        );
        assert!(!target.has_earned_work());
        assert_eq!(target.active_chain(), snapshot.active_chain);
    }
}

#[test]
fn phase165_rebind_metadata_undo_and_counts_must_match() {
    // Arrange
    let source = live();
    // Act / Assert
    for alteration in 0..4 {
        let mut snapshot = source.snapshot();
        match alteration {
            0 => snapshot.active_chain[0].chain_work = ProofWork::from_le_bytes([255; 32]),
            1 => snapshot.active_chain[0].median_time_past += 1,
            2 => {
                snapshot
                    .undo_by_block
                    .insert(BlockHash::default(), BlockUndo::default());
            }
            _ => snapshot.maybe_confirmed_txid_counts = None,
        }
        let mut target = Chainstate::from_snapshot(snapshot);
        let before = target.snapshot();
        assert_eq!(
            source.clone().rebind_into(&mut target),
            Err(ChainstateError::WorkRebindMismatch)
        );
        assert_eq!(target.snapshot(), before);
    }
}

#[test]
fn phase165_rebind_success_rotates_both_prior_stage_owners() {
    // Arrange
    let source = live();
    let block = next_block(&source);
    let mut target = source.clone();
    let source_stage = source
        .stage_connect_block_with_current_time(
            &block,
            ProofWork::ZERO,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("source stage");
    let target_stage = target
        .stage_connect_block_with_current_time(
            &block,
            ProofWork::ZERO,
            i64::from(block.header.time),
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("target stage");
    // Act
    source
        .rebind_into(&mut target)
        .expect("complete identical transfer");
    let before = target.snapshot();
    // Assert
    for staged in [source_stage, target_stage] {
        assert_eq!(
            target.commit_staged_connect(staged),
            Err(ChainstateError::StageOwnerMismatch)
        );
    }
    assert_eq!(target.snapshot(), before);
    assert!(target.has_earned_work());
    assert!(
        target
            .connect_block(
                &block,
                ProofWork::ZERO,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            )
            .is_ok()
    );
}

#[test]
fn phase165_rebind_preview_source_or_target_refuses() {
    // Arrange
    let mut preview = live();
    let child = next_block(&preview);
    let stage = preview
        .stage_reorg(
            &[],
            &[AnchoredBlock {
                block: child.clone(),
                chain_work: ProofWork::ZERO,
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("stage");
    preview
        .install_staged_reorg_preview(&stage)
        .expect("preview");
    let mut target = Chainstate::from_snapshot(preview.snapshot());
    let before = target.snapshot();
    // Act
    let source_refusal = preview.rebind_into(&mut target);
    let mut accepted = live();
    accepted
        .connect_block(
            &child,
            ProofWork::ZERO,
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("actual acceptance");
    let mut other_preview = live();
    let stage = other_preview
        .stage_reorg(
            &[],
            &[AnchoredBlock {
                block: child,
                chain_work: ProofWork::ZERO,
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("stage");
    other_preview
        .install_staged_reorg_preview(&stage)
        .expect("preview");
    let target_refusal = accepted.rebind_into(&mut other_preview);
    // Assert
    assert_eq!(source_refusal, Err(ChainstateError::UntrustedWorkLineage));
    assert_eq!(target_refusal, Err(ChainstateError::StaleStage));
    assert_eq!(target.snapshot(), before);
}

#[test]
fn phase165_rebind_closed_flush_preserves_live_authority() {
    // Arrange
    let mut source = live();
    let before = source.snapshot();
    let _bytes = CoinsFlush::estimated_cache_bytes(&source);
    let _count = CoinsFlush::cache_entry_count(&source);
    // Act
    CoinsFlush::prepare_current_best_block(&mut source).expect("current B");
    CoinsFlush::flush(&mut source).expect("closed flush");
    CoinsFlush::sync(&mut source).expect("closed sync");
    let mut target = Chainstate::from_snapshot(source.snapshot());
    source
        .rebind_into(&mut target)
        .expect("authority survives benign persistence");
    // Assert
    assert_eq!(target.snapshot(), before);
    assert!(target.has_earned_work());
}

#[test]
fn phase165_rebind_preview_closed_flush_refuses_without_effects() {
    // Arrange
    let mut source = live();
    let stage = source
        .stage_reorg(
            &[],
            &[AnchoredBlock {
                block: next_block(&source),
                chain_work: ProofWork::ZERO,
            }],
            ScriptVerifyFlags::P2SH,
            ConsensusParams::default(),
        )
        .expect("stage");
    source
        .install_staged_reorg_preview(&stage)
        .expect("preview");
    let before = source.snapshot();
    // Act
    let prepare = CoinsFlush::prepare_current_best_block(&mut source);
    let flush = CoinsFlush::flush(&mut source);
    let sync = CoinsFlush::sync(&mut source);
    // Assert
    assert_eq!(prepare, Err(ChainstateError::StaleStage));
    assert_eq!(flush, Err(ChainstateError::StaleStage));
    assert_eq!(sync, Err(ChainstateError::StaleStage));
    assert_eq!(source.snapshot(), before);
    assert!(source.absorb_staged_reorg(stage).is_ok());
}

#[test]
fn phase165_rebind_error_display_is_specific() {
    // Arrange
    let errors = [
        ChainstateError::CompleteCoinsSnapshotUnavailable,
        ChainstateError::WorkRebindMismatch,
    ];
    // Act
    let messages = errors.map(|error| error.to_string());
    // Assert
    assert_eq!(
        messages,
        [
            "complete coins snapshot unavailable",
            "work authority transfer state mismatch"
        ]
    );
}

#[test]
fn phase165_rebind_owned_empty_origin_differs_from_raw_empty_import() {
    // Arrange
    let genuine = Chainstate::new();
    let raw = Chainstate::from_snapshot(genuine.snapshot());
    // Act
    let live_origin = genuine.validate_live_origin();
    let raw_origin = raw.validate_live_origin();
    // Assert
    assert_eq!(live_origin, Ok(()));
    assert_eq!(raw_origin, Err(ChainstateError::UntrustedWorkLineage));
    assert!(!genuine.has_earned_work());
    assert!(!raw.has_earned_work());
}

#[test]
fn phase165_rebind_preserves_selected_parameter_binding() {
    // Arrange
    let source = live();
    let child = next_block(&source);
    let mut target = Chainstate::from_snapshot(source.snapshot());
    source.rebind_into(&mut target).expect("genuine transfer");
    let before = target.snapshot();
    let changed = ConsensusParams {
        coinbase_maturity: 1,
        ..ConsensusParams::default()
    };
    // Act
    let result = target.connect_block(&child, ProofWork::ZERO, ScriptVerifyFlags::P2SH, changed);
    // Assert
    assert_eq!(result, Err(ChainstateError::WorkParametersMismatch));
    assert_eq!(target.snapshot(), before);
    assert!(
        target
            .connect_block(
                &child,
                ProofWork::ZERO,
                ScriptVerifyFlags::P2SH,
                ConsensusParams::default()
            )
            .is_ok()
    );
}

#[test]
fn phase165_rebind_restricted_cache_surface_preserves_effective_state() {
    for (sync, populated) in [(false, false), (false, true), (true, true)] {
        // Arrange
        let mut cache = CoinsCache::from_parent(MemoryCoinsView::default());
        if populated {
            let (key, coin) = live().utxos().into_iter().next().expect("coin");
            cache.add_coin(key, coin, false).expect("dirty coin");
            cache.set_best_block(BlockHash::from_byte_array([3; 32]));
        }
        let before = cache.complete_snapshot().expect("effective state");
        let bytes = CoinsFlush::estimated_cache_bytes(&cache);
        let entries = CoinsFlush::cache_entry_count(&cache);
        // Act
        CoinsFlush::prepare_current_best_block(&mut cache).expect("only current B");
        if sync {
            CoinsFlush::sync(&mut cache).expect("sync");
        } else {
            CoinsFlush::flush(&mut cache).expect("flush");
        }
        // Assert
        assert_eq!(cache.complete_snapshot().expect("effective state"), before);
        assert_eq!(entries, u64::from(populated));
        assert_eq!(bytes > 0, populated);
        assert_eq!(cache.cache_entry_count(), u64::from(sync && populated));
        for key in before.coins.keys() {
            assert!(cache.parent().have_coin(key).expect("persisted coin"));
            if sync {
                assert!(
                    !cache
                        .cached_entry(key)
                        .expect("clean retained entry")
                        .is_dirty()
                );
            }
        }
    }
}

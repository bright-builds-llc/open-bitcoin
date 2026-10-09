// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;
use crate::chainstate::ReorgFixture;

mod faults;

fn prepared(fixture: &ReorgFixture, count: usize) -> PreparedChainstateReorg {
    let (staged, _) = fixture.stage(count);
    let disconnect = staged
        .transition()
        .disconnected
        .iter()
        .map(|p| {
            fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect::<Vec<_>>();
    let replacement = staged
        .transition()
        .connected
        .iter()
        .map(|p| AnchoredBlock {
            block: fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained"),
            chain_work: p.chain_work,
        })
        .collect::<Vec<_>>();
    fixture
        .manager
        .prepare_reorg(
            &disconnect,
            &replacement,
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("prepared genuine reorg")
}

#[test]
fn phase158_manager_reorg_preview_freezes_already_owned_append_and_turn() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-preview", 2);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let old = fixture
        .manager
        .maybe_basic_index_progress()
        .expect("progress");
    let turn = old.prepare_turn().expect("turn");
    let proof = fixture
        .store
        .maybe_basic_filter_append_proof_with_budget(ReorgFixture::budget())
        .expect("proof")
        .expect("active");
    let append = fixture
        .store
        .prepare_basic_filter_append(proof, &fixture.records[2..])
        .expect("already owned batch");
    let prepared = prepared(&fixture, 2);
    let positions = fixture.manager.chainstate.active_chain().to_vec();
    let fence = open_bitcoin_core::chainstate::VerifiedChainstateFence::new(
        fixture.store.coins_view().best_block().expect("coins"),
        Some(&positions),
    )
    .expect("fence");
    let token = fixture
        .store
        .maybe_basic_filter_work(&fence)
        .expect("work")
        .expect("active");
    // Act
    fixture
        .manager
        .install_prepared_reorg_preview(&prepared)
        .expect("frozen");
    // Assert
    assert!(fixture.store.complete_basic_filter_append(append).is_err());
    assert!(
        fixture
            .store
            .persist_basic_filter_records(&token, &fence, &fixture.records[2..])
            .is_err()
    );
    assert!(fixture.store.maybe_basic_filter_work(&fence).is_err());
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    assert_eq!(
        fixture.manager.maybe_basic_index_accepted_target(),
        Some(old.accepted_target())
    );
    assert!(
        fixture
            .manager
            .complete_basic_index_turn(turn, &[fixture.records[2].identity()])
            .is_err()
    );
    assert_eq!(
        fixture
            .manager
            .maybe_basic_index_progress()
            .expect("progress")
            .accepted_target(),
        old.accepted_target()
    );
    assert!(
        fixture
            .manager
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(1),
                u64::MAX
            )
            .is_err()
    );
    drop(prepared);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_preabsorb_failure_retains_preview_identity_and_suspension() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-preview-raw", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let old = fixture.manager.maybe_basic_index_progress().expect("old");
    let prepared = prepared(&fixture, 2);
    fixture
        .manager
        .install_prepared_reorg_preview(&prepared)
        .expect("preview");
    let best = fixture.store.coins_view().best_block().expect("old coins");
    fixture
        .store
        .coins_view()
        .batch_write(
            CoinsBatch {
                entries: Default::default(),
            },
            best,
        )
        .expect("actual foreign raw writer");
    // Act
    let result = fixture.manager.commit_prepared_reorg(prepared);
    // Assert
    assert!(result.is_err());
    let owner = fixture
        .manager
        .maybe_basic_index_owner
        .as_ref()
        .expect("owner");
    assert_eq!(owner.reorg_state, BasicIndexReorgState::PreviewFrozen);
    assert_eq!(owner.progress.accepted_target(), old.accepted_target());
    assert_eq!(owner.progress.protection(), old.protection());
    assert!(fixture.store.maybe_basic_filter_append_proof().is_err());
    assert!(
        fixture
            .manager
            .flush_with_mode(
                FlushMode::Always,
                FlushPolicyTime::from_unix_seconds(10),
                u64::MAX
            )
            .is_err()
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_preview_freeze_failure_precedes_core_preview() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-preview-refused", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let old_tip = fixture.manager.chainstate.tip().cloned();
    let old = fixture.manager.maybe_basic_index_progress();
    let prepared = prepared(&fixture, 2);
    fixture
        .store
        .invalidate_basic_filter_append()
        .expect("foreign invalidation");
    // Act
    let result = fixture.manager.install_prepared_reorg_preview(&prepared);
    // Assert
    assert!(result.is_err());
    assert_eq!(fixture.manager.chainstate.tip(), old_tip.as_ref());
    assert_eq!(fixture.manager.maybe_basic_index_progress(), old);
    drop(prepared);
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_ordinary_persistence_faults_keep_accepted_branch() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeUndo,
        FilterPublicationFault::BeforeCoins,
        FilterPublicationFault::BeforeChainMeta,
    ] {
        // Arrange
        let mut fixture = ReorgFixture::new("manager-persistence-fault", 3);
        let path = fixture.path().to_owned();
        fixture
            .manager
            .initialize_basic_index_owner(ReorgFixture::budget())
            .expect("owner");
        let prepared = prepared(&fixture, 2);
        let target = prepared
            .transition()
            .connected
            .last()
            .map(|p| (p.height, p.block_hash));
        fixture
            .manager
            .commit_prepared_reorg(prepared)
            .expect("accepted unflushed");
        let before = fixture
            .manager
            .maybe_basic_index_progress()
            .expect("shared progress");
        fixture.store.set_basic_filter_fault(fault);
        // Act
        let result = fixture.manager.flush_with_mode(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(10),
            u64::MAX,
        );
        // Assert
        assert!(result.is_err());
        assert_eq!(
            fixture.manager.maybe_basic_index_accepted_target(),
            target.map(|(height, hash)| AcceptedIndexTarget::new(height, hash))
        );
        let owner = fixture
            .manager
            .maybe_basic_index_owner
            .as_ref()
            .expect("owner");
        assert_eq!(
            owner.reorg_state,
            BasicIndexReorgState::AcceptedReplacement {
                maybe_target: target
            }
        );
        assert_eq!(owner.progress.accepted_target(), before.accepted_target());
        assert_eq!(
            owner.progress.maybe_safe_durable_endpoint(),
            before.maybe_safe_durable_endpoint()
        );
        assert_eq!(owner.progress.protection(), before.protection());
        assert!(owner.progress.prepare_turn().is_err());
        assert_eq!(
            owner.maybe_failure,
            Some(super::super::filter_index::AcceptedBasicIndexFailure::Persistence)
        );
        drop(fixture);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase158_manager_reorg_equal_height_restores_owner_without_flush() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-equal", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let old = fixture
        .manager
        .maybe_basic_index_progress()
        .expect("progress");
    let prepared = prepared(&fixture, 2);
    let new_tip = prepared
        .transition()
        .connected
        .last()
        .expect("replacement")
        .clone();
    // Act
    fixture
        .manager
        .install_prepared_reorg_preview(&prepared)
        .expect("frozen");
    fixture
        .manager
        .commit_prepared_reorg(prepared)
        .expect("accepted owner");
    // Assert
    let progress = fixture
        .manager
        .maybe_basic_index_progress()
        .expect("owner survives");
    assert_eq!(
        progress.accepted_target(),
        AcceptedIndexTarget::new(new_tip.height, new_tip.block_hash)
    );
    assert_eq!(progress.generation().value(), old.generation().value() + 1);
    assert_eq!(
        progress.maybe_processed_endpoint(),
        Some(fixture.records[0].identity())
    );
    assert_eq!(
        progress.maybe_safe_durable_endpoint(),
        Some(fixture.records[0].identity())
    );
    assert!(progress.prepare_turn().is_ok());
    assert_eq!(
        fixture.store.coins_view().best_block().expect("coins"),
        Some(old.accepted_target().block_hash())
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_lower_higher_and_lagging_targets_keep_shared_safe() {
    for (indexed, count) in [(3, 1), (3, 3), (1, 3)] {
        // Arrange
        let mut fixture = ReorgFixture::new("manager-heights", indexed);
        let path = fixture.path().to_owned();
        fixture
            .manager
            .initialize_basic_index_owner(ReorgFixture::budget())
            .expect("owner");
        let old = fixture
            .manager
            .maybe_basic_index_progress()
            .expect("progress");
        let prepared = prepared(&fixture, count);
        let target = prepared
            .transition()
            .connected
            .last()
            .expect("replacement")
            .clone();
        // Act
        fixture
            .manager
            .commit_prepared_reorg(prepared)
            .expect("direct staged commit");
        // Assert
        let progress = fixture
            .manager
            .maybe_basic_index_progress()
            .expect("surviving owner");
        assert_eq!(
            progress.accepted_target(),
            AcceptedIndexTarget::new(target.height, target.block_hash)
        );
        assert_eq!(
            progress.maybe_processed_endpoint(),
            Some(fixture.records[0].identity())
        );
        assert_eq!(
            progress.maybe_safe_durable_endpoint(),
            Some(fixture.records[0].identity())
        );
        assert_eq!(
            fixture.store.coins_view().best_block().expect("coins"),
            Some(old.accepted_target().block_hash())
        );
        assert!(progress.prepare_turn().is_ok());
        drop(fixture);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase158_manager_reorg_accepted_publication_error_records_new_identity_and_pending_flush() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let mut fixture = ReorgFixture::new("manager-publication-fault", 3);
        let path = fixture.path().to_owned();
        fixture
            .manager
            .initialize_basic_index_owner(ReorgFixture::budget())
            .expect("owner");
        let old = fixture.manager.maybe_basic_index_progress().expect("old");
        let prepared = prepared(&fixture, 2);
        let target = prepared
            .transition()
            .connected
            .last()
            .map(|p| (p.height, p.block_hash));
        fixture
            .manager
            .install_prepared_reorg_preview(&prepared)
            .expect("preview");
        fixture.store.set_basic_filter_fault(fault);
        // Act
        let result = fixture.manager.commit_prepared_reorg(prepared);
        // Assert
        assert!(result.is_err());
        let owner = fixture
            .manager
            .maybe_basic_index_owner
            .as_ref()
            .expect("owner");
        assert_eq!(
            owner.reorg_state,
            BasicIndexReorgState::AcceptedReplacement {
                maybe_target: target
            }
        );
        assert_eq!(
            owner.progress.maybe_safe_durable_endpoint(),
            old.maybe_safe_durable_endpoint()
        );
        assert_eq!(
            owner.maybe_failure,
            Some(super::super::filter_index::AcceptedBasicIndexFailure::Persistence)
        );
        assert!(owner.progress.prepare_turn().is_err());
        let lineage = fixture
            .manager
            .maybe_validated_lineage
            .as_ref()
            .expect("retained pending lineage");
        assert!(lineage.reorg_is_pending());
        assert!(
            lineage
                .maybe_prepare(fixture.manager.chainstate.tip())
                .is_err()
        );
        drop(fixture);
        std::fs::remove_dir_all(path).expect("cleanup");
    }
}

#[test]
fn phase158_manager_reorg_full_disconnect_records_absence_without_flush_receipt() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-empty", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let disconnect = fixture
        .manager
        .chainstate
        .active_chain()
        .iter()
        .rev()
        .map(|p| {
            fixture
                .store
                .load_block(p.block_hash)
                .expect("body")
                .expect("retained")
        })
        .collect::<Vec<_>>();
    // Act
    fixture
        .manager
        .reorg(
            &disconnect,
            &[],
            ScriptVerifyFlags::P2SH,
            ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("full disconnect");
    // Assert
    assert!(fixture.manager.chainstate.tip().is_none());
    assert!(
        fixture
            .manager
            .maybe_basic_index_accepted_target()
            .is_none()
    );
    assert_eq!(
        fixture
            .manager
            .maybe_basic_index_owner
            .as_ref()
            .expect("owner")
            .reorg_state,
        BasicIndexReorgState::AcceptedReplacement { maybe_target: None }
    );
    assert!(
        fixture
            .manager
            .maybe_validated_lineage
            .as_ref()
            .expect("lineage")
            .maybe_prepare(None)
            .expect("empty lineage")
            .is_none()
    );
    assert_eq!(
        fixture
            .store
            .maybe_basic_filter_checkpoint()
            .expect("checkpoint"),
        Some(FilterCheckpoint::new(IndexPrefix::Empty))
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

#[test]
fn phase158_manager_reorg_ordinary_own_flush_promotes_only_replacement_fence() {
    // Arrange
    let mut fixture = ReorgFixture::new("manager-own-flush", 3);
    let path = fixture.path().to_owned();
    fixture
        .manager
        .initialize_basic_index_owner(ReorgFixture::budget())
        .expect("owner");
    let (_, replacement) = fixture.stage(2);
    let prepared = prepared(&fixture, 2);
    fixture
        .manager
        .commit_prepared_reorg(prepared)
        .expect("genuine replacement");
    fixture
        .append_replacement(&replacement)
        .expect("positions from achieved current manager");
    let before = fixture.store.maybe_basic_filter_checkpoint().expect("safe");
    assert_eq!(
        before,
        Some(FilterCheckpoint::new(IndexPrefix::Committed(
            fixture.records[0].identity()
        )))
    );
    // Act
    fixture
        .manager
        .flush_with_mode(
            FlushMode::Always,
            FlushPolicyTime::from_unix_seconds(10),
            u64::MAX,
        )
        .expect("own ordinary flush");
    fixture.append_replacement(&[]).expect("earned promotion");
    // Assert
    let target = Some((2, replacement[1].identity().block_hash()));
    assert_eq!(
        fixture
            .manager
            .maybe_basic_index_owner
            .as_ref()
            .expect("owner")
            .reorg_state,
        BasicIndexReorgState::DurablyFencedReplacement {
            maybe_target: target
        }
    );
    assert_eq!(
        fixture.store.maybe_basic_filter_checkpoint().expect("safe"),
        Some(FilterCheckpoint::new(IndexPrefix::Committed(
            replacement[1].identity()
        )))
    );
    drop(fixture);
    std::fs::remove_dir_all(path).expect("cleanup");
}

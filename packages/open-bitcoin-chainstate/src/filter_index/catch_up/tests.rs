// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::filter_index::{FilterRecordIdentity, IndexInputProtection};
use open_bitcoin_consensus::compute_filter_header;
use open_bitcoin_primitives::{BlockHash, FilterHash, FilterHeader};

mod budget;
mod durability;
mod reorg;

fn hash(value: u8) -> BlockHash {
    BlockHash::from_byte_array([value; 32])
}

fn record(height: u32, maybe_previous: Option<FilterRecordIdentity>) -> FilterRecordIdentity {
    let parent = maybe_previous.map_or(BlockHash::default(), |r| r.block_hash());
    let previous = maybe_previous.map_or(FilterHeader::default(), |r| r.filter_header());
    let filter_hash = FilterHash::from_byte_array([height as u8; 32]);
    FilterRecordIdentity::new_with_predecessor_facts(
        height,
        hash(height as u8 + 1),
        parent,
        filter_hash,
        compute_filter_header(filter_hash, previous),
        previous,
        maybe_previous.map(|r| (r.height(), r.block_hash(), r.filter_header())),
    )
    .expect("valid consecutive fixture")
}

fn fresh(target: u32) -> BasicIndexProgress {
    BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(target, hash(target as u8 + 1)),
        None,
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("fresh owner")
}

#[test]
fn phase157_owner_genesis_starts_at_zero() {
    // Arrange
    let progress = fresh(1);
    // Act
    let next = progress.maybe_next_height();
    // Assert
    assert_eq!(next, Ok(Some(0)));
    assert!(!progress.initially_synchronized());
    assert_eq!(progress.current_lag(), 2);
    assert_eq!(progress.generation(), IndexGeneration::new(4));
    assert_eq!(progress.branch_identity(), hash(99));
}

#[test]
fn phase157_owner_accepted_growth_keeps_ordered_next_height() {
    // Arrange
    let mut progress = fresh(1);
    // Act
    progress
        .observe_validated_connect(AcceptedIndexTarget::new(2, hash(3)), hash(2))
        .expect("accepted consecutive connect");
    // Assert
    assert_eq!(progress.maybe_next_height(), Ok(Some(0)));
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(progress.accepted_target().block_hash(), hash(3));
    assert_eq!(progress.current_lag(), 3);
}

#[test]
fn phase157_owner_invalid_acceptance_preserves_state() {
    // Arrange
    let mut progress = fresh(1);
    let before = progress;
    // Act
    let result = progress.observe_validated_connect(AcceptedIndexTarget::new(3, hash(4)), hash(2));
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::NonConsecutive));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_initial_completion_latches_despite_later_lag() {
    // Arrange
    let mut progress = fresh(0);
    let prepared = progress.prepare_turn().expect("active turn");
    // Act
    progress
        .complete_turn(prepared, &[record(0, None)])
        .expect("genesis processed");
    progress
        .observe_validated_connect(AcceptedIndexTarget::new(1, hash(2)), hash(1))
        .expect("accepted connect");
    // Assert
    assert!(progress.initially_synchronized());
    assert_eq!(progress.current_lag(), 1);
    assert_eq!(progress.maybe_next_height(), Ok(Some(1)));
    assert_eq!(progress.maybe_safe_durable_endpoint(), None);
}

#[test]
fn phase157_owner_stale_frontier_preserves_checkpoint_and_protection() {
    // Arrange
    let mut progress = fresh(1);
    let prepared = progress.prepare_turn().expect("active turn");
    let genesis = record(0, None);
    progress
        .complete_turn(prepared, &[genesis])
        .expect("first completion");
    let before = progress;
    // Act
    let result = progress.complete_turn(prepared, &[record(1, Some(genesis))]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_durable_ahead_of_processed_is_invalid() {
    // Arrange
    let genesis = record(0, None);
    // Act
    let result = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, hash(2)),
        None,
        Some(genesis),
        IndexInputProtection::FromHeight(1),
    );
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::InvalidProgress));
}

#[test]
fn phase157_owner_persistence_failure_retains_accepted_progress() {
    // Arrange
    let mut progress = fresh(1);
    progress
        .observe_validated_connect(AcceptedIndexTarget::new(2, hash(3)), hash(2))
        .expect("accepted before persistence");
    // Act
    progress.pause(BasicIndexPause::Persistence);
    // Assert
    assert_eq!(progress.accepted_target().height(), 2);
    assert_eq!(
        progress.state(),
        BasicIndexState::Paused(BasicIndexPause::Persistence)
    );
    assert_eq!(progress.maybe_safe_durable_endpoint(), None);
    assert_eq!(progress.protection(), IndexInputProtection::FromHeight(0));
    assert_eq!(
        progress.prepare_turn(),
        Err(BasicIndexCatchUpError::NotActive)
    );
}

#[test]
fn phase157_owner_acceptance_parent_mismatch_is_unchanged() {
    // Arrange
    let mut progress = fresh(0);
    let before = progress;
    // Act
    let result = progress.observe_validated_connect(AcceptedIndexTarget::new(1, hash(2)), hash(8));
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::NonConsecutive));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_processing_is_consecutive_and_atomic() {
    // Arrange
    let mut progress = fresh(2);
    let genesis = record(0, None);
    let child = record(1, Some(genesis));
    let before = progress;
    let prepared = progress.prepare_turn().expect("prepared");
    // Act
    let result = progress.complete_turn(prepared, &[genesis, record(2, Some(child))]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::NonConsecutive));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_wrong_record_parent_is_refused() {
    // Arrange
    let mut progress = fresh(2);
    let genesis = record(0, None);
    let foreign = FilterRecordIdentity::new_with_predecessor_facts(
        1,
        hash(2),
        hash(8),
        FilterHash::default(),
        compute_filter_header(FilterHash::default(), genesis.filter_header()),
        genesis.filter_header(),
        Some((0, hash(8), genesis.filter_header())),
    )
    .expect("locally valid fork");
    let prepared = progress.prepare_turn().expect("prepared");
    // Act
    let result = progress.complete_turn(prepared, &[genesis, foreign]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::NonConsecutive));
    assert_eq!(progress.maybe_processed_endpoint(), None);
}

#[test]
fn phase157_owner_wrong_target_hash_is_refused() {
    // Arrange
    let mut progress = fresh(0);
    progress.accepted_target = AcceptedIndexTarget::new(0, hash(8));
    let before = progress;
    // Act
    let result = progress.complete_turn(
        progress.prepare_turn().expect("prepared"),
        &[record(0, None)],
    );
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_prepared_old_target_stays_bound_after_accepted_growth() {
    // Arrange
    let mut progress = fresh(0);
    let prepared = progress.prepare_turn().expect("prepared");
    progress
        .observe_validated_connect(AcceptedIndexTarget::new(1, hash(2)), hash(1))
        .expect("accepted");
    let genesis = record(0, None);
    // Act
    progress
        .complete_turn(prepared, &[genesis])
        .expect("old frontier is still valid");
    // Assert
    assert!(!progress.initially_synchronized());
    assert_eq!(progress.maybe_processed_endpoint(), Some(genesis));
    assert_eq!(progress.current_lag(), 1);
}

#[test]
fn phase157_owner_foreign_generation_and_branch_are_stale() {
    // Arrange
    let original = fresh(0);
    let prepared = original.prepare_turn().expect("prepared");
    // Act / Assert
    for foreign in [
        BasicIndexProgress {
            generation: IndexGeneration::new(5),
            ..original
        },
        BasicIndexProgress {
            branch_identity: hash(77),
            ..original
        },
    ] {
        let mut progress = foreign;
        assert_eq!(
            progress.complete_turn(prepared, &[record(0, None)]),
            Err(BasicIndexCatchUpError::StaleWork)
        );
        assert_eq!(progress, foreign);
    }
}

#[test]
fn phase157_owner_empty_and_excessive_turns_refuse() {
    // Arrange
    let mut progress = fresh(1);
    let prepared = progress.prepare_turn().expect("prepared");
    // Act / Assert
    assert_eq!(
        progress.complete_turn(prepared, &[]),
        Err(BasicIndexCatchUpError::InvalidTurn)
    );
    assert_eq!(
        progress.complete_turn(prepared, &[record(0, None); 129]),
        Err(BasicIndexCatchUpError::InvalidTurn)
    );
    assert_eq!(progress, fresh(1));
}

#[test]
fn phase157_owner_invalid_recovered_endpoint_combinations_refuse() {
    // Arrange
    let genesis = record(0, None);
    let child = record(1, Some(genesis));
    let wrong = FilterRecordIdentity::new_with_predecessor_facts(
        0,
        hash(9),
        BlockHash::default(),
        FilterHash::default(),
        compute_filter_header(FilterHash::default(), FilterHeader::default()),
        FilterHeader::default(),
        None,
    )
    .expect("other genesis");
    // Act / Assert
    for (target, processed, safe) in [
        (AcceptedIndexTarget::new(0, hash(1)), Some(child), None),
        (AcceptedIndexTarget::new(0, hash(9)), Some(genesis), None),
        (
            AcceptedIndexTarget::new(1, hash(2)),
            Some(genesis),
            Some(child),
        ),
        (
            AcceptedIndexTarget::new(0, hash(1)),
            Some(genesis),
            Some(wrong),
        ),
    ] {
        assert_eq!(
            BasicIndexProgress::new(
                IndexGeneration::new(4),
                hash(99),
                target,
                processed,
                safe,
                IndexInputProtection::FromHeight(0)
            ),
            Err(BasicIndexCatchUpError::InvalidProgress)
        );
    }
}

#[test]
fn phase157_owner_weak_protection_is_refused() {
    // Arrange / Act
    let result = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(1, hash(2)),
        None,
        None,
        IndexInputProtection::FromHeight(1),
    );
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::WeakProtection));
}

#[test]
fn phase157_owner_disable_invalidates_prepared_work_idempotently() {
    // Arrange
    let mut progress = fresh(0);
    let prepared = progress.prepare_turn().expect("prepared");
    // Act
    progress.disable().expect("disable");
    let disabled = progress;
    progress.disable().expect("idempotent");
    // Assert
    assert_eq!(progress, disabled);
    assert_eq!(progress.generation.value(), 5);
    assert_eq!(
        progress.complete_turn(prepared, &[record(0, None)]),
        Err(BasicIndexCatchUpError::StaleWork)
    );
    assert_eq!(
        progress.prepare_turn(),
        Err(BasicIndexCatchUpError::NotActive)
    );
    assert_eq!(
        progress.observe_validated_connect(AcceptedIndexTarget::new(1, hash(2)), hash(1)),
        Err(BasicIndexCatchUpError::NotActive)
    );
    assert_eq!(
        progress.resume(IndexGeneration::new(5), hash(99)),
        Err(BasicIndexCatchUpError::StaleWork)
    );
}

#[test]
fn phase157_owner_generation_exhaustion_preserves_state() {
    // Arrange
    let mut progress = BasicIndexProgress {
        generation: IndexGeneration::new(u64::MAX),
        ..fresh(0)
    };
    let before = progress;
    // Act
    let result = progress.disable();
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::GenerationExhausted));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_resume_requires_same_identity() {
    // Arrange
    let mut progress = fresh(0);
    progress.pause(BasicIndexPause::MissingHistory);
    let paused = progress;
    // Act / Assert
    assert_eq!(
        progress.resume(IndexGeneration::new(5), hash(99)),
        Err(BasicIndexCatchUpError::StaleWork)
    );
    assert_eq!(
        progress.resume(IndexGeneration::new(4), hash(98)),
        Err(BasicIndexCatchUpError::StaleWork)
    );
    assert_eq!(progress, paused);
    progress
        .resume(IndexGeneration::new(4), hash(99))
        .expect("same identity re-proven");
    assert_eq!(progress.state(), BasicIndexState::Active);
}

#[test]
fn phase157_owner_max_height_lag_uses_u64_and_connect_refuses() {
    // Arrange
    let mut progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(u32::MAX, hash(9)),
        None,
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("maximum accepted target");
    let before = progress;
    // Act
    let result = progress.observe_validated_connect(AcceptedIndexTarget::new(0, hash(8)), hash(9));
    // Assert
    assert_eq!(progress.current_lag(), 1_u64 << 32);
    assert_eq!(result, Err(BasicIndexCatchUpError::HeightExhausted));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_snapshot_refuses_work_above_prepared_target() {
    // Arrange
    let mut progress = fresh(0);
    let prepared = progress.prepare_turn().expect("prepared");
    progress
        .observe_validated_connect(AcceptedIndexTarget::new(1, hash(2)), hash(1))
        .expect("accepted");
    let before = progress;
    let genesis = record(0, None);
    // Act
    let result = progress.complete_turn(prepared, &[genesis, record(1, Some(genesis))]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(progress, before);
}

#[test]
fn phase157_owner_pausing_disabled_owner_cannot_resurrect_it() {
    // Arrange
    let mut progress = fresh(0);
    progress.disable().expect("checked disable");
    let disabled = progress;
    // Act
    progress.pause(BasicIndexPause::Invalidated);
    // Assert
    assert_eq!(progress, disabled);
    assert_eq!(
        progress.resume(progress.generation, progress.branch_identity),
        Err(BasicIndexCatchUpError::StaleWork)
    );
}

#[test]
fn phase157_owner_error_categories_have_bounded_diagnostics() {
    // Arrange
    let errors = [
        BasicIndexCatchUpError::InvalidProgress,
        BasicIndexCatchUpError::InvalidDurability,
        BasicIndexCatchUpError::WeakProtection,
        BasicIndexCatchUpError::NonConsecutive,
        BasicIndexCatchUpError::StaleWork,
        BasicIndexCatchUpError::NotActive,
        BasicIndexCatchUpError::HeightExhausted,
        BasicIndexCatchUpError::GenerationExhausted,
        BasicIndexCatchUpError::InvalidTurn,
    ];
    // Act / Assert
    for error in errors {
        assert!(error.to_string().contains("BASIC"));
    }
}

#[test]
fn phase157_owner_maximum_processed_height_is_complete_without_wraparound() {
    // Arrange
    let endpoint = FilterRecordIdentity::new_with_predecessor_facts(
        u32::MAX,
        hash(8),
        hash(7),
        FilterHash::default(),
        compute_filter_header(FilterHash::default(), FilterHeader::default()),
        FilterHeader::default(),
        Some((u32::MAX - 1, hash(7), FilterHeader::default())),
    )
    .expect("maximum local edge");
    let progress = BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(u32::MAX, hash(8)),
        Some(endpoint),
        Some(endpoint),
        IndexInputProtection::HeightSpaceExhausted,
    )
    .expect("verified recovered facts");
    // Act
    let next = progress.maybe_next_height();
    // Assert
    assert_eq!(next, Ok(None));
    assert_eq!(progress.current_lag(), 0);
    assert!(progress.initially_synchronized());
}

#[test]
fn phase157_owner_consecutive_multi_record_turn_reaches_initial_target() {
    // Arrange
    let mut progress = fresh(1);
    let genesis = record(0, None);
    let child = record(1, Some(genesis));
    let prepared = progress.prepare_turn().expect("prepared");
    // Act
    progress
        .complete_turn(prepared, &[genesis, child])
        .expect("ordered prefix publication");
    // Assert
    assert!(progress.initially_synchronized());
    assert_eq!(progress.maybe_processed_endpoint(), Some(child));
    assert_eq!(progress.current_lag(), 0);
    assert_eq!(progress.maybe_safe_durable_endpoint(), None);
    assert_eq!(progress.protection(), IndexInputProtection::FromHeight(0));
}

#[test]
fn phase157_owner_adjacent_recovered_endpoints_must_share_parent_and_header() {
    // Arrange
    let genesis = record(0, None);
    let different_header = FilterHeader::from_byte_array([8; 32]);
    // Act / Assert
    for (parent, previous_header) in [
        (hash(7), genesis.filter_header()),
        (genesis.block_hash(), different_header),
    ] {
        let child = FilterRecordIdentity::new_with_predecessor_facts(
            1,
            hash(2),
            parent,
            FilterHash::default(),
            compute_filter_header(FilterHash::default(), previous_header),
            previous_header,
            Some((0, parent, previous_header)),
        )
        .expect("local fork commitments");
        assert_eq!(
            BasicIndexProgress::new(
                IndexGeneration::new(4),
                hash(99),
                AcceptedIndexTarget::new(1, hash(2)),
                Some(child),
                Some(genesis),
                IndexInputProtection::FromHeight(1)
            ),
            Err(BasicIndexCatchUpError::InvalidProgress)
        );
    }
}

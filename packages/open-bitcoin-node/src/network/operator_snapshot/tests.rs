// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

//! Pure BASIC projection and incarnation controls.

use super::*;
use open_bitcoin_core::{
    chainstate::filter_index::lifecycle::IndexGeneration, primitives::BlockHash,
};

#[test]
fn phase161_operator_authority_zero_lag_without_earned_latch_is_catching_up() {
    use open_bitcoin_core::{
        chainstate::{FilterRecordIdentity, filter_index::catch_up::BasicIndexReplacementFacts},
        consensus::compute_filter_header,
        primitives::{FilterHash, FilterHeader},
    };
    // Arrange
    let genesis = BlockHash::from_byte_array([1; 32]);
    let old_branch = BlockHash::from_byte_array([2; 32]);
    let filter_hash = FilterHash::default();
    let previous = FilterHeader::default();
    let record = FilterRecordIdentity::new(
        0,
        genesis,
        BlockHash::default(),
        filter_hash,
        compute_filter_header(filter_hash, previous),
        previous,
        None,
    )
    .expect("genesis identity");
    let original = BasicIndexProgress::new(
        IndexGeneration::new(1),
        old_branch,
        AcceptedIndexTarget::new(1, old_branch),
        Some(record),
        Some(record),
        IndexInputProtection::FromHeight(1),
    )
    .expect("incomplete owner");
    let target = AcceptedIndexTarget::new(0, genesis);
    let progress = original
        .replace_validated_branch(BasicIndexReplacementFacts {
            expected_generation: original.generation(),
            expected_branch_identity: old_branch,
            old_target: original.accepted_target(),
            new_target: target,
            maybe_common_ancestor: Some(target),
            maybe_indexed_common: Some(record),
            maybe_shared_safe: Some(record),
            achieved_generation: IndexGeneration::new(2),
            achieved_branch_identity: genesis,
            protection: original.protection(),
        })
        .expect("replacement preserves unearned latch");
    let authority = BasicFilterOperatorAuthority {
        integrity: true,
        suspended: false,
        maybe_lifecycle: Some(IndexLifecycle::Active {
            generation: progress.generation(),
        }),
        maybe_publication: Some((
            progress.generation(),
            progress.branch_identity(),
            endpoint_checkpoint(Some(record)),
            endpoint_checkpoint(Some(record)),
            progress.protection(),
        )),
        has_state: true,
    };
    let mut status = BasicIndexOperatorStatus {
        maybe_failure: FieldAvailability::available(None),
        ..Default::default()
    };
    // Act
    project_basic_owner(
        &mut status,
        progress,
        Some(target),
        Some(target),
        &authority,
    );
    // Assert
    assert_eq!(
        status.maybe_lag_blocks,
        FieldAvailability::available(Some(0))
    );
    assert_eq!(
        status.initial_sync_complete,
        FieldAvailability::available(false)
    );
    assert_eq!(
        status.operational_state,
        FieldAvailability::available(BasicIndexOperationalState::CatchingUp)
    );
    assert_eq!(
        status.next_action,
        FieldAvailability::available(BasicIndexNextAction::InitialCatchUp)
    );
}

#[test]
fn phase161_operator_authority_protection_boundaries_never_lower_first_height() {
    // Arrange
    let last = u32::MAX - open_bitcoin_core::chainstate::PRUNE_LOCK_BUFFER;
    // Act / Assert
    assert_eq!(
        project_protection(IndexInputProtection::FromHeight(last)),
        BasicIndexProtection::InclusiveRange(
            BasicIndexProtectionRange::new(last, last).expect("exact last")
        )
    );
    for first in [last + 1, u32::MAX] {
        assert_eq!(
            project_protection(IndexInputProtection::FromHeight(first)),
            BasicIndexProtection::HeightSpaceExhausted
        );
    }
}

#[test]
fn phase161_operator_authority_detailed_failure_never_uses_coarse_pause() {
    // Arrange
    let cases = [
        (
            AcceptedBasicIndexFailure::RequiredBody,
            BasicIndexFailureCategory::RequiredBody,
            BasicIndexNextAction::RequiredBody,
        ),
        (
            AcceptedBasicIndexFailure::RequiredUndo,
            BasicIndexFailureCategory::RequiredUndo,
            BasicIndexNextAction::RequiredUndo,
        ),
        (
            AcceptedBasicIndexFailure::Persistence,
            BasicIndexFailureCategory::Backend,
            BasicIndexNextAction::Backend,
        ),
        (
            AcceptedBasicIndexFailure::InvalidFacts,
            BasicIndexFailureCategory::IntegrityOrInvalidFacts,
            BasicIndexNextAction::IntegrityOrInvalidFacts,
        ),
        (
            AcceptedBasicIndexFailure::FactLimit,
            BasicIndexFailureCategory::ResourceBound,
            BasicIndexNextAction::ResourceBound,
        ),
        (
            AcceptedBasicIndexFailure::Invalidated,
            BasicIndexFailureCategory::Invalidated,
            BasicIndexNextAction::Invalidated,
        ),
    ];
    for (owner, expected_failure, expected_action) in cases {
        // Act
        let failure = failure_category(owner);
        // Assert
        assert_eq!(failure, expected_failure);
        assert_eq!(failure_action(failure), expected_action);
    }
}

#[test]
fn phase161_operator_authority_empty_prefix_counts_genesis_and_checks_full_incarnation() {
    // Arrange
    let generation = IndexGeneration::new(1);
    let branch = BlockHash::from_byte_array([1; 32]);
    let target = AcceptedIndexTarget::new(0, branch);
    let progress = BasicIndexProgress::new(
        generation,
        branch,
        target,
        None,
        None,
        IndexInputProtection::FromHeight(0),
    )
    .expect("empty owner");
    for mismatch in 0..8 {
        let mut authority = BasicFilterOperatorAuthority {
            integrity: true,
            suspended: false,
            maybe_lifecycle: Some(IndexLifecycle::Active { generation }),
            maybe_publication: Some((
                generation,
                branch,
                endpoint_checkpoint(None),
                endpoint_checkpoint(None),
                IndexInputProtection::FromHeight(0),
            )),
            has_state: true,
        };
        let mut maybe_target = Some(target);
        let mut maybe_active_target = Some(target);
        match mismatch {
            1 => {
                maybe_target = Some(AcceptedIndexTarget::new(
                    0,
                    BlockHash::from_byte_array([2; 32]),
                ))
            }
            2 => {
                authority.maybe_lifecycle = Some(IndexLifecycle::Active {
                    generation: IndexGeneration::new(2),
                })
            }
            3 => authority.suspended = true,
            4 => authority.integrity = false,
            5 => {
                authority.maybe_publication.as_mut().expect("publication").1 =
                    BlockHash::from_byte_array([3; 32])
            }
            6 => {
                authority.maybe_publication.as_mut().expect("publication").0 =
                    IndexGeneration::new(3)
            }
            7 => maybe_active_target = Some(AcceptedIndexTarget::new(1, branch)),
            _ => {}
        }
        let mut status = BasicIndexOperatorStatus {
            maybe_failure: FieldAvailability::available(None),
            ..Default::default()
        };
        // Act
        project_basic_owner(
            &mut status,
            progress,
            maybe_target,
            maybe_active_target,
            &authority,
        );
        // Assert
        if mismatch == 0 {
            assert_eq!(
                status.maybe_processed_height,
                FieldAvailability::available(None)
            );
            assert_eq!(
                status.maybe_safe_durable_height,
                FieldAvailability::available(None)
            );
            assert_eq!(
                status.maybe_lag_blocks,
                FieldAvailability::available(Some(1))
            );
            assert_eq!(
                status.next_action,
                FieldAvailability::available(BasicIndexNextAction::EmptyPrefix)
            );
        } else {
            assert_eq!(
                status.operational_state,
                FieldAvailability::available(BasicIndexOperationalState::Invalidated)
            );
            assert!(matches!(
                status.maybe_lag_blocks,
                FieldAvailability::Unavailable { .. }
            ));
            assert!(matches!(
                status.maybe_safe_durable_height,
                FieldAvailability::Unavailable { .. }
            ));
        }
    }
}

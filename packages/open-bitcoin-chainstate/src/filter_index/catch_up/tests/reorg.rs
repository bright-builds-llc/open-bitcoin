// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

fn history() -> [FilterRecordIdentity; 4] {
    let genesis = record(0, None);
    let child = record(1, Some(genesis));
    let grandchild = record(2, Some(child));
    [genesis, child, grandchild, record(3, Some(grandchild))]
}

fn progress(maybe_processed: Option<usize>, maybe_safe: Option<usize>) -> BasicIndexProgress {
    let rows = history();
    BasicIndexProgress::new(
        IndexGeneration::new(4),
        hash(99),
        AcceptedIndexTarget::new(3, hash(4)),
        maybe_processed.map(|i| rows[i]),
        maybe_safe.map(|i| rows[i]),
        IndexInputProtection::FromHeight(0),
    )
    .expect("verified old branch")
}

fn facts(old: BasicIndexProgress, ancestor: usize) -> BasicIndexReplacementFacts {
    let rows = history();
    let maybe_indexed_common = old
        .maybe_processed_endpoint()
        .map(|p| rows[(p.height() as usize).min(ancestor)]);
    let maybe_shared_safe = old
        .maybe_safe_durable_endpoint()
        .map(|s| rows[(s.height() as usize).min(ancestor)]);
    BasicIndexReplacementFacts {
        expected_generation: old.generation(),
        expected_branch_identity: old.branch_identity(),
        old_target: old.accepted_target(),
        new_target: AcceptedIndexTarget::new(3, hash(88)),
        maybe_common_ancestor: Some(AcceptedIndexTarget::new(
            ancestor as u32,
            rows[ancestor].block_hash(),
        )),
        maybe_indexed_common,
        maybe_shared_safe,
        achieved_generation: IndexGeneration::new(5),
        achieved_branch_identity: hash(88),
        protection: IndexInputProtection::FromHeight(0),
    }
}

#[test]
fn phase158_reorg_fully_indexed_equal_height_rewinds_exact_common_prefix() {
    // Arrange
    let old = progress(Some(3), Some(3));
    // Act
    let next = old
        .replace_validated_branch(facts(old, 1))
        .expect("replacement");
    // Assert
    assert_eq!(next.maybe_processed_endpoint(), Some(history()[1]));
    assert_eq!(next.maybe_safe_durable_endpoint(), Some(history()[1]));
    assert_eq!(
        next.accepted_target(),
        AcceptedIndexTarget::new(3, hash(88))
    );
    assert_eq!(next.generation(), IndexGeneration::new(5));
    assert!(next.initially_synchronized());
    assert_eq!(next.current_lag(), 2);
}

#[test]
fn phase158_reorg_behind_ancestor_keeps_actual_processed_prefix() {
    // Arrange
    let old = progress(Some(0), Some(0));
    // Act
    let next = old
        .replace_validated_branch(facts(old, 2))
        .expect("replacement");
    // Assert
    assert_eq!(next.maybe_processed_endpoint(), Some(history()[0]));
    assert_eq!(next.maybe_safe_durable_endpoint(), Some(history()[0]));
    assert!(!next.initially_synchronized());
}

#[test]
fn phase158_reorg_within_suffix_preserves_safe_below_fork() {
    // Arrange
    let old = progress(Some(2), Some(0));
    // Act
    let next = old
        .replace_validated_branch(facts(old, 1))
        .expect("replacement");
    // Assert
    assert_eq!(next.maybe_processed_endpoint(), Some(history()[1]));
    assert_eq!(next.maybe_safe_durable_endpoint(), Some(history()[0]));
}

#[test]
fn phase158_reorg_empty_index_stays_empty() {
    // Arrange
    let old = progress(None, None);
    // Act
    let next = old
        .replace_validated_branch(facts(old, 1))
        .expect("replacement");
    // Assert
    assert_eq!(next.maybe_processed_endpoint(), None);
    assert_eq!(next.maybe_safe_durable_endpoint(), None);
    assert_eq!(next.maybe_next_height(), Ok(Some(0)));
}

#[test]
fn phase158_reorg_disconnect_only_does_not_invent_initial_sync() {
    // Arrange
    let old = progress(Some(1), Some(0));
    let mut replacement = facts(old, 1);
    replacement.new_target = AcceptedIndexTarget::new(1, hash(2));
    replacement.achieved_branch_identity = hash(2);
    // Act
    let next = old
        .replace_validated_branch(replacement)
        .expect("shorter branch");
    // Assert
    assert_eq!(next.current_lag(), 0);
    assert!(!next.initially_synchronized());
}

#[test]
fn phase158_reorg_return_to_same_branch_incarnation_renews_generation() {
    // Arrange
    let old = BasicIndexProgress {
        branch_identity: hash(2),
        ..progress(Some(3), Some(0))
    };
    let replacement = BasicIndexReplacementFacts {
        new_target: AcceptedIndexTarget::new(1, hash(2)),
        achieved_branch_identity: hash(2),
        ..facts(old, 1)
    };
    // Act
    let next = old
        .replace_validated_branch(replacement)
        .expect("disconnect to original branch endpoint");
    // Assert
    assert_eq!(next.branch_identity(), old.branch_identity());
    assert_eq!(next.generation(), IndexGeneration::new(5));
    assert_eq!(next.current_lag(), 0);
    assert!(next.initially_synchronized());
}

#[test]
fn phase158_reorg_stale_prepared_turn_refuses_without_mutation() {
    // Arrange
    let old = progress(Some(2), Some(0));
    let prepared = old.prepare_turn().expect("old turn");
    let mut next = old
        .replace_validated_branch(facts(old, 1))
        .expect("replacement");
    let before = next;
    // Act
    let result = next.complete_turn(prepared, &[history()[3]]);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::StaleWork));
    assert_eq!(next, before);
}

#[test]
fn phase158_reorg_malformed_facts_refuse_atomically() {
    // Arrange
    let old = progress(Some(3), Some(2));
    let good = facts(old, 1);
    let bad = [
        BasicIndexReplacementFacts {
            old_target: AcceptedIndexTarget::new(3, hash(9)),
            ..good
        },
        BasicIndexReplacementFacts {
            expected_branch_identity: hash(9),
            ..good
        },
        BasicIndexReplacementFacts {
            expected_generation: IndexGeneration::new(3),
            ..good
        },
        BasicIndexReplacementFacts {
            achieved_generation: IndexGeneration::new(6),
            ..good
        },
        BasicIndexReplacementFacts {
            achieved_branch_identity: hash(9),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_common_ancestor: Some(AcceptedIndexTarget::new(1, hash(9))),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_indexed_common: Some(history()[0]),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_shared_safe: Some(history()[2]),
            ..good
        },
        BasicIndexReplacementFacts {
            protection: IndexInputProtection::FromHeight(2),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_shared_safe: Some(history()[0]),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_shared_safe: None,
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_indexed_common: None,
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_common_ancestor: Some(AcceptedIndexTarget::new(4, hash(5))),
            ..good
        },
        BasicIndexReplacementFacts {
            maybe_common_ancestor: Some(AcceptedIndexTarget::new(3, hash(9))),
            ..good
        },
        BasicIndexReplacementFacts {
            new_target: AcceptedIndexTarget::new(0, hash(1)),
            achieved_branch_identity: hash(1),
            ..good
        },
    ];
    // Act / Assert
    for replacement in bad {
        assert!(old.replace_validated_branch(replacement).is_err());
        assert_eq!(old, progress(Some(3), Some(2)));
    }
}

#[test]
fn phase158_reorg_disabled_owner_refuses() {
    // Arrange
    let mut old = progress(Some(2), Some(0));
    old.disable().expect("disable");
    // Act
    let result = old.replace_validated_branch(facts(old, 1));
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::NotActive));
}

#[test]
fn phase158_reorg_generation_exhaustion_refuses_atomically() {
    // Arrange
    let old = BasicIndexProgress {
        generation: IndexGeneration::new(u64::MAX),
        ..progress(Some(2), Some(0))
    };
    // Act
    let result = old.replace_validated_branch(facts(old, 1));
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::GenerationExhausted));
    assert_eq!(old.generation().value(), u64::MAX);
}

#[test]
fn phase158_reorg_root_replacement_requires_empty_shared_progress() {
    // Arrange
    let old = progress(Some(3), Some(2));
    let replacement = BasicIndexReplacementFacts {
        maybe_common_ancestor: None,
        maybe_indexed_common: None,
        maybe_shared_safe: None,
        ..facts(old, 1)
    };
    // Act
    let next = old.replace_validated_branch(replacement).expect("new root");
    // Assert
    assert_eq!(next.maybe_next_height(), Ok(Some(0)));
    assert_eq!(next.maybe_safe_durable_endpoint(), None);
    assert!(
        old.replace_validated_branch(BasicIndexReplacementFacts {
            maybe_indexed_common: Some(history()[0]),
            ..replacement
        })
        .is_err()
    );
}

#[test]
fn phase158_reorg_empty_processed_cannot_fabricate_ancestor() {
    // Arrange
    let old = progress(None, None);
    let replacement = BasicIndexReplacementFacts {
        maybe_indexed_common: Some(history()[1]),
        ..facts(old, 1)
    };
    // Act
    let result = old.replace_validated_branch(replacement);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::InvalidProgress));
}

#[test]
fn phase158_reorg_paused_owner_resumes_with_new_generation() {
    // Arrange
    let mut old = progress(Some(2), None);
    old.pause(BasicIndexPause::Persistence);
    // Act
    let next = old
        .replace_validated_branch(facts(old, 1))
        .expect("achieved replacement");
    // Assert
    assert_eq!(next.state(), BasicIndexState::Active);
    assert_eq!(next.maybe_safe_durable_endpoint(), None);
}

#[test]
fn phase158_reorg_maximum_height_lag_remains_checked() {
    // Arrange
    let old = progress(None, None);
    let replacement = BasicIndexReplacementFacts {
        new_target: AcceptedIndexTarget::new(u32::MAX, hash(88)),
        ..facts(old, 1)
    };
    // Act
    let next = old
        .replace_validated_branch(replacement)
        .expect("maximum replacement height");
    // Assert
    assert_eq!(next.current_lag(), 1_u64 << 32);
    assert_eq!(next.maybe_next_height(), Ok(Some(0)));
}

#[test]
fn phase158_reorg_shared_adjacent_record_must_match_old_parent_and_header() {
    // Arrange
    let old = progress(Some(2), Some(0));
    let valid = history()[1];
    let changed_hash = FilterHash::from_byte_array([77; 32]);
    let altered = FilterRecordIdentity::new_with_predecessor_facts(
        1,
        valid.block_hash(),
        valid.parent_hash(),
        changed_hash,
        compute_filter_header(changed_hash, valid.previous_header()),
        valid.previous_header(),
        Some((0, valid.parent_hash(), valid.previous_header())),
    )
    .expect("locally valid changed record");
    let replacement = BasicIndexReplacementFacts {
        maybe_indexed_common: Some(altered),
        ..facts(old, 1)
    };
    // Act
    let result = old.replace_validated_branch(replacement);
    // Assert
    assert_eq!(result, Err(BasicIndexCatchUpError::InvalidProgress));
}

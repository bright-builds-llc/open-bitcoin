// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;
use crate::filter_index::lifecycle::*;

fn checkpoint() -> IndexCheckpointIdentity {
    let chain = positions();
    IndexCheckpointIdentity::new(
        FilterCheckpoint::new(IndexPrefix::Empty),
        chain[1].height,
        chain[1].block_hash,
    )
}

fn ownership(lifecycle: IndexLifecycle) -> EffectiveIndexOwnership {
    let protection = IndexInputProtection::FromHeight(0);
    EffectiveIndexOwnership::maybe_from_artifacts(
        Some(lifecycle),
        Some(checkpoint()),
        Some(protection),
        protection.maybe_prune_lock().as_ref(),
        false,
        false,
    )
    .expect("valid ownership")
    .expect("present ownership")
}

#[test]
fn disable_increments_active_generation() {
    // Arrange
    let active = IndexLifecycle::Active {
        generation: IndexGeneration::new(7),
    };
    // Act
    let disabled = active.disable();
    // Assert
    assert_eq!(
        disabled,
        Ok(IndexLifecycle::Disabled {
            generation: IndexGeneration::new(8)
        })
    );
}

#[test]
fn disabled_retry_is_idempotent_even_at_exhaustion() {
    // Arrange
    let disabled = IndexLifecycle::Disabled {
        generation: IndexGeneration::new(u64::MAX),
    };
    // Act / Assert
    assert_eq!(disabled.disable(), Ok(disabled));
}

#[test]
fn reenable_increments_disabled_generation() {
    // Arrange
    let disabled = IndexLifecycle::Disabled {
        generation: IndexGeneration::new(8),
    };
    // Act / Assert
    assert_eq!(
        disabled.enable(),
        Ok(IndexLifecycle::Active {
            generation: IndexGeneration::new(9)
        })
    );
}

#[test]
fn transitions_refuse_generation_exhaustion() {
    // Arrange
    let generation = IndexGeneration::new(u64::MAX);
    // Act / Assert
    assert_eq!(
        IndexLifecycle::Active { generation }.disable(),
        Err(IndexLifecycleError::GenerationExhausted)
    );
    assert_eq!(
        IndexLifecycle::Disabled { generation }.enable(),
        Err(IndexLifecycleError::GenerationExhausted)
    );
}

#[test]
fn legacy_absent_requires_all_artifacts_absent() {
    // Arrange
    let protection = IndexInputProtection::FromHeight(0).maybe_prune_lock();
    // Act / Assert
    assert_eq!(
        EffectiveIndexOwnership::maybe_from_artifacts(None, None, None, None, false, false),
        Ok(None)
    );
    for (maybe_owner, maybe_lock, records, projection) in [
        (
            Some(IndexLifecycle::Active {
                generation: IndexGeneration::new(0),
            }),
            None,
            false,
            false,
        ),
        (None, protection.as_ref(), false, false),
        (None, None, true, false),
        (None, None, false, true),
    ] {
        assert_eq!(
            EffectiveIndexOwnership::maybe_from_artifacts(
                maybe_owner,
                None,
                None,
                maybe_lock,
                records,
                projection
            ),
            Err(IndexLifecycleError::Integrity(
                FilterIndexError::PartialState
            ))
        );
    }
}

#[test]
fn legacy_active_is_generation_zero_with_saved_protection() {
    // Arrange
    let protection = IndexInputProtection::FromHeight(0);
    // Act
    let owner = EffectiveIndexOwnership::maybe_from_artifacts(
        None,
        Some(checkpoint()),
        Some(protection),
        protection.maybe_prune_lock().as_ref(),
        false,
        false,
    )
    .expect("legacy owner")
    .expect("active owner");
    // Assert
    assert_eq!(
        owner.lifecycle(),
        IndexLifecycle::Active {
            generation: IndexGeneration::new(0)
        }
    );
    assert_eq!(owner.maybe_effective_protection(), Some(protection));
}

#[test]
fn active_refuses_missing_or_weaker_lock() {
    // Arrange
    let active = IndexLifecycle::Active {
        generation: IndexGeneration::new(0),
    };
    let weaker = IndexInputProtection::FromHeight(1).maybe_prune_lock();
    // Act / Assert
    for maybe_lock in [None, weaker.as_ref()] {
        assert_eq!(
            EffectiveIndexOwnership::maybe_from_artifacts(
                Some(active),
                Some(checkpoint()),
                Some(IndexInputProtection::FromHeight(0)),
                maybe_lock,
                false,
                false
            ),
            Err(IndexLifecycleError::Integrity(
                FilterIndexError::WeakProtection
            ))
        );
    }
}

#[test]
fn disabled_accepts_authorized_absence_or_extra_conservative_protection() {
    // Arrange
    let disabled = IndexLifecycle::Disabled {
        generation: IndexGeneration::new(1),
    };
    let retained = ownership(disabled);
    // Act
    let released = EffectiveIndexOwnership::maybe_from_artifacts(
        Some(disabled),
        Some(checkpoint()),
        Some(IndexInputProtection::FromHeight(0)),
        None,
        false,
        false,
    )
    .expect("released")
    .expect("owner");
    // Assert
    assert_eq!(released.maybe_effective_protection(), None);
    assert_eq!(
        retained.maybe_effective_protection(),
        Some(IndexInputProtection::FromHeight(0))
    );
}

#[test]
fn exact_work_identity_accepts_only_unchanged_active_owner_and_fence() {
    // Arrange
    let active = ownership(IndexLifecycle::Active {
        generation: IndexGeneration::new(4),
    });
    let work = IndexWorkIdentity::new(active, 1, positions()[1].block_hash);
    // Act / Assert
    assert_eq!(
        active.check_work(work, 1, positions()[1].block_hash),
        Ok(())
    );
    assert_eq!(
        active.check_work(work, 1, BlockHash::default()),
        Err(IndexLifecycleError::StaleWork)
    );
    assert_eq!(
        active.check_work(work, 2, positions()[1].block_hash),
        Err(IndexLifecycleError::StaleWork)
    );
    for lifecycle in [
        IndexLifecycle::Disabled {
            generation: IndexGeneration::new(4),
        },
        IndexLifecycle::Active {
            generation: IndexGeneration::new(5),
        },
    ] {
        assert_eq!(
            ownership(lifecycle).check_work(work, 1, positions()[1].block_hash),
            Err(IndexLifecycleError::StaleWork)
        );
    }
}

#[test]
fn same_generation_newer_checkpoint_refuses_old_work() {
    // Arrange
    let old = ownership(IndexLifecycle::Active {
        generation: IndexGeneration::new(4),
    });
    let work = IndexWorkIdentity::new(old, 1, positions()[1].block_hash);
    let newer_checkpoint = IndexCheckpointIdentity::new(
        FilterCheckpoint::new(IndexPrefix::Committed(identity(&positions()[0], None))),
        1,
        positions()[1].block_hash,
    );
    let newer = EffectiveIndexOwnership::maybe_from_artifacts(
        Some(old.lifecycle()),
        Some(newer_checkpoint),
        Some(IndexInputProtection::FromHeight(0)),
        IndexInputProtection::FromHeight(0)
            .maybe_prune_lock()
            .as_ref(),
        true,
        true,
    )
    .expect("newer")
    .expect("owner");
    // Act / Assert
    assert_eq!(
        newer.check_work(work, 1, positions()[1].block_hash),
        Err(IndexLifecycleError::StaleWork)
    );
}

#[test]
fn active_enable_retry_keeps_generation() {
    // Arrange
    let active = IndexLifecycle::Active {
        generation: IndexGeneration::new(u64::MAX),
    };
    // Act / Assert
    assert_eq!(active.enable(), Ok(active));
    assert_eq!(active.generation().value(), u64::MAX);
}

#[test]
fn saved_state_requires_protection_covering_its_checkpoint() {
    // Arrange / Act / Assert
    for maybe_saved in [None, Some(IndexInputProtection::FromHeight(1))] {
        assert!(
            EffectiveIndexOwnership::maybe_from_artifacts(
                None,
                Some(checkpoint()),
                maybe_saved,
                None,
                false,
                false
            )
            .is_err()
        );
    }
    assert!(
        EffectiveIndexOwnership::maybe_from_artifacts(
            None,
            None,
            Some(IndexInputProtection::FromHeight(0)),
            None,
            false,
            false
        )
        .is_err()
    );
}

#[test]
fn disabled_still_refuses_malformed_retained_protection() {
    // Arrange
    let disabled = IndexLifecycle::Disabled {
        generation: IndexGeneration::new(1),
    };
    let mut lock = IndexInputProtection::FromHeight(0)
        .maybe_prune_lock()
        .expect("lock");
    lock.height_last = u32::MAX;
    // Act / Assert
    assert_eq!(
        EffectiveIndexOwnership::maybe_from_artifacts(
            Some(disabled),
            Some(checkpoint()),
            Some(IndexInputProtection::FromHeight(0)),
            Some(&lock),
            false,
            false
        ),
        Err(IndexLifecycleError::Integrity(
            FilterIndexError::MalformedProtection
        ))
    );
}

#[test]
fn terminal_checkpoint_allows_active_without_wrapped_lock() {
    // Arrange
    let mut terminal = identity(&positions()[0], None);
    terminal.height = u32::MAX;
    let saved = IndexCheckpointIdentity::new(
        FilterCheckpoint::new(IndexPrefix::Committed(terminal)),
        u32::MAX,
        terminal.block_hash(),
    );
    // Act
    let owner = EffectiveIndexOwnership::maybe_from_artifacts(
        None,
        Some(saved),
        Some(IndexInputProtection::HeightSpaceExhausted),
        None,
        true,
        true,
    )
    .expect("terminal")
    .expect("owner");
    // Assert
    assert_eq!(
        owner.maybe_effective_protection(),
        Some(IndexInputProtection::HeightSpaceExhausted)
    );
    assert_eq!(
        owner.saved_protection(),
        IndexInputProtection::HeightSpaceExhausted
    );
    assert_eq!(owner.checkpoint().fence_height(), u32::MAX);
    assert_eq!(owner.checkpoint().fence_hash(), terminal.block_hash());
}

#[test]
fn same_generation_changed_fence_or_lock_refuses_work() {
    // Arrange
    let chain = positions();
    let saved = IndexCheckpointIdentity::new(
        FilterCheckpoint::new(IndexPrefix::Committed(identity(&chain[0], None))),
        1,
        chain[1].block_hash,
    );
    let lifecycle = IndexLifecycle::Active {
        generation: IndexGeneration::new(4),
    };
    let current = EffectiveIndexOwnership::maybe_from_artifacts(
        Some(lifecycle),
        Some(saved),
        Some(IndexInputProtection::FromHeight(1)),
        IndexInputProtection::FromHeight(1)
            .maybe_prune_lock()
            .as_ref(),
        true,
        true,
    )
    .expect("current")
    .expect("owner");
    let work = IndexWorkIdentity::new(current, 1, chain[1].block_hash);
    let other_fence = IndexCheckpointIdentity::new(saved.checkpoint(), 1, BlockHash::default());
    // Act / Assert
    for (checkpoint, protection) in [
        (other_fence, IndexInputProtection::FromHeight(1)),
        (saved, IndexInputProtection::FromHeight(0)),
    ] {
        let changed = EffectiveIndexOwnership::maybe_from_artifacts(
            Some(lifecycle),
            Some(checkpoint),
            Some(IndexInputProtection::FromHeight(1)),
            protection.maybe_prune_lock().as_ref(),
            true,
            true,
        )
        .expect("changed")
        .expect("owner");
        assert_eq!(
            changed.check_work(work, 1, chain[1].block_hash),
            Err(IndexLifecycleError::StaleWork)
        );
    }
}

#[test]
fn ownership_protects_genesis_and_height_one_directly() {
    // Arrange
    let owner = ownership(IndexLifecycle::Active {
        generation: IndexGeneration::new(0),
    });
    let protection = owner
        .maybe_effective_protection()
        .expect("active protection");
    // Act / Assert
    for height in [0, 1] {
        assert_eq!(
            protection.check_prune_intent(height),
            Err(FilterIndexError::UnsafePruneIntent)
        );
    }
}

#[test]
fn lifecycle_refusals_have_bounded_diagnostics() {
    // Arrange / Act / Assert
    assert_eq!(
        IndexLifecycleError::GenerationExhausted.to_string(),
        "BASIC index generation exhausted"
    );
    assert_eq!(
        IndexLifecycleError::StaleWork.to_string(),
        "stale or disabled BASIC index work"
    );
    assert_eq!(
        IndexLifecycleError::Integrity(FilterIndexError::PartialState).to_string(),
        FilterIndexError::PartialState.to_string()
    );
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/node/chainstate.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp
// - packages/bitcoin-knots/src/validation.cpp

use super::*;

#[test]
fn execute_flush_none_does_not_write_coins() {
    // Arrange
    let (path, store) = open_temp_store("none-no-write");
    let tip = BlockHash::from_byte_array([7_u8; 32]);
    let outpoint = OutPoint {
        txid: Txid::from_byte_array([1_u8; 32]),
        vout: 0,
    };
    let mut planted = FjallCoinsView::from_store(&store);
    planted
        .batch_write(dirty_unspent_batch(&[(outpoint, sample_coin())]), Some(tip))
        .expect("plant consistent tip");
    let (mut lifecycle, view, mut cache) = initialize_ready(&store);
    let before = view.best_block().expect("best before");

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::IfNeeded,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("none flush");

    // Assert
    assert!(matches!(execution.decision, FlushDecision::None(_)));
    assert!(!execution.wrote_coins);
    assert_eq!(view.best_block().expect("best after"), before);
    assert_eq!(before, Some(tip));
    remove_dir_if_exists(&path);
}

#[test]
fn execute_flush_aborts_coins_when_undo_save_fails() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();
    let undo_hash = BlockHash::from_byte_array([0x22; 32]);

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut UndoFailingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[(undo_hash, BlockUndo::default())],
            &[],
            &[],
            &[],
        ),
        "undo persist aborts",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::BackendFailure {
                ref message,
                ..
            } if message == "undo persist failed"
        ),
        "expected undo persist failure, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_always_uses_cache_flush_kind_from_decide_flush() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    lifecycle.set_next_write(policy_now());
    let mut cache = dirty_recording_cache();

    // Act
    let execution = lifecycle
        .execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        )
        .expect("always flush");

    // Assert
    assert!(
        matches!(execution.decision, FlushDecision::Flush(_)),
        "Always must map to cache Flush, got {:?}",
        execution.decision
    );
    assert!(execution.wrote_coins);
    assert_eq!(cache.parent().writes.get(), 1);
}

#[test]
fn execute_flush_before_ready_does_not_write() {
    // Arrange
    let mut lifecycle = FlushLifecycle::not_ready_for_test();
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            u64::MAX,
            &[],
            &[],
            &[],
            &[],
        ),
        "flush before ready",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "flush before ready"
        ),
        "expected flush before ready, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

#[test]
fn execute_flush_refuse_disk_space_does_not_write_coins() {
    // Arrange
    let mut lifecycle =
        FlushLifecycle::ready_for_test(default_coins_cache_byte_limit(), 0, policy_now(), false);
    let mut cache = dirty_recording_cache();

    // Act
    let error = expect_error(
        lifecycle.execute_flush(
            &mut SucceedingSink,
            &mut cache,
            FlushMode::Always,
            policy_now(),
            0,
            &[],
            &[],
            &[],
            &[],
        ),
        "refuse disk space",
    );

    // Assert
    assert!(
        matches!(
            error,
            StorageError::Corruption {
                namespace: StorageNamespace::Coins,
                ref detail,
                action: StorageRecoveryAction::Repair,
            } if detail == "refuse disk space"
        ),
        "expected refuse disk space, got {error:?}"
    );
    assert_eq!(cache.parent().writes.get(), 0);
}

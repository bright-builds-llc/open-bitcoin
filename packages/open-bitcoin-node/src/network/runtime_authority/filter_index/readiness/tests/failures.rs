// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

use super::*;

#[test]
fn phase159_basic_readiness_owner_direct_relay_poison_wakes_before_repoll() {
    // Arrange
    let history = ValidatedHistory::new("readiness-direct-relay-poison", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&count));
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    runtime.network.poison_for_test();
    // Act
    assert!(runtime.network.prepare_peer_relay_effect(1).is_err());
    // Assert
    assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::AuthorityUnavailable
        )))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_poisoned_authority_wakes_typed_failure() {
    // Arrange
    let history = ValidatedHistory::new("readiness-authority-poison", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&count));
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    runtime.network.poison_for_test();
    // Act
    assert!(runtime.network.maybe_basic_index_summary().is_err());
    // Assert
    assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::AuthorityUnavailable
        )))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_invalid_historical_body_pauses_and_wakes() {
    // Arrange
    let fixture = TurnHistory::new(1, 4, 1);
    let runtime = configured(&fixture.history);
    accept(&runtime, &fixture.history.blocks[2]);
    accept(&runtime, &fixture.history.blocks[3]);
    let hash = fixture.history.records[3].identity().block_hash();
    let mut barrier = pending(&runtime, hash);
    let key = codec::record_key(hash).replacen(codec::RECORD_PREFIX, "block:", 1);
    runtime
        .store()
        .write_raw_for_test(StorageNamespace::BlockIndex, &key, vec![0])
        .expect("corrupt required body");
    // Act
    assert!(runtime.network.drive_basic_filter_index_turn().is_err());
    // Assert
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::OwnerFailed
        )))
    ));
    assert_eq!(
        runtime
            .network
            .maybe_basic_index_progress()
            .expect("read")
            .expect("owner")
            .state(),
        open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexState::Paused(
            open_bitcoin_core::chainstate::filter_index::catch_up::BasicIndexPause::MissingHistory
        )
    );
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_completion_rejects_a_different_authority_incarnation() {
    // Arrange
    let history = ValidatedHistory::new("readiness-incarnation-a", false);
    let other_history = ValidatedHistory::new("readiness-incarnation-b", false);
    let runtime = configured(&history);
    let other = configured(&other_history);
    accept(&runtime, &history.blocks[2]);
    accept(&other, &other_history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let mut barrier = pending(&runtime, hash);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("original owner");
    other
        .network
        .drive_basic_filter_index_turn()
        .expect("other owner");
    // Act
    let result = other
        .network
        .complete_basic_filter_read(hash, ready(&mut barrier));
    // Assert
    assert!(matches!(
        result,
        Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Invalidated
        ))
    ));
    drop(runtime);
    drop(other);
    history.cleanup();
    other_history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_publication_faults_wake_terminal_failure() {
    use crate::storage::fjall_store::filters::FilterPublicationFault;
    for fault in [
        FilterPublicationFault::BeforeRecords,
        FilterPublicationFault::BeforeCheckpoint,
        FilterPublicationFault::BeforeProtection,
        FilterPublicationFault::AfterCommit,
    ] {
        // Arrange
        let history = ValidatedHistory::new("readiness-fault", false);
        let runtime = configured(&history);
        accept(&runtime, &history.blocks[2]);
        let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
        let count = Arc::new(WakeCount::default());
        let waker = Waker::from(Arc::clone(&count));
        assert!(
            Pin::new(&mut barrier)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        runtime.store().set_basic_filter_fault(fault);
        // Act
        assert!(runtime.network.drive_basic_filter_index_turn().is_err());
        // Assert
        assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert!(matches!(
            poll(&mut barrier),
            Poll::Ready(Err(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::OwnerFailed
            )))
        ));
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn phase159_basic_readiness_owner_accepted_before_persistence_failure_settles() {
    use crate::storage::fjall_store::validation_history::HistoryPublicationFault;
    for fault in [
        HistoryPublicationFault::BeforeCommit,
        HistoryPublicationFault::AfterCommit,
    ] {
        // Arrange
        let history = ValidatedHistory::new("readiness-history-fault", false);
        let runtime = configured(&history);
        runtime.store().set_validation_history_fault(fault);
        // Act
        assert!(
            runtime
                .network
                .connect_local_block(
                    &history.blocks[2],
                    open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
                    open_bitcoin_core::consensus::ConsensusParams {
                        coinbase_maturity: 1,
                        ..Default::default()
                    }
                )
                .is_err()
        );
        let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
        // Assert
        assert!(matches!(
            poll(&mut barrier),
            Poll::Ready(Err(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::OwnerFailed
            )))
        ));
        drop(runtime);
        history.cleanup();
    }
}

#[test]
fn phase159_basic_readiness_owner_explicit_stop_and_last_handle_drop_settle() {
    for explicit in [true, false] {
        // Arrange
        let history = ValidatedHistory::new("readiness-stop", false);
        let runtime = configured(&history);
        accept(&runtime, &history.blocks[2]);
        let hash = history.records[2].identity().block_hash();
        let mut barrier = pending(&runtime, hash);
        let count = Arc::new(WakeCount::default());
        let waker = Waker::from(Arc::clone(&count));
        assert!(
            Pin::new(&mut barrier)
                .poll(&mut Context::from_waker(&waker))
                .is_pending()
        );
        // Act
        if explicit {
            runtime
                .network
                .stop_basic_filter_readiness()
                .expect("owner shutdown");
            assert!(matches!(
                runtime.network.basic_filter_query(hash),
                Err(BasicFilterQueryError::Readiness(
                    BasicFilterReadFailure::OwnerStopped
                ))
            ));
        }
        drop(runtime);
        // Assert
        assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
        assert!(matches!(
            poll(&mut barrier),
            Poll::Ready(Err(BasicFilterQueryError::Readiness(
                BasicFilterReadFailure::OwnerStopped
            )))
        ));
        history.cleanup();
    }
}

#[test]
fn phase159_basic_readiness_owner_final_read_rejects_raw_integrity_invalidation() {
    // Arrange
    let history = ValidatedHistory::new("readiness-final-integrity", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let mut barrier = pending(&runtime, hash);
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("owner");
    let completion = ready(&mut barrier);
    runtime
        .store()
        .remove_validation_filter_for_test(hash)
        .expect("raw clone invalidates");
    // Act
    let result = runtime.network.complete_basic_filter_read(hash, completion);
    // Assert
    assert!(matches!(
        result,
        Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Invalidated
        ))
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_reorg_invalidates_exact_branch_and_keeps_latch() {
    use crate::sync::tests::filter_index::reorg::fixtures::ForkFixture;
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 14, 0x61);
    fixture.apply(&branch);
    let hash = branch.records.last().expect("tip").identity().block_hash();
    let mut barrier = pending(&fixture.runtime, hash);
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&count));
    assert!(
        Pin::new(&mut barrier)
            .poll(&mut Context::from_waker(&waker))
            .is_pending()
    );
    // Retain the first branch's actual accepted undo before the next reorg.
    let snapshot = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("accepted branch");
    for (hash, undo) in &snapshot.undo_by_block {
        fixture
            .runtime
            .store()
            .save_undo(*hash, undo, PersistMode::Sync)
            .expect("retain actual undo");
    }
    let next = fixture.fork(10, 13, 0x62);
    // Act
    fixture.apply(&next);
    // Assert
    assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(
        poll(&mut barrier),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Invalidated
        )))
    ));
    assert!(
        fixture
            .runtime
            .network
            .maybe_basic_index_summary()
            .expect("summary")
            .expect("enabled")
            .synced
    );
    let history = fixture.history;
    drop(fixture.runtime);
    history.cleanup();
}

// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp

//! Readiness earned by the ordinary owner over actual accepted, unflushed work.

use super::*;
use crate::{BasicFilterQuery, BasicFilterQueryError, BasicFilterReadFailure};
use std::{
    future::Future,
    pin::Pin,
    sync::Arc,
    task::{Context, Poll, Wake, Waker},
};

#[path = "tests/failures.rs"]
mod failures;
#[path = "tests/interleavings.rs"]
mod interleavings;

#[derive(Default)]
struct WakeCount(std::sync::atomic::AtomicUsize);
impl Wake for WakeCount {
    fn wake(self: Arc<Self>) {
        self.wake_by_ref();
    }
    fn wake_by_ref(self: &Arc<Self>) {
        self.0.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
}

fn pending(runtime: &DurableSyncRuntime, hash: BlockHash) -> crate::BasicFilterReadBarrier {
    let query = runtime.network.basic_filter_query(hash).expect("query");
    let BasicFilterQuery::Pending(barrier) = query else {
        panic!("pending: {query:?}");
    };
    barrier
}

fn accept(runtime: &DurableSyncRuntime, block: &open_bitcoin_core::primitives::Block) {
    runtime
        .network
        .connect_local_block(
            block,
            open_bitcoin_core::consensus::ScriptVerifyFlags::P2SH,
            open_bitcoin_core::consensus::ConsensusParams {
                coinbase_maturity: 1,
                ..Default::default()
            },
        )
        .expect("genuine acceptance");
}

fn poll(
    barrier: &mut crate::BasicFilterReadBarrier,
) -> Poll<Result<crate::BasicFilterReadCompletion, BasicFilterQueryError>> {
    Pin::new(barrier).poll(&mut Context::from_waker(Waker::noop()))
}

fn configured(history: &ValidatedHistory) -> DurableSyncRuntime {
    DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime")
}

fn ready(barrier: &mut crate::BasicFilterReadBarrier) -> crate::BasicFilterReadCompletion {
    let Poll::Ready(Ok(completion)) = poll(barrier) else {
        panic!("earned completion");
    };
    completion
}

#[test]
fn phase159_basic_readiness_capacity_cancellation_and_checked_ids() {
    // Arrange
    let history = ValidatedHistory::new("readiness-capacity", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let mut barriers = (0..crate::MAX_BASIC_FILTER_WAITERS)
        .map(|_| pending(&runtime, hash))
        .collect::<Vec<_>>();
    // Act / Assert
    assert!(matches!(
        runtime.network.basic_filter_query(hash),
        Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Capacity
        ))
    ));
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 64);
    barriers.truncate(32);
    let extra = pending(&runtime, hash);
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 33);
    drop(extra);
    drop(barriers);
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 0);
    runtime
        .network
        .set_basic_filter_waiter_id_for_test(u64::MAX);
    assert!(matches!(
        runtime.network.basic_filter_query(hash),
        Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::CounterExhausted
        ))
    ));
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 0);
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_completion_before_poll_has_no_lost_wake() {
    // Arrange
    let history = ValidatedHistory::new("readiness-no-lost-wake", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let mut barrier = pending(&runtime, hash);
    // Act
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("owner");
    // Assert
    let completion = ready(&mut barrier);
    assert_eq!(runtime.network.basic_filter_waiter_count_for_test(), 0);
    assert!(matches!(
        runtime
            .network
            .complete_basic_filter_read(hash, completion)
            .expect("final read"),
        BasicFilterQuery::Found(_)
    ));
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_original_target_does_not_chase_later_acceptance() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::{BasicIndexTurnBudget, TurnWork};
    // Arrange
    let fixture = TurnHistory::new(1, 4, 1);
    let runtime = configured(&fixture.history);
    accept(&runtime, &fixture.history.blocks[2]);
    let hash = fixture.history.records[2].identity().block_hash();
    let mut first = pending(&runtime, hash);
    accept(&runtime, &fixture.history.blocks[3]);
    let mut second = pending(&runtime, hash);
    let work = TurnWork {
        blocks: 1,
        ..injected_budget().normal()
    };
    let budget = BasicIndexTurnBudget::new(work, work).expect("single actual owner block");
    // Act
    runtime
        .network
        .drive_basic_filter_index_turn_with_budget(budget)
        .expect("owner reaches first only");
    // Assert
    assert_eq!(first.accepted_height(), 2);
    assert_eq!(second.accepted_height(), 3);
    assert!(poll(&mut second).is_pending());
    let completion = ready(&mut first);
    assert!(matches!(
        runtime
            .network
            .complete_basic_filter_read(hash, completion)
            .expect("no newer capture"),
        BasicFilterQuery::Found(_)
    ));
    assert!(
        runtime
            .network
            .maybe_basic_index_summary()
            .expect("independent summary")
            .expect("enabled")
            .synced
    );
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("next scheduled turn");
    ready(&mut second);
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_wrong_hash_token_rejected() {
    // Arrange
    let history = ValidatedHistory::new("readiness-wrong-hash", false);
    let runtime = configured(&history);
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("owner");
    // Act
    let result = runtime.network.complete_basic_filter_read(
        history.records[1].identity().block_hash(),
        ready(&mut barrier),
    );
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
fn phase159_basic_readiness_owner_connected_during_wait_keeps_request_provenance() {
    use open_bitcoin_core::chainstate::filter_index::catch_up::{BasicIndexTurnBudget, TurnWork};
    // Arrange: height 3 is known historical recipe, never accepted by this manager.
    let fixture = TurnHistory::new(1, 4, 1);
    let runtime = configured(&fixture.history);
    accept(&runtime, &fixture.history.blocks[2]);
    let hash = fixture.history.records[3].identity().block_hash();
    let provenance = runtime
        .network
        .basic_filter_query(hash)
        .expect("request-time query");
    let BasicFilterQuery::Pending(mut barrier) = provenance else {
        panic!("captured wait");
    };
    accept(&runtime, &fixture.history.blocks[3]);
    let work = TurnWork {
        blocks: 1,
        ..injected_budget().normal()
    };
    // Act: ordinary owner processes only the originally captured height 2.
    runtime
        .network
        .drive_basic_filter_index_turn_with_budget(
            BasicIndexTurnBudget::new(work, work).expect("budget"),
        )
        .expect("owner");
    let result = runtime
        .network
        .complete_basic_filter_read(hash, ready(&mut barrier))
        .expect("final read");
    // Assert: raw-seeded known history remains honestly legacy at request time.
    assert!(matches!(
        result,
        BasicFilterQuery::Missing {
            provenance: crate::BasicBlockValidationProvenance::UnknownLegacy,
            initially_synchronized: true
        }
    ));
    drop(runtime);
    fixture.history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_unflushed_completion_and_final_read() {
    // Arrange
    let history = ValidatedHistory::new("readiness-unflushed", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    accept(&runtime, &history.blocks[2]);
    let hash = history.records[2].identity().block_hash();
    let mut barrier = pending(&runtime, hash);
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&count));
    let mut context = Context::from_waker(&waker);
    assert!(Pin::new(&mut barrier).poll(&mut context).is_pending());
    // Act
    runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("ordinary owner");
    // Assert
    assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    let Poll::Ready(Ok(completion)) = Pin::new(&mut barrier).poll(&mut context) else {
        panic!("earned ready");
    };
    assert!(matches!(
        runtime
            .network
            .complete_basic_filter_read(hash, completion)
            .expect("checked read"),
        BasicFilterQuery::Found(_)
    ));
    let progress = runtime
        .network
        .maybe_basic_index_progress()
        .expect("read")
        .expect("owner");
    assert_eq!(
        progress
            .maybe_processed_endpoint()
            .expect("processed")
            .height(),
        2
    );
    assert_eq!(
        progress
            .maybe_safe_durable_endpoint()
            .expect("durable")
            .height(),
        1
    );
    drop(runtime);
    history.cleanup();
}

#[test]
fn phase159_basic_readiness_owner_disable_wakes_and_cancels() {
    // Arrange
    let history = ValidatedHistory::new("readiness-disable", false);
    let runtime = DurableSyncRuntime::open_configured(
        history.seed(1),
        sync_config(),
        crate::chainstate::BasicFilterStartupMode::Enabled,
    )
    .expect("runtime");
    accept(&runtime, &history.blocks[2]);
    let mut barrier = pending(&runtime, history.records[2].identity().block_hash());
    let count = Arc::new(WakeCount::default());
    let waker = Waker::from(Arc::clone(&count));
    let mut context = Context::from_waker(&waker);
    assert!(Pin::new(&mut barrier).poll(&mut context).is_pending());
    // Act
    runtime
        .network
        .disable_basic_filter_index()
        .expect("disable");
    // Assert
    assert_eq!(count.0.load(std::sync::atomic::Ordering::SeqCst), 1);
    assert!(matches!(
        Pin::new(&mut barrier).poll(&mut context),
        Poll::Ready(Err(BasicFilterQueryError::Readiness(
            BasicFilterReadFailure::Invalidated
        )))
    ));
    drop(runtime);
    history.cleanup();
}

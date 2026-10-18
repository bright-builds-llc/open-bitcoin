// Parity breadcrumbs:
// - packages/bitcoin-knots/src/index/base.cpp
// - packages/bitcoin-knots/src/index/blockfilterindex.cpp
// - packages/bitcoin-knots/src/node/blockstorage.cpp

use super::*;

#[test]
fn phase158_validated_reorg_weaker_shorter_refuses_without_effects() {
    // Arrange
    let fixture = ForkFixture::compact(16, 8, 24);
    let branch = fixture.fork(10, 4, 0x61);
    let before = fixture
        .runtime
        .network
        .chainstate_snapshot()
        .expect("before");
    let progress = fixture
        .runtime
        .network
        .maybe_basic_index_progress()
        .expect("before");
    let checkpoint_before = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("before");
    // Act
    let result = fixture.runtime.network.reorg_to_branch(
        &branch.disconnect,
        &branch.connected,
        ReorgLifecycleContext::new(PolicyTime::from_unix_seconds(50_000)),
        fixture.runtime.verify_flags,
        fixture.runtime.consensus_params,
    );
    // Assert
    assert!(matches!(
        result,
        Err(crate::ManagedNetworkAuthorityError::Operation(
            crate::ManagedNetworkError::Chainstate(
                open_bitcoin_core::chainstate::ChainstateError::BranchNotPreferred
            )
        ))
    ));
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("after"),
        before
    );
    assert_eq!(
        fixture
            .runtime
            .network
            .maybe_basic_index_progress()
            .expect("after"),
        progress
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("after"),
        checkpoint_before
    );
    for record in &fixture.history.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable retained row"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

fn explicit_disconnect_case(retained: usize) {
    // Arrange: a deliberate operator action has no candidate work-preference claim.
    let fixture = ForkFixture::compact(16, 8, 24);
    let old_tip = fixture.history.records[23].identity();
    let checkpoint_before = fixture
        .runtime
        .store()
        .maybe_basic_filter_checkpoint()
        .expect("old checkpoint");
    // Act
    for block in fixture.history.blocks[retained..].iter().rev() {
        fixture
            .runtime
            .network
            .disconnect_fixture_tip(block)
            .expect("explicit real managed disconnect");
    }
    let turn = fixture
        .runtime
        .network
        .drive_basic_filter_index_turn()
        .expect("paused invalidated maintenance");
    // Assert: existing durable projection survives; it cannot be relabeled as accepted empty history.
    assert_eq!(
        fixture
            .runtime
            .network
            .chainstate_snapshot()
            .expect("actual active state")
            .active_chain
            .len(),
        retained
    );
    assert_eq!(
        turn.maybe_accepted_target
            .expect("saved stale target")
            .block_hash(),
        old_tip.block_hash()
    );
    assert_eq!(turn.work.blocks, 0);
    assert_eq!(
        fixture
            .runtime
            .store()
            .maybe_basic_filter_checkpoint()
            .expect("physical checkpoint"),
        checkpoint_before
    );
    assert_eq!(
        fixture
            .runtime
            .store()
            .coins_view()
            .best_block()
            .expect("unflushed original coins fence"),
        Some(old_tip.block_hash())
    );
    for record in &fixture.history.records {
        assert_eq!(
            fixture
                .runtime
                .store()
                .load_basic_filter_record(record.identity().block_hash())
                .expect("immutable retained row"),
            Some(record.clone())
        );
        assert_eq!(
            fixture
                .runtime
                .store()
                .maybe_active_basic_filter_record(record.identity().height())
                .expect("unchanged physical projection"),
            Some(record.clone())
        );
    }
    fixture.cleanup();
}

#[test]
fn phase158_validated_reorg_explicit_disconnect_to_ancestor_invalidates_basic_without_fabrication()
{
    explicit_disconnect_case(11);
}

#[test]
fn phase158_validated_reorg_explicit_full_disconnect_preserves_physical_checkpoint() {
    explicit_disconnect_case(0);
}
